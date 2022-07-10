use crate::Id;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RoomError {
    #[error("attempted to spawn task for room id {0:?} with no matcher attached")]
    NoMatcher(Id),
    #[error("attempted to spawn task for room id {0:?} with no receiver attached")]
    NoReceiver(Id),
}
