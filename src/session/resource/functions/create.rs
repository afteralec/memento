use super::{
    super::error::{AuthStepError, PlayerStepError},
    steps::{actor_step, auth_step, player_step, room_step, session_step},
};
use crate::{
    actor::{
        model::proxy::ActorProxy,
        resource::{proxy::ActorResourceProxy, ActorResourceEvent},
    },
    auth::resource::{
        proxy::AuthResourceProxy, AuthRequest, AuthResourceEvent, AuthResponse, Credential,
    },
    messaging::traits::Raise,
    player::{
        model::proxy::PlayerProxy,
        resource::{proxy::PlayerResourceProxy, PlayerResourceEvent},
    },
    room::{
        model::proxy::RoomProxy,
        resource::{proxy::RoomResourceProxy, RoomResourceEvent},
    },
    session::{
        model::{proxy::SessionProxy, types::SessionStream},
        resource::{event::SessionResourceEvent, proxy::SessionResourceProxy},
    },
    Id,
};
use anyhow::{Error, Result};
use futures::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::{net::TcpStream, sync::oneshot};
use tokio_util::codec::{Framed, LinesCodec};

type ResourceProxies = (
    SessionResourceProxy,
    AuthResourceProxy,
    PlayerResourceProxy,
    ActorResourceProxy,
    RoomResourceProxy,
);

pub async fn create_session(
    (mut lines, addr): (Framed<TcpStream, LinesCodec>, SocketAddr),
    (
        session_resource_proxy,
        auth_resource_proxy,
        player_resource_proxy,
        actor_resource_proxy,
        room_resource_proxy,
    ): ResourceProxies,
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

    // Build the Credential from the username and password
    let credential = Credential::UserNameAndPassword(username, password);

    // Split the player's Lines into Sink and Stream
    let (sink, stream) = lines.split();

    // @TODO: Let this match on the underlying AuthStepError and PlayerStepError to redirect to the appropriate interface
    let (mut actor, player, room, session) = resource_steps(
        stream,
        credential,
        &auth_resource_proxy,
        &actor_resource_proxy,
        &player_resource_proxy,
        &room_resource_proxy,
        &session_resource_proxy,
    )
    .await?;

    // Register the sink to the player
    player.attach_sink(sink)?;

    // Attach the player to the actor
    actor.attach_player(&player)?;

    // Send the room to the session
    session.new_room(&room)?;

    // @TODO: Attach the player to the session and the actor to the room. Voila!

    Ok(())
}

// @TODO: Extract this to its own ResourceProxies struct
async fn resource_steps(
    stream: SessionStream,
    credential: Credential,
    auth_resource_proxy: &AuthResourceProxy,
    actor_resource_proxy: &ActorResourceProxy,
    player_resource_proxy: &PlayerResourceProxy,
    room_resource_proxy: &RoomResourceProxy,
    session_resource_proxy: &SessionResourceProxy,
) -> Result<(ActorProxy, PlayerProxy, RoomProxy, SessionProxy)> {
    // @TODO: Put this pattern of building oneshot channels and raising a resource event on the individual ResourceProxy instead
    let (auth_reply_sender, auth_reply_receiver) = oneshot::channel();
    auth_resource_proxy.raise(AuthResourceEvent::Request(
        AuthRequest::WithCredential(credential),
        auth_reply_sender,
    ))?;

    // @TODO: Use these auth ids to get good metrics/logs around the auth process
    let (id, player_id) = match auth_step(auth_reply_receiver).await? {
        AuthResponse::Authenticated { id, player_id } => (id, player_id),
        AuthResponse::Forbidden => return Err(Error::new(AuthStepError::Forbidden)),
    };

    let (player_reply_sender, player_reply_receiver) = oneshot::channel();
    player_resource_proxy.raise(PlayerResourceEvent::GetPlayerById(
        player_id,
        player_reply_sender,
    ))?;
    let player = player_step(player_reply_receiver).await?;

    let current_actor_id = if let Some(current_actor_id) = player.current_actor_id() {
        current_actor_id
    } else {
        // @TODO: Redirect to a character creation/no-current-character interface
        return Err(Error::new(PlayerStepError::NoActorOwned(player.id())));
    };

    let (actor_reply_sender, actor_reply_receiver) = oneshot::channel();
    actor_resource_proxy.raise(ActorResourceEvent::GetActorById(
        current_actor_id,
        actor_reply_sender,
    ))?;
    let actor = actor_step(actor_reply_receiver).await?;

    let (room_reply_sender, room_reply_receiver) = oneshot::channel();
    room_resource_proxy.raise(RoomResourceEvent::GetRoomById(
        // @TODO: Implement persistence so this gets the last known room ID of the player/actor instead
        Id::new(1),
        room_reply_sender,
    ))?;
    let room = room_step(room_reply_receiver).await?;

    let (session_reply_sender, session_reply_receiver) = oneshot::channel();
    session_resource_proxy.raise(SessionResourceEvent::NewSession(stream, session_reply_sender))?;
    let session = session_step(session_reply_receiver).await?;

    Ok((actor, player, room, session))
}
