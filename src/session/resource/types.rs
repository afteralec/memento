use super::{
    event::{SessionResourceEvent, SessionResourceReplyEvent},
    resolver::SessionResourceResolver,
};
use crate::messaging::{messenger::Messenger, types::Sender};
use tokio::sync::oneshot;

pub type SessionResourceMessenger = Messenger<SessionResourceEvent, SessionResourceResolver>;

pub type SessionResourceSender = Sender<SessionResourceEvent>;

pub type SessionResourceReplySender = oneshot::Sender<SessionResourceReplyEvent>;
pub type SessionResourceReplyReceiver = oneshot::Receiver<SessionResourceReplyEvent>;
