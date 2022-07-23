use crate::Id;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SessionResourceError {}

#[derive(Debug, Error)]
pub enum AuthStepError {
    #[error("authentication denied")]
    Forbidden,
}

#[derive(Debug, Error)]
pub enum PlayerStepError {
    #[error("failed to get player for id {0}")]
    NoPlayerFound(Id),
    #[error("player id {0} has no currently owned actor")]
    NoActorOwned(Id),
    #[error("received wrong reply for player resource operation")]
    WrongReplyReceived,
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
