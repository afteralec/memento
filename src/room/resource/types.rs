use super::event::{RoomResourceEvent, RoomResourceReplyEvent};
use tokio::sync::{mpsc, oneshot};

pub type RoomResourceSender = mpsc::UnboundedSender<RoomResourceEvent>;
pub type RoomResourceReceiver = mpsc::UnboundedReceiver<RoomResourceEvent>;

pub type RoomResourceReplySender = oneshot::Sender<RoomResourceReplyEvent>;
pub type RoomResourceReplyReceiver = oneshot::Receiver<RoomResourceReplyEvent>;
