use super::{
    event::SessionResourceEvent,
    proxy::SessionResourceProxy,
    resolver::SessionResourceResolver,
    types::{SessionResourceReceiver, SessionResourceSender},
};
use crate::{
    actor::resource::proxy::ActorResourceProxy,
    auth::resource::proxy::AuthResourceProxy,
    messaging::{
        error::DetachError,
        functions::resolve_receiver,
        traits::{Detach, ProvideProxy, Raise, Spawn},
    },
    player::resource::proxy::PlayerResourceProxy,
    room::resource::proxy::RoomResourceProxy,
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

        let mut session_resource = SessionResource {
            sender,
            receiver: Some(receiver),
            resolver: Some(SessionResourceResolver::default()),
        };

        session_resource.hydrate_resolver_session_proxy();

        session_resource
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
        let receiver = self
            .receiver
            .take()
            .ok_or_else(|| DetachError::NoReceiver("session resource".to_owned()))?;

        let resolver = self
            .resolver
            .take()
            .ok_or_else(|| DetachError::NoResolver("session resource".to_owned()))?;

        if !resolver.configured() {
            return Err(Error::new(DetachError::ResolverMisconfigured(
                "session resource".to_owned(),
            )));
        };

        self.spawn_and_trace(resolve_receiver(receiver, resolver));

        Ok(())
    }
}

impl ProvideProxy<SessionResourceProxy> for SessionResource {}

impl SessionResource {
    pub fn new() -> Self {
        SessionResource {
            ..Default::default()
        }
    }

    pub fn set_auth_resource_proxy(&mut self, proxy: AuthResourceProxy) {
        if let Some(resolver) = self.resolver.as_mut() {
            let _ = resolver.set_auth_resource_proxy(proxy);
        }
    }

    pub fn set_actor_resource_proxy(&mut self, proxy: ActorResourceProxy) {
        if let Some(resolver) = self.resolver.as_mut() {
            let _ = resolver.set_actor_resource_proxy(proxy);
        }
    }

    pub fn set_player_resource_proxy(&mut self, proxy: PlayerResourceProxy) {
        if let Some(resolver) = self.resolver.as_mut() {
            let _ = resolver.set_player_resource_proxy(proxy);
        }
    }

    pub fn set_room_resource_proxy(&mut self, proxy: RoomResourceProxy) {
        if let Some(resolver) = self.resolver.as_mut() {
            let _ = resolver.set_room_resource_proxy(proxy);
        }
    }

    fn hydrate_resolver_session_proxy(&mut self) {
        let proxy = self.proxy();

        if let Some(resolver) = self.resolver.as_mut() {
            let _ = resolver.set_session_resource_proxy(proxy);
        }
    }
}
