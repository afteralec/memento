use crate::room::model::Room;
use serde_derive::Deserialize;
use std::{fs, ops::Index};

pub fn read(root_dir: &str) -> Vec<RoomData> {
    let room_id_list_path = format!("{}/ids.toml", root_dir);

    let room_id_list_contents = read_file_contents(&room_id_list_path);

    let room_ids: RoomIds = read_toml_from_contents(&room_id_list_contents);

    let mut room_data_vec = Vec::new();
    for room_id in room_ids.list {
        let room_file_path = format!("{}/{}.toml", root_dir, room_id);
        let room_file_contents = read_file_contents(&room_file_path);

        let room_data: RoomData = read_toml_from_contents(&room_file_contents);

        room_data_vec.push(room_data);
    }

    room_data_vec
}

fn read_file_contents(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(c) => c,
        Err(err) => {
            tracing::error!("Could not read file `{}`, got error: {:?}", path, err);
            "".to_owned()
        }
    }
}

fn read_toml_from_contents<'a, T: serde::Deserialize<'a>>(contents: &'a str) -> T {
    match toml::from_str(contents) {
        Ok(d) => d,
        Err(err) => {
            tracing::error!("Provided string is not valid TOML, got error: {:?}", err);
            panic!("Panicking because there's no error here yet.");
        }
    }
}

#[derive(Debug, Deserialize)]
struct RoomIds {
    list: Vec<i64>,
}

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
