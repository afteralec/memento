use super::{
    super::model::Player,
    event::{PlayerResourceEvent, PlayerResourceReplyEvent},
};
use crate::{messaging::traits::{ProvideProxy, Resolver}, Id};
use anyhow::Result;
use async_trait::async_trait;
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

#[async_trait]
impl Resolver<PlayerResourceEvent> for PlayerResourceResolver {
    fn resolve_on(&mut self, event: PlayerResourceEvent) -> Result<()> {
        match event {
            PlayerResourceEvent::GetPlayerById(id, reply_sender) => {
                if let Some(player) = self.state.players.get(&id) {
                    reply_sender
                        .send(PlayerResourceReplyEvent::GotPlayerById(id, player.proxy()))?;
                } else {
                    reply_sender.send(PlayerResourceReplyEvent::NoPlayerAtId(id))?;
                }

                Ok(())
            }
        }
    }

    async fn resolve_async(&mut self, _: PlayerResourceEvent) -> Result<()> {
        unimplemented!("Async resolution isn't needed for PlayerResource, use resolve_on instead");
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
