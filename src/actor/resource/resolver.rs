use super::{
    super::{data::ActorData, resolver::ActorResolver, types::ActorMessenger},
    event::{ActorResourceEvent, ActorResourceReplyEvent},
};
use crate::{
    messaging::traits::{Provide, Resolver},
    Id,
};
use anyhow::Result;
use async_trait::async_trait;
use std::{collections::HashMap, sync::Arc};

#[readonly::make]
#[derive(Debug)]
pub struct ActorResourceResolver {
    pub(crate) state: ActorResourceState,
}

#[async_trait]
impl Resolver<ActorResourceEvent> for ActorResourceResolver {
    fn resolve_on(&mut self, event: ActorResourceEvent) -> Result<()> {
        match event {
            ActorResourceEvent::GetActorById(id, reply_sender) => {
                if let Some(actor) = self.state.messengers.get(&id) {
                    reply_sender
                        .send(ActorResourceReplyEvent::GotActorById(id, Arc::new(actor.provide())))?;
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
    pub fn new(actor_iter: impl Iterator<Item = ActorData>) -> Self {
        ActorResourceResolver {
            state: ActorResourceState::new(actor_iter),
        }
    }
}

#[readonly::make]
#[derive(Debug)]
pub struct ActorResourceState {
    pub(crate) actors: HashMap<Id, ActorData>,
    pub(crate) messengers: HashMap<Id, ActorMessenger>,
}

impl ActorResourceState {
    pub fn new(actor_iter: impl Iterator<Item = ActorData>) -> Self {
        let (actors, messengers) = actor_iter.fold(
            (HashMap::new(), HashMap::new()),
            |(mut actors, mut messengers), actor| {
                let name = format!("actor {}", actor.id);
                messengers.insert(
                    Id(actor.id),
                    ActorMessenger::new(&name, ActorResolver::new(&actor)),
                );
                actors.insert(Id(actor.id), actor);

                (actors, messengers)
            },
        );

        ActorResourceState { actors, messengers }
    }
}
