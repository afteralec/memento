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

pub async fn auth_step(receiver: AuthResourceReplyReceiver) -> Result<AuthResponse> {
    loop {
        match receiver.await? {
            AuthResourceReplyEvent::Response(auth_response) => {
                return Ok(auth_response);
            }
        }
    }
}

pub async fn player_step(receiver: PlayerResourceReplyReceiver) -> Result<Player> {
    loop {
        match receiver.await? {
            PlayerResourceReplyEvent::GotPlayerById(_, player) => {
                return Ok(player);
            }
            PlayerResourceReplyEvent::NoPlayerAtId(id) => {
                return Err(Error::new(PlayerStepError::NoPlayerFound(id)));
            }
            _ => return Err(Error::new(PlayerStepError::WrongReplyReceived)),
        }
    }
}

pub async fn actor_step(receiver: ActorResourceReplyReceiver) -> Result<Actor> {
    loop {
        match receiver.await? {
            ActorResourceReplyEvent::GotActorById(_, actor) => {
                return Ok(actor);
            }
            ActorResourceReplyEvent::NoActorAtId(id) => {
                return Err(Error::new(ActorStepError::NoActorFound(id)));
            }
        }
    }
}

pub async fn room_step(receiver: RoomResourceReplyReceiver) -> Result<Room> {
    loop {
        match receiver.await? {
            RoomResourceReplyEvent::GotRoomById(_, room_proxy) => {
                return Ok(room_proxy);
            }
            RoomResourceReplyEvent::NoRoomAtId(id) => {
                return Err(Error::new(RoomStepError::NoRoomFound(id)));
            }
        }
    }
}

pub async fn session_step(receiver: SessionResourceReplyReceiver) -> Result<Session> {
    loop {
        match receiver.await? {
            SessionResourceReplyEvent::NewSession(session) => {
                return Ok(session);
            }
        }
    }
}
