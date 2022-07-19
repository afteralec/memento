use super::{
    super::traits::AuthClient, AuthResourceError, AuthResourceEvent, AuthResourceReceiver,
    AuthResourceResolver, AuthResourceSender,
};
use crate::{
    messaging,
    messaging::traits::{Detach, Spawn},
};

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

impl<T> Spawn for AuthResource<T> where T: 'static + Send + Sync + Debug + AuthClient + Default {}

impl<T> Detach for AuthResource<T>
where
    T: 'static + Send + Sync + Debug + AuthClient + Default,
    Self: Spawn,
{
    fn detach(&mut self) -> Result<()> {
        tracing::info!("Spawning Auth Resource...");

        let resolver = self
            .resolver
            .take()
            .ok_or_else(|| AuthResourceError::NoResolver)?;

        let receiver = self
            .receiver
            .take()
            .ok_or_else(|| AuthResourceError::NoReceiver)?;

        self.spawn_and_trace(messaging::functions::resolve_receiver(receiver, resolver));

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
