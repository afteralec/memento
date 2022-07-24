use super::{event::SessionEvent, resolver::SessionResolver};
use crate::messaging::messenger::Messenger;
use tokio::sync::mpsc;

pub type SessionMessenger = Messenger<SessionEvent, SessionResolver>;
pub type SessionSender = mpsc::UnboundedSender<SessionEvent>;
