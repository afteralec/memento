use super::{
    event::ActorResourceEvent,
    types::{ActorResourceMessenger, ActorResourceSender},
};
use crate::messaging::traits::{Interface, Raise};
use anyhow::Result;

#[readonly::make]
#[derive(Debug, Clone)]
pub struct ActorResource {
    sender: ActorResourceSender,
}

impl Raise<ActorResourceEvent> for ActorResource {
    fn raise(&self, event: ActorResourceEvent) -> Result<()> {
        self.sender.send(event)?;

        Ok(())
    }
}

impl Interface<ActorResourceMessenger> for ActorResource {
    fn of(m: &ActorResourceMessenger) -> Self {
        ActorResource {
            sender: m.sender.clone(),
        }
    }
}
