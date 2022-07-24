use super::{
    event::SessionEvent,
    types::{SessionMessenger, SessionSender},
};
use crate::{
    messaging::traits::{Interface, Raise},
    room::interface::Room, player::interface::Player,
};
use anyhow::Result;

#[readonly::make]
#[derive(Debug, Clone)]
pub struct Session {
    sender: SessionSender,
}

impl Raise<SessionEvent> for Session {
    fn raise(&self, event: SessionEvent) -> Result<()> {
        self.sender.send(event)?;

        Ok(())
    }
}

impl Interface<SessionMessenger> for Session {
    fn of(m: &SessionMessenger) -> Self {
        Session {
            sender: m.sender.clone(),
        }
    }
}

impl Session {
    pub fn new_room(&self, room: Room) -> Result<()> {
        self.raise(SessionEvent::NewRoom(room))?;

        Ok(())
    }

    pub fn attach_player(&self, player: Player) -> Result<()> {
        self.raise(SessionEvent::AttachPlayer(player))?;

        Ok(())
    }
}
