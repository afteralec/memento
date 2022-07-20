use super::{super::model::proxy::PlayerProxy, types::PlayerResourceReplySender};
use crate::Id;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlayerResourceEvent {
    #[error("PlayerResource::GetPlayerById raised with id {0} but channel is closed")]
    GetPlayerById(Id, PlayerResourceReplySender),
}

#[derive(Debug, Error)]
pub enum PlayerResourceReplyEvent {
    #[error("PlayerResourceReply::GotPlayerById raised with id {0} but channel is closed")]
    GotPlayerById(Id, PlayerProxy),
    #[error("PlayerResourceeReply::NoActorAtId raised with id {0} but channel is closed")]
    NoPlayerAtId(Id),
}
