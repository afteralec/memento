use super::{interface::Session, error::SessionResourceError, event::SessionResourceEvent};
use crate::{
    actor::ActorResourceSender,
    auth::{
        AuthRequest, AuthResourceEvent, AuthResourceReplyEvent, AuthResourceSender, AuthResponse, AuthResourceReplyReceiver
    },
    messaging::ResolverMut,
    player::PlayerResourceSender,
    room::RoomResourceSender,
    Id,
};
use anyhow::Result;
use futures::{SinkExt, StreamExt};
use std::{collections::HashMap, default::Default};
use tokio::sync::{oneshot, oneshot::error::TryRecvError};
use tokio_util::codec::{Framed, LinesCodec};

pub fn auth_step(mut receiver: AuthResourceReplyReceiver) -> Result<()> {
    loop {
        match receiver.try_recv() {
            Ok(reply) => {
                match reply {
                    AuthResourceReplyEvent::Response(auth_response) => {
                        match auth_response {
                            AuthResponse::Authenticated {
                                id,
                                player_id,
                                actor_owned,
                            } => {
                                // @TODO: Carry forward; player is authenticated here
                            }
                            Forbidden => {
                                // @TODO: Raise an error; authentication failed
                                break;
                            }
                        }
                    }
                }
            }
            Err(err) => {
                match err {
                    TryRecvError::Closed => {
                        // @TODO: Raise an error; the sender is now closed
                        break;
                    }
                    TryRecvError::Empty => (),
                }
            }
        };
    };

    Ok(())
}
