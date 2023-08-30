use async_graphql::{Context, Guard, Result};
use async_trait::async_trait;

use crate::AuthenticatedUser;

pub struct AuthenticationGuard;

impl AuthenticationGuard {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Guard for AuthenticationGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let AuthenticatedUser(user) = ctx.data::<AuthenticatedUser>()?;

        if user.is_none() {
            return Err("Forbidden".into());
        }

        Ok(())
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Role {
    Admin,
    Member,
}

pub struct RoleGuard {
    role: Role,
}

impl RoleGuard {
    pub fn new(role: Role) -> Self {
        Self { role }
    }
}

#[async_trait]
impl Guard for RoleGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        if ctx.data_opt::<Role>() == Some(&self.role) {
            Ok(())
        } else {
            Err("Forbidden".into())
        }
    }
}
