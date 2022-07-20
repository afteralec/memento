use super::{event::PlayerEvent, interface::Player, types::PlayerSender};
use crate::{
    keywords::util::Keywords,
    messaging::traits::{Proxy, Raise},
    Id,
};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug)]
pub struct PlayerProxy {
    id: Id,
    names: HashMap<Id, String>,
    keywords: Keywords,
    current_actor_id: Option<Id>,
    sender: PlayerSender,
}

impl Raise<PlayerEvent> for PlayerProxy {
    fn sender(&self) -> PlayerSender {
        self.sender.clone()
    }

    fn raise(&self, event: PlayerEvent) -> Result<()> {
        self.sender.send(event)?;

        Ok(())
    }
}

impl Proxy<Player> for PlayerProxy {
    fn proxy(player: &Player) -> Self {
        PlayerProxy {
            id: player.id(),
            names: player.names(),
            keywords: player.keywords(),
            current_actor_id: player.current_actor_id(),
            sender: player.sender(),
        }
    }
}

impl PlayerProxy {
    pub fn id(&self) -> Id {
        self.id
    }

    pub fn current_actor_id(&self) -> Option<Id> {
        self.current_actor_id
    }
}
