use async_graphql::http::GraphiQLSource;
use axum::response::{Html, IntoResponse};

pub async fn graphiql_handler() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}
