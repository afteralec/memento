use super::{
    ActorResourceError, ActorResourceReceiver, ActorResourceResolver, ActorResourceSender,
};
use crate::{messaging, messaging::Spawn};
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

impl Spawn for ActorResource {
    fn spawn(&mut self) -> Result<()> {
        let resolver = self
            .resolver
            .take()
            .ok_or_else(|| ActorResourceError::NoResolver)?;

        let receiver = self
            .receiver
            .take()
            .ok_or_else(|| ActorResourceError::NoReceiver)?;

        self.spawn_and_trace(messaging::resolve_receiver(receiver, resolver));

        Ok(())
    }
}

impl ActorResource {
    pub fn new() -> Self {
        ActorResource {
            ..Default::default()
        }
    }
}
