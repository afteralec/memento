use super::toml;
use crate::actor::data::ActorData;

pub fn read(root_dir: &str, ids: &[i64]) -> Vec<Result<ActorData, serde_json::Error>> {
    toml::read(root_dir, ids)
        .iter()
        .map(|actor_json| serde_json::from_value(actor_json.clone()))
        .collect()
}
