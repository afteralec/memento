use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlayerResourceError {
    #[error("attempted to spawn task for player resource with no resolver attached")]
    NoResolver,
    #[error("attempted to spawn task for player resource with no receiver attached")]
    NoReceiver,
}
