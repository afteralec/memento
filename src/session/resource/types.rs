use super::event::{SessionResourceEvent, SessionResourceReplyEvent};
use tokio::sync::{mpsc, oneshot};

pub type SessionResourceSender = mpsc::UnboundedSender<SessionResourceEvent>;
pub type SessionResourceReceiver = mpsc::UnboundedReceiver<SessionResourceEvent>;

pub type SessionResourceReplySender = oneshot::Sender<SessionResourceReplyEvent>;
pub type SessionResourceReplyReceiver = oneshot::Receiver<SessionResourceReplyEvent>;
