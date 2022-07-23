use super::{event::PlayerResourceEvent, interface::PlayerResource, types::PlayerResourceSender};
use crate::messaging::traits::{Detach, Proxy, Raise};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct PlayerResourceProxy {
    sender: PlayerResourceSender,
}

impl Proxy<PlayerResource> for PlayerResourceProxy {
    fn proxy(player_resource: &PlayerResource) -> Self {
        PlayerResourceProxy {
            sender: player_resource.sender(),
        }
    }
}

impl Raise<PlayerResourceEvent> for PlayerResourceProxy {
    fn raise(&self, event: PlayerResourceEvent) -> Result<()> {
        self.sender.send(event)?;

        Ok(())
    }
}
