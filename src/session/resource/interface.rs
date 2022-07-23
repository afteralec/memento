use super::{
    event::SessionResourceEvent,
    proxy::SessionResourceProxy,
    resolver::SessionResourceResolver,
    types::{SessionResourceReceiver, SessionResourceSender},
};
use crate::messaging::{
    error::DetachError,
    functions::{resolve_receiver, spawn_and_trace},
    traits::{Detach, ProvideProxy, Raise},
};
use anyhow::Result;
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

        let session_resource = SessionResource {
            sender,
            receiver: Some(receiver),
            resolver: Some(SessionResourceResolver::default()),
        };

        session_resource
    }
}

impl Raise<SessionResourceEvent> for SessionResource {
    fn raise(&self, event: SessionResourceEvent) -> Result<()> {
        self.sender.send(event)?;

        Ok(())
    }
}

impl Detach<SessionResourceEvent> for SessionResource {
    fn sender(&self) -> SessionResourceSender {
        self.sender.clone()
    }

    fn detach(&mut self) -> Result<()> {
        let receiver = self
            .receiver
            .take()
            .ok_or_else(|| DetachError::NoReceiver("session resource".to_owned()))?;

        let resolver = self
            .resolver
            .take()
            .ok_or_else(|| DetachError::NoResolver("session resource".to_owned()))?;

        spawn_and_trace(resolve_receiver(receiver, resolver));

        Ok(())
    }
}

impl ProvideProxy<SessionResourceProxy> for SessionResource {}

impl SessionResource {
    pub fn new() -> Self {
        SessionResource {
            ..Default::default()
        }
    }
}
