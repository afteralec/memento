use crate::Id;
use thiserror::Error;

#[derive(Debug, Clone, Copy, Error)]
pub enum ActorError {
    #[error("attempted to access player for actor {0}, but no player attached")]
    NoPlayer(Id),
    #[error("cannot attach a player {1} to actor {0}, player {2} already attached")]
    PlayerAlreadyAttached(Id, Id, Id),
}
