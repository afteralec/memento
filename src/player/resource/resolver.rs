use super::{
    super::{data::PlayerData, resolver::PlayerResolver, types::PlayerMessenger},
    event::{PlayerResourceEvent, PlayerResourceReplyEvent},
};
use crate::{
    messaging::traits::{Detach, Provide, Resolver},
    Id,
};
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;

#[derive(Debug)]
pub struct PlayerResourceResolver {
    state: PlayerResourceState,
}

#[async_trait]
impl Resolver<PlayerResourceEvent> for PlayerResourceResolver {
    fn resolve_on(&mut self, event: PlayerResourceEvent) -> Result<()> {
        match event {
            PlayerResourceEvent::GetPlayerById(id, reply_sender) => {
                if let Some(player) = self.state.messengers.get(&id) {
                    reply_sender.send(PlayerResourceReplyEvent::GotPlayerById(
                        id,
                        player.provide(),
                    ))?;
                } else {
                    reply_sender.send(PlayerResourceReplyEvent::NoPlayerAtId(id))?;
                }

                Ok(())
            }
            PlayerResourceEvent::DetachPlayerById(id, reply_sender) => {
                if let Some(player) = self.state.messengers.get_mut(&id) {
                    match player.detach() {
                        Ok(_) => {
                            reply_sender.send(PlayerResourceReplyEvent::PlayerDetached(id))?;
                        }
                        Err(_) => {
                            reply_sender
                                .send(PlayerResourceReplyEvent::PlayerAlreadyDetached(id))?;
                        }
                    };
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
    pub fn new(player_iter: impl Iterator<Item = PlayerData>) -> Self {
        PlayerResourceResolver {
            state: PlayerResourceState::new(player_iter),
        }
    }
}

#[derive(Debug)]
pub struct PlayerResourceState {
    players: HashMap<Id, PlayerData>,
    messengers: HashMap<Id, PlayerMessenger>,
}

impl PlayerResourceState {
    pub fn new(player_iter: impl Iterator<Item = PlayerData>) -> Self {
        let (players, messengers) = player_iter.fold(
            (HashMap::new(), HashMap::new()),
            |(mut players, mut messengers), player| {
                let messenger_name = format!("player {}", player.id);
                players.insert(Id(player.id), player);
                messengers.insert(
                    Id(player.id),
                    PlayerMessenger::new(&messenger_name, PlayerResolver::new(&player)),
                );

                (players, messengers)
            },
        );

        PlayerResourceState {
            players,
            messengers,
        }
    }
}
