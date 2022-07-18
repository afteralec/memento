use super::{ActorResourceEvent, ActorResourceReplyEvent};
use tokio::sync::{mpsc, oneshot};

pub type ActorResourceSender = mpsc::UnboundedSender<ActorResourceEvent>;
pub type ActorResourceReceiver = mpsc::UnboundedReceiver<ActorResourceEvent>;

pub type ActorResourceReplySender = oneshot::Sender<ActorResourceReplyEvent>;
pub type ActorResourceReplyReceiver = oneshot::Receiver<ActorResourceReplyEvent>;
