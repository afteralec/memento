use super::{super::interface::Session, types::SessionResourceReplySender};
use crate::server::resources::interface::Resources;
use std::net::SocketAddr;
use thiserror::Error;
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LinesCodec};

#[derive(Debug)]
pub enum SessionResourceEvent {
    CreateSession {
        lines: Framed<TcpStream, LinesCodec>,
        addr: SocketAddr,
        resources: Resources,
    },
    NewSession(SessionResourceReplySender),
}

#[derive(Debug, Error)]
pub enum SessionResourceReplyEvent {
    #[error(
        "SessionResourceReplyEvent::NewSession raised with session {0:?} but channel is closed"
    )]
    NewSession(Session),
}
