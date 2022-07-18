use super::{
    SessionResourceError, SessionResourceEvent, SessionResourceReceiver, SessionResourceResolver,
    SessionResourceSender,
};
use crate::{
    messaging, messaging::traits::Spawn, actor::resource::ActorResourceSender, auth::resource::AuthResourceSender,
    player::resource::PlayerResourceSender, room::resource::RoomResourceSender,
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

impl Spawn for SessionResource {
    fn spawn(&mut self) -> Result<()> {
        tracing::info!("Spawning Session Resource...");

        let resolver = self
            .resolver
            .take()
            .ok_or_else(|| SessionResourceError::NoResolver)?;

        if !resolver.configured_for_spawn() {
            return Err(Error::new(SessionResourceError::ResolverMisconfigured));
        };

        let receiver = self
            .receiver
            .take()
            .ok_or_else(|| SessionResourceError::NoReceiver)?;

        self.spawn_and_trace(messaging::functions::resolve_receiver(receiver, resolver));

        tracing::info!("Session Resource spawned successfully");

        Ok(())
    }
}

impl SessionResource {
    pub fn new() -> Self {
        SessionResource {
            ..Default::default()
        }
    }

    pub fn sender(&self) -> SessionResourceSender {
        self.sender.clone()
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
