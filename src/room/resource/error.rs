use thiserror::Error;

#[derive(Debug, Error)]
pub enum RoomResourceError {
    #[error("attempted to spawn task for room resource with no resolver attached")]
    NoResolver,
    #[error("attempted to spawn task for room resource with no receiver attached")]
    NoReceiver,
}
