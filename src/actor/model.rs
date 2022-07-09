use super::error;
use crate::{player, session, Id, Result};

#[derive(Debug, Clone)]
pub struct Actor {
    id: Id,
    gender: Gender,
    short_description: String,
    keywords: Vec<String>,
    player: Option<player::Player>,
}

impl Actor {
    pub fn new(id: u64, gender: &str, short_description: &str, keywords: &Vec<String>) -> Self {
        let gender = match gender {
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
            keywords: keywords.clone(),
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

    pub fn attach_player(&mut self, player: player::Player) -> Result<()> {
        if self.player.is_some() {
            Err(Box::new(error::ActorError::new(
                error::ActorErrorKind::PlayerAlreadyAttached,
                &format!(
                    "attempted to attach player to actor id {}, but player {} is already attached",
                    self.id.val(),
                    player.id().val()
                ),
            )))
        } else {
            let _ = self.player.insert(player);

            Ok(())
        }
    }

    pub fn unattach_player(&mut self) -> Option<player::Player> {
        self.player.take()
    }

    pub fn write(&self, string: &str) -> Result<()> {
        if let Some(player) = &self.player {
            player.write(string)?;

            Ok(())
        } else {
            Err(Box::new(error::ActorError::new(
                error::ActorErrorKind::NoPlayer,
                &format!("actor {} has no player attached", &self.id.val()),
            )))
        }
    }

    pub fn send(&self, event: session::SessionEvent) -> Result<()> {
        if let Some(player) = &self.player {
            player.send(event)?;

            Ok(())
        } else {
            Err(Box::new(error::ActorError::new(
                error::ActorErrorKind::NoPlayer,
                &format!("actor {} has no player attached", &self.id.val()),
            )))
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Gender {
    NonBinary,
    Male,
    Female,
}
