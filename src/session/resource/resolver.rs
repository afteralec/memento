use super::{
    super::{
        functions::create::create_session, resolver::SessionResolver, types::SessionMessenger,
    },
    event::{SessionResourceEvent, SessionResourceReplyEvent},
};
use crate::{
    messaging::{
        functions::spawn_and_trace,
        traits::{Detach, Provide, Resolver},
    },
    Id,
};
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;
use unique_id::{sequence::SequenceGenerator, Generator};

#[derive(Debug)]
pub struct SessionResourceResolver {
    state: SessionResourceState,
}

#[async_trait]
impl Resolver<SessionResourceEvent> for SessionResourceResolver {
    fn resolve_on(&mut self, event: SessionResourceEvent) -> Result<()> {
        match event {
            SessionResourceEvent::CreateSession {
                lines,
                addr,
                resources,
            } => {
                spawn_and_trace(create_session((lines, addr), resources));

                Ok(())
            }
            SessionResourceEvent::NewSession(reply_sender) => {
                let id = self.state.id_gen.next_id();

                let name = format!("session {}", id);
                let mut messenger = SessionMessenger::new(&name, SessionResolver::new());

                messenger.detach()?;

                let session = messenger.provide();
                self.state.messengers.insert(Id(id), messenger);

                reply_sender.send(SessionResourceReplyEvent::NewSession(session))?;

                Ok(())
            }
        }
    }

    async fn resolve_async(&mut self, _: SessionResourceEvent) -> Result<()> {
        unimplemented!(
            "Async resolution not supported for SessionResourceResolver, use resolve_on instead."
        );
    }
}

impl SessionResourceResolver {
    pub fn new() -> Self {
        SessionResourceResolver {
            state: SessionResourceState::new(),
        }
    }
}

#[readonly::make]
#[derive(Debug)]
pub struct SessionResourceState {
    pub(crate) messengers: HashMap<Id, SessionMessenger>,
    pub(crate) id_gen: SequenceGenerator,
}

impl SessionResourceState {
    pub fn new() -> Self {
        SessionResourceState {
            messengers: HashMap::new(),
            id_gen: SequenceGenerator::default(),
        }
    }
}
