use async_graphql::{ComplexObject, Context, Result, SimpleObject, ID};

use crate::db::user;
use crate::{schema::project::types::Product, Database};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct User {
    pub id: ID,
    pub uid: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub image: Option<String>,
}

#[ComplexObject]
impl User {
    async fn projects(&self, ctx: &Context<'_>) -> Result<Vec<Product>> {
        let db = ctx.data::<Database>()?;
        let user_id = &self.id;
        let user = db
            .user()
            .find_unique(user::id::equals(user_id.clone().0.into()))
            .with(user::projects::fetch(vec![]))
            .exec()
            .await?
            .ok_or("User not found")?;

        let projects = user.projects()?;
        Ok(projects.into_iter().map(|p| p.to_owned().into()).collect())
    }
}

impl From<user::Data> for User {
    fn from(user: user::Data) -> Self {
        Self {
            id: user.id.into(),
            uid: user.uid,
            name: user.name,
            email: user.email,
            email_verified: user.email_verified,
            image: user.image,
        }
    }
}
