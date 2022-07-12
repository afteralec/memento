use super::{error::SessionStateError, SessionEvent};
use crate::{messaging::ResolverMut, Id, RoomSender};
use anyhow::{Error, Result};
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

impl ResolverMut<SessionEvent> for SessionResolver {
    fn resolve_on(&mut self, event: SessionEvent) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct SessionState {
    id: Id,
    room_sender: Option<RoomSender>,
}

impl Default for SessionState {
    fn default() -> Self {
        SessionState {
            id: Id(0),
            room_sender: None,
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
