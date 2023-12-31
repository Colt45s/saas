use std::sync::Arc;

use anyhow::anyhow;
use async_graphql::connection::{query, Connection, Edge};
use async_graphql::{ComplexObject, Context, Result, SimpleObject, ID};
use shaku::HasProvider;

use crate::db::user;
use crate::schema::project::types::Project;
use crate::services::user::UserService;
use crate::services::Injector;

use super::query::UserBy;

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
    async fn projects(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<usize, Project>> {
        let injector = ctx.data::<Arc<Injector>>()?;
        let user_service: Box<dyn UserService> =
            injector.provide().map_err(|e| anyhow!(e.to_string()))?;

        let user = user_service
            .get_user(UserBy::Id(self.id.clone()))
            .await?
            .ok_or(anyhow!("User not found"))?;

        let projects = user.projects()?;

        query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let mut start = after.map(|after| after + 1).unwrap_or(0);
                let mut end = before.unwrap_or(projects.len());

                if let Some(first) = first {
                    end = (start + first).min(end);
                }

                if let Some(last) = last {
                    start = if last > end - start { end } else { end - last }
                }

                let slice = projects[start..end].to_vec();
                let mut connection = Connection::new(start > 0, end < projects.len());
                connection.edges.extend(
                    slice
                        .iter()
                        .enumerate()
                        .map(|(idx, item)| Edge::new(start + idx, item.to_owned().into())),
                );
                Ok::<_, async_graphql::Error>(connection)
            },
        )
        .await
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
