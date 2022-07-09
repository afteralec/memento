use std::fmt;

#[derive(Debug)]
pub struct PlayerError {
    kind: PlayerErrorKind,
    message: String,
}

impl PlayerError {
    pub fn new(kind: PlayerErrorKind, message: &str) -> Self {
        PlayerError {
            kind,
            message: message.to_owned(),
        }
    }

    pub fn kind(&self) -> PlayerErrorKind {
        self.kind
    }
}

impl fmt::Display for PlayerError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{:?}:{}", self.kind, self.message)
    }
}

impl std::error::Error for PlayerError {}

#[derive(Debug, Clone, Copy)]
pub enum PlayerErrorKind {
    NoWriter,
    NoSessionSender,
}
