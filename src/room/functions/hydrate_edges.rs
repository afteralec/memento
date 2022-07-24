use crate::{messaging::traits::Provide, room::types::RoomMessenger, Id};
use std::collections::HashMap;

pub fn hydrate_edges(messenger: &mut RoomMessenger, rooms: &HashMap<Id, RoomMessenger>) {
    if let Some(resolver) = messenger.resolver.as_mut() {
        let edges = resolver
            .state
            .edge_ids
            .iter()
            .map(|edge_id| {
                if let Some(edge_id) = edge_id {
                    if let Some(room) = rooms.get(edge_id) {
                        Some(room.provide())
                    } else {
                        panic!(
                            "attempted to hydrate edge for room id {} with invalid room id {}",
                            resolver.state.id, &edge_id
                        )
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let edges_slice = &edges[..];

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

        resolver.replace_edges(edges);
    };
}
