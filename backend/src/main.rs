#[allow(warnings, unused)]
mod db;
pub mod firebase;
mod schema;

use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Result;
use async_graphql::http::GraphiQLSource;
use async_graphql::{extensions, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Extension, Router, Server};
use axum_auth::AuthBearer;
use dotenv::dotenv;
use prisma_client_rust::prisma_errors::query_engine::{RecordNotFound, UniqueKeyViolation};
use prisma_client_rust::QueryError;
use schema::{MutationRoot, QueryRoot};
use std::env;

use crate::firebase::FirebaseConfig;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    dotenv()?;

    let firebase_config = FirebaseConfig::new(env::var("FIREBASE_PROJECT_ID")?);

    let db = Arc::new(db::new_client().await?);

    tracing::info!("Connected to database");

    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .extension(extensions::Logger)
    .extension(extensions::Tracing)
    .finish();

    // sdl
    tracing::info!("{}", schema.sdl());

    let app = Router::new()
        .route("/graphiql", get(graphiql_handler))
        .route("/graphql", post(graphql_handler))
        .layer(Extension(firebase_config))
        .layer(Extension(db))
        .layer(Extension(schema));

    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 4001));

    tracing::info!("Listening on {}", addr);
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}

async fn graphiql_handler() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

pub type Database = Arc<db::PrismaClient>;

async fn graphql_handler(
    user: AuthenticatedUser,
    Extension(schema): Extension<schema::Schema>,
    Extension(db): Extension<Arc<db::PrismaClient>>,
    Extension(firebase_config): Extension<FirebaseConfig>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = req.into_inner();
    request = request.data(user);
    request = request.data(db);
    request = request.data(firebase_config);
    schema.execute(request).await.into()
}

pub struct AuthenticatedUser(Option<db::user::Data>);

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = Response;
    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        use firebase::auth::verify;
        let Ok(AuthBearer(token)) = AuthBearer::from_request_parts(req, state).await else {
            tracing::info!("AuthBearer not found");
            return Ok(Self(None));
        };

        let Some(firebase_config) = req.extensions.get::<FirebaseConfig>() else {
            tracing::info!("firebase_config not found");
            return Ok(Self(None));
        };

        let db = req
            .extensions
            .get::<Arc<db::PrismaClient>>()
            .ok_or(AppError::InternalServerError.into_response())?;

        tracing::info!("token: {}", token);
        let Ok(claims) = verify(firebase_config, &token).await else {
            tracing::info!("verify failed");
            return Ok(Self(None));
        };

        tracing::info!("uid: {}", claims.sub.clone());
        Ok(Self(
            db.user()
                .find_unique(db::user::uid::equals(claims.sub.clone()))
                .exec()
                .await
                .map_err(|e| AppError::from(e).into_response())?,
        ))
    }
}

type AppResult<T> = Result<T, AppError>;

enum AppError {
    PrismaError(QueryError),
    NotFound,
    InternalServerError,
    BadRequest,
    Forbidden,
}

impl From<QueryError> for AppError {
    fn from(error: QueryError) -> Self {
        match error {
            e if e.is_prisma_error::<RecordNotFound>() => AppError::NotFound,
            e => AppError::PrismaError(e),
        }
    }
}

// Error to response mapping
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::PrismaError(error) if error.is_prisma_error::<UniqueKeyViolation>() => {
                StatusCode::CONFLICT
            }
            AppError::PrismaError(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest => StatusCode::BAD_REQUEST,
            AppError::Forbidden => StatusCode::FORBIDDEN,
        };

        status.into_response()
    }
}
