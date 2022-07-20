use super::{
    error::SessionResourceError,
    event::SessionResourceEvent,
    proxy::SessionResourceProxy,
    resolver::SessionResourceResolver,
    types::{SessionResourceReceiver, SessionResourceSender},
};
use crate::{
    actor::resource::ActorResourceSender,
    auth::resource::AuthResourceSender,
    messaging::{
        error::SpawnError,
        functions::resolve_receiver,
        traits::{Detach, ProvideProxy, Proxy, Raise, Spawn},
    },
    player::resource::PlayerResourceSender,
    room::resource::RoomResourceSender,
};
use anyhow::{Error, Result};
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

        SessionResource {
            sender,
            receiver: Some(receiver),
            resolver: Some(SessionResourceResolver::default()),
        }
    }
}

impl Raise<SessionResourceEvent> for SessionResource {
    fn sender(&self) -> SessionResourceSender {
        self.sender.clone()
    }

    fn raise(&self, event: SessionResourceEvent) -> Result<()> {
        self.sender.send(event)?;

        Ok(())
    }
}

impl Spawn for SessionResource {}

impl Detach for SessionResource
where
    Self: Spawn,
{
    fn detach(&mut self) -> Result<()> {
        let resolver = self
            .resolver
            .take()
            .ok_or_else(|| SpawnError::NoResolver("session resource".to_owned()))?;

        if !resolver.configured_for_spawn() {
            return Err(Error::new(SpawnError::ResolverMisconfigured(
                "session resource".to_owned(),
            )));
        };

        let receiver = self
            .receiver
            .take()
            .ok_or_else(|| SpawnError::NoReceiver("session resource".to_owned()))?;

        self.spawn_and_trace(resolve_receiver(receiver, resolver));

        Ok(())
    }
}

impl ProvideProxy<SessionResourceProxy> for SessionResource {
    fn proxy(&self) -> SessionResourceProxy {
        SessionResourceProxy::proxy(&self)
    }
}

impl SessionResource {
    pub fn new() -> Self {
        SessionResource {
            ..Default::default()
        }
    }

    pub fn set_auth_resource_sender(&mut self, sender: AuthResourceSender) {
        if let Some(resolver) = self.resolver.as_mut() {
            let _ = resolver.set_auth_resource_sender(sender);
        }
    }

    pub fn set_actor_resource_sender(&mut self, sender: ActorResourceSender) {
        if let Some(resolver) = self.resolver.as_mut() {
            let _ = resolver.set_actor_resource_sender(sender);
        }
    }

    pub fn set_player_resource_sender(&mut self, sender: PlayerResourceSender) {
        if let Some(resolver) = self.resolver.as_mut() {
            let _ = resolver.set_player_resource_sender(sender);
        }
    }

    pub fn set_room_resource_sender(&mut self, sender: RoomResourceSender) {
        if let Some(resolver) = self.resolver.as_mut() {
            let _ = resolver.set_room_resource_sender(sender);
        }
    }
}
