use super::SessionResourceEvent;
use tokio::sync::mpsc;

pub type SessionResourceSender = mpsc::UnboundedSender<SessionResourceEvent>;
pub type SessionResourceReceiver = mpsc::UnboundedReceiver<SessionResourceEvent>;
