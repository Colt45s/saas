pub mod auth;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct FirebaseConfig {
    pub project_id: String,
}

impl FirebaseConfig {
    pub fn new(project_id: String) -> Self {
        Self { project_id }
    }
}
