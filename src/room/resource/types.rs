use super::{
    event::{RoomResourceEvent, RoomResourceReplyEvent},
    resolver::RoomResourceResolver,
};
use crate::messaging::messenger::Messenger;
use tokio::sync::{mpsc, oneshot};

pub type RoomResourceMessenger = Messenger<RoomResourceEvent, RoomResourceResolver>;

pub type RoomResourceSender = mpsc::UnboundedSender<RoomResourceEvent>;

pub type RoomResourceReplySender = oneshot::Sender<RoomResourceReplyEvent>;
pub type RoomResourceReplyReceiver = oneshot::Receiver<RoomResourceReplyEvent>;
