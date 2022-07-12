use super::AuthResourceEvent;
use crate::messaging::ResolverMut;
use anyhow::Result;
use std::default::Default;

#[derive(Debug)]
pub struct AuthResourceResolver {}

impl Default for AuthResourceResolver {
    fn default() -> Self {
        AuthResourceResolver {}
    }
}

impl ResolverMut<AuthResourceEvent> for AuthResourceResolver {
    fn resolve_on(&mut self, _event: AuthResourceEvent) -> Result<()> {
        Ok(())
    }
}
