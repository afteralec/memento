use super::{Player, PlayerResourceEvent, PlayerResourceReplyEvent};
use crate::{messaging::ResolverMut, Id};
use anyhow::Result;
use std::{collections::HashMap, default::Default};

#[derive(Debug)]
pub struct PlayerResourceResolver {
    state: PlayerResourceState,
}

impl Default for PlayerResourceResolver {
    fn default() -> Self {
        PlayerResourceResolver {
            state: PlayerResourceState::default(),
        }
    }
}

impl ResolverMut<PlayerResourceEvent> for PlayerResourceResolver {
    fn resolve_on(&mut self, event: PlayerResourceEvent) -> Result<()> {
        match event {
            PlayerResourceEvent::GetPlayerById(id, reply_sender) => {
                if let Some(player) = self.state.players.get(&id) {
                    reply_sender
                        .send(PlayerResourceReplyEvent::GotPlayerById(id, player.clone()))?;
                } else {
                    reply_sender.send(PlayerResourceReplyEvent::NoPlayerAtId(id))?;
                }

                Ok(())
            }
        }
    }
}

impl PlayerResourceResolver {
    pub fn new(player_iter: impl Iterator<Item = Player>) -> Self {
        PlayerResourceResolver {
            state: PlayerResourceState::new(player_iter),
        }
    }
}

#[derive(Debug)]
pub struct PlayerResourceState {
    players: HashMap<Id, Player>,
}

impl Default for PlayerResourceState {
    fn default() -> Self {
        PlayerResourceState {
            players: HashMap::default(),
        }
    }
}

impl PlayerResourceState {
    pub fn new(player_iter: impl Iterator<Item = Player>) -> Self {
        let players = player_iter.fold(HashMap::new(), |mut players, player| {
            players.insert(player.id(), player);

            players
        });

        PlayerResourceState { players }
    }
}
