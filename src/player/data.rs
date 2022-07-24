use serde_derive::Deserialize;

#[readonly::make]
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct PlayerData {
    pub(crate) id: i64,
    pub(crate) current_actor_id: Option<i64>,
}

impl PlayerData {
    pub fn new(id: i64, current_actor_id: Option<i64>) -> Self {
        PlayerData {
            id,
            current_actor_id,
        }
    }
}
