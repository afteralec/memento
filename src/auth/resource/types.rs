use super::{
    event::{AuthResourceEvent, AuthResourceReplyEvent},
    resolver::AuthResourceResolver,
};
use crate::messaging::messenger::Messenger;
use tokio::sync::{mpsc, oneshot};

pub type AuthResourceMessenger<C> = Messenger<AuthResourceEvent, AuthResourceResolver<C>>;

pub type AuthResourceSender = mpsc::UnboundedSender<AuthResourceEvent>;

pub type AuthResourceReplySender = oneshot::Sender<AuthResourceReplyEvent>;
pub type AuthResourceReplyReceiver = oneshot::Receiver<AuthResourceReplyEvent>;
