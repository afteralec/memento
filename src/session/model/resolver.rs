use super::event::SessionEvent;
use crate::{messaging::traits::Resolver, room::model::proxy::RoomProxy};
use anyhow::Result;
use async_trait::async_trait;
use std::default::Default;

#[derive(Debug)]
pub struct SessionResolver {
    state: SessionState,
}

impl Default for SessionResolver {
    fn default() -> Self {
        SessionResolver {
            state: SessionState::default(),
        }
    }
}

#[async_trait]
impl Resolver<SessionEvent> for SessionResolver {
    fn resolve_on(&mut self, event: SessionEvent) -> Result<()> {
        Ok(())
    }

    async fn resolve_async(&mut self, _: SessionEvent) -> Result<()> {
        unimplemented!(
            "Async resolution not supported for SessionResolver, use resolve_on instead."
        );
    }
}

#[derive(Debug)]
pub struct SessionState {
    room: Option<RoomProxy>,
}

impl Default for SessionState {
    fn default() -> Self {
        SessionState {
            room: None,
        }
    }
}
