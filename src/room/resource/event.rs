use super::{super::interface::Room, types::RoomResourceReplySender};
use crate::Id;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RoomResourceEvent {
    #[error("RoomResource::GetRoomById raised with id {0} but channel is closed")]
    GetRoomById(Id, RoomResourceReplySender),
}

#[derive(Debug, Error)]
pub enum RoomResourceReplyEvent {
    #[error("RoomResourceReply::GotRoomById raised with id {0} but channel is closed")]
    GotRoomById(Id, Arc<Room>),
    #[error("RoomResourceReply::NoRoomAtId raised with id {0} but channel is closed")]
    NoRoomAtId(Id),
}
