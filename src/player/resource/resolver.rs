use super::PlayerResourceEvent;
use crate::messaging::ResolverMut;
use anyhow::Result;
use std::default::Default;

#[derive(Debug)]
pub struct PlayerResourceResolver {}

impl Default for PlayerResourceResolver {
    fn default() -> Self {
        PlayerResourceResolver {}
    }
}

impl ResolverMut<PlayerResourceEvent> for PlayerResourceResolver {
    fn resolve_on(&mut self, _event: PlayerResourceEvent) -> Result<()> {
        Ok(())
    }
}
