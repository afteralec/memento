use crate::messaging::traits::Resolver;
use anyhow::Result;
use async_trait::async_trait;
use std::default::Default;

#[derive(Debug)]
pub struct SessionStreamResolver {
    state: SessionStreamState,
}

impl Default for SessionStreamResolver {
    fn default() -> Self {
        SessionStreamResolver {
            state: SessionStreamState::default(),
        }
    }
}

#[async_trait]
impl Resolver<String> for SessionStreamResolver {
    fn resolve_on(&mut self, input: String) -> Result<()> {
        tracing::debug!("Got input: {}", input);
        Ok(())
    }

    async fn resolve_async(&mut self, _: String) -> Result<()> {
        unimplemented!(
            "Async resolution not supported for SessionStreamResolver, use resolve_on instead."
        );
    }
}

// @TODO: This is where the layer that parses player input should go
#[derive(Debug)]
pub struct SessionStreamState {}

impl Default for SessionStreamState {
    fn default() -> Self {
        SessionStreamState {}
    }
}
