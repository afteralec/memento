use super::Actor;
use serde_derive::Deserialize;

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
            None,
        )
    }
}
