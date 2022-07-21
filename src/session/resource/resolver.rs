use super::{
    super::model::Session,
    error::SessionResourceError,
    event::{SessionResourceEvent, SessionResourceReplyEvent},
    functions::create::create_session,
    proxy::SessionResourceProxy,
};
use crate::{
    actor::resource::proxy::ActorResourceProxy,
    auth::resource::proxy::AuthResourceProxy,
    messaging::traits::{Detach, ProvideProxy, Resolver, Spawn},
    player::resource::proxy::PlayerResourceProxy,
    room::resource::proxy::RoomResourceProxy,
    Id,
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use std::{collections::HashMap, default::Default};
use unique_id::{sequence::SequenceGenerator, Generator};

#[derive(Debug)]
pub struct SessionResourceResolver {
    state: SessionResourceState,
}

impl Default for SessionResourceResolver {
    fn default() -> Self {
        SessionResourceResolver {
            state: SessionResourceState::default(),
        }
    }
}

impl Spawn for SessionResourceResolver {}

#[async_trait]
impl Resolver<SessionResourceEvent> for SessionResourceResolver {
    fn resolve_on(&mut self, event: SessionResourceEvent) -> Result<()> {
        match event {
            SessionResourceEvent::CreateSession { lines, addr } => {
                let auth_resource_proxy = self.auth_resource_proxy()?;
                let player_resource_proxy = self.player_resource_proxy()?;
                let actor_resource_proxy = self.actor_resource_proxy()?;
                let room_resource_proxy = self.room_resource_proxy()?;
                let session_resource_proxy = self.session_resource_proxy()?;

                let _ = self.spawn_and_trace(create_session(
                    (lines, addr),
                    (
                        session_resource_proxy,
                        auth_resource_proxy,
                        player_resource_proxy,
                        actor_resource_proxy,
                        room_resource_proxy,
                    ),
                ));

                Ok(())
            }
            SessionResourceEvent::NewSession(stream, reply_sender) => {
                let id = self.state.id_gen.next_id();
                let mut session = Session::new(id, stream);
                session.detach()?;

                let session_proxy = session.proxy();

                self.state.sessions.insert(session.id(), session);

                reply_sender.send(SessionResourceReplyEvent::NewSession(session_proxy))?;

                Ok(())
            }
        }
    }

    async fn resolve_async(&mut self, _: SessionResourceEvent) -> Result<()> {
        unimplemented!(
            "Async resolution not supported for SessionResourceResolver, use resolve_on instead."
        );
    }
}

impl SessionResourceResolver {
    pub fn configured(&self) -> bool {
        self.state.session_resource_proxy.is_some()
            && self.state.auth_resource_proxy.is_some()
            && self.state.player_resource_proxy.is_some()
            && self.state.actor_resource_proxy.is_some()
            && self.state.room_resource_proxy.is_some()
    }

    pub fn session_resource_proxy(&self) -> Result<SessionResourceProxy> {
        self.state
            .session_resource_proxy
            .as_ref()
            .cloned()
            .ok_or_else(|| {
                Error::new(SessionResourceError::MissingResourceSender(
                    "session resource",
                ))
            })
    }

    pub fn set_session_resource_proxy(&mut self, proxy: SessionResourceProxy) {
        let _ = self.state.set_session_resource_proxy(proxy);
    }

    pub fn auth_resource_proxy(&self) -> Result<AuthResourceProxy> {
        self.state
            .auth_resource_proxy
            .as_ref()
            .cloned()
            .ok_or_else(|| Error::new(SessionResourceError::MissingResourceSender("auth resource")))
    }

    pub fn set_auth_resource_proxy(&mut self, proxy: AuthResourceProxy) {
        let _ = self.state.set_auth_resource_proxy(proxy);
    }

    pub fn actor_resource_proxy(&self) -> Result<ActorResourceProxy> {
        self.state
            .actor_resource_proxy
            .as_ref()
            .cloned()
            .ok_or_else(|| {
                Error::new(SessionResourceError::MissingResourceSender(
                    "actor resource",
                ))
            })
    }

    pub fn set_actor_resource_proxy(&mut self, proxy: ActorResourceProxy) {
        let _ = self.state.set_actor_resource_proxy(proxy);
    }

    pub fn player_resource_proxy(&self) -> Result<PlayerResourceProxy> {
        self.state
            .player_resource_proxy
            .as_ref()
            .cloned()
            .ok_or_else(|| {
                Error::new(SessionResourceError::MissingResourceSender(
                    "player resource",
                ))
            })
    }

    pub fn set_player_resource_proxy(&mut self, proxy: PlayerResourceProxy) {
        let _ = self.state.set_player_resource_proxy(proxy);
    }

    pub fn room_resource_proxy(&self) -> Result<RoomResourceProxy> {
        self.state
            .room_resource_proxy
            .as_ref()
            .cloned()
            .ok_or_else(|| Error::new(SessionResourceError::MissingResourceSender("room resource")))
    }

    pub fn set_room_resource_proxy(&mut self, proxy: RoomResourceProxy) {
        let _ = self.state.set_room_resource_proxy(proxy);
    }
}

#[derive(Debug)]
pub struct SessionResourceState {
    session_resource_proxy: Option<SessionResourceProxy>,
    auth_resource_proxy: Option<AuthResourceProxy>,
    actor_resource_proxy: Option<ActorResourceProxy>,
    player_resource_proxy: Option<PlayerResourceProxy>,
    room_resource_proxy: Option<RoomResourceProxy>,
    sessions: HashMap<Id, Session>,
    id_gen: SequenceGenerator,
}

impl Default for SessionResourceState {
    fn default() -> Self {
        SessionResourceState {
            session_resource_proxy: None,
            auth_resource_proxy: None,
            actor_resource_proxy: None,
            player_resource_proxy: None,
            room_resource_proxy: None,
            sessions: HashMap::default(),
            id_gen: SequenceGenerator::default(),
        }
    }
}

impl SessionResourceState {
    pub fn set_session_resource_proxy(&mut self, proxy: SessionResourceProxy) {
        let _ = self.session_resource_proxy.insert(proxy);
    }

    pub fn set_auth_resource_proxy(&mut self, proxy: AuthResourceProxy) {
        let _ = self.auth_resource_proxy.insert(proxy);
    }

    pub fn set_actor_resource_proxy(&mut self, proxy: ActorResourceProxy) {
        let _ = self.actor_resource_proxy.insert(proxy);
    }

    pub fn set_player_resource_proxy(&mut self, proxy: PlayerResourceProxy) {
        let _ = self.player_resource_proxy.insert(proxy);
    }

    pub fn set_room_resource_proxy(&mut self, proxy: RoomResourceProxy) {
        let _ = self.room_resource_proxy.insert(proxy);
    }
}
