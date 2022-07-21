use thiserror::Error;

#[derive(Debug, Error)]
pub enum DetachError {
    #[error("attempted to spawn task for {0:?} with no resolver attached")]
    NoResolver(String),
    #[error("attempted to spawn task for {0:?} with no receiver attached")]
    NoReceiver(String),
    #[error("attempted to spawn task for {0:?} with no stream attached")]
    NoStream(String),
    #[error("attempted to spawn task for {0:?} with no stream resolver attached")]
    NoStreamResolver(String),
    #[error("attempted to spawn task for {0:?} but resolver misconfigured")]
    ResolverMisconfigured(String),
}
