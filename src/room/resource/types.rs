use super::event::RoomResourceEvent;
use tokio::sync::mpsc;

pub type RoomResourceSender = mpsc::UnboundedSender<RoomResourceEvent>;
pub type RoomResourceReceiver = mpsc::UnboundedReceiver<RoomResourceEvent>;
