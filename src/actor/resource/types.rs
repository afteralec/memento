use super::{
    event::{ActorResourceEvent, ActorResourceReplyEvent},
    resolver::ActorResourceResolver,
};
use crate::messaging::messenger::Messenger;
use tokio::sync::{mpsc, oneshot};

pub type ActorResourceMessenger = Messenger<ActorResourceEvent, ActorResourceResolver>;

pub type ActorResourceSender = mpsc::UnboundedSender<ActorResourceEvent>;

pub type ActorResourceReplySender = oneshot::Sender<ActorResourceReplyEvent>;
pub type ActorResourceReplyReceiver = oneshot::Receiver<ActorResourceReplyEvent>;
