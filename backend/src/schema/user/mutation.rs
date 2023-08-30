use async_graphql::{Context, InputObject, Object, Result, ID};

use super::types::User;
use crate::{
    db::user::{self, SetParam},
    firebase::{self, FirebaseConfig},
    Database,
};

#[derive(Default)]
pub struct UserMutation {}

#[Object]
impl UserMutation {
    async fn signin(&self, ctx: &Context<'_>, token: String) -> Result<User> {
        let db = ctx.data::<Database>()?;
        let firebase_config = ctx.data::<FirebaseConfig>()?;
        let claims: firebase::auth::Claims =
            firebase::auth::verify(firebase_config, &token).await?;
        tracing::info!("claims: {:?}", claims);
        let user = db
            .user()
            .find_unique(user::uid::equals(claims.sub.clone()))
            .exec()
            .await?;

        match user {
            Some(user) => Ok(user.into()),
            None => Ok(db
                .user()
                .create(
                    claims.sub.clone(),
                    vec![
                        SetParam::SetName(claims.name),
                        SetParam::SetEmail(claims.email),
                        SetParam::SetEmailVerified(claims.email_verified),
                        SetParam::SetImage(claims.picture),
                    ],
                )
                .exec()
                .await?
                .into()),
        }
    }

    async fn create_user(&self, ctx: &Context<'_>, input: CreateUserInput) -> Result<User> {
        let db = ctx.data::<Database>()?;

        Ok(db
            .user()
            .create(
                input.uid,
                vec![
                    SetParam::SetName(input.name),
                    SetParam::SetEmail(input.email),
                    SetParam::SetEmailVerified(input.email_verified),
                    SetParam::SetImage(input.image),
                ],
            )
            .exec()
            .await?
            .into())
    }

    async fn update_user(&self, ctx: &Context<'_>, input: UpdateUserInput) -> Result<User> {
        let db = ctx.data::<Database>()?;

        Ok(db
            .user()
            .update(
                user::id::equals(input.id.0.into()),
                vec![
                    SetParam::SetName(input.name),
                    SetParam::SetEmail(input.email),
                    SetParam::SetEmailVerified(input.email_verified.map(|v| v.into())),
                    SetParam::SetImage(input.image),
                ],
            )
            .exec()
            .await?
            .into())
    }

    async fn delete_user(&self, ctx: &Context<'_>, user_id: ID) -> Result<User> {
        let db = ctx.data::<Database>()?;

        Ok(db
            .user()
            .delete(user::id::equals(user_id.0.into()))
            .exec()
            .await?
            .into())
    }
}

#[derive(InputObject)]
struct CreateUserInput {
    pub uid: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub image: Option<String>,
}

#[derive(InputObject)]
struct UpdateUserInput {
    pub id: ID,
    pub name: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub image: Option<String>,
}
