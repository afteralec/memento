use super::{
    error::PlayerError,
    types::{PlayerReceiver, PlayerSink, PlayerWriter},
};
use crate::{
    keywords::util::Keywords,
    session::model::{SessionEvent, SessionSender},
    Id,
};
use anyhow::{Error, Result};
use std::{collections::HashMap, default::Default};

pub type Names = HashMap<Id, String>;

#[derive(Debug)]
pub struct Player {
    id: Id,
    names: Names,
    keywords: Keywords,
    current_actor_id: Option<Id>,
    sink: Option<PlayerSink>,
    writer: Option<PlayerWriter>,
    receiver: Option<PlayerReceiver>,
    session_sender: Option<SessionSender>,
}

impl Clone for Player {
    fn clone(&self) -> Self {
        Player {
            id: self.id.clone(),
            names: self.names.clone(),
            keywords: self.keywords.clone(),
            current_actor_id: self.current_actor_id.clone(),
            sink: None,
            writer: None,
            receiver: None,
            session_sender: None,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Player {
            id: Id::new(0),
            names: Names::default(),
            keywords: Keywords::default(),
            current_actor_id: None,
            sink: None,
            writer: None,
            receiver: None,
            session_sender: None,
        }
    }
}

impl Player {
    pub fn new(id: u64) -> Self {
        Player {
            id: Id::new(id),
            ..Default::default()
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn assign_ownership(&mut self, id: &Id) -> Result<()> {
        if let Some(owned_id) = &self.current_actor_id {
            Err(Error::new(PlayerError::AlreadyAssigned(
                self.id, *id, *owned_id,
            )))
        } else {
            let _ = self.current_actor_id.insert(*id);
            Ok(())
        }
    }

    pub fn attach_sink(&mut self, sink: PlayerSink) {
        let _ = self.sink.insert(sink);
    }

    pub fn attach_writer(&mut self, writer: PlayerWriter) {
        let _ = self.writer.insert(writer);
    }

    pub fn get_current_actor_id(&self) -> Option<Id> {
        self.current_actor_id
    }

    pub fn write(&self, string: &str) -> Result<()> {
        if let Some(writer) = &self.writer {
            writer.send(string.to_owned())?;

            Ok(())
        } else {
            Err(Error::new(PlayerError::NoWriter(self.id)))
        }
    }

    pub fn send(&self, event: SessionEvent) -> Result<()> {
        if let Some(session_sender) = &self.session_sender {
            session_sender.send(event)?;

            Ok(())
        } else {
            Err(Error::new(PlayerError::NoSessionSender(self.id)))
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
