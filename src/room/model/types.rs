use super::event::RoomEvent;
use tokio::sync::mpsc;

pub type RoomSender = mpsc::UnboundedSender<RoomEvent>;
pub type RoomReceiver = mpsc::UnboundedReceiver<RoomEvent>;

pub type RoomEdges<T> = [Option<T>; 12];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct RoomSize(u8);

impl RoomSize {
    pub fn new(size: u8) -> Self {
        if size > 4 {
            panic!("attempted to create room with invalid size {}", size)
        }

        RoomSize(size)
    }
}
