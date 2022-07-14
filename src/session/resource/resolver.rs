use super::{Session, SessionResourceError, SessionResourceEvent, functions::auth_step};
use crate::{
    actor::ActorResourceSender,
    auth::{
        AuthRequest, AuthResourceEvent, AuthResourceReplyEvent, AuthResourceSender, AuthResponse,
    },
    messaging::ResolverMut,
    player::PlayerResourceSender,
    room::RoomResourceSender,
    Id,
};
use anyhow::{Error, Result};
use futures::{SinkExt, StreamExt};
use std::{collections::HashMap, default::Default};
use tokio::sync::{oneshot, oneshot::error::TryRecvError};
use tokio_util::codec::{Framed, LinesCodec};

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
                credential,
            } => {
                let auth_resource_sender =
                    self.state.auth_resource_sender.as_ref().ok_or_else(|| {
                        Error::new(SessionResourceError::MissingResourceSender("auth resource"))
                    })?;

                let (auth_reply_sender, auth_reply_receiver) = oneshot::channel();

                auth_resource_sender.send(AuthResourceEvent::Request(
                    AuthRequest::WithCredential(credential),
                    auth_reply_sender,
                ))?;

                match auth_step(auth_reply_receiver)? {
                    AuthResponse::Authenticated { id, player_id, actor_owned } => {
                        tracing::debug!("id: {:?}, player_id: {:?}, actor_owned: {:?}", id, player_id, actor_owned);
                    },
                    AuthResponse::Forbidden => (),
                }

                Ok(())
            }
        }
    }
}

impl SessionResourceResolver {
    pub fn configured_for_spawn(&self) -> bool {
        self.state.actor_resource_sender.is_some()
            && self.state.player_resource_sender.is_some()
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
