use anyhow::Result;
use async_graphql::ID;
use async_trait::async_trait;
use shaku::{Interface, Provider};
use std::sync::Arc;

use crate::db::{
    self,
    user::{self, SetParam},
};
use crate::prisma::client::PrismaClient;
use crate::schema::user::{
    mutation::{CreateUserInput, UpdateUserInput},
    query::UserBy,
};

#[async_trait]
pub trait UserService: Interface {
    async fn get_user_by_uid(&self, uid: String) -> Result<Option<db::user::Data>>;
    async fn get_user(&self, by: UserBy) -> Result<Option<db::user::Data>>;
    async fn create_user(&self, input: CreateUserInput) -> Result<db::user::Data>;
    async fn update_user(&self, input: UpdateUserInput) -> Result<db::user::Data>;
    async fn delete_user(&self, user_id: ID) -> Result<db::user::Data>;
}

#[derive(Provider)]
#[shaku(interface = UserService)]
pub struct UserServiceImpl {
    #[shaku(inject)]
    prisma: Arc<dyn PrismaClient>,
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn get_user_by_uid(&self, uid: String) -> Result<Option<db::user::Data>> {
        let prisma_client = self.prisma.get_client()?;
        Ok(prisma_client
            .user()
            .find_unique(db::user::uid::equals(uid))
            .exec()
            .await?)
    }
    async fn get_user(&self, by: UserBy) -> Result<Option<db::user::Data>> {
        let where_param = match by {
            UserBy::Id(id) => db::user::id::equals(id.0),
            UserBy::Email(email) => db::user::email::equals(email),
        };

        let prisma_client = self.prisma.get_client()?;

        Ok(prisma_client
            .user()
            .find_unique(where_param)
            .with(user::projects::fetch(vec![]))
            .exec()
            .await?)
    }
    async fn create_user(&self, input: CreateUserInput) -> Result<db::user::Data> {
        let prisma_client = self.prisma.get_client()?;

        Ok(prisma_client
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
            .await?)
    }
    async fn update_user(&self, input: UpdateUserInput) -> Result<db::user::Data> {
        let prisma_client = self.prisma.get_client()?;

        Ok(prisma_client
            .user()
            .update(
                user::id::equals(input.id.0),
                vec![
                    SetParam::SetName(input.name),
                    SetParam::SetEmail(input.email),
                    SetParam::SetEmailVerified(input.email_verified),
                    SetParam::SetImage(input.image),
                ],
            )
            .exec()
            .await?)
    }
    async fn delete_user(&self, user_id: ID) -> Result<db::user::Data> {
        let prisma_client = self.prisma.get_client()?;

        Ok(prisma_client
            .user()
            .delete(user::id::equals(user_id.0))
            .exec()
            .await?)
    }
}
