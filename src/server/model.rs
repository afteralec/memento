use super::{builder::ServerBuilder, resource_proxy::ResourceProxies};
use crate::{
    actor::resource::ActorResource,
    core::AuthClient,
    messaging::traits::{Detach, Raise},
    resource::{AuthResource, PlayerResource, RoomResource, SessionResource},
    session::resource::{proxy::SessionResourceProxy, SessionResourceEvent},
};
use anyhow::Result;
use std::fmt::Debug;
use tokio::net;
use tokio_util::codec::{Framed, LinesCodec};

#[derive(Debug)]
pub struct Server<T>
where
    T: 'static + Send + Sync + Debug + Default + AuthClient,
{
    pub(crate) ip: String,
    pub(crate) port: String,
    pub(crate) actor_resource: ActorResource,
    pub(crate) auth_resource: AuthResource<T>,
    pub(crate) player_resource: PlayerResource,
    pub(crate) room_resource: RoomResource,
    pub(crate) session_resource: SessionResource,
    pub(crate) resource_proxies: ResourceProxies,
}

impl<T> Server<T>
where
    T: 'static + Send + Sync + Debug + Default + AuthClient,
{
    pub fn builder() -> ServerBuilder<T> {
        ServerBuilder::default()
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", &self.ip, &self.port)
    }

    pub fn detach_all(&mut self) -> Result<()> {
        self.actor_resource.detach()?;
        self.auth_resource.detach()?;
        self.player_resource.detach()?;
        self.room_resource.detach()?;
        self.session_resource.detach()?;

        Ok(())
    }

    pub fn listen(&mut self) -> Result<()> {
        let addr = self.addr();

        let session_resource_proxy = self.resource_proxies.session_resource_proxy.clone();

        tokio::spawn(async move {
            if let Err(err) = listen(addr, session_resource_proxy).await {
                // @TODO: Error handling
                tracing::error!("{:?}", err);
            }
        });

        Ok(())
    }
}

pub async fn listen(addr: String, session_resource_proxy: SessionResourceProxy) -> Result<()> {
    let listener = net::TcpListener::bind(&addr).await?;

    tracing::info!("server running on {}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        let lines = Framed::new(stream, LinesCodec::new());

        session_resource_proxy.raise(SessionResourceEvent::CreateSession { lines, addr })?;
    }
}
