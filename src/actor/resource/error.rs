use thiserror::Error;

#[derive(Debug, Error)]
pub enum ActorResourceError {
    #[error("attempted to spawn task for actor resource with no resolver attached")]
    NoResolver,
    #[error("attempted to spawn task for actor resource with no receiver attached")]
    NoReceiver,
}
