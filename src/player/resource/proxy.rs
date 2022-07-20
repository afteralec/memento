use super::{event::PlayerResourceEvent, interface::PlayerResource, types::PlayerResourceSender};
use crate::messaging::traits::{Proxy, Raise};
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
    fn sender(&self) -> PlayerResourceSender {
        self.sender.clone()
    }

    fn raise(&self, event: PlayerResourceEvent) -> Result<()> {
        self.sender.send(event)?;

        Ok(())
    }
}
