use super::{event::PlayerEvent, types::PlayerSink};
use crate::messaging::traits::Resolver;
use anyhow::Result;
use async_trait::async_trait;
use futures::SinkExt;
use std::default::Default;

#[derive(Debug)]
pub struct PlayerResolver {
    state: PlayerState,
}

impl Default for PlayerResolver {
    fn default() -> Self {
        PlayerResolver {
            state: PlayerState::default(),
        }
    }
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
        };

        Ok(())
    }
}

#[derive(Debug)]
pub struct PlayerState {
    sink: Option<PlayerSink>,
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState { sink: None }
    }
}
