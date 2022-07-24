use super::{
    error::DetachError,
    functions::{resolve_receiver, spawn_and_trace},
    traits::{Detach, DetachAll, Interface, Provide, Raise, Resolver},
    types::{Receiver, Sender},
};
use anyhow::{Error, Result};
use std::fmt::Debug;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct Messenger<E, R>
where
    E: 'static + Send + Sync + Debug,
    R: 'static + Send + Sync + Debug + Resolver<E>,
{
    pub(crate) name: String,
    pub(crate) sender: Sender<E>,
    pub(crate) receiver: Option<Receiver<E>>,
    pub(crate) resolver: Option<R>,
}

impl<E, R> Clone for Messenger<E, R>
where
    E: 'static + Send + Sync + Debug,
    R: 'static + Send + Sync + Debug + Resolver<E>,
{
    fn clone(&self) -> Self {
        Messenger {
            name: self.name.clone(),
            sender: self.sender.clone(),
            receiver: None,
            resolver: None,
        }
    }
}

impl<E, R> Raise<E> for Messenger<E, R>
where
    E: 'static + Send + Sync + Debug,
    R: 'static + Send + Sync + Debug + Resolver<E>,
{
    fn raise(&self, event: E) -> Result<()> {
        self.sender.send(event)?;

        Ok(())
    }
}

impl<E, R> Detach<E> for Messenger<E, R>
where
    E: 'static + Send + Sync + Debug,
    R: 'static + Send + Sync + Debug + Resolver<E>,
{
    fn sender(&self) -> Sender<E> {
        self.sender.clone()
    }

    fn detach(&mut self) -> Result<()> {
        let receiver = self
            .receiver
            .take()
            .ok_or_else(|| DetachError::NoReceiver(self.name.clone()))?;

        let resolver = self
            .resolver
            .take()
            .ok_or_else(|| DetachError::NoResolver(self.name.clone()))?;

        spawn_and_trace(resolve_receiver(receiver, resolver));

        Ok(())
    }
}

impl<E, R> DetachAll for Messenger<E, R>
where
    E: 'static + Send + Sync + Debug,
    R: 'static + Send + Sync + Debug + Resolver<E> + DetachAll,
{
    fn detach_all(&mut self) -> Result<()> {
        if let Some(resolver) = self.resolver.as_mut() {
            resolver.detach_all()?;

            Ok(())
        } else {
            Err(Error::new(DetachError::NoResolver(self.name.clone())))
        }
    }
}

impl<E, R, I> Provide<I> for Messenger<E, R>
where
    E: 'static + Send + Sync + Debug,
    R: 'static + Send + Sync + Debug + Resolver<E>,
    I: 'static + Send + Sync + Debug + Interface<Self>,
{
}

impl<E, R> Messenger<E, R>
where
    E: 'static + Send + Sync + Debug,
    R: 'static + Send + Sync + Debug + Resolver<E>,
{
    pub fn new(name: &str, resolver: R) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();

        Messenger {
            name: name.to_owned(),
            sender,
            receiver: Some(receiver),
            resolver: Some(resolver),
        }
    }
}
