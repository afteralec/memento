use crate::{
    messaging::traits::{Raise, Resolver},
    session::{event::SessionEvent, interface::Session},
};
use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug)]
pub struct StreamResolver {
    state: StreamState,
}

#[async_trait]
impl Resolver<String> for StreamResolver {
    fn resolve_on(&mut self, input: String) -> Result<()> {
        if let Some(session) = self.state.session.as_ref() {
            session.raise(SessionEvent::Input(input))?;
        }
        Ok(())
    }

    async fn resolve_async(&mut self, _: String) -> Result<()> {
        unimplemented!("async resolution not supported for StreamResolver, use resolve_on instead");
    }
}

impl StreamResolver {
    pub fn new(session: &Session) -> Self {
        StreamResolver {
            state: StreamState::new(session),
        }
    }
}

// @TODO: This is where the layer that parses player input should go
#[derive(Debug)]
pub struct StreamState {
    session: Option<Session>,
}

impl StreamState {
    pub fn new(session: &Session) -> Self {
        StreamState {
            session: Some(session.clone()),
        }
    }
}
