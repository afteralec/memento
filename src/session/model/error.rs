use crate::Id;
use thiserror::Error;

// @TODO Move this kind of StateError to its own error type
#[derive(Debug, Error)]
pub enum SessionStateError {
    #[error("attempted to set id to {0:?} for session, but id was already valid with {1:?}")]
    IdAlreadyValid(Id, Id),
}
