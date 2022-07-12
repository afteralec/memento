use crate::Id;
use thiserror::Error;

// @TODO: Move this NoMatcher and NoReceiver error to its own error type
#[derive(Debug, Error)]
pub enum RoomError {
    #[error("attempted to spawn task for room id {0:?} with no resolver attached")]
    NoResolver(Id),
    #[error("attempted to spawn task for room id {0:?} with no receiver attached")]
    NoReceiver(Id),
}
