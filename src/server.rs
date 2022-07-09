use crate::session;
use crate::{Id, Result};

use std::collections::HashMap;
use tokio::net;

#[derive(Debug)]
pub struct Server {
    ip: String,
    port: String,
    sessions: HashMap<Id, session::SessionSender>,
}

impl Server {
    pub fn new(ip: &str, port: &str) -> Self {
        Server {
            ip: ip.to_owned(),
            port: port.to_owned(),
            ..Default::default()
        }
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", &self.ip, &self.port)
    }

    pub fn listen(&mut self) {
        let addr = self.addr();

        tokio::spawn(async move {
            if let Err(err) = listen(addr).await {
                // @TODO: Error handling
                tracing::error!(err);
            }
        });
    }
}

impl Default for Server {
    fn default() -> Self {
        Server {
            ip: "127.0.0.1".to_owned(),
            port: "8080".to_owned(),
            sessions: HashMap::default(),
        }
    }
}

pub async fn listen(addr: String) -> Result<()> {
    let listener = net::TcpListener::bind(&addr).await?;

    tracing::info!("server running on {}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;

        tracing::debug!("{}:{:?}", addr, stream);
    }
}
