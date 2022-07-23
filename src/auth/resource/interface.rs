use super::{
    super::traits::AuthClient,
    event::AuthResourceEvent,
    proxy::AuthResourceProxy,
    resolver::AuthResourceResolver,
    types::{AuthResourceReceiver, AuthResourceSender},
};
use crate::messaging::{
    error::DetachError,
    functions::{resolve_receiver, spawn_and_trace},
    traits::{Detach, ProvideProxy, Raise},
};

use anyhow::Result;
use std::{default::Default, fmt::Debug};
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct AuthResource<T>
where
    T: 'static + Send + Sync + Debug + Default + AuthClient,
{
    sender: AuthResourceSender,
    receiver: Option<AuthResourceReceiver>,
    resolver: Option<AuthResourceResolver<T>>,
}

impl<T> Default for AuthResource<T>
where
    T: 'static + Send + Sync + Debug + Default + AuthClient,
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

impl<T> Raise<AuthResourceEvent> for AuthResource<T>
where
    T: 'static + Send + Sync + Debug + Default + AuthClient,
{
    fn raise(&self, event: AuthResourceEvent) -> Result<()> {
        self.sender.send(event)?;

        Ok(())
    }
}

impl<T> Detach<AuthResourceEvent> for AuthResource<T>
where
    T: 'static + Send + Sync + Debug + Default + AuthClient,
{
    fn sender(&self) -> AuthResourceSender {
        self.sender.clone()
    }

    fn detach(&mut self) -> Result<()> {
        let receiver = self
            .receiver
            .take()
            .ok_or_else(|| DetachError::NoReceiver("auth resource".to_owned()))?;

        let resolver = self
            .resolver
            .take()
            .ok_or_else(|| DetachError::NoResolver("auth resource".to_owned()))?;

        spawn_and_trace(resolve_receiver(receiver, resolver));

        Ok(())
    }
}

impl<T> ProvideProxy<AuthResourceProxy> for AuthResource<T> where
    T: 'static + Send + Sync + Debug + Default + AuthClient
{
}

impl<T> AuthResource<T>
where
    T: 'static + Send + Sync + Debug + Default + AuthClient,
{
    pub fn new(client: T) -> Self {
        AuthResource {
            resolver: Some(AuthResourceResolver::new(client)),
            ..Default::default()
        }
    }
}
