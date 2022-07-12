use super::{AuthResourceError, AuthResourceReceiver, AuthResourceResolver, AuthResourceSender};
use crate::{messaging, messaging::Spawn};

use anyhow::Result;
use std::default::Default;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct AuthResource {
    sender: AuthResourceSender,
    receiver: Option<AuthResourceReceiver>,
    resolver: Option<AuthResourceResolver>,
}

impl Default for AuthResource {
    fn default() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();

        AuthResource {
            sender,
            receiver: Some(receiver),
            resolver: Some(AuthResourceResolver::default()),
        }
    }
}

impl Spawn for AuthResource {
    fn spawn(&mut self) -> Result<()> {
        let resolver = self
            .resolver
            .take()
            .ok_or_else(|| AuthResourceError::NoResolver)?;

        let receiver = self
            .receiver
            .take()
            .ok_or_else(|| AuthResourceError::NoReceiver)?;

        self.spawn_and_trace(messaging::resolve_receiver(receiver, resolver));

        Ok(())
    }
}

impl AuthResource {
    fn new() -> Self {
        AuthResource {
            ..Default::default()
        }
    }

    fn sender(&self) -> AuthResourceSender {
        self.sender.clone()
    }
}
