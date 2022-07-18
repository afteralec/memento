use super::{SessionError, SessionReceiver, SessionResolver, SessionSender};
use crate::{messaging, messaging::traits::Spawn, Id};
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
            resolver: None,
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

impl Spawn for Session {
    fn spawn(&mut self) -> Result<()> {
        let resolver = self
            .resolver
            .take()
            .ok_or_else(|| SessionError::NoResolver(self.id))?;

        let receiver = self
            .receiver
            .take()
            .ok_or_else(|| SessionError::NoReceiver(self.id))?;

        self.spawn_and_trace(messaging::functions::resolve_receiver(receiver, resolver));

        Ok(())
    }
}

impl Session {
    pub fn new(id: u64) -> Self {
        Session {
            id: Id(id),
            ..Default::default()
        }
    }
}
