use super::{error::SessionStateError, SessionEvent};
use crate::{messaging::traits::Resolver, room::model::Room, Id};
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

impl Resolver<SessionEvent> for SessionResolver {
    fn resolve_on(&mut self, event: SessionEvent) -> Result<()> {
        Ok(())
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
