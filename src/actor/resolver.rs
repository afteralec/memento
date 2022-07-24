use super::{data::ActorData, event::ActorEvent, interface::Gender};
use crate::{messaging::traits::Resolver, Id};
use anyhow::Result;
use async_trait::async_trait;

#[readonly::make]
#[derive(Debug)]
pub struct ActorResolver {
    pub(crate) state: ActorState,
}

#[async_trait]
impl Resolver<ActorEvent> for ActorResolver {
    fn resolve_on(&mut self, _event: ActorEvent) -> Result<()> {
        Ok(())
    }

    async fn resolve_async(&mut self, _: ActorEvent) -> Result<()> {
        unimplemented!(
            "async resolution is not enabled for ActorResolver, use resolve_on instead."
        );
    }
}

impl ActorResolver {
    pub fn new(actor: &ActorData) -> Self {
        ActorResolver {
            state: ActorState::new(actor),
        }
    }
}

#[readonly::make]
#[derive(Debug)]
pub struct ActorState {
    pub(crate) id: Id,
    pub(crate) gender: Gender,
    pub(crate) short_description: String,
    pub(crate) keywords: Vec<String>,
}

impl ActorState {
    pub fn new(actor: &ActorData) -> Self {
        ActorState {
            id: Id(actor.id),
            gender: Gender::from(&actor.gender[..]),
            short_description: actor.short_description.clone(),
            keywords: actor.keywords.clone(),
        }
    }
}
