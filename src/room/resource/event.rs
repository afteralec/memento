use super::Room;
use crate::Id;
use thiserror::Error;
use tokio::sync::oneshot;

#[derive(Debug, Error)]
pub enum RoomResourceEvent {
    #[error("RoomResource::GetRoomById raised with id {} but channel is closed", .id)]
    GetRoomById {
        id: Id,
        reply_sender: oneshot::Sender<RoomResourceReplyEvent>,
    },
}

#[derive(Debug, Error)]
pub enum RoomResourceReplyEvent {
    #[error("RoomResourceReply::GotRoomById raised with id {0} but channel is closed")]
    GotRoomById(Id, Room),
    #[error("RoomResourceReply::RoomNotFound raised with id {0} but channel is closed")]
    RoomNotFound(Id),
}
