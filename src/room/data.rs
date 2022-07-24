use serde_derive::Deserialize;
use std::{default::Default, ops::Index};

#[readonly::make]
#[derive(Clone, Debug, Deserialize)]
pub struct RoomData {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub size: u8,
    #[serde(rename(deserialize = "exits", serialize = "edges"))]
    pub edges: RoomDataEdges,
}

impl Default for RoomData {
    fn default() -> Self {
        RoomData {
            id: 0,
            title: "Room Title".to_owned(),
            description: "This room has a long description.".to_owned(),
            size: 1,
            edges: RoomDataEdges::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct RoomDataEdges {
    pub(crate) northwest: Option<i64>,
    pub(crate) north: Option<i64>,
    pub(crate) northeast: Option<i64>,
    pub(crate) east: Option<i64>,
    pub(crate) southeast: Option<i64>,
    pub(crate) south: Option<i64>,
    pub(crate) southwest: Option<i64>,
    pub(crate) west: Option<i64>,
    pub(crate) r#in: Option<i64>,
    pub(crate) out: Option<i64>,
    pub(crate) up: Option<i64>,
    pub(crate) down: Option<i64>,
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
