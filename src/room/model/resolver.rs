use crate::{
    actor, keywords,
    messaging::traits::Resolver,
    room::model::{Room, RoomEdges, RoomEvent, RoomProxy, RoomSize},
    Id,
};
use anyhow::Result;
use async_trait::async_trait;
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

#[async_trait]
impl Resolver<RoomEvent> for RoomResolver {
    fn resolve_on(&mut self, _event: RoomEvent) -> Result<()> {
        Ok(())
    }

    async fn resolve_async(&mut self, _: RoomEvent) -> Result<()> {
        unimplemented!("Async resolution is not enabled for RoomResolver, use resolve_on instead.");
    }
}

impl RoomResolver {
    pub fn new(room: &Room) -> Self {
        RoomResolver {
            state: RoomState::new(room),
        }
    }

    pub fn replace_edges(&mut self, edges: RoomEdges<RoomProxy>) {
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
    edges: RoomEdges<RoomProxy>,
    keywords: keywords::util::Keywords,
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

    pub fn replace_edges(&mut self, edges: RoomEdges<RoomProxy>) {
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

    pub fn _set_edges(&mut self, edges: &[(usize, &Option<RoomProxy>)]) {
        for (edge_index, edge) in edges {
            self._set_edge(edge_index, *edge);
        }
    }

    pub fn _set_edge(&mut self, edge_index: &usize, edge: &Option<RoomProxy>) {
        if *edge_index > 11 {
            panic!(
                "attempted to update_edge for invalid edge_index {}",
                edge_index
            )
        };

        self.edges[*edge_index] = edge.clone()
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
            actors: RoomActors::default(),
        }
    }
}
