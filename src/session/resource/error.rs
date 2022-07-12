use thiserror::Error;

#[derive(Debug, Error)]
pub enum SessionResourceError {
    #[error("attempted to spawn task for session resource with no matcher attached")]
    NoMatcher,
    #[error("attempted to spawn task for session resource with no receiver attached")]
    NoReceiver,
}
