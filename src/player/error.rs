use crate::Id;
use thiserror::Error;

#[derive(Debug, Clone, Copy, Error)]
pub enum PlayerError {
    #[error("attempted to assign ownership of {1} to player {0}, but already owns {2}")]
    AlreadyAssigned(Id, Id, Id),
}
