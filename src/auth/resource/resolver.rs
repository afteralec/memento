use super::{
    super::traits::AuthClient,
    event::{AuthRequest, AuthResourceEvent, AuthResourceReplyEvent},
};
use crate::messaging::traits::Resolver;
use anyhow::Result;
use async_trait::async_trait;
use std::{default::Default, fmt::Debug};

#[derive(Debug)]
pub struct AuthResourceResolver<C>
where
    C: 'static + Send + Sync + Debug + Default + AuthClient,
{
    state: AuthResourceState<C>,
}

impl<C> Default for AuthResourceResolver<C>
where
    C: 'static + Send + Sync + Debug + Default + AuthClient,
{
    fn default() -> Self {
        AuthResourceResolver {
            state: AuthResourceState::default(),
        }
    }
}

#[async_trait]
impl<C> Resolver<AuthResourceEvent> for AuthResourceResolver<C>
where
    C: 'static + Send + Sync + Debug + Default + AuthClient,
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

    async fn resolve_async(&mut self, _: AuthResourceEvent) -> Result<()> {
        unimplemented!(
            "Async resolution not supported for AuthResourceResolver, use resolve_on instead."
        );
    }
}

impl<C> AuthResourceResolver<C>
where
    C: 'static + Send + Sync + Debug + Default + AuthClient,
{
    pub fn new(client: C) -> Self {
        AuthResourceResolver {
            state: AuthResourceState::new(client),
        }
    }
}

#[derive(Debug)]
pub struct AuthResourceState<C>
where
    C: 'static + Send + Sync + Debug + Default + AuthClient,
{
    client: C,
}

impl<C> Default for AuthResourceState<C>
where
    C: 'static + Send + Sync + Debug + Default + AuthClient,
{
    fn default() -> Self {
        AuthResourceState {
            client: C::default(),
        }
    }
}

impl<C> AuthResourceState<C>
where
    C: 'static + Send + Sync + Debug + Default + AuthClient,
{
    pub fn new(client: C) -> Self {
        AuthResourceState { client }
    }
}
