use async_trait::async_trait;
use shaku::{Interface, Provider};
use std::sync::Arc;

use crate::prisma::client::PrismaClient;

#[async_trait]
pub trait ProjectService: Interface {}

#[derive(Provider)]
#[shaku(interface = ProjectService)]
pub struct ProjectServiceImpl {
    #[shaku(inject)]
    prisma: Arc<dyn PrismaClient>,
}

#[async_trait]
impl ProjectService for ProjectServiceImpl {}
