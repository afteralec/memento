use super::{
    AuthClient, AuthResourceError, AuthResourceEvent, AuthResourceReceiver, AuthResourceResolver,
    AuthResourceSender,
};
use crate::{messaging, messaging::Spawn};

use anyhow::Result;
use std::{default::Default, fmt::Debug};
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct AuthResource<T>
where
    T: 'static + Send + Sync + Debug + AuthClient + Default,
{
    sender: AuthResourceSender,
    receiver: Option<AuthResourceReceiver>,
    resolver: Option<AuthResourceResolver<T>>,
}

impl<T> Default for AuthResource<T>
where
    T: 'static + Send + Sync + Debug + AuthClient + Default,
{
    fn default() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();

        AuthResource {
            sender,
            receiver: Some(receiver),
            resolver: Some(AuthResourceResolver::default()),
        }
    }
}

impl<T> Spawn for AuthResource<T>
where
    T: 'static + Send + Sync + Debug + AuthClient + Default,
{
    fn spawn(&mut self) -> Result<()> {
        tracing::info!("Spawning Auth Resource...");

        let resolver = self
            .resolver
            .take()
            .ok_or_else(|| AuthResourceError::NoResolver)?;

        let receiver = self
            .receiver
            .take()
            .ok_or_else(|| AuthResourceError::NoReceiver)?;

        self.spawn_and_trace(messaging::resolve_receiver(receiver, resolver));

        tracing::info!("Auth Resource spawned successfully");

        Ok(())
    }
}

impl<T> AuthResource<T>
where
    T: 'static + Send + Sync + Debug + AuthClient + Default,
{
    pub fn new(client: T) -> Self {
        AuthResource {
            resolver: Some(AuthResourceResolver::new(client)),
            ..Default::default()
        }
    }

    pub fn send(&self, event: AuthResourceEvent) -> Result<()> {
        self.sender.send(event)?;

        Ok(())
    }

    pub fn sender(&self) -> AuthResourceSender {
        self.sender.clone()
    }
}
