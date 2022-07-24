use super::{
    event::RoomResourceEvent,
    types::{RoomResourceMessenger, RoomResourceSender},
};
use crate::messaging::traits::{Interface, Raise};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct RoomResource {
    sender: RoomResourceSender,
}

impl Raise<RoomResourceEvent> for RoomResource {
    fn raise(&self, event: RoomResourceEvent) -> Result<()> {
        self.sender.send(event)?;

        Ok(())
    }
}

impl Interface<RoomResourceMessenger> for RoomResource {
    fn of(m: &RoomResourceMessenger) -> Self {
        RoomResource {
            sender: m.sender.clone(),
        }
    }
}
