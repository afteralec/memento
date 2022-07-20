use super::{error::SessionStateError, event::SessionEvent};
use crate::{messaging::traits::Resolver, room::model::Room, Id};
use anyhow::{Error, Result};
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
    id: Id,
    room: Option<Room>,
}

impl Default for SessionState {
    fn default() -> Self {
        SessionState {
            id: Id(0),
            room: None,
        }
    }
}

impl SessionState {
    pub fn set_id(&mut self, id: Id) -> Result<()> {
        if self.id.is_valid() {
            Err(Error::new(SessionStateError::IdAlreadyValid(self.id, id)))
        } else {
            self.id = id;

            Ok(())
        }
    }
}
