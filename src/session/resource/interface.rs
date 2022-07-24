use super::{
    event::SessionResourceEvent,
    types::{SessionResourceMessenger, SessionResourceSender},
};
use crate::messaging::traits::{Interface, Raise};
use anyhow::Result;

#[derive(Clone, Debug)]
pub struct SessionResource {
    sender: SessionResourceSender,
}

impl Raise<SessionResourceEvent> for SessionResource {
    fn raise(&self, event: SessionResourceEvent) -> Result<()> {
        self.sender.send(event)?;

        Ok(())
    }
}

impl Interface<SessionResourceMessenger> for SessionResource {
    fn of(m: &SessionResourceMessenger) -> Self {
        SessionResource {
            sender: m.sender.clone(),
        }
    }
}
