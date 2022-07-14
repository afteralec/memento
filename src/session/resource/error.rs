use thiserror::Error;

#[derive(Debug, Error)]
pub enum SessionResourceError<'a> {
    #[error("attempted to spawn task for session resource with no resolver attached")]
    NoResolver,
    #[error("attempted to spawn task for session resource but resolver misconfigured")]
    ResolverMisconfigured,
    #[error("attempted to spawn task for session resource with no receiver attached")]
    NoReceiver,
    #[error("attempted to resolve SessionResourceEvent, but was missing sender for {0}")]
    MissingResourceSender(&'a str),
}
