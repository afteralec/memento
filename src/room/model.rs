use crate::{
    messaging, messaging::Spawn, room::broker::RoomMatcher, Id, RoomReceiver, RoomSender,
};
use super::error::RoomError;

use anyhow::Result;
use std::{collections::HashMap, default::Default};
use tokio::sync::mpsc;

pub type RoomEdges<T> = [Option<T>; 12];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct RoomSize(u8);

impl RoomSize {
    pub fn new(size: u8) -> Self {
        if size > 4 {
            panic!("attempted to create room with invalid size {}", size)
        }

        RoomSize(size)
    }
}

#[derive(Debug)]
pub struct Room {
    id: Id,
    title: String,
    description: String,
    size: RoomSize,
    edges: RoomEdges<Id>,
    sender: RoomSender,
    receiver: Option<RoomReceiver>,
    matcher: Option<RoomMatcher>,
}

impl Default for Room {
    fn default() -> Self {
        let (room_sender, room_receiver) = mpsc::unbounded_channel();

        Room {
            id: Id(0),
            title: "Room Title".to_owned(),
            description: "This room has a long description.".to_owned(),
            size: RoomSize(1),
            edges: [
                None, None, None, None, None, None, None, None, None, None, None, None,
            ],
            sender: room_sender,
            receiver: Some(room_receiver),
            matcher: None,
        }
    }
}

impl Spawn for Room {}

impl Room {
    pub fn new(
        id: u64,
        title: &str,
        description: &str,
        size: u8,
        edges: [Option<u64>; 12],
    ) -> Self {
        let mut room = Room {
            id: Id(id),
            title: title.to_owned(),
            description: description.to_owned(),
            size: RoomSize::new(size),
            edges: [
                make_id(edges[0]),
                make_id(edges[1]),
                make_id(edges[2]),
                make_id(edges[3]),
                make_id(edges[4]),
                make_id(edges[5]),
                make_id(edges[6]),
                make_id(edges[7]),
                make_id(edges[8]),
                make_id(edges[9]),
                make_id(edges[10]),
                make_id(edges[11]),
            ],
            ..Default::default()
        };

        room.generate_matcher();

        room
    }

    fn generate_matcher(&mut self) {
        let _ = self.matcher.insert(RoomMatcher::new(&self));
    }

    pub fn hydrate_edges(&mut self, rooms: &HashMap<Id, RoomSender>) {
        let edge_senders = self
            .edges
            .iter()
            .map(|edge_id| {
                if let Some(edge_id) = edge_id {
                    if let Some(room_sender) = rooms.get(edge_id) {
                        Some(room_sender.clone())
                    } else {
                        panic!(
                            "attempted to hydrate edge for room id {} with invalid room id {}",
                            self.id(),
                            &edge_id
                        )
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if let Some(matcher) = &mut self.matcher {
            let edges_slice = &edge_senders[..];

            let edges = [
                edges_slice[0].clone(),
                edges_slice[1].clone(),
                edges_slice[2].clone(),
                edges_slice[3].clone(),
                edges_slice[4].clone(),
                edges_slice[5].clone(),
                edges_slice[6].clone(),
                edges_slice[7].clone(),
                edges_slice[8].clone(),
                edges_slice[9].clone(),
                edges_slice[10].clone(),
                edges_slice[11].clone(),
            ];

            matcher.replace_edges(edges);
        }
    }

    pub fn spawn(&mut self) -> Result<()>
    where
        Self: Spawn,
    {
        let matcher = self.matcher.take().ok_or_else(|| {
            RoomError::NoMatcher(self.id)
        })?;

        let receiver = self.receiver.take().ok_or_else(|| {
            RoomError::NoReceiver(self.id)
        })?;

        self.spawn_and_trace(messaging::match_receiver(receiver, matcher));

        Ok(())
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn size(&self) -> RoomSize {
        self.size
    }

    pub fn sender(&self) -> RoomSender {
        self.sender.clone()
    }
}

fn make_id(id: Option<u64>) -> Option<Id> {
    Some(Id(id?))
}
