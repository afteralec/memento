use super::{data::PlayerData, event::PlayerEvent, types::PlayerSink};
use crate::{keywords::util::Keywords, messaging::traits::{Resolver, Raise}, session::interface::Session, Id};
use anyhow::Result;
use async_trait::async_trait;
use futures::SinkExt;
use std::collections::HashMap;

#[readonly::make]
#[derive(Debug)]
pub struct PlayerResolver {
    pub(crate) state: PlayerState,
}

#[async_trait]
impl Resolver<PlayerEvent> for PlayerResolver {
    fn resolve_on(&mut self, _: PlayerEvent) -> Result<()> {
        unimplemented!(
            "Resolution for player input is required to be async; use resolve_async instead."
        );
    }

    async fn resolve_async(&mut self, event: PlayerEvent) -> Result<()> {
        match event {
            PlayerEvent::Write(item) => {
                if let Some(sink) = self.state.sink.as_mut() {
                    sink.send(item).await?;
                }
            }
            PlayerEvent::SendToSession(event) => {
                if let Some(session) = self.state.session.as_ref() {
                    session.raise(event)?;
                }
            }
            PlayerEvent::AttachSink(sink) => {
                // @TODO: Should this raise an error if there is already a sink registered?
                let _ = self.state.sink.insert(sink);
            }
        };

        Ok(())
    }
}

impl PlayerResolver {
    pub fn new(player: &PlayerData) -> Self {
        PlayerResolver {
            state: PlayerState::new(player),
        }
    }
}

#[readonly::make]
#[derive(Debug)]
pub struct PlayerState {
    pub(crate) id: Id,
    pub(crate) names: HashMap<Id, String>,
    pub(crate) keywords: Keywords,
    pub(crate) current_actor_id: Option<Id>,
    session: Option<Session>,
    sink: Option<PlayerSink>,
}

impl PlayerState {
    pub fn new(player: &PlayerData) -> Self {
        PlayerState {
            id: Id(player.id),
            // @TODO: Get this to load instead of being fresh on login
            names: HashMap::new(),
            // @TODO: Get this to load instead of being fresh on login
            keywords: Keywords::new(),
            current_actor_id: player.current_actor_id.map(|id| Id(id)),
            session: None,
            sink: None,
        }
    }
}
