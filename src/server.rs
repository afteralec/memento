use crate::{
    messaging::traits::Raise,
    session::resource::{proxy::SessionResourceProxy, SessionResourceEvent},
};
use anyhow::Result;
use thiserror::Error;
use tokio::net;
use tokio_util::codec::{Framed, LinesCodec};

#[derive(Debug)]
pub struct Server {
    ip: String,
    port: String,
    session_resource_proxy: Option<SessionResourceProxy>,
}

impl Server {
    pub fn new(ip: &str, port: &str) -> Self {
        Server {
            ip: ip.to_owned(),
            port: port.to_owned(),
            ..Default::default()
        }
    }

    pub fn set_session_resource_proxy(&mut self, proxy: SessionResourceProxy) {
        let _ = self.session_resource_proxy.insert(proxy);
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", &self.ip, &self.port)
    }

    pub fn listen(&mut self) -> Result<()> {
        let addr = self.addr();

        let session_resource_proxy = self
            .session_resource_proxy
            .as_ref()
            .cloned()
            .ok_or_else(|| anyhow::Error::new(ServerError::NoSessionResourceProxy))?;

        tokio::spawn(async move {
            if let Err(err) = listen(addr, session_resource_proxy).await {
                // @TODO: Error handling
                tracing::error!("{:?}", err);
            }
        });

        Ok(())
    }
}

impl Default for Server {
    fn default() -> Self {
        Server {
            ip: "127.0.0.1".to_owned(),
            port: "8080".to_owned(),
            session_resource_proxy: None,
        }
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

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("attempted to start listening on server without a SessionResourceProxy")]
    NoSessionResourceProxy,
}
