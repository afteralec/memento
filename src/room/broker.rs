use super::{delay::RoomDelayState, model::{Room, RoomEdges, RoomSize}};
use crate::{actor, keywords, Id};
use merchant;
use std::collections::HashMap;
use tokio::sync::mpsc;

#[derive(Debug)]
pub enum RoomEvent {}

pub type RoomSender = mpsc::UnboundedSender<RoomEvent>;
pub type RoomReceiver = mpsc::UnboundedReceiver<RoomEvent>;
pub type RoomBroker = merchant::Broker<RoomEvent>;
pub type RoomActors = HashMap<Id, actor::Actor>;

#[derive(Debug)]
pub struct RoomState {
    id: Id,
    title: String,
    description: String,
    size: RoomSize,
    edges: RoomEdges<RoomSender>,
    keywords: keywords::Keywords,
    delays: RoomDelayState,
    actors: RoomActors,
}

impl std::default::Default for RoomState {
    fn default() -> Self {
        RoomState {
            id: Id(0),
            title: "Room Title".to_owned(),
            description: "This room has a description".to_owned(),
            size: RoomSize::new(1),
            edges: [
                None, None, None, None, None, None, None, None, None, None, None, None,
            ],
            keywords: keywords::Keywords::default(),
            delays: RoomDelayState::default(),
            actors: RoomActors::default(),
        }
    }
}

impl RoomState {
    pub fn from_room(room: Room) -> Self {
        RoomState {
            id: room.id(),
            title: room.title(),
            description: room.description(),
            ..Default::default()
        }
    }

    pub fn replace_edges(&mut self, edges: RoomEdges<RoomSender>) {
        self.edges = [
            edges[0].clone(),
            edges[1].clone(),
            edges[2].clone(),
            edges[3].clone(),
            edges[4].clone(),
            edges[5].clone(),
            edges[6].clone(),
            edges[7].clone(),
            edges[8].clone(),
            edges[9].clone(),
            edges[10].clone(),
            edges[11].clone(),
        ];
    }

    pub fn set_edges(&mut self, edges: &[(usize, Option<RoomSender>)]) {
        for (edge_index, edge) in edges {
            self.set_edge(edge_index, edge);
        }
    }

    pub fn set_edge(&mut self, edge_index: &usize, edge: &Option<RoomSender>) {
        if *edge_index > 11 {
            panic!(
                "attempted to update_edge for invalid edge_index {}",
                edge_index
            )
        };

        self.edges[*edge_index] = edge.clone()
    }
}

#[derive(Debug)]
pub struct RoomMatcher {
    state: RoomState,
}

impl merchant::MatcherMut<RoomEvent> for RoomMatcher {
    fn match_on_mut(&mut self, event: RoomEvent) -> merchant::Result<()> {
        Ok(())
    }
}
