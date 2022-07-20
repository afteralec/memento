use super::{
    super::model::Room,
    event::RoomResourceEvent,
    proxy::RoomResourceProxy,
    resolver::RoomResourceResolver,
    types::{RoomResourceReceiver, RoomResourceSender},
};
use crate::messaging::{
    error::SpawnError,
    functions::resolve_receiver,
    traits::{Detach, ProvideProxy, Proxy, Raise, Spawn},
};
use anyhow::{Error, Result};
use std::default::Default;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct RoomResource {
    sender: RoomResourceSender,
    receiver: Option<RoomResourceReceiver>,
    resolver: Option<RoomResourceResolver>,
}

impl Default for RoomResource {
    fn default() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();

        RoomResource {
            sender,
            receiver: Some(receiver),
            resolver: Some(RoomResourceResolver::default()),
        }
    }
}

impl Raise<RoomResourceEvent> for RoomResource {
    fn sender(&self) -> RoomResourceSender {
        self.sender.clone()
    }

    fn raise(&self, event: RoomResourceEvent) -> Result<()> {
        self.sender.send(event)?;

        Ok(())
    }
}

impl Spawn for RoomResource {}

impl Detach for RoomResource
where
    Self: Spawn,
{
    fn detach(&mut self) -> Result<()> {
        self.detach_all()?;

        let resolver = self
            .resolver
            .take()
            .ok_or_else(|| SpawnError::NoResolver("room resource".to_owned()))?;

        let receiver = self
            .receiver
            .take()
            .ok_or_else(|| SpawnError::NoReceiver("room resource".to_owned()))?;

        self.spawn_and_trace(resolve_receiver(receiver, resolver));

        Ok(())
    }
}

impl ProvideProxy<RoomResourceProxy> for RoomResource {}

impl RoomResource {
    pub fn new(room_iter: impl Iterator<Item = Room>) -> Self {
        RoomResource {
            resolver: Some(RoomResourceResolver::new(room_iter)),
            ..Default::default()
        }
    }

    pub fn detach_all(&mut self) -> Result<()> {
        if let Some(resolver) = self.resolver.as_mut() {
            resolver.detach_all()?;

            Ok(())
        } else {
            Err(Error::new(SpawnError::NoResolver(
                "room resource".to_owned(),
            )))
        }
    }
}
