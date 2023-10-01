#[allow(warnings, unused)]
mod db;
pub mod firebase_auth;
mod prisma;
mod schema;
mod services;

use anyhow::Result;
use async_graphql::{extensions, http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use async_trait::async_trait;
use axum::{
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Extension, Router, Server,
};
use axum_auth::AuthBearer;
use dotenv::dotenv;
use prisma_client_rust::{
    prisma_errors::query_engine::{RecordNotFound, UniqueKeyViolation},
    QueryError,
};
use schema::{MutationRoot, QueryRoot};
use shaku::HasProvider;
use std::env;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;

use crate::firebase_auth::FirebaseAuth;
use crate::services::{user::UserService, Injector};

#[derive(Clone)]
pub struct AppState {
    pub module: Arc<Injector>,
    pub schema: schema::Schema,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    dotenv()?;

    let firebase_auth = FirebaseAuth::new(env::var("FIREBASE_PROJECT_ID")?);

    tracing::info!("Connected to database");

    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .extension(extensions::Logger)
    .extension(extensions::Tracing)
    .finish();

    tracing::info!("{}", schema.sdl());

    let state = Arc::new(AppState {
        module: Arc::new(
            Injector::create()
                .await
                .expect("Failed to create the dependency injector"),
        ),
        schema,
    });

    let app = Router::new()
        .route("/graphiql", get(graphiql_handler))
        .route("/graphql", post(graphql_handler))
        .layer(Extension(firebase_auth))
        .with_state(state);

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
    State(state): State<Arc<AppState>>,
    user: AuthenticatedUser,
    Extension(firebase_auth): Extension<FirebaseAuth>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let state = Arc::from_ref(&state);
    let mut request = req.into_inner();
    request = request.data(firebase_auth);
    request = request.data(user);
    request = request.data(state.module.clone());
    state.schema.execute(request).await.into()
}

pub struct AuthenticatedUser(Option<db::user::Data>);

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    Arc<AppState>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Response;
    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Ok(AuthBearer(token)) = AuthBearer::from_request_parts(req, state).await else {
            tracing::info!("AuthBearer not found");
            return Ok(Self(None));
        };

        let Some(firebase_auth) = req.extensions.get::<FirebaseAuth>() else {
            tracing::info!("firebase_auth not found");
            return Ok(Self(None));
        };

        tracing::info!("token: {}", token);
        let Ok(claims) = firebase_auth.verify(&token).await else {
            tracing::info!("verify failed");
            return Ok(Self(None));
        };

        let state = Arc::from_ref(state);
        let module: Box<dyn UserService> = state
            .module
            .provide()
            .map_err(|_| AppError::InternalServerError.into_response())?;

        match module.get_user_by_uid(claims.sub.clone()).await {
            Ok(Some(user)) => Ok(AuthenticatedUser(Some(user))),
            Ok(None) => Ok(AuthenticatedUser(None)),
            Err(_) => Ok(AuthenticatedUser(None)),
        }
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
