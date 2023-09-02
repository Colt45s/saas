use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};

use crate::db::project;

#[derive(SimpleObject)]
pub struct Project {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

impl From<project::Data> for Project {
    fn from(project: project::Data) -> Self {
        Self {
            id: project.id,
            slug: project.slug,
            name: project.name,
            created_at: project.created_at.into(),
            modified_at: project.modified_at.into(),
        }
    }
}
