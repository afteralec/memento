use super::PlayerResourceEvent;
use tokio::sync::mpsc;

pub type PlayerResourceSender = mpsc::UnboundedSender<PlayerResourceEvent>;
pub type PlayerResourceReceiver = mpsc::UnboundedReceiver<PlayerResourceEvent>;
