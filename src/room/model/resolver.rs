use super::delay::RoomDelayState;
use crate::{
    actor, keywords, messaging,
    room::model::{Room, RoomEdges, RoomEvent, RoomSender, RoomSize},
    Id,
};
use anyhow::Result;
use std::{collections::HashMap, default::Default};

#[derive(Debug)]
pub struct RoomResolver {
    state: RoomState,
}

impl Default for RoomResolver {
    fn default() -> Self {
        RoomResolver {
            state: RoomState::default(),
        }
    }
}

impl messaging::traits::ResolverMut<RoomEvent> for RoomResolver {
    fn resolve_on(&mut self, _event: RoomEvent) -> Result<()> {
        Ok(())
    }
}

impl RoomResolver {
    pub fn new(room: &Room) -> Self {
        RoomResolver {
            state: RoomState::new(room),
        }
    }

    pub fn replace_edges(&mut self, edges: RoomEdges<RoomSender>) {
        self.state.replace_edges(edges);
    }
}

pub type RoomActors = HashMap<Id, actor::model::Actor>;

#[derive(Debug)]
pub struct RoomState {
    id: Id,
    title: String,
    description: String,
    size: RoomSize,
    edges: RoomEdges<RoomSender>,
    keywords: keywords::util::Keywords,
    delays: RoomDelayState,
    actors: RoomActors,
}

impl RoomState {
    pub fn new(room: &Room) -> Self {
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

    pub fn id(&self) -> Id {
        self.id
    }
}

impl Default for RoomState {
    fn default() -> Self {
        RoomState {
            id: Id(0),
            title: "Room Title".to_owned(),
            description: "This room has a description".to_owned(),
            size: RoomSize::new(1),
            edges: [
                None, None, None, None, None, None, None, None, None, None, None, None,
            ],
            keywords: keywords::util::Keywords::default(),
            delays: RoomDelayState::default(),
            actors: RoomActors::default(),
        }
    }
}
