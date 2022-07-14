use super::{
    SessionResourceError, SessionResourceEvent, SessionResourceReceiver, SessionResourceResolver,
    SessionResourceSender,
};
use crate::{messaging, messaging::Spawn, AuthResourceSender, ActorResourceSender, PlayerResourceSender, RoomResourceSender};
use anyhow::{Error, Result};
use std::default::Default;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct SessionResource {
    sender: SessionResourceSender,
    receiver: Option<SessionResourceReceiver>,
    resolver: Option<SessionResourceResolver>,
}

impl Default for SessionResource {
    fn default() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();

        SessionResource {
            sender,
            receiver: Some(receiver),
            resolver: Some(SessionResourceResolver::default()),
        }
    }
}

impl Spawn for SessionResource {
    fn spawn(&mut self) -> Result<()> {
        let resolver = self
            .resolver
            .take()
            .ok_or_else(|| SessionResourceError::NoResolver)?;

        if !resolver.configured_for_spawn() {
            return Err(Error::new(SessionResourceError::ResolverMisconfigured));
        };

        let receiver = self
            .receiver
            .take()
            .ok_or_else(|| SessionResourceError::NoReceiver)?;

        self.spawn_and_trace(messaging::resolve_receiver(receiver, resolver));

        Ok(())
    }
}

impl SessionResource {
    pub fn new() -> Self {
        SessionResource {
            ..Default::default()
        }
    }
}
