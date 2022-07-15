use super::{super::error::AuthStepError, auth_step, player_step};
use crate::{
    ActorResourceSender, AuthRequest, AuthResourceEvent, AuthResourceSender, AuthResponse,
    Credential, PlayerResourceEvent, PlayerResourceSender, RoomResourceSender,
};
use anyhow::{Error, Result};
use futures::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::{net::TcpStream, sync::oneshot};
use tokio_util::codec::{Framed, LinesCodec};

type ResourceSenders = (
    AuthResourceSender,
    PlayerResourceSender,
    ActorResourceSender,
    RoomResourceSender,
);

pub async fn create_session(
    (mut lines, addr): (Framed<TcpStream, LinesCodec>, SocketAddr),
    (auth_resource_sender, player_resource_sender, actor_resource_sender, room_resource_sender): ResourceSenders,
) -> Result<()> {

    // @TODO: Extract this to a login screen async function
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

    let (auth_reply_sender, auth_reply_receiver) = oneshot::channel();

    let result = auth_resource_sender.send(AuthResourceEvent::Request(
        AuthRequest::WithCredential(credential),
        auth_reply_sender,
    ))?;

    // @TODO: Use these auth ids to get good metrics/logs around the auth process
    let (id, player_id, actor_owned) = match auth_step(auth_reply_receiver).await? {
        AuthResponse::Authenticated {
            id,
            player_id,
            actor_owned,
        } => (id, player_id, actor_owned),
        AuthResponse::Forbidden => return Err(Error::new(AuthStepError::Forbidden)),
    };

    let (player_reply_sender, player_reply_receiver) = oneshot::channel();
    player_resource_sender.send(PlayerResourceEvent::GetPlayerById(
        player_id,
        player_reply_sender,
    ))?;
    let mut player = player_step(player_reply_receiver).await?;

    tracing::debug!("{:?}", player);

    Ok(())
}
