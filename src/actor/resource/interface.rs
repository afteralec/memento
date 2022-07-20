use super::{
    super::model::Actor,
    error::ActorResourceError,
    resolver::ActorResourceResolver,
    types::{ActorResourceReceiver, ActorResourceSender},
};
use crate::messaging::{
    functions::resolve_receiver,
    traits::{Detach, Spawn},
};
use anyhow::Result;
use std::default::Default;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct ActorResource {
    sender: ActorResourceSender,
    receiver: Option<ActorResourceReceiver>,
    resolver: Option<ActorResourceResolver>,
}

impl Default for ActorResource {
    fn default() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();

        ActorResource {
            sender,
            receiver: Some(receiver),
            resolver: Some(ActorResourceResolver::default()),
        }
    }
}

impl Spawn for ActorResource {}

impl Detach for ActorResource
where
    Self: Spawn,
{
    fn detach(&mut self) -> Result<()> {
        tracing::info!("Spawning Actor Resource...");

        let resolver = self
            .resolver
            .take()
            .ok_or_else(|| ActorResourceError::NoResolver)?;

        let receiver = self
            .receiver
            .take()
            .ok_or_else(|| ActorResourceError::NoReceiver)?;

        self.spawn_and_trace(resolve_receiver(receiver, resolver));

        tracing::info!("Actor Resource spawned successfully");

        Ok(())
    }
}

impl ActorResource {
    pub fn new(actor_iter: impl Iterator<Item = Actor>) -> Self {
        ActorResource {
            resolver: Some(ActorResourceResolver::new(actor_iter)),
            ..Default::default()
        }
    }

    pub fn sender(&self) -> ActorResourceSender {
        self.sender.clone()
    }
}
