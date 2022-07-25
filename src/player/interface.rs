use super::{
    error::PlayerError,
    event::PlayerEvent,
    types::{PlayerMessenger, PlayerSender, PlayerSink},
};
use crate::{
    keywords::util::Keywords,
    messaging::traits::{Interface, Raise},
    Id,
};
use anyhow::{Error, Result};
use std::{collections::HashMap, fmt::Debug};

#[readonly::make]
#[derive(Clone, Debug)]
pub struct Player {
    pub(crate) id: Id,
    pub(crate) names: HashMap<Id, String>,
    pub(crate) keywords: Keywords,
    pub(crate) current_actor_id: Option<Id>,
    sender: PlayerSender,
}

impl Raise<PlayerEvent> for Player {
    fn raise(&self, event: PlayerEvent) -> Result<()> {
        self.sender.send(event)?;

        Ok(())
    }
}

impl Interface<PlayerMessenger> for Player {
    fn of(m: &PlayerMessenger) -> Self {
        let state = &m.resolver.as_ref().unwrap().state;

        Player {
            id: state.id,
            names: state.names.clone(),
            keywords: state.keywords.clone(),
            current_actor_id: state.current_actor_id,
            sender: m.sender.clone(),
        }
    }
}

impl Player {
    pub fn write(&self, string: &str) -> Result<()> {
        self.raise(PlayerEvent::Write(string.to_owned()))?;

        Ok(())
    }

    pub fn attach_sink(&self, sink: PlayerSink) -> Result<()> {
        self.raise(PlayerEvent::AttachSink(sink))?;

        Ok(())
    }

    pub fn assign_actor(&mut self, player_id: &Id, id: &Id) -> Result<()> {
        if let Some(assigned_id) = &self.current_actor_id {
            Err(Error::new(PlayerError::AlreadyAssigned(
                *player_id,
                *id,
                *assigned_id,
            )))
        } else {
            let _ = self.current_actor_id.insert(*id);
            Ok(())
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
