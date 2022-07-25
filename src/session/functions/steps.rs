use super::super::error::{ActorStepError, PlayerStepError, RoomStepError};
use crate::{
    actor::{
        interface::Actor,
        resource::{ActorResourceReplyEvent, ActorResourceReplyReceiver},
    },
    auth::resource::{
        event::{AuthResourceReplyEvent, AuthResponse},
        types::AuthResourceReplyReceiver,
    },
    player::{
        interface::Player,
        resource::{event::PlayerResourceReplyEvent, types::PlayerResourceReplyReceiver},
    },
    room::{
        interface::Room,
        resource::{event::RoomResourceReplyEvent, types::RoomResourceReplyReceiver},
    },
    session::{
        interface::Session,
        resource::{event::SessionResourceReplyEvent, types::SessionResourceReplyReceiver},
    },
};
use anyhow::{Error, Result};
use std::sync::Arc;

pub async fn auth_step(receiver: AuthResourceReplyReceiver) -> Result<AuthResponse> {
    match receiver.await? {
        AuthResourceReplyEvent::Response(auth_response) => {
            Ok(auth_response)
        }
    }
}

pub async fn player_step(receiver: PlayerResourceReplyReceiver) -> Result<Player> {
    match receiver.await? {
        PlayerResourceReplyEvent::GotPlayerById(_, player) => {
            Ok(Arc::try_unwrap(player).unwrap())
        }
        PlayerResourceReplyEvent::NoPlayerAtId(id) => {
            Err(Error::new(PlayerStepError::NoPlayerFound(id)))
        }
        _ => Err(Error::new(PlayerStepError::WrongReplyReceived)),
    }
}

pub async fn actor_step(receiver: ActorResourceReplyReceiver) -> Result<Actor> {
    match receiver.await? {
        ActorResourceReplyEvent::GotActorById(_, actor) => {
            Ok(Arc::try_unwrap(actor).unwrap())
        }
        ActorResourceReplyEvent::NoActorAtId(id) => {
            Err(Error::new(ActorStepError::NoActorFound(id)))
        }
    }
}

pub async fn room_step(receiver: RoomResourceReplyReceiver) -> Result<Room> {
    match receiver.await? {
        RoomResourceReplyEvent::GotRoomById(_, room) => {
            Ok(Arc::try_unwrap(room).unwrap())
        }
        RoomResourceReplyEvent::NoRoomAtId(id) => {
            Err(Error::new(RoomStepError::NoRoomFound(id)))
        }
    }
}

pub async fn session_step(receiver: SessionResourceReplyReceiver) -> Result<Session> {
    match receiver.await? {
        SessionResourceReplyEvent::NewSession(session) => {
        Ok(session)
        }
    }
}
