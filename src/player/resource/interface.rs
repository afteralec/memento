use super::{
    event::PlayerResourceEvent,
    types::{PlayerResourceMessenger, PlayerResourceSender},
};
use crate::messaging::traits::{Interface, Raise};
use anyhow::Result;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct PlayerResource {
    sender: PlayerResourceSender,
}

impl Raise<PlayerResourceEvent> for PlayerResource {
    fn raise(&self, event: PlayerResourceEvent) -> Result<()> {
        self.sender.send(event)?;

        Ok(())
    }
}

impl Interface<PlayerResourceMessenger> for PlayerResource {
    fn of(m: &PlayerResourceMessenger) -> Self {
        PlayerResource {
            sender: m.sender.clone(),
        }
    }
}
