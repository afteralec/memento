use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthResourceError {
    #[error("attempted to spawn task for auth resource with no resolver attached")]
    NoResolver,
    #[error("attempted to spawn task for auth resource with no receiver attached")]
    NoReceiver,
}
