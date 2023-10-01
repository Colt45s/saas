pub mod project;
pub mod user;

use std::sync::Arc;

use crate::{db, prisma::client::PrismaClientImpl};
use anyhow::Result;
use shaku::module;

module! {
    pub Injector {
      components = [PrismaClientImpl],
      providers = [user::UserServiceImpl, project::ProjectServiceImpl],
    }
}

impl Injector {
    pub async fn create() -> Result<Self> {
        let client = db::new_client().await?;
        Ok(Injector::builder()
            .with_component_parameters::<PrismaClientImpl>(PrismaClientImpl::get_parameters(
                Arc::new(client),
            ))
            .build())
    }
}
