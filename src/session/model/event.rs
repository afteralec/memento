use crate::room::model::proxy::RoomProxy;

#[derive(Debug)]
pub enum SessionEvent {
    NewRoom(RoomProxy),
}
