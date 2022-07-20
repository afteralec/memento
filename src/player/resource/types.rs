use super::event::{PlayerResourceEvent, PlayerResourceReplyEvent};
use tokio::sync::{mpsc, oneshot};

pub type PlayerResourceSender = mpsc::UnboundedSender<PlayerResourceEvent>;
pub type PlayerResourceReceiver = mpsc::UnboundedReceiver<PlayerResourceEvent>;

pub type PlayerResourceReplySender = oneshot::Sender<PlayerResourceReplyEvent>;
pub type PlayerResourceReplyReceiver = oneshot::Receiver<PlayerResourceReplyEvent>;
