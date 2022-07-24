use super::{builder::ServerBuilder, resources::interface::Resources};
use crate::{messaging::traits::Raise, session::resource::SessionResourceEvent};
use anyhow::Result;
use std::fmt::Debug;
use tokio::net;
use tokio_util::codec::{Framed, LinesCodec};

#[derive(Debug)]
pub struct Server {
    pub(crate) ip: String,
    pub(crate) port: String,
    pub(crate) resources: Resources,
}

impl Server {
    pub fn builder() -> ServerBuilder {
        ServerBuilder::default()
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", &self.ip, &self.port)
    }

    pub fn listen(&mut self) -> Result<()> {
        let addr = self.addr();
        let resources = self.resources.clone();

        tokio::spawn(async move {
            if let Err(err) = listen(addr, resources).await {
                // @TODO: Error handling
                tracing::error!("{:?}", err);
            }
        });

        Ok(())
    }
}

pub async fn listen(addr: String, resources: Resources) -> Result<()> {
    let listener = net::TcpListener::bind(&addr).await?;

    tracing::info!("server running on {}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        let lines = Framed::new(stream, LinesCodec::new());

        resources
            .session
            .raise(SessionResourceEvent::CreateSession {
                lines,
                addr,
                resources: resources.clone(),
            })?;
    }
}
