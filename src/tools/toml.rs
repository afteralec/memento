use serde_json::Value;
use std::fs;

pub fn read(dir: &str, ids: &[i64]) -> Vec<Value> {
    ids.iter()
        .map(|id| {
            let path = format!("{}/{}.toml", dir, id);
            read_data_from_contents(&read_file_contents(&path))
        })
        .collect()
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

fn read_data_from_contents<'a, T: serde::Deserialize<'a>>(contents: &'a str) -> T {
    match toml::from_str(contents) {
        Ok(d) => d,
        Err(err) => {
            panic!("provided string is not valid TOML, got error: {:?}", err)
        }
    }
}
