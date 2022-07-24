use super::{
    super::{
        data::RoomData, functions::hydrate_edges, resolver::RoomResolver, types::RoomMessenger,
    },
    event::{RoomResourceEvent, RoomResourceReplyEvent},
};
use crate::{
    messaging::traits::{Detach, DetachAll, Provide, Resolver},
    Id,
};
use anyhow::Result;
use async_trait::async_trait;
use std::{collections::HashMap, iter::Iterator};

#[derive(Debug)]
pub struct RoomResourceResolver {
    state: RoomResourceState,
}

#[async_trait]
impl Resolver<RoomResourceEvent> for RoomResourceResolver {
    fn resolve_on(&mut self, event: RoomResourceEvent) -> Result<()> {
        match event {
            RoomResourceEvent::GetRoomById(id, reply_sender) => {
                if let Some(messenger) = self.state.messengers.get(&id) {
                    reply_sender
                        .send(RoomResourceReplyEvent::GotRoomById(id, messenger.provide()))?;
                } else {
                    reply_sender.send(RoomResourceReplyEvent::NoRoomAtId(id))?;
                }

                Ok(())
            }
        }
    }

    async fn resolve_async(&mut self, _: RoomResourceEvent) -> Result<()> {
        unimplemented!(
            "async resolution not supported for RoomResourceResolver, use resolve_on instead."
        );
    }
}

impl DetachAll for RoomResourceResolver {
    fn detach_all(&mut self) -> Result<()> {
        self.state.detach_all()?;

        Ok(())
    }
}

impl RoomResourceResolver {
    pub fn new(room_iter: impl Iterator<Item = RoomData>) -> Self {
        RoomResourceResolver {
            state: RoomResourceState::new(room_iter),
        }
    }
}

#[derive(Debug)]
pub struct RoomResourceState {
    rooms: HashMap<Id, RoomData>,
    messengers: HashMap<Id, RoomMessenger>,
}

impl DetachAll for RoomResourceState {
    fn detach_all(&mut self) -> Result<()> {
        for messenger in self.messengers.values_mut() {
            messenger.detach()?;
        }

        Ok(())
    }
}

impl RoomResourceState {
    pub fn new(room_iter: impl Iterator<Item = RoomData>) -> Self {
        let (rooms, messengers) = room_iter.fold(
            (HashMap::new(), HashMap::new()),
            |(mut rooms, mut messengers), room| {
                let name = format!("room {}", &room.id);
                messengers.insert(
                    Id(room.id.clone()),
                    RoomMessenger::new(&name, RoomResolver::new(&room)),
                );
                rooms.insert(Id(room.id.clone()), room);
                (rooms, messengers)
            },
        );

        let mut room_resource_state = RoomResourceState { rooms, messengers };

        room_resource_state.hydrate_edges();

        room_resource_state
    }

    fn hydrate_edges(&mut self) {
        let mut messengers: HashMap<Id, RoomMessenger> =
            self.messengers
                .iter()
                .fold(HashMap::new(), |mut messengers, (id, messenger)| {
                    messengers.insert(*id, messenger.clone());

                    messengers
                });

        for room in messengers.values_mut() {
            hydrate_edges(room, &self.messengers);
        }
    }
}
