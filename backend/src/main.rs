#[allow(warnings, unused)]
mod db;
mod error;
pub mod firebase_auth;
mod handlers;
mod prisma;
mod schema;
mod services;

use anyhow::Result;
use async_graphql::{extensions, EmptySubscription, Schema};
use async_trait::async_trait;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router, Server,
};
use axum_auth::AuthBearer;
use dotenv::dotenv;
use schema::{MutationRoot, QueryRoot};
use shaku::HasProvider;
use std::env;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;

use crate::error::AppError;
use crate::firebase_auth::FirebaseAuth;
use crate::handlers::{graphiql_handler::graphiql_handler, graphql_handler::graphql_handler};
use crate::services::{user::UserService, Injector};

#[derive(Clone)]
pub struct AppState {
    pub module: Arc<Injector>,
    pub schema: schema::Schema,
    pub firebase_auth: FirebaseAuth,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    dotenv()?;

    let firebase_auth = FirebaseAuth::new(env::var("FIREBASE_PROJECT_ID")?);

    tracing::info!("Connected to database");

    let module = Arc::new(
        Injector::create()
            .await
            .expect("Failed to create the dependency injector"),
    );

    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .extension(extensions::Logger)
    .extension(extensions::Tracing)
    .data(module.to_owned())
    .data(firebase_auth.to_owned())
    .finish();

    tracing::info!("{}", schema.sdl());

    let state = Arc::new(AppState {
        module,
        schema,
        firebase_auth,
    });

    let app = Router::new()
        .route("/graphiql", get(graphiql_handler))
        .route("/graphql", post(graphql_handler))
        .with_state(state);

    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 4001));

    tracing::info!("Listening on {}", addr);
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}

pub type Database = Arc<db::PrismaClient>;
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
        let state = Arc::from_ref(state);

        let firebase_auth = state.firebase_auth.to_owned();
        tracing::info!("token: {}", token);
        let Ok(claims) = firebase_auth.verify(&token).await else {
            tracing::info!("verify failed");
            return Ok(Self(None));
        };

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
