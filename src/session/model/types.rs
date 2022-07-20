use super::event::SessionEvent;
use tokio::sync::mpsc;

pub type SessionSender = mpsc::UnboundedSender<SessionEvent>;
pub type SessionReceiver = mpsc::UnboundedReceiver<SessionEvent>;
