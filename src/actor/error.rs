use std::fmt;

#[derive(Debug)]
pub struct ActorError {
    kind: ActorErrorKind,
    message: String,
}

impl ActorError {
    pub fn new(kind: ActorErrorKind, message: &str) -> Self {
        ActorError {
            kind,
            message: message.to_owned(),
        }
    }

    pub fn kind(&self) -> ActorErrorKind {
        self.kind
    }
}

impl fmt::Display for ActorError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{:?}:{}", self.kind, self.message)
    }
}

impl std::error::Error for ActorError {}

#[derive(Debug, Clone, Copy)]
pub enum ActorErrorKind {
    NoPlayer,
    PlayerAlreadyAttached,
}
