use super::{
    super::model::Player, PlayerResourceError, PlayerResourceEvent, PlayerResourceReceiver,
    PlayerResourceResolver, PlayerResourceSender,
};
use crate::{
    messaging,
    messaging::traits::{Detach, Spawn},
};
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

impl Spawn for PlayerResource {}

impl Detach for PlayerResource
where
    Self: Spawn,
{
    fn detach(&mut self) -> Result<()> {
        tracing::info!("Spawning Player Resource...");

        let resolver = self
            .resolver
            .take()
            .ok_or_else(|| PlayerResourceError::NoResolver)?;

        let receiver = self
            .receiver
            .take()
            .ok_or_else(|| PlayerResourceError::NoReceiver)?;

        self.spawn_and_trace(messaging::functions::resolve_receiver(receiver, resolver));

        tracing::info!("Player Resource spawned successfully");

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

    pub fn sender(&self) -> PlayerResourceSender {
        self.sender.clone()
    }
}
