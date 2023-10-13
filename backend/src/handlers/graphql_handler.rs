use std::sync::Arc;

use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::{FromRef, State};

use crate::{AppState, AuthenticatedUser};

pub async fn graphql_handler(
    State(state): State<Arc<AppState>>,
    user: AuthenticatedUser,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let state = Arc::from_ref(&state);
    let mut request = req.into_inner();
    request = request.data(user);
    state.schema.execute(request).await.into()
}
