use super::Room;
use serde_derive::Deserialize;
use std::ops::Index;

#[derive(Debug, Deserialize)]
pub struct RoomData {
    id: i64,
    title: String,
    description: String,
    size: u8,
    exits: RoomDataEdges,
}

impl RoomData {
    pub fn to_room(&self) -> Room {
        Room::new(
            self.id,
            &self.title,
            &self.description,
            self.size,
            self.exits.to_slice(),
        )
    }
}

#[derive(Debug, Deserialize)]
struct RoomDataEdges {
    northwest: Option<i64>,
    north: Option<i64>,
    northeast: Option<i64>,
    east: Option<i64>,
    southeast: Option<i64>,
    south: Option<i64>,
    southwest: Option<i64>,
    west: Option<i64>,
    r#in: Option<i64>,
    out: Option<i64>,
    up: Option<i64>,
    down: Option<i64>,
}

impl RoomDataEdges {
    pub fn to_slice(&self) -> [Option<i64>; 12] {
        [
            self.northwest,
            self.north,
            self.northeast,
            self.east,
            self.southeast,
            self.south,
            self.southwest,
            self.west,
            self.r#in,
            self.out,
            self.up,
            self.down,
        ]
    }
}

impl Index<&'_ usize> for RoomDataEdges {
    type Output = Option<i64>;

    fn index(&self, index: &usize) -> &Option<i64> {
        match index {
            0 => &self.northwest,
            1 => &self.north,
            2 => &self.northeast,
            3 => &self.east,
            4 => &self.southeast,
            5 => &self.south,
            6 => &self.southwest,
            7 => &self.west,
            8 => &self.r#in,
            9 => &self.out,
            10 => &self.up,
            11 => &self.down,
            _ => panic!("Index out of bounds"),
        }
    }
}
