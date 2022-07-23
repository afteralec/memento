use super::{
    super::traits::AuthClient, event::AuthResourceEvent, interface::AuthResource,
    types::AuthResourceSender,
};
use crate::messaging::traits::{Detach, Proxy, Raise};
use anyhow::Result;
use core::fmt::Debug;

#[derive(Debug, Clone)]
pub struct AuthResourceProxy {
    sender: AuthResourceSender,
}

impl Raise<AuthResourceEvent> for AuthResourceProxy {
    fn raise(&self, event: AuthResourceEvent) -> Result<()> {
        self.sender.send(event)?;

        Ok(())
    }
}

impl<T> Proxy<AuthResource<T>> for AuthResourceProxy
where
    T: 'static + Send + Sync + Debug + Default + AuthClient,
{
    fn proxy(auth_resource: &AuthResource<T>) -> Self {
        AuthResourceProxy {
            sender: auth_resource.sender(),
        }
    }
}
