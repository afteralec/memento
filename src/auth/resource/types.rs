use super::{AuthResourceEvent, AuthResourceReplyEvent};
use crate::Id;
use tokio::sync::{mpsc, oneshot};

pub type AuthResourceSender = mpsc::UnboundedSender<AuthResourceEvent>;
pub type AuthResourceReceiver = mpsc::UnboundedReceiver<AuthResourceEvent>;

pub type AuthResourceReplySender = oneshot::Sender<AuthResourceReplyEvent>;
pub type AuthResourceReplyReceiver = oneshot::Receiver<AuthResourceReplyEvent>;
