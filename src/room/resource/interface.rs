use super::{
    Room, RoomResourceError, RoomResourceReceiver, RoomResourceResolver, RoomResourceSender,
};
use crate::{messaging, messaging::Spawn};
use anyhow::{Result, Error};
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

impl Spawn for RoomResource {
    fn spawn(&mut self) -> Result<()> {
        tracing::info!("Spawning Room Resource...");

        self.spawn_all()?;

        let resolver = self
            .resolver
            .take()
            .ok_or_else(|| RoomResourceError::NoResolver)?;

        let receiver = self
            .receiver
            .take()
            .ok_or_else(|| RoomResourceError::NoReceiver)?;

        self.spawn_and_trace(messaging::resolve_receiver(receiver, resolver));

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

    pub fn spawn_all(&mut self) -> Result<()> {
        if let Some(resolver) = self.resolver.as_mut() {
            resolver.spawn_all()?;

            Ok(())
        } else {
            Err(Error::new(RoomResourceError::NoResolver))
        }
    }
}
