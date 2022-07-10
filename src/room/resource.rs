use crate::Id;
use super::Room;

use std::{collections::HashMap, default::Default, iter::Iterator};

#[derive(Debug)]
pub struct RoomResource {
    rooms: HashMap<Id, Room>,
}

impl Default for RoomResource {
    fn default() -> Self {
        RoomResource {
            rooms: HashMap::default(),
        }
    }
}

impl RoomResource {
    pub fn new(room_iter: impl Iterator<Item = Room>) -> Self {
        let mut rooms = HashMap::new();

        for room in room_iter {
            rooms.insert(room.id(), room);
        }

        let mut room_resource = RoomResource {
            rooms,
            ..Default::default()
        };

        room_resource.hydrate_room_edges();

        room_resource
    }

    fn hydrate_room_edges(&mut self) {
        let room_senders =
            &self
                .rooms
                .iter()
                .fold(HashMap::new(), |mut room_senders, (room_id, room)| {
                    room_senders.insert(*room_id, room.sender());
                    room_senders
                });

        for (_, room) in self.rooms.iter_mut() {
            room.hydrate_edges(&room_senders);
        }
    }
}
