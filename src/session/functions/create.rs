use super::{
    super::error::{AuthStepError, PlayerStepError},
    steps::{actor_step, auth_step, player_step, room_step, session_step},
};
use crate::{
    actor::{interface::Actor, resource::event::ActorResourceEvent},
    auth::resource::event::{AuthRequest, AuthResourceEvent, AuthResponse, Credential},
    messaging::{functions::spawn_and_trace, traits::Raise},
    player::{interface::Player, resource::event::PlayerResourceEvent},
    room::{interface::Room, resource::event::RoomResourceEvent},
    server::resources::interface::Resources,
    session::{interface::Session, resource::event::SessionResourceEvent},
    stream::{resolve::resolve_stream, resolver::StreamResolver, types::Stream},
    Id,
};
use anyhow::{Error, Result};
use futures::{SinkExt, StreamExt};
use std::{net::SocketAddr};
use tokio::{net::TcpStream, sync::oneshot};
use tokio_util::codec::{Framed, LinesCodec};

pub async fn create_session(
    (mut lines, addr): (Framed<TcpStream, LinesCodec>, SocketAddr),
    resources: Resources,
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
    let (mut actor, player, room, session) = resource_steps(stream, credential, &resources).await?;

    // @TODO: Reimplement these setup operations on the interfaces
    // // Register the sink to the player
    player.attach_sink(sink)?;

    // Attach the player to the actor
    actor.attach_player(player.clone())?;

    // Attach the player to the session
    session.attach_player(player)?;

    // Send the room to the session
    session.new_room(room)?;

    // @TODO: Attach the player to the session and the actor to the room. Voila!

    Ok(())
}

// @TODO: Extract this to its own Resources struct and move the individual operations to the interfaces
async fn resource_steps(
    stream: Stream,
    credential: Credential,
    resources: &Resources,
) -> Result<(Actor, Player, Room, Session)> {
    // @TODO: Put this pattern of building oneshot channels and raising a resource event on the individual ResourceProxy instead
    let (auth_reply_sender, auth_reply_receiver) = oneshot::channel();
    resources.auth.raise(AuthResourceEvent::Request(
        AuthRequest::WithCredential(credential),
        auth_reply_sender,
    ))?;

    let player_id = match auth_step(auth_reply_receiver).await? {
        AuthResponse::Authenticated { player_id } => player_id,
        AuthResponse::Forbidden => return Err(Error::new(AuthStepError::Forbidden)),
    };

    let (player_reply_sender, player_reply_receiver) = oneshot::channel();
    resources.player.raise(PlayerResourceEvent::GetPlayerById(
        player_id,
        player_reply_sender,
    ))?;
    let player = player_step(player_reply_receiver).await?;

    let current_actor_id = if let Some(current_actor_id) = player.current_actor_id {
        current_actor_id
    } else {
        // @TODO: Redirect to a character creation/no-current-character interface
        return Err(Error::new(PlayerStepError::NoActorOwned(player.id)));
    };

    let (actor_reply_sender, actor_reply_receiver) = oneshot::channel();
    resources.actor.raise(ActorResourceEvent::GetActorById(
        current_actor_id,
        actor_reply_sender,
    ))?;
    let actor = actor_step(actor_reply_receiver).await?;

    let last_room_id = if let Some(last_room_id) = actor.last_room_id {
        last_room_id
    } else {
        // @TODO: This means this is probably a new character that never made it to a room;
        //   engage a flow to find the correct room to send them to
        Id::new(1)
    };

    let (room_reply_sender, room_reply_receiver) = oneshot::channel();
    resources.room.raise(RoomResourceEvent::GetRoomById(
        last_room_id,
        room_reply_sender,
    ))?;
    let room = room_step(room_reply_receiver).await?;

    let (session_reply_sender, session_reply_receiver) = oneshot::channel();
    resources
        .session
        .raise(SessionResourceEvent::NewSession(session_reply_sender))?;
    let session = session_step(session_reply_receiver).await?;
    let stream_resolver = StreamResolver::new(&session);

    spawn_and_trace(resolve_stream(stream, stream_resolver));

    Ok((actor, player, room, session))
}
