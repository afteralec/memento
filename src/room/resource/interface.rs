use super::{
    super::model::Room, RoomResourceError, RoomResourceReceiver, RoomResourceResolver,
    RoomResourceSender,
};
use crate::{
    messaging,
    messaging::traits::{Detach, Spawn},
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

impl Spawn for RoomResource {}

impl Detach for RoomResource
where
    Self: Spawn,
{
    fn detach(&mut self) -> Result<()> {
        tracing::info!("Spawning Room Resource...");

        self.detach_all()?;

        let resolver = self
            .resolver
            .take()
            .ok_or_else(|| RoomResourceError::NoResolver)?;

        let receiver = self
            .receiver
            .take()
            .ok_or_else(|| RoomResourceError::NoReceiver)?;

        self.spawn_and_trace(messaging::functions::resolve_receiver(receiver, resolver));

        tracing::info!("Room Resource spawned successfully");

        Ok(())
    }
}

impl RoomResource {
    pub fn new(room_iter: impl Iterator<Item = Room>) -> Self {
        RoomResource {
            resolver: Some(RoomResourceResolver::new(room_iter)),
            ..Default::default()
        }
    }

    pub fn sender(&self) -> RoomResourceSender {
        self.sender.clone()
    }

    pub fn detach_all(&mut self) -> Result<()> {
        if let Some(resolver) = self.resolver.as_mut() {
            resolver.detach_all()?;

            Ok(())
        } else {
            Err(Error::new(RoomResourceError::NoResolver))
        }
    }
}
