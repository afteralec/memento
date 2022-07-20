use super::error::ActorError;
use crate::{player::model::Player, session::model::SessionEvent, Id};
use anyhow::{Error, Result};

#[derive(Debug, Clone)]
pub struct Actor {
    id: Id,
    gender: Gender,
    short_description: String,
    keywords: Vec<String>,
    player: Option<Player>,
}

impl Actor {
    pub fn new(id: u64, gender: &str, short_description: &str, keywords: &Vec<String>) -> Self {
        let gender = match &gender.to_lowercase()[..] {
            "nonbinary" => Gender::NonBinary,
            "male" => Gender::Male,
            "female" => Gender::Female,
            _ => {
                panic!(
                    "invalid gender at creation of actor with id: {}, got invalid string: {}",
                    id, gender
                );
            }
        };

        Actor {
            id: Id(id),
            gender,
            short_description: short_description.to_owned(),
            keywords: keywords.to_owned(),
            player: None,
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn gender(&self) -> Gender {
        self.gender
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

    pub fn keyword_iter(&self) -> std::slice::Iter<String> {
        self.keywords.iter()
    }

    pub fn attach_player(&mut self, player: &Player) -> Result<()> {
        if let Some(assigned_player) = &self.player {
            Err(Error::new(ActorError::PlayerAlreadyAttached(
                self.id,
                player.id(),
                assigned_player.id(),
            )))
        } else {
            let _ = self.player.insert(player.clone());

            Ok(())
        }
    }

    pub fn unattach_player(&mut self) -> Option<Player> {
        self.player.take()
    }

    pub fn write(&self, _string: &str) -> Result<()> {
        if let Some(_player) = &self.player {
            // player.write(string)?;

            Ok(())
        } else {
            Err(Error::new(ActorError::NoPlayer(self.id)))
        }
    }

    pub fn send(&self, _event: SessionEvent) -> Result<()> {
        if let Some(_player) = &self.player {
            Ok(())
        } else {
            Err(Error::new(ActorError::NoPlayer(self.id)))
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Gender {
    NonBinary,
    Male,
    Female,
}
