use super::{
    super::model::Actor,
    error::ActorResourceError,
    event::ActorResourceEvent,
    proxy::ActorResourceProxy,
    resolver::ActorResourceResolver,
    types::{ActorResourceReceiver, ActorResourceSender},
};
use crate::messaging::{
    functions::{resolve_receiver, spawn_and_trace},
    traits::{Detach, ProvideProxy},
};
use anyhow::Result;
use std::{default::Default, fmt::Debug};
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

impl Detach<ActorResourceEvent> for ActorResource {
    fn sender(&self) -> ActorResourceSender {
        self.sender.clone()
    }

    fn detach(&mut self) -> Result<()> {
        let receiver = self
            .receiver
            .take()
            .ok_or_else(|| ActorResourceError::NoReceiver)?;

        let resolver = self
            .resolver
            .take()
            .ok_or_else(|| ActorResourceError::NoResolver)?;

        spawn_and_trace(resolve_receiver(receiver, resolver));

        Ok(())
    }
}

impl ProvideProxy<ActorResourceProxy> for ActorResource {}

impl ActorResource {
    pub fn new(actor_iter: impl Iterator<Item = Actor>) -> Self {
        ActorResource {
            resolver: Some(ActorResourceResolver::new(actor_iter)),
            ..Default::default()
        }
    }
}
