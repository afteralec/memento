use super::{
    event::RoomEvent,
    interface::Room,
    types::{RoomEdges, RoomSender, RoomSize},
};
use crate::{
    messaging::traits::{Detach, Proxy, Raise},
    Id,
};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct RoomProxy {
    id: Id,
    title: String,
    description: String,
    size: RoomSize,
    edges: RoomEdges<Id>,
    sender: RoomSender,
}

impl Raise<RoomEvent> for RoomProxy {
    fn raise(&self, event: RoomEvent) -> Result<()> {
        self.sender.send(event)?;

        Ok(())
    }
}

impl Proxy<Room> for RoomProxy {
    fn proxy(room: &Room) -> Self {
        RoomProxy {
            id: room.id(),
            title: room.title(),
            description: room.description(),
            size: room.size(),
            edges: room.edges(),
            sender: room.sender(),
        }
    }
}

impl RoomProxy {
    pub fn id(&self) -> Id {
        self.id
    }

    pub fn title(&self) -> String {
        self.title.to_owned()
    }

    pub fn description(&self) -> String {
        self.description.to_owned()
    }

    pub fn size(&self) -> RoomSize {
        self.size
    }

    pub fn edges(&self) -> RoomEdges<Id> {
        self.edges
    }
}
