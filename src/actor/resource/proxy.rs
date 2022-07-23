use super::{event::ActorResourceEvent, interface::ActorResource, types::ActorResourceSender};
use crate::messaging::traits::{Detach, Proxy, Raise};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct ActorResourceProxy {
    sender: ActorResourceSender,
}

impl Proxy<ActorResource> for ActorResourceProxy {
    fn proxy(actor_resource: &ActorResource) -> Self {
        ActorResourceProxy {
            sender: actor_resource.sender(),
        }
    }
}

impl Raise<ActorResourceEvent> for ActorResourceProxy {
    fn raise(&self, event: ActorResourceEvent) -> Result<()> {
        self.sender.send(event)?;

        Ok(())
    }
}
