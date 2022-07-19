use super::types::PlayerSink;
use crate::messaging::traits::Resolver;
use anyhow::Result;
use async_trait::async_trait;
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
impl Resolver<String> for PlayerResolver {
    fn resolve_on(&mut self, input: String) -> Result<()> {
        panic!("Resolution for player input is required to be async; use resolve_async instead.");
    }

    async fn resolve_async(&mut self, input: String) -> Result<()> {
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
