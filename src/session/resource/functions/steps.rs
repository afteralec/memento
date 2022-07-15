use super::super::error::PlayerStepError;
use crate::{
    auth::{AuthResourceReplyEvent, AuthResourceReplyReceiver, AuthResponse},
    player::{Player, PlayerResourceReplyEvent, PlayerResourceReplyReceiver},
};
use anyhow::{Error, Result};
use tokio::sync::oneshot::error::RecvError;

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
        }
    }
}
