use super::{
    proxy::SessionProxy,
    resolver::SessionResolver,
    types::{SessionReceiver, SessionSender},
    SessionEvent,
};
use crate::{
    messaging::{
        error::DetachError,
        functions::{resolve_receiver, spawn_and_trace},
        traits::{Detach, ProvideProxy},
    },
    Id,
};
use anyhow::Result;
use std::default::Default;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct Session {
    id: Id,
    sender: SessionSender,
    receiver: Option<SessionReceiver>,
    resolver: Option<SessionResolver>,
}

impl Default for Session {
    fn default() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();

        Session {
            id: Id(0),
            sender,
            receiver: Some(receiver),
            resolver: Some(SessionResolver::default()),
        }
    }
}

impl Clone for Session {
    fn clone(&self) -> Self {
        Session {
            id: self.id,
            sender: self.sender.clone(),
            receiver: None,
            resolver: None,
        }
    }
}

impl Detach<SessionEvent> for Session {
    fn sender(&self) -> SessionSender {
        self.sender.clone()
    }

    fn detach(&mut self) -> Result<()> {
        let receiver = self
            .receiver
            .take()
            .ok_or_else(|| DetachError::NoReceiver(format!("session id {}", self.id)))?;

        let resolver = self
            .resolver
            .take()
            .ok_or_else(|| DetachError::NoResolver(format!("session id {}", self.id)))?;

        spawn_and_trace(resolve_receiver(receiver, resolver));

        Ok(())
    }
}

impl ProvideProxy<SessionProxy> for Session {}

impl Session {
    pub fn new(id: i64) -> Self {
        Session {
            id: Id(id),
            ..Default::default()
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }
}
