use super::{event::RoomResourceEvent, interface::RoomResource, types::RoomResourceSender};
use crate::messaging::traits::{Proxy, Raise};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct RoomResourceProxy {
    sender: RoomResourceSender,
}

impl Proxy<RoomResource> for RoomResourceProxy {
    fn proxy(room_resource: &RoomResource) -> Self {
        RoomResourceProxy {
            sender: room_resource.sender(),
        }
    }
}

impl Raise<RoomResourceEvent> for RoomResourceProxy {
    fn sender(&self) -> RoomResourceSender {
        self.sender.clone()
    }

    fn raise(&self, event: RoomResourceEvent) -> Result<()> {
        self.sender.send(event)?;

        Ok(())
    }
}