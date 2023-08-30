mod guard;
mod project;
mod user;

use async_graphql::EmptySubscription;
use async_graphql::MergedObject;
use project::{mutation::ProjectMutation, query::ProjectQuery};
use user::{mutation::UserMutation, query::UserQuery};

#[derive(MergedObject, Default)]
pub struct MutationRoot(ProjectMutation, UserMutation);

#[derive(MergedObject, Default)]
pub struct QueryRoot(ProjectQuery, UserQuery);

// GraphQL schema
pub type Schema = async_graphql::Schema<QueryRoot, MutationRoot, EmptySubscription>;
