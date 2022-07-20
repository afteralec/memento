use thiserror::Error;

#[derive(Debug, Error)]
pub enum SpawnError {
    #[error("attempted to spawn task for {0:?} with no resolver attached")]
    NoResolver(String),
    #[error("attempted to spawn task for {0:?} with no receiver attached")]
    NoReceiver(String),
    #[error("attempted to spawn task for {0:?} but resolver misconfigured")]
    ResolverMisconfigured(String),
}
