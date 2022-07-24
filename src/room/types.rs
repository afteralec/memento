use super::{event::RoomEvent, resolver::RoomResolver};
use crate::messaging::messenger::Messenger;
use tokio::sync::mpsc;

pub type RoomMessenger = Messenger<RoomEvent, RoomResolver>;
pub type RoomSender = mpsc::UnboundedSender<RoomEvent>;
pub type RoomEdges<T> = [Option<T>; 12];

#[readonly::make]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct RoomSize(u8);

impl RoomSize {
    pub fn set(&mut self, size: u8) {
        self.0 = size;
    }
}

impl RoomSize {
    pub fn new(size: u8) -> Self {
        if size > 4 {
            panic!("attempted to create room with invalid size {}", size)
        }

        RoomSize(size)
    }
}
