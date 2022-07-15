use super::{
    functions::create::create_session, Session, SessionResourceError, SessionResourceEvent,
};
use crate::{
    actor::ActorResourceSender,
    auth::{
        Credential,
        AuthRequest, AuthResourceEvent, AuthResourceReplyEvent, AuthResourceSender, AuthResponse,
    },
    messaging,
    messaging::ResolverMut,
    player::PlayerResourceSender,
    room::RoomResourceSender,
    Id,
};
use anyhow::{Error, Result};
use std::{collections::HashMap, default::Default};

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

impl ResolverMut<SessionResourceEvent> for SessionResourceResolver {
    fn resolve_on(&mut self, event: SessionResourceEvent) -> Result<()> {
        match event {
            SessionResourceEvent::NewSession {
                lines,
                addr,
            } => {
                let auth_resource_sender = self
                    .state
                    .auth_resource_sender
                    .as_ref()
                    .cloned()
                    .ok_or_else(|| {
                        Error::new(SessionResourceError::MissingResourceSender("auth resource"))
                    })?;

                let player_resource_sender = self
                    .state
                    .player_resource_sender
                    .as_ref()
                    .cloned()
                    .ok_or_else(|| {
                        Error::new(SessionResourceError::MissingResourceSender(
                            "player resource",
                        ))
                    })?;
                let player_resource_sender = player_resource_sender.clone();

                let actor_resource_sender = self
                    .state
                    .actor_resource_sender
                    .as_ref()
                    .cloned()
                    .ok_or_else(|| {
                        Error::new(SessionResourceError::MissingResourceSender(
                            "actor resource",
                        ))
                    })?;

                let room_resource_sender = self
                    .state
                    .room_resource_sender
                    .as_ref()
                    .cloned()
                    .ok_or_else(|| {
                        Error::new(SessionResourceError::MissingResourceSender("room resource"))
                    })?;

                let _ = messaging::functions::spawn_and_trace(create_session(
                    (lines, addr),
                    (
                        auth_resource_sender,
                        player_resource_sender,
                        actor_resource_sender,
                        room_resource_sender,
                    ),
                ));

                Ok(())
            }
        }
    }
}

impl SessionResourceResolver {
    pub fn configured_for_spawn(&self) -> bool {
        self.state.auth_resource_sender.is_some()
            && self.state.player_resource_sender.is_some()
            && self.state.actor_resource_sender.is_some()
            && self.state.room_resource_sender.is_some()
    }

    pub fn set_auth_resource_sender(&mut self, sender: AuthResourceSender) {
        let _ = self.state.set_auth_resource_sender(sender);
    }

    pub fn set_actor_resource_sender(&mut self, sender: ActorResourceSender) {
        let _ = self.state.set_actor_resource_sender(sender);
    }

    pub fn set_player_resource_sender(&mut self, sender: PlayerResourceSender) {
        let _ = self.state.set_player_resource_sender(sender);
    }

    pub fn set_room_resource_sender(&mut self, sender: RoomResourceSender) {
        let _ = self.state.set_room_resource_sender(sender);
    }
}

#[derive(Debug)]
pub struct SessionResourceState {
    auth_resource_sender: Option<AuthResourceSender>,
    actor_resource_sender: Option<ActorResourceSender>,
    player_resource_sender: Option<PlayerResourceSender>,
    room_resource_sender: Option<RoomResourceSender>,
    sessions: HashMap<Id, Session>,
}

impl Default for SessionResourceState {
    fn default() -> Self {
        SessionResourceState {
            auth_resource_sender: None,
            actor_resource_sender: None,
            player_resource_sender: None,
            room_resource_sender: None,
            sessions: HashMap::default(),
        }
    }
}

impl SessionResourceState {
    pub fn set_auth_resource_sender(&mut self, sender: AuthResourceSender) {
        let _ = self.auth_resource_sender.insert(sender);
    }

    pub fn set_actor_resource_sender(&mut self, sender: ActorResourceSender) {
        let _ = self.actor_resource_sender.insert(sender);
    }

    pub fn set_player_resource_sender(&mut self, sender: PlayerResourceSender) {
        let _ = self.player_resource_sender.insert(sender);
    }

    pub fn set_room_resource_sender(&mut self, sender: RoomResourceSender) {
        let _ = self.room_resource_sender.insert(sender);
    }
}
