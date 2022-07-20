use super::{
    super::model::Actor,
    event::{ActorResourceEvent, ActorResourceReplyEvent},
};
use crate::{messaging::traits::Resolver, Id};
use anyhow::Result;
use async_trait::async_trait;
use std::{collections::HashMap, default::Default};

#[derive(Debug)]
pub struct ActorResourceResolver {
    state: ActorResourceState,
}

impl Default for ActorResourceResolver {
    fn default() -> Self {
        ActorResourceResolver {
            state: ActorResourceState::default(),
        }
    }
}

#[async_trait]
impl Resolver<ActorResourceEvent> for ActorResourceResolver {
    fn resolve_on(&mut self, event: ActorResourceEvent) -> Result<()> {
        match event {
            ActorResourceEvent::GetActorById(id, reply_sender) => {
                if let Some(actor) = self.state.actors.get(&id) {
                    reply_sender.send(ActorResourceReplyEvent::GotActorById(id, actor.clone()))?;
                } else {
                    reply_sender.send(ActorResourceReplyEvent::NoActorAtId(id))?;
                }

                Ok(())
            }
        }
    }

    async fn resolve_async(&mut self, _: ActorResourceEvent) -> Result<()> {
        unimplemented!(
            "Async resolution isn't supported for ActorResourceResolver, use resolve_on instead."
        );
    }
}

impl ActorResourceResolver {
    pub fn new(actor_iter: impl Iterator<Item = Actor>) -> Self {
        ActorResourceResolver {
            state: ActorResourceState::new(actor_iter),
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub struct ActorResourceState {
    actors: HashMap<Id, Actor>,
}

impl Default for ActorResourceState {
    fn default() -> Self {
        ActorResourceState {
            actors: HashMap::default(),
        }
    }
}

impl ActorResourceState {
    pub fn new(actor_iter: impl Iterator<Item = Actor>) -> Self {
        let actors = actor_iter.fold(HashMap::new(), |mut actors, actor| {
            actors.insert(actor.id(), actor);

            actors
        });

        ActorResourceState { actors }
    }
}
