use super::{
    super::model::Player,
    event::PlayerResourceEvent,
    proxy::PlayerResourceProxy,
    resolver::PlayerResourceResolver,
    types::{PlayerResourceReceiver, PlayerResourceSender},
};
use crate::messaging::{
    error::DetachError,
    functions::resolve_receiver,
    traits::{Detach, ProvideProxy, Raise, Spawn},
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
        let receiver = self
            .receiver
            .take()
            .ok_or_else(|| DetachError::NoReceiver("player resource".to_owned()))?;

        let resolver = self
            .resolver
            .take()
            .ok_or_else(|| DetachError::NoResolver("player resource".to_owned()))?;

        self.spawn_and_trace(resolve_receiver(receiver, resolver));

        Ok(())
    }
}

impl ProvideProxy<PlayerResourceProxy> for PlayerResource {}

impl PlayerResource {
    pub fn new(player_iter: impl Iterator<Item = Player>) -> Self {
        PlayerResource {
            resolver: Some(PlayerResourceResolver::new(player_iter)),
            ..Default::default()
        }
    }
}
