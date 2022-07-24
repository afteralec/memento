use super::{
    event::AuthResourceEvent,
    types::{AuthResourceMessenger, AuthResourceSender},
};
use crate::{
    auth::traits::AuthClient,
    messaging::traits::{Interface, Raise},
};
use anyhow::Result;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct AuthResource {
    sender: AuthResourceSender,
}

impl Raise<AuthResourceEvent> for AuthResource {
    fn raise(&self, event: AuthResourceEvent) -> Result<()> {
        self.sender.send(event)?;

        Ok(())
    }
}

impl<C> Interface<AuthResourceMessenger<C>> for AuthResource
where
    C: 'static + Send + Sync + Debug + Default + AuthClient,
{
    fn of(m: &AuthResourceMessenger<C>) -> Self {
        AuthResource {
            sender: m.sender.clone(),
        }
    }
}
