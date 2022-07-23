use super::{
    error::DetachError,
    functions::{resolve_receiver, spawn_and_trace},
    traits::{Detach, Raise, Resolver},
    types::{Receiver, Sender},
};
use anyhow::Result;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Messenger<E, RV>
where
    E: 'static + Send + Sync + Debug,
    RV: 'static + Send + Sync + Debug + Resolver<E>,
{
    name: String,
    sender: Sender<E>,
    receiver: Option<Receiver<E>>,
    resolver: Option<RV>,
}

impl<E, RV> Raise<E> for Messenger<E, RV>
where
    E: 'static + Send + Sync + Debug,
    RV: 'static + Send + Sync + Debug + Resolver<E>,
{
    fn raise(&self, event: E) -> Result<()> {
        self.sender.send(event)?;

        Ok(())
    }
}

impl<E, RV> Detach<E> for Messenger<E, RV>
where
    E: 'static + Send + Sync + Debug,
    RV: 'static + Send + Sync + Debug + Resolver<E>,
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
