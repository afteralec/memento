use super::{
    error::PlayerError,
    event::PlayerEvent,
    proxy::PlayerProxy,
    resolver::PlayerResolver,
    types::{PlayerReceiver, PlayerSender},
};
use crate::{
    keywords::util::Keywords,
    messaging::{
        error::DetachError,
        functions::resolve_receiver,
        traits::{Detach, ProvideProxy, Raise, Spawn},
    },
    Id,
};
use anyhow::{Error, Result};
use std::{collections::HashMap, default::Default};
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct Player {
    id: Id,
    names: HashMap<Id, String>,
    keywords: Keywords,
    current_actor_id: Option<Id>,
    sender: PlayerSender,
    receiver: Option<PlayerReceiver>,
    resolver: Option<PlayerResolver>,
}

impl Clone for Player {
    fn clone(&self) -> Self {
        Player {
            id: self.id.clone(),
            names: self.names.clone(),
            keywords: self.keywords.clone(),
            current_actor_id: self.current_actor_id.clone(),
            sender: self.sender.clone(),
            receiver: None,
            resolver: None,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();

        Player {
            id: Id::new(0),
            names: HashMap::default(),
            keywords: Keywords::default(),
            current_actor_id: None,
            sender,
            receiver: Some(receiver),
            resolver: Some(PlayerResolver::default()),
        }
    }
}

impl Raise<PlayerEvent> for Player {
    fn sender(&self) -> PlayerSender {
        self.sender.clone()
    }

    fn raise(&self, event: PlayerEvent) -> Result<()> {
        self.sender.send(event)?;

        Ok(())
    }
}

impl Spawn for Player {}

impl Detach for Player {
    fn detach(&mut self) -> Result<()> {
        let receiver = self
            .receiver
            .take()
            .ok_or_else(|| DetachError::NoReceiver(format!("player id {}", self.id)))?;

        let resolver = self
            .resolver
            .take()
            .ok_or_else(|| DetachError::NoResolver(format!("player id {}", self.id)))?;

        self.spawn_and_trace(resolve_receiver(receiver, resolver));

        Ok(())
    }
}

impl ProvideProxy<PlayerProxy> for Player {}

impl Player {
    pub fn new(id: i64) -> Self {
        Player {
            id: Id::new(id),
            ..Default::default()
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn names(&self) -> HashMap<Id, String> {
        self.names.clone()
    }

    pub fn keywords(&self) -> Keywords {
        self.keywords.clone()
    }

    pub fn current_actor_id(&self) -> Option<Id> {
        self.current_actor_id.clone()
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

    pub fn get_current_actor_id(&self) -> Option<Id> {
        self.current_actor_id
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
