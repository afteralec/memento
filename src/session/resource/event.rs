use super::{super::model::proxy::SessionProxy, types::SessionResourceReplySender};
use crate::server::resource_proxy::ResourceProxies;
use std::net::SocketAddr;
use thiserror::Error;
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LinesCodec};

#[derive(Debug)]
pub enum SessionResourceEvent {
    CreateSession {
        lines: Framed<TcpStream, LinesCodec>,
        addr: SocketAddr,
        resource_proxies: ResourceProxies,
    },
    NewSession(SessionResourceReplySender),
}

#[derive(Debug, Error)]
pub enum SessionResourceReplyEvent {
    #[error(
        "SessionResourceReplyEvent::NewSession raised with session {0:?} but channel is closed"
    )]
    NewSession(SessionProxy),
}
