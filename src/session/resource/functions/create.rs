use super::{
    super::error::{AuthStepError, PlayerStepError},
    actor_step, auth_step, player_step, room_step,
};
use crate::{
    actor::{
        model::Actor,
        resource::{ActorResourceEvent, ActorResourceSender},
    },
    auth::resource::{
        AuthRequest, AuthResourceEvent, AuthResourceSender, AuthResponse, Credential,
    },
    player::{
        model::Player,
        resource::{PlayerResourceEvent, PlayerResourceSender},
    },
    room::{
        model::RoomProxy,
        resource::{RoomResourceEvent, RoomResourceSender},
    },
    session::resource::SessionResourceSender,
    Id,
};
use anyhow::{Error, Result};
use futures::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::{net::TcpStream, sync::oneshot};
use tokio_util::codec::{Framed, LinesCodec};

type ResourceSenders = (
    SessionResourceSender,
    AuthResourceSender,
    PlayerResourceSender,
    ActorResourceSender,
    RoomResourceSender,
);

pub async fn create_session(
    (mut lines, addr): (Framed<TcpStream, LinesCodec>, SocketAddr),
    (
        session_resource_sender,
        auth_resource_sender,
        player_resource_sender,
        actor_resource_sender,
        room_resource_sender,
    ): ResourceSenders,
) -> Result<()> {
    // @TODO: Extract this to a login screen async function that can be injected from outside
    lines.send("Please enter your username:").await?;

    let username = match lines.next().await {
        Some(Ok(line)) => line,
        _ => {
            tracing::error!("Failed to get username from {}. Client disconnected.", addr);
            return Ok(());
        }
    };

    lines.send("Please enter your password:").await?;

    let password = match lines.next().await {
        Some(Ok(line)) => line,
        _ => {
            tracing::error!("Failed to get username from {}. Client disconnected.", addr);
            return Ok(());
        }
    };

    let credential = Credential::UserNameAndPassword(username, password);

    // @TODO: Let this match on the underlying AuthStepError and PlayerStepError to redirect to the appropriate interface
    let (mut actor, mut player, room) = resource_steps(
        credential,
        &auth_resource_sender,
        &actor_resource_sender,
        &player_resource_sender,
        &room_resource_sender,
    )
    .await?;

    // Split the player's Lines into Stream and Writer
    // let (sink, stream) = lines.split();

    // Attach the player to the Actor
    // actor.attach_player(&player)?;

    Ok(())
}

async fn resource_steps(
    credential: Credential,
    auth_resource_sender: &AuthResourceSender,
    actor_resource_sender: &ActorResourceSender,
    player_resource_sender: &PlayerResourceSender,
    room_resource_sender: &RoomResourceSender,
) -> Result<(Actor, Player, RoomProxy)> {
    let (auth_reply_sender, auth_reply_receiver) = oneshot::channel();

    auth_resource_sender.send(AuthResourceEvent::Request(
        AuthRequest::WithCredential(credential),
        auth_reply_sender,
    ))?;

    // @TODO: Use these auth ids to get good metrics/logs around the auth process
    let (id, player_id) = match auth_step(auth_reply_receiver).await? {
        AuthResponse::Authenticated { id, player_id } => (id, player_id),
        AuthResponse::Forbidden => return Err(Error::new(AuthStepError::Forbidden)),
    };

    let (player_reply_sender, player_reply_receiver) = oneshot::channel();
    player_resource_sender.send(PlayerResourceEvent::GetPlayerById(
        player_id,
        player_reply_sender,
    ))?;
    let player = player_step(player_reply_receiver).await?;

    let current_actor_id = if let Some(current_actor_id) = player.get_current_actor_id() {
        current_actor_id
    } else {
        // @TODO: Redirect to a character creation/no-current-character interface
        return Err(Error::new(PlayerStepError::NoActorOwned(player.id())));
    };

    let (actor_reply_sender, actor_reply_receiver) = oneshot::channel();
    actor_resource_sender.send(ActorResourceEvent::GetActorById(
        current_actor_id,
        actor_reply_sender,
    ))?;
    let actor = actor_step(actor_reply_receiver).await?;

    let (room_reply_sender, room_reply_receiver) = oneshot::channel();
    room_resource_sender.send(RoomResourceEvent::GetRoomById(
        Id::new(1),
        room_reply_sender,
    ))?;
    let room = room_step(room_reply_receiver).await?;

    Ok((actor, player, room))
}
