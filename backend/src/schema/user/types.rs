use async_graphql::connection::{query, Connection, Edge};
use async_graphql::{ComplexObject, Context, Result, SimpleObject, ID};

use crate::db::user;
use crate::{schema::project::types::Project, Database};

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
        let db = ctx.data::<Database>()?;
        let user_id = &self.id;
        let user = db
            .user()
            .find_unique(user::id::equals(user_id.clone().0))
            .with(user::projects::fetch(vec![]))
            .exec()
            .await?
            .ok_or("User not found")?;

        let projects = user.projects()?;

        query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let mut start = 0usize;
                let mut end = projects.len();

                if let Some(after) = after {
                    if after >= projects.len() {
                        return Ok(Connection::new(false, false));
                    }
                    start = after + 1;
                }

                if let Some(before) = before {
                    if before == 0 {
                        return Ok(Connection::new(false, false));
                    }

                    end = before;
                }

                let mut slice = projects[start..end].to_vec();

                if let Some(first) = first {
                    slice = slice[..first.min(slice.len())].to_vec();
                    end -= first.min(slice.len());
                } else if let Some(last) = last {
                    slice = slice[slice.len() - last.min(slice.len())..].to_vec();
                    start += last.min(slice.len());
                }

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
