use super::{
    super::error::{AuthStepError, PlayerStepError},
    steps::{actor_step, auth_step, player_step, room_step, session_step},
};
use crate::{
    actor::{model::proxy::ActorProxy, resource::event::ActorResourceEvent},
    auth::resource::event::{AuthRequest, AuthResourceEvent, AuthResponse, Credential},
    messaging::{functions::spawn_and_trace, traits::Raise},
    player::{model::proxy::PlayerProxy, resource::event::PlayerResourceEvent},
    room::{model::proxy::RoomProxy, resource::event::RoomResourceEvent},
    server::resource_proxy::ResourceProxies,
    session::{model::proxy::SessionProxy, resource::event::SessionResourceEvent},
    stream::{resolve::resolve_stream, resolver::StreamResolver, types::Stream},
    Id,
};
use anyhow::{Error, Result};
use futures::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::{net::TcpStream, sync::oneshot};
use tokio_util::codec::{Framed, LinesCodec};

pub async fn create_session(
    (mut lines, addr): (Framed<TcpStream, LinesCodec>, SocketAddr),
    resource_proxies: ResourceProxies,
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
    let (mut actor, player, room, session) =
        resource_steps(stream, credential, &resource_proxies).await?;

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
    stream: Stream,
    credential: Credential,
    resource_proxies: &ResourceProxies,
) -> Result<(ActorProxy, PlayerProxy, RoomProxy, SessionProxy)> {
    // @TODO: Put this pattern of building oneshot channels and raising a resource event on the individual ResourceProxy instead
    let (auth_reply_sender, auth_reply_receiver) = oneshot::channel();
    resource_proxies
        .auth_resource_proxy
        .raise(AuthResourceEvent::Request(
            AuthRequest::WithCredential(credential),
            auth_reply_sender,
        ))?;

    // @TODO: Use these auth ids to get good metrics/logs around the auth process
    let (id, player_id) = match auth_step(auth_reply_receiver).await? {
        AuthResponse::Authenticated { id, player_id } => (id, player_id),
        AuthResponse::Forbidden => return Err(Error::new(AuthStepError::Forbidden)),
    };

    let (player_reply_sender, player_reply_receiver) = oneshot::channel();
    resource_proxies
        .player_resource_proxy
        .raise(PlayerResourceEvent::GetPlayerById(
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
    resource_proxies
        .actor_resource_proxy
        .raise(ActorResourceEvent::GetActorById(
            current_actor_id,
            actor_reply_sender,
        ))?;
    let actor = actor_step(actor_reply_receiver).await?;

    let last_room_id = if let Some(last_room_id) = actor.last_room_id() {
        last_room_id
    } else {
        // @TODO: This means this is probably a new character that never made it to a room;
        //   engage a flow to find the correct room to send them to
        Id::new(1)
    };

    let (room_reply_sender, room_reply_receiver) = oneshot::channel();
    resource_proxies
        .room_resource_proxy
        .raise(RoomResourceEvent::GetRoomById(
            last_room_id,
            room_reply_sender,
        ))?;
    let room = room_step(room_reply_receiver).await?;

    let (session_reply_sender, session_reply_receiver) = oneshot::channel();
    resource_proxies
        .session_resource_proxy
        .raise(SessionResourceEvent::NewSession(session_reply_sender))?;
    let session = session_step(session_reply_receiver).await?;
    let stream_resolver = StreamResolver::new(&session);

    spawn_and_trace(resolve_stream(stream, stream_resolver));

    Ok((actor, player, room, session))
}
