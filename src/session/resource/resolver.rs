use super::{
    super::model::Session,
    event::{SessionResourceEvent, SessionResourceReplyEvent},
    functions::create::create_session,
};
use crate::{
    messaging::traits::{Detach, ProvideProxy, Resolver, Spawn},
    server::resource_proxy::ResourceProxies,
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

impl Spawn for SessionResourceResolver {}

#[async_trait]
impl Resolver<SessionResourceEvent> for SessionResourceResolver {
    fn resolve_on(&mut self, event: SessionResourceEvent) -> Result<()> {
        match event {
            SessionResourceEvent::CreateSession { lines, addr } => {
                let resource_proxies = self.state.resource_proxies.as_ref().ok_or_else(|| {
                    panic!("No Resource Proxies interface present on SessionResource");
                }).unwrap();

                let _ = self.spawn_and_trace(create_session((lines, addr), resource_proxies.clone()));

                Ok(())
            }
            SessionResourceEvent::NewSession(stream, reply_sender) => {
                let id = self.state.id_gen.next_id();
                let mut session = Session::new(id, stream);
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

impl SessionResourceResolver {
    pub fn configured(&self) -> bool {
        self.state.resource_proxies.is_some()
    }

    pub fn set_resource_proxies(&mut self, resource_proxies: ResourceProxies) {
        self.state.set_resource_proxies(resource_proxies);
    }
}

#[derive(Debug)]
pub struct SessionResourceState {
    resource_proxies: Option<ResourceProxies>,
    sessions: HashMap<Id, Session>,
    id_gen: SequenceGenerator,
}

impl Default for SessionResourceState {
    fn default() -> Self {
        SessionResourceState {
            resource_proxies: None,
            sessions: HashMap::default(),
            id_gen: SequenceGenerator::default(),
        }
    }
}

impl SessionResourceState {
    pub fn set_resource_proxies(&mut self, resource_proxies: ResourceProxies) {
        let _ = self.resource_proxies.insert(resource_proxies);
    }
}
