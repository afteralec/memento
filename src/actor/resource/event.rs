use super::{super::model::proxy::ActorProxy, types::ActorResourceReplySender};
use crate::Id;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ActorResourceEvent {
    #[error("ActorResource::GetActorById raised with id {0} but channel is closed")]
    GetActorById(Id, ActorResourceReplySender),
}

#[derive(Debug, Error)]
pub enum ActorResourceReplyEvent {
    #[error("ActorResourceReply::GotActorById raised with id {0} but channel is closed")]
    GotActorById(Id, ActorProxy),
    #[error("ActorResourceReply::NoActorAtId raised with id {0} but channel is closed")]
    NoActorAtId(Id),
}
