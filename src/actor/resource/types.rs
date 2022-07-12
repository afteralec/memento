use super::ActorResourceEvent;
use tokio::sync::mpsc;

pub type ActorResourceSender = mpsc::UnboundedSender<ActorResourceEvent>;
pub type ActorResourceReceiver = mpsc::UnboundedReceiver<ActorResourceEvent>;
