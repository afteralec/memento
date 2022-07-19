use super::{Room, RoomEdges, RoomSender, RoomSize};
use crate::{messaging::traits::Proxy, Id};

// @TODO: Look into use Arc to provide read access across threads to the same data
#[derive(Debug)]
pub struct RoomProxy {
    id: Id,
    title: String,
    description: String,
    size: RoomSize,
    edges: RoomEdges<Id>,
    sender: RoomSender,
}

impl Proxy for RoomProxy {}

impl RoomProxy {
    pub fn from(room: &Room) -> Self {
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
