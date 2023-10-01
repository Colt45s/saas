use std::sync::Arc;

use anyhow::anyhow;
use async_graphql::{Context, Object, OneofObject, Result, ID};
use shaku::HasProvider;

use super::types::User;
use crate::{
    schema::guard::AuthenticationGuard,
    services::{user::UserService, Injector},
    AuthenticatedUser,
};

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
        let injector = ctx.data::<Arc<Injector>>()?;
        let user_service: Box<dyn UserService> =
            injector.provide().map_err(|e| anyhow!(e.to_string()))?;

        match user_service.get_user(by).await {
            Ok(Some(user)) => Ok(user.into()),
            Ok(None) => Err(anyhow!("User not found").into()),
            Err(e) => Err(e.into()),
        }
    }

    // #[graphql(guard = "RoleGuard::new(Role::Admin)")]
}
