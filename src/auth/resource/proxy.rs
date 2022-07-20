use super::{
    super::traits::AuthClient, event::AuthResourceEvent, interface::AuthResource,
    types::AuthResourceSender,
};
use crate::messaging::traits::{Proxy, Raise};
use anyhow::Result;
use core::fmt::Debug;

#[derive(Debug, Clone)]
pub struct AuthResourceProxy {
    sender: AuthResourceSender,
}

impl Raise<AuthResourceEvent> for AuthResourceProxy {
    fn sender(&self) -> AuthResourceSender {
        self.sender.clone()
    }

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
