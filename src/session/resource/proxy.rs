use super::{
    event::SessionResourceEvent, interface::SessionResource, types::SessionResourceSender,
};
use crate::messaging::traits::{Proxy, Raise};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct SessionResourceProxy {
    sender: SessionResourceSender,
}

impl Proxy<SessionResource> for SessionResourceProxy {
    fn proxy(session_resource: &SessionResource) -> Self {
        SessionResourceProxy {
            sender: session_resource.sender(),
        }
    }
}

impl Raise<SessionResourceEvent> for SessionResourceProxy {
    fn sender(&self) -> SessionResourceSender {
        self.sender.clone()
    }

    fn raise(&self, event: SessionResourceEvent) -> Result<()> {
        self.sender.send(event)?;

        Ok(())
    }
}