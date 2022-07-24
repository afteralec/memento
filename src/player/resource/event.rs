use super::{super::interface::Player, types::PlayerResourceReplySender};
use crate::Id;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlayerResourceEvent {
    #[error("PlayerResource::GetPlayerById raised with id {0} but channel is closed")]
    GetPlayerById(Id, PlayerResourceReplySender),
    #[error("PlayerResource::DetachPlayerById raised with id {0} but channel is closed")]
    DetachPlayerById(Id, PlayerResourceReplySender),
}

#[derive(Debug, Error)]
pub enum PlayerResourceReplyEvent {
    #[error("PlayerResourceReply::GotPlayerById raised with id {0} but channel is closed")]
    GotPlayerById(Id, Player),
    #[error("PlayerResourceReply::PlayerDetached raised with id {0} but channel is closed")]
    PlayerDetached(Id),
    #[error("PlayerResourceReply::PlayerAlreadyDetached raised with id {0} but channel is closed")]
    PlayerAlreadyDetached(Id),
    #[error("PlayerResourceeReply::NoActorAtId raised with id {0} but channel is closed")]
    NoPlayerAtId(Id),
}
