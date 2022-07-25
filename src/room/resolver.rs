use super::{
    data::RoomData,
    event::RoomEvent,
    interface::Room,
    types::{RoomEdges, RoomSize},
};
use crate::{actor::interface::Actor, keywords::util::Keywords, messaging::traits::Resolver, Id};
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;

#[readonly::make]
#[derive(Debug)]
pub struct RoomResolver {
    pub(crate) state: RoomState,
}

#[async_trait]
impl Resolver<RoomEvent> for RoomResolver {
    fn resolve_on(&mut self, _event: RoomEvent) -> Result<()> {
        Ok(())
    }

    async fn resolve_async(&mut self, _: RoomEvent) -> Result<()> {
        unimplemented!("async resolution is not enabled for RoomResolver, use resolve_on instead.");
    }
}

impl RoomResolver {
    pub fn new(room: &RoomData) -> Self {
        RoomResolver {
            state: RoomState::new(room),
        }
    }

    pub fn replace_edges(&mut self, edges: RoomEdges<Room>) {
        self.state.replace_edges(edges);
    }
}

pub type RoomActors = HashMap<Id, Actor>;

#[readonly::make]
#[derive(Debug)]
pub struct RoomState {
    pub(crate) id: Id,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) size: RoomSize,
    pub(crate) edge_ids: RoomEdges<Id>,
    pub(crate) edges: RoomEdges<Room>,
    pub(crate) keywords: Keywords,
    pub(crate) actors: RoomActors,
}

impl RoomState {
    pub fn new(room: &RoomData) -> Self {
        let edge_ids = room.edges.to_slice().map(|edge| edge.map(Id));

        RoomState {
            id: Id(room.id),
            title: room.title.clone(),
            description: room.description.clone(),
            size: RoomSize::new(room.size),
            edge_ids,
            edges: [
                None, None, None, None, None, None, None, None, None, None, None, None,
            ],
            keywords: Keywords::new(),
            actors: RoomActors::new(),
        }
    }

    pub fn replace_edges(&mut self, edges: RoomEdges<Room>) {
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
}
