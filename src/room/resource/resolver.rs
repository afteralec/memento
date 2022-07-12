use super::{RoomResourceEvent, RoomResourceReplyEvent};
use crate::{messaging, Id, Room};

use anyhow::Result;
use std::{collections::HashMap, default::Default, iter::Iterator};

#[derive(Debug)]
pub struct RoomResourceResolver {
    state: RoomResourceState,
}

impl Default for RoomResourceResolver {
    fn default() -> Self {
        RoomResourceResolver {
            state: RoomResourceState::default(),
        }
    }
}

impl messaging::ResolverMut<RoomResourceEvent> for RoomResourceResolver {
    fn resolve_on(&mut self, event: RoomResourceEvent) -> Result<()> {
        match event {
            RoomResourceEvent::GetRoomById { id, reply_sender } => {
                if let Some(room) = self.state.rooms.get(&id) {
                    reply_sender.send(RoomResourceReplyEvent::GotRoomById(id, room.clone()))?;
                } else {
                    reply_sender.send(RoomResourceReplyEvent::NoRoomAtId(id))?;
                }

                Ok(())
            }
        }
    }
}

impl RoomResourceResolver {
    pub fn new(room_iter: impl Iterator<Item = Room>) -> Self {
        RoomResourceResolver {
            state: RoomResourceState::new(room_iter),
        }
    }
}

#[derive(Debug)]
pub struct RoomResourceState {
    rooms: HashMap<Id, Room>,
}

impl Default for RoomResourceState {
    fn default() -> Self {
        RoomResourceState {
            rooms: HashMap::default(),
        }
    }
}

impl RoomResourceState {
    pub fn new(room_iter: impl Iterator<Item = Room>) -> Self {
        let mut rooms = room_iter.fold(HashMap::new(), |mut rooms, room| {
            rooms.insert(room.id(), room);

            rooms
        });

        let mut room_resource_state = RoomResourceState {
            rooms,
            ..Default::default()
        };

        room_resource_state.hydrate_room_edges();

        room_resource_state
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
