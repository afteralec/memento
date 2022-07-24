use super::{
    event::{PlayerResourceEvent, PlayerResourceReplyEvent},
    resolver::PlayerResourceResolver,
};
use crate::messaging::{messenger::Messenger, types::Sender};
use tokio::sync::oneshot;

pub type PlayerResourceMessenger = Messenger<PlayerResourceEvent, PlayerResourceResolver>;

pub type PlayerResourceSender = Sender<PlayerResourceEvent>;

pub type PlayerResourceReplySender = oneshot::Sender<PlayerResourceReplyEvent>;
pub type PlayerResourceReplyReceiver = oneshot::Receiver<PlayerResourceReplyEvent>;
