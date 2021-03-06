use super::{super::interface::Actor, types::ActorResourceReplySender};
use crate::Id;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ActorResourceEvent {
    #[error("ActorResource::GetActorById raised with id {0} but channel is closed")]
    GetActorById(Id, ActorResourceReplySender),
}

#[derive(Debug, Error)]
pub enum ActorResourceReplyEvent {
    #[error("ActorResourceReply::GotActorById raised with id {0} but channel is closed")]
    GotActorById(Id, Arc<Actor>),
    #[error("ActorResourceReply::NoActorAtId raised with id {0} but channel is closed")]
    NoActorAtId(Id),
}
