use super::error;
use crate::{keywords, server, session, Id};

use anyhow::{Error, Result};
use std::collections::HashMap;

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
            Err(Error::new(error::PlayerError::AlreadyAssigned(
                self.id, *id, *owned_id,
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
            Err(Error::new(error::PlayerError::NoWriter(self.id)))
        }
    }

    pub fn send(&self, event: session::SessionEvent) -> Result<()> {
        if let Some(session_sender) = &self.session_sender {
            session_sender.send(event)?;

            Ok(())
        } else {
            Err(Error::new(error::PlayerError::NoSessionSender(self.id)))
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
