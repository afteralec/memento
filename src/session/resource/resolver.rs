use super::{
    super::model::Session,
    event::{SessionResourceEvent, SessionResourceReplyEvent},
    functions::create::create_session,
};
use crate::{
    messaging::{
        functions::spawn_and_trace,
        traits::{Detach, ProvideProxy, Resolver},
    },
    Id,
};
use anyhow::Result;
use async_trait::async_trait;
use std::{collections::HashMap, default::Default};
use unique_id::{sequence::SequenceGenerator, Generator};

#[derive(Debug)]
pub struct SessionResourceResolver {
    state: SessionResourceState,
}

impl Default for SessionResourceResolver {
    fn default() -> Self {
        SessionResourceResolver {
            state: SessionResourceState::default(),
        }
    }
}

#[async_trait]
impl Resolver<SessionResourceEvent> for SessionResourceResolver {
    fn resolve_on(&mut self, event: SessionResourceEvent) -> Result<()> {
        match event {
            SessionResourceEvent::CreateSession {
                lines,
                addr,
                resource_proxies,
            } => {
                spawn_and_trace(create_session((lines, addr), resource_proxies));

                Ok(())
            }
            SessionResourceEvent::NewSession(reply_sender) => {
                let id = self.state.id_gen.next_id();
                let mut session = Session::new(id);
                session.detach()?;

                let session_proxy = session.proxy();

                self.state.sessions.insert(session.id(), session);

                reply_sender.send(SessionResourceReplyEvent::NewSession(session_proxy))?;

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

#[derive(Debug)]
pub struct SessionResourceState {
    sessions: HashMap<Id, Session>,
    id_gen: SequenceGenerator,
}

impl Default for SessionResourceState {
    fn default() -> Self {
        SessionResourceState {
            sessions: HashMap::default(),
            id_gen: SequenceGenerator::default(),
        }
    }
}
