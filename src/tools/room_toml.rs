use super::toml;
use crate::room::data::RoomData;

pub fn read<'a>(root_dir: &str, ids: &[i64]) -> Vec<Result<RoomData, serde_json::Error>> {
    toml::read(root_dir, ids)
        .iter()
        .map(|room_json| serde_json::from_value(room_json.clone()))
        .collect()
}
