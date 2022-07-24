use super::{event::ActorEvent, resolver::ActorResolver};
use crate::messaging::{messenger::Messenger, types::Sender};

pub type ActorMessenger = Messenger<ActorEvent, ActorResolver>;
pub type ActorSender = Sender<ActorEvent>;
