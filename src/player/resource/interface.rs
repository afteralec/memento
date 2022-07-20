use super::{
    super::model::Player,
    event::PlayerResourceEvent,
    proxy::PlayerResourceProxy,
    resolver::PlayerResourceResolver,
    types::{PlayerResourceReceiver, PlayerResourceSender},
};
use crate::messaging::{
    error::SpawnError,
    functions::resolve_receiver,
    traits::{Detach, ProvideProxy, Proxy, Raise, Spawn},
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

impl Raise<PlayerResourceEvent> for PlayerResource {
    fn sender(&self) -> PlayerResourceSender {
        self.sender.clone()
    }

    fn raise(&self, event: PlayerResourceEvent) -> Result<()> {
        self.sender.send(event)?;

        Ok(())
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
            .ok_or_else(|| SpawnError::NoResolver("player resource".to_owned()))?;

        let receiver = self
            .receiver
            .take()
            .ok_or_else(|| SpawnError::NoReceiver("player resource".to_owned()))?;

        self.spawn_and_trace(resolve_receiver(receiver, resolver));

        tracing::info!("Player Resource spawned successfully");

        Ok(())
    }
}

impl ProvideProxy<PlayerResourceProxy> for PlayerResource {
    fn proxy(&self) -> PlayerResourceProxy {
        PlayerResourceProxy::proxy(&self)
    }
}

impl PlayerResource {
    pub fn new(player_iter: impl Iterator<Item = Player>) -> Self {
        PlayerResource {
            resolver: Some(PlayerResourceResolver::new(player_iter)),
            ..Default::default()
        }
    }
}
