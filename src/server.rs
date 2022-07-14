use crate::{Credential, SessionResourceEvent, SessionResourceSender};
use anyhow::Result;
use futures::{SinkExt, StreamExt};
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

        let mut lines = Framed::new(stream, LinesCodec::new());

        // @TODO: Extract this to a login screen async function
        lines.send("Please enter your username:").await?;

        let username = match lines.next().await {
            Some(Ok(line)) => line,
            _ => {
                tracing::error!("Failed to get username from {}. Client disconnected.", addr);
                return Ok(());
            }
        };

        lines.send("Please enter your password:").await?;

        let password = match lines.next().await {
            Some(Ok(line)) => line,
            _ => {
                tracing::error!("Failed to get username from {}. Client disconnected.", addr);
                return Ok(());
            }
        };

        session_resource_sender.send(SessionResourceEvent::NewSession {
            lines,
            addr,
            credential: Credential::UserNameAndPassword(username, password),
        })?;
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("attempted to start listening on server without a SessionResourceSender")]
    NoSessionResourceSender,
}
