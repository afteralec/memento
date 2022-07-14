use super::{AuthClient, AuthRequest, AuthResourceEvent, AuthResourceReplyEvent};
use crate::messaging::ResolverMut;
use anyhow::Result;
use std::{default::Default, fmt::Debug};

#[derive(Debug)]
pub struct AuthResourceResolver<T>
where
    T: 'static + Send + Sync + Debug + AuthClient + Default,
{
    state: AuthResourceState<T>,
}

impl<T> Default for AuthResourceResolver<T>
where
    T: 'static + Send + Sync + Debug + AuthClient + Default,
{
    fn default() -> Self {
        AuthResourceResolver {
            state: AuthResourceState::default(),
        }
    }
}

impl<T> ResolverMut<AuthResourceEvent> for AuthResourceResolver<T>
where
    T: 'static + Send + Sync + Debug + AuthClient + Default,
{
    fn resolve_on(&mut self, event: AuthResourceEvent) -> Result<()> {
        match event {
            AuthResourceEvent::Request(auth_request, reply_sender) => match auth_request {
                AuthRequest::WithCredential(credential) => {
                    let response = self.state.client.validate(credential);
                    reply_sender.send(AuthResourceReplyEvent::Response(response))?;

                    Ok(())
                }
            },
        }
    }
}

impl<T> AuthResourceResolver<T>
where
    T: 'static + Send + Sync + Debug + AuthClient + Default,
{
    pub fn new(client: T) -> Self {
        AuthResourceResolver {
            state: AuthResourceState::new(client),
        }
    }
}

#[derive(Debug)]
pub struct AuthResourceState<T>
where
    T: 'static + Send + Sync + Debug + AuthClient + Default,
{
    client: T,
}

impl<T> Default for AuthResourceState<T>
where
    T: 'static + Send + Sync + Debug + AuthClient + Default,
{
    fn default() -> Self {
        AuthResourceState {
            client: T::default(),
        }
    }
}

impl<T> AuthResourceState<T>
where
    T: 'static + Send + Sync + Debug + AuthClient + Default,
{
    pub fn new(client: T) -> Self {
        AuthResourceState { client }
    }
}
