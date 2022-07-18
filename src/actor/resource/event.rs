use super::{super::model::Actor, ActorResourceReplySender};
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
    GotActorById(Id, Actor),
    #[error("ActorResourceReply::NoActorAtId raised with id {0} but channel is closed")]
    NoActorAtId(Id),
}
