use super::{
    event::RoomEvent,
    types::{RoomEdges, RoomMessenger, RoomSender, RoomSize},
};
use crate::{
    messaging::traits::{Interface, Raise},
    Id,
};
use anyhow::Result;
use std::fmt::Debug;

#[readonly::make]
#[derive(Clone, Debug)]
pub struct Room {
    pub id: Id,
    pub size: RoomSize,
    pub edges: RoomEdges<Id>,
    sender: RoomSender,
}

impl Raise<RoomEvent> for Room {
    fn raise(&self, event: RoomEvent) -> Result<()> {
        self.sender.send(event)?;

        Ok(())
    }
}

impl Interface<RoomMessenger> for Room {
    fn of(m: &RoomMessenger) -> Self {
        let state = &m.resolver.as_ref().unwrap().state;

        Room {
            id: state.id,
            size: state.size,
            edges: state.edge_ids,
            sender: m.sender.clone(),
        }
    }
}
