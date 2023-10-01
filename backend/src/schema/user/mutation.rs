use std::sync::Arc;

use anyhow::anyhow;
use async_graphql::{Context, InputObject, Object, Result, ID};
use shaku::HasProvider;

use super::types::User;
use crate::{
    firebase_auth::FirebaseAuth,
    services::{user::UserService, Injector},
};

#[derive(Default)]
pub struct UserMutation {}

#[Object]
impl UserMutation {
    async fn signin(
        &self,
        ctx: &Context<'_>,
        token: String,
        refresh_token: Option<String>,
    ) -> Result<User> {
        let firebase_auth = ctx.data::<FirebaseAuth>()?;
        let claims = firebase_auth.verify(&token).await?;
        tracing::info!("claims: {:?}", claims);

        let injector = ctx.data::<Arc<Injector>>()?;
        let user_service: Box<dyn UserService> =
            injector.provide().map_err(|e| anyhow!(e.to_string()))?;

        match user_service.get_user_by_uid(claims.sub.clone()).await {
            Ok(Some(user)) => Ok(user.into()),
            Ok(None) => Ok(user_service
                .create_user(CreateUserInput {
                    uid: claims.sub.clone(),
                    name: claims.name,
                    email: claims.email,
                    email_verified: claims.email_verified,
                    image: claims.picture,
                })
                .await?
                .into()),
            Err(e) => Err(e.into()),
        }
    }

    async fn create_user(&self, ctx: &Context<'_>, input: CreateUserInput) -> Result<User> {
        let injector = ctx.data::<Arc<Injector>>()?;
        let user_service: Box<dyn UserService> =
            injector.provide().map_err(|e| anyhow!(e.to_string()))?;
        Ok(user_service.create_user(input).await?.into())
    }

    async fn update_user(&self, ctx: &Context<'_>, input: UpdateUserInput) -> Result<User> {
        let injector = ctx.data::<Arc<Injector>>()?;
        let user_service: Box<dyn UserService> =
            injector.provide().map_err(|e| anyhow!(e.to_string()))?;
        Ok(user_service.update_user(input).await?.into())
    }

    async fn delete_user(&self, ctx: &Context<'_>, user_id: ID) -> Result<User> {
        let injector = ctx.data::<Arc<Injector>>()?;
        let user_service: Box<dyn UserService> =
            injector.provide().map_err(|e| anyhow!(e.to_string()))?;
        Ok(user_service.delete_user(user_id).await?.into())
    }
}

#[derive(InputObject)]
pub struct CreateUserInput {
    pub uid: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub image: Option<String>,
}

#[derive(InputObject)]
pub struct UpdateUserInput {
    pub id: ID,
    pub name: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub image: Option<String>,
}
