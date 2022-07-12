use super::Actor;
use crate::Id;
use thiserror::Error;
use tokio::sync::oneshot;

#[derive(Debug, Error)]
pub enum ActorResourceEvent {
    #[error("ActorResource::GetActorById raised with id {} but channel is closed", .id)]
    GetActorById {
        id: Id,
        reply_sender: oneshot::Sender<ActorResourceReplyEvent>,
    },
}

#[derive(Debug, Error)]
pub enum ActorResourceReplyEvent {
    #[error("ActorResourceReply::GotActorById raised with id {0} but channel is closed")]
    GotActorById(Id, Actor),
    #[error("ActorResourceReply::NoActorAtId raised with id {0} but channel is closed")]
    NoActorAtId(Id),
}