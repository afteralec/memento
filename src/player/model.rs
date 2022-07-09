use std::collections::HashMap;

use super::error;
use crate::{keywords, server, session, Id, Result};

pub type Names = HashMap<Id, String>;

#[derive(Debug, Clone)]
pub struct Player {
    id: Id,
    names: Names,
    keywords: keywords::Keywords,
    owns: Option<Id>,
    writer: Option<server::StreamWriter>,
    session_sender: Option<session::SessionSender>,
}

impl Player {
    pub fn id(&self) -> Id {
        self.id
    }

    pub fn write(&self, string: &str) -> Result<()> {
        if let Some(writer) = &self.writer {
            writer.send(string.to_owned())?;

            Ok(())
        } else {
            Err(Box::new(error::PlayerError::new(
                error::PlayerErrorKind::NoWriter,
                &format!("no writer available for player {}", self.id.val()),
            )))
        }
    }

    pub fn send(&self, event: session::SessionEvent) -> Result<()> {
        if let Some(session_sender) = &self.session_sender {
            session_sender.send(event)?;

            Ok(())
        } else {
            Err(Box::new(error::PlayerError::new(
                error::PlayerErrorKind::NoSessionSender,
                &format!("no writer available for player {}", self.id.val()),
            )))
        }
    }
}
