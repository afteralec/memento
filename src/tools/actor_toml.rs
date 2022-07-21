use crate::actor::model::Actor;
use serde_derive::Deserialize;
use std::fs;

pub fn read(root_dir: &str) -> Vec<ActorData> {
    let actor_id_list_path = format!("{}/ids.toml", root_dir);

    let actor_id_list_contents = read_file_contents(&actor_id_list_path);

    let actor_ids: ActorIds = read_toml_from_contents(&actor_id_list_contents);

    let mut actor_data_vec = Vec::new();
    for actor_id in actor_ids.list {
        let actor_file_path = format!("{}/{}.toml", root_dir, actor_id);
        let actor_file_contents = read_file_contents(&actor_file_path);

        let actor_data: ActorData = read_toml_from_contents(&actor_file_contents);

        actor_data_vec.push(actor_data);
    }

    actor_data_vec
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
struct ActorIds {
    list: Vec<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ActorData {
    id: i64,
    gender: String,
    short_description: String,
    keywords: Vec<String>,
}

impl ActorData {
    pub fn to_actor(&self) -> Actor {
        Actor::new(
            self.id,
            &self.gender,
            &self.short_description,
            &self.keywords,
        )
    }
}
