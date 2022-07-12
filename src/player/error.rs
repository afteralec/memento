use crate::Id;
use thiserror::Error;

#[derive(Debug, Clone, Copy, Error)]
pub enum PlayerError {
    #[error("attempted a write to player {0}, but there is no writer attached")]
    NoWriter(Id),
    #[error("attempted to send to player {0}'s session, but there is no session sender attached")]
    NoSessionSender(Id),
    #[error("attempted to assign ownership of {1} to player {0}, but already owns {2}")]
    AlreadyAssigned(Id, Id, Id),
}
