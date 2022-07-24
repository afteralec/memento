use serde_derive::Deserialize;

#[readonly::make]
#[derive(Debug, Deserialize)]
pub struct ActorData {
    pub(crate) id: i64,
    pub(crate) gender: String,
    pub(crate) short_description: String,
    pub(crate) keywords: Vec<String>,
}
