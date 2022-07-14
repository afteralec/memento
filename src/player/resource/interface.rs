use super::{
    Player, PlayerResourceError, PlayerResourceEvent, PlayerResourceReceiver,
    PlayerResourceResolver, PlayerResourceSender,
};
use crate::{messaging, messaging::Spawn};
use anyhow::Result;
use std::default::Default;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct PlayerResource {
    sender: PlayerResourceSender,
    receiver: Option<PlayerResourceReceiver>,
    resolver: Option<PlayerResourceResolver>,
}

impl Default for PlayerResource {
    fn default() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        PlayerResource {
            sender,
            receiver: Some(receiver),
            resolver: Some(PlayerResourceResolver::default()),
        }
    }
}

impl Spawn for PlayerResource {
    fn spawn(&mut self) -> Result<()> {
        let resolver = self
            .resolver
            .take()
            .ok_or_else(|| PlayerResourceError::NoResolver)?;

        let receiver = self
            .receiver
            .take()
            .ok_or_else(|| PlayerResourceError::NoReceiver)?;

        self.spawn_and_trace(messaging::resolve_receiver(receiver, resolver));

        Ok(())
    }
}

impl PlayerResource {
    pub fn new(player_iter: impl Iterator<Item = Player>) -> Self {
        PlayerResource {
            resolver: Some(PlayerResourceResolver::new(player_iter)),
            ..Default::default()
        }
    }

    pub fn send(&self, event: PlayerResourceEvent) -> Result<()> {
        self.sender.send(event)?;

        Ok(())
    }
}
