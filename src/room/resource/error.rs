use thiserror::Error;

#[derive(Debug, Error)]
pub enum RoomResourceError {
    #[error("attempted to spawn task for room resource with no matcher attached")]
    NoMatcher,
    #[error("attempted to spawn task for room resource with no receiver attached")]
    NoReceiver,
}
