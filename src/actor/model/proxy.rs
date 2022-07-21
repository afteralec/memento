use super::{
    error::ActorError,
    interface::{Actor, Gender},
};
use crate::{
    messaging::traits::Proxy, player::model::proxy::PlayerProxy, session::model::SessionEvent, Id,
};
use anyhow::{Error, Result};

#[derive(Debug, Clone)]
pub struct ActorProxy {
    id: Id,
    gender: Gender,
    short_description: String,
    keywords: Vec<String>,
    player: Option<PlayerProxy>,
}

impl Proxy<Actor> for ActorProxy {
    fn proxy(actor: &Actor) -> Self {
        ActorProxy {
            id: actor.id(),
            gender: actor.gender(),
            short_description: actor.short_description(),
            keywords: actor.keywords(),
            player: None,
        }
    }
}

impl ActorProxy {
    pub fn attach_player(&mut self, player: &PlayerProxy) -> Result<()> {
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

    pub fn detach_player(&mut self) -> Option<PlayerProxy> {
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
