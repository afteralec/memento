use super::{Player, PlayerResourceReplySender};
use crate::Id;
use thiserror::Error;
use tokio::sync::oneshot;

#[derive(Debug, Error)]
pub enum PlayerResourceEvent {
    #[error("PlayerResource::GetPlayerById raised with id {0} but channel is closed")]
    GetPlayerById(Id, oneshot::Sender<PlayerResourceReplyEvent>),
}

#[derive(Debug, Error)]
pub enum PlayerResourceReplyEvent {
    #[error("PlayerResourceReply::GotPlayerById raised with id {0} but channel is closed")]
    GotPlayerById(Id, Player),
    #[error("PlayerResourceeReply::NoActorAtId raised with id {0} but channel is closed")]
    NoPlayerAtId(Id),
}
