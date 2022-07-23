use super::{event::SessionEvent, interface::Session, types::SessionSender};
use crate::{
    messaging::traits::{Detach, Proxy, Raise},
    room::model::proxy::RoomProxy,
};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct SessionProxy {
    sender: SessionSender,
}

impl Proxy<Session> for SessionProxy {
    fn proxy(session: &Session) -> Self {
        SessionProxy {
            sender: session.sender(),
        }
    }
}

impl Raise<SessionEvent> for SessionProxy {
    fn raise(&self, event: SessionEvent) -> Result<()> {
        self.sender.send(event)?;

        Ok(())
    }
}

impl SessionProxy {
    pub fn new_room(&self, room: &RoomProxy) -> Result<()> {
        self.raise(SessionEvent::NewRoom(room.clone()))?;

        Ok(())
    }
}
