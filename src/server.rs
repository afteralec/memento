use crate::session::resource::{SessionResourceEvent, SessionResourceSender};
use anyhow::Result;
use thiserror::Error;
use tokio::{net, sync::mpsc};
use tokio_util::codec::{Framed, LinesCodec};

pub type StreamWriter = mpsc::UnboundedSender<String>;

#[derive(Debug)]
pub struct Server {
    ip: String,
    port: String,
    session_resource_sender: Option<SessionResourceSender>,
}

impl Server {
    pub fn new(ip: &str, port: &str) -> Self {
        Server {
            ip: ip.to_owned(),
            port: port.to_owned(),
            ..Default::default()
        }
    }

    pub fn set_session_resource_sender(&mut self, sender: SessionResourceSender) {
        let _ = self.session_resource_sender.insert(sender);
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", &self.ip, &self.port)
    }

    pub fn listen(&mut self) -> Result<()> {
        let addr = self.addr();

        let session_resource_sender = self
            .session_resource_sender
            .as_ref()
            .ok_or_else(|| anyhow::Error::new(ServerError::NoSessionResourceSender))?;

        let session_resource_sender = session_resource_sender.clone();

        tokio::spawn(async move {
            if let Err(err) = listen(addr, session_resource_sender).await {
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
            session_resource_sender: None,
        }
    }
}

pub async fn listen(addr: String, session_resource_sender: SessionResourceSender) -> Result<()> {
    let listener = net::TcpListener::bind(&addr).await?;

    tracing::info!("server running on {}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        let lines = Framed::new(stream, LinesCodec::new());

        session_resource_sender.send(SessionResourceEvent::NewSession { lines, addr })?;
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("attempted to start listening on server without a SessionResourceSender")]
    NoSessionResourceSender,
}
