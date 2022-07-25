use super::{
    error::ActorError,
    event::ActorEvent,
    types::{ActorMessenger, ActorSender},
};
use crate::{
    messaging::traits::{Interface, Raise},
    player::interface::Player,
    Id,
};
use anyhow::{Error, Result};

#[readonly::make]
#[derive(Debug)]
pub struct Actor {
    pub(crate) id: Id,
    pub(crate) gender: Gender,
    pub(crate) short_description: String,
    pub(crate) keywords: Vec<String>,
    pub(crate) last_room_id: Option<Id>,
    player: Option<Player>,
    sender: ActorSender,
}

impl Raise<ActorEvent> for Actor {
    fn raise(&self, event: ActorEvent) -> Result<()> {
        self.sender.send(event)?;

        Ok(())
    }
}

impl Interface<ActorMessenger> for Actor {
    fn of(m: &ActorMessenger) -> Self {
        let state = &m.resolver.as_ref().unwrap().state;

        Actor {
            id: state.id,
            gender: state.gender,
            short_description: state.short_description.clone(),
            keywords: state.keywords.clone(),
            // @TODO: Pull this up from load
            last_room_id: Some(Id(1)),
            player: None,
            sender: m.sender.clone(),
        }
    }
}

impl Actor {
    pub fn attach_player(&mut self, player: Player) -> Result<()> {
        if let Some(assigned_player) = self.player.as_ref() {
            Err(Error::new(ActorError::PlayerAlreadyAttached(
                self.id,
                player.id,
                assigned_player.id,
            )))
        } else {
            let _ = self.player.insert(player);

            Ok(())
        }
    }

    pub fn write(&mut self, string: &str) -> Result<()> {
        if let Some(player) = self.player.as_ref() {
            player.write(string)?;

            Ok(())
        } else {
            Err(Error::new(ActorError::NoPlayer(self.id)))
        }
    }

    pub fn self_gendered(&self) -> String {
        match self.gender {
            Gender::Male => "himself",
            Gender::Female => "herself",
            Gender::NonBinary => "themself",
        }
        .to_owned()
    }

    pub fn possessive_gendered(&self) -> String {
        match self.gender {
            Gender::Male => "his",
            Gender::Female => "her",
            Gender::NonBinary => "their",
        }
        .to_owned()
    }

    pub fn short_desc_the(&self) -> String {
        format!("the {}", &self.short_description)
    }

    pub fn short_desc_a(&self) -> String {
        format!("a {}", &self.short_description)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Gender {
    NonBinary,
    Male,
    Female,
}

impl From<&str> for Gender {
    fn from(gender: &str) -> Self {
        match &gender.to_lowercase()[..] {
            "nonbinary" => Gender::NonBinary,
            "male" => Gender::Male,
            "female" => Gender::Female,
            _ => {
                // @TODO: Move this panic into actor loading and have the constructor require a valid Gender enum
                panic!("invalid gender; got invalid string: {}", gender);
            }
        }
    }
}
