use async_graphql::{Context, Object, OneofObject, Result, ID};

use super::types::User;
use crate::{db::user, schema::guard::AuthenticationGuard, AuthenticatedUser, Database};

#[derive(Default)]
pub struct UserQuery;

#[derive(OneofObject)]
pub enum UserBy {
    Id(ID),
    Email(String),
}

#[Object]
impl UserQuery {
    #[graphql(guard = "AuthenticationGuard::new()")]
    async fn viewer(&self, ctx: &Context<'_>) -> Result<User> {
        let AuthenticatedUser(user) = ctx.data::<AuthenticatedUser>()?;

        match user {
            Some(user) => Ok(user.clone().into()),
            None => Err("Forbidden".into()),
        }
    }

    async fn user(&self, ctx: &Context<'_>, by: UserBy) -> Result<User> {
        let db = ctx.data::<Database>()?;

        let where_param = match by {
            UserBy::Id(id) => user::id::equals(id.0),
            UserBy::Email(email) => user::email::equals(email),
        };

        Ok(db
            .user()
            .find_unique(where_param)
            .exec()
            .await?
            .ok_or("user not found")?
            .into())
    }

    // #[graphql(guard = "RoleGuard::new(Role::Admin)")]
}
