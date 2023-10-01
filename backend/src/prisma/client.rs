use std::sync::Arc;

use crate::db;
use anyhow::Result;
use async_trait::async_trait;
use shaku::{Component, Interface};

#[async_trait]
pub trait PrismaClient: Interface {
    fn get_client(&self) -> Result<Arc<db::PrismaClient>>;
}

#[derive(Component)]
#[shaku(interface = PrismaClient)]
pub struct PrismaClientImpl {
    client: Arc<db::PrismaClient>,
}

#[async_trait]
impl PrismaClient for PrismaClientImpl {
    fn get_client(&self) -> Result<Arc<db::PrismaClient>> {
        Ok(self.client.clone())
    }
}

impl PrismaClientImpl {
    pub fn get_parameters(client: Arc<db::PrismaClient>) -> PrismaClientImplParameters {
        PrismaClientImplParameters { client }
    }
}
