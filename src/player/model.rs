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

    pub fn assign_ownership(&mut self, id: &Id) -> Result<()> {
        if let Some(owned_id) = &self.owns {
            Err(Box::new(error::PlayerError::new(
                error::PlayerErrorKind::AlreadyAssigned,
                &format!("attempted to assign ownership of actor id {} to player id {}, but already owns actor id {}", id.val(), self.id.val(), owned_id.val())
            )))
        } else {
            let _ = self.owns.insert(*id);
            Ok(())
        }
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
                &format!("no session sender available for player {}", self.id.val()),
            )))
        }
    }

    pub fn add_name(&mut self, id: &Id, name: &str) {
        self.names.insert(*id, name.to_owned().to_lowercase());
    }

    pub fn assign_keyword(&mut self, id: &Id, keyword: &str, sdesc: &str) {
        self.add_keyword(id, keyword);
        self.add_sdesc_to_id(id, sdesc);
    }

    pub fn add_keyword(&mut self, id: &Id, keyword: &str) {
        self.keywords.add_keyword(id, keyword);
    }

    pub fn add_sdesc_to_id(&mut self, id: &Id, sdesc: &str) {
        self.keywords.add_sdesc_to_id(id, sdesc);
    }
}
