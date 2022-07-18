use crate::Id;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SessionResourceError<'a> {
    #[error("attempted to spawn task for session resource with no resolver attached")]
    NoResolver,
    #[error("attempted to spawn task for session resource but resolver misconfigured")]
    ResolverMisconfigured,
    #[error("attempted to spawn task for session resource with no receiver attached")]
    NoReceiver,
    #[error("attempted to resolve SessionResourceEvent, but was missing sender for {0}")]
    MissingResourceSender(&'a str),
}

#[derive(Debug, Error)]
pub enum AuthStepError {
    #[error("authentication denied")]
    Forbidden,
}

#[derive(Debug, Error)]
pub enum PlayerStepError {
    #[error("failed to get player for id {0}")]
    NoPlayerFound(Id),
}

#[derive(Debug, Error)]
pub enum ActorStepError {
    #[error("failed to get actor for id {0}")]
    NoActorFound(Id),
}

#[derive(Debug, Error)]
pub enum RoomStepError {
    #[error("failed to get actor for id {0}")]
    NoRoomFound(Id),
}
