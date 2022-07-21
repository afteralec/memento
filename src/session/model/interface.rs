use super::{
    proxy::SessionProxy,
    resolver::SessionResolver,
    stream_resolver::SessionStreamResolver,
    types::{SessionReceiver, SessionSender, SessionStream},
};
use crate::{
    messaging::{
        error::DetachError,
        functions::resolve_stream_and_receiver,
        traits::{Detach, ProvideProxy, Spawn},
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
    stream: Option<SessionStream>,
    stream_resolver: Option<SessionStreamResolver>,
}

impl Default for Session {
    fn default() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();

        Session {
            id: Id(0),
            sender,
            receiver: Some(receiver),
            resolver: Some(SessionResolver::default()),
            stream: None,
            stream_resolver: Some(SessionStreamResolver::default()),
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
            stream: None,
            stream_resolver: None,
        }
    }
}

impl Spawn for Session {}

impl Detach for Session
where
    Self: Spawn,
{
    fn detach(&mut self) -> Result<()> {
        let receiver = self
            .receiver
            .take()
            .ok_or_else(|| DetachError::NoReceiver(format!("session id {}", self.id)))?;

        let resolver = self
            .resolver
            .take()
            .ok_or_else(|| DetachError::NoResolver(format!("session id {}", self.id)))?;

        let stream = self
            .stream
            .take()
            .ok_or_else(|| DetachError::NoStream(format!("session id {}", self.id)))?;

        let stream_resolver = self
            .stream_resolver
            .take()
            .ok_or_else(|| DetachError::NoStream(format!("session id {}", self.id)))?;

        self.spawn_and_trace(resolve_stream_and_receiver(
            stream,
            stream_resolver,
            receiver,
            resolver,
        ));

        Ok(())
    }
}

impl ProvideProxy<SessionProxy> for Session {}

impl Session {
    pub fn new(id: i64, stream: SessionStream) -> Self {
        Session {
            id: Id(id),
            stream: Some(stream),
            ..Default::default()
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn sender(&self) -> SessionSender {
        self.sender.clone()
    }
}
