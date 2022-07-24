use crate::{player::interface::Player, room::interface::Room};

#[derive(Debug)]
pub enum SessionEvent {
    NewRoom(Room),
    AttachPlayer(Player),
    Input(String),
}
