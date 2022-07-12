use crate::Id;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("attempted to spawn task for session id {0:?} with no resolver attached")]
    NoResolver(Id),
    #[error("attempted to spawn task for session id {0:?} with no receiver attached")]
    NoReceiver(Id),
}

// @TODO Move this kind of StateError to its own error type
#[derive(Debug, Error)]
pub enum SessionStateError {
    #[error("attempted to set id to {0:?} for session, but id was already valid with {1:?}")]
    IdAlreadyValid(Id, Id),
}
