use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LinesCodec};

#[derive(Debug)]
pub enum SessionResourceEvent {
    NewSession {
        lines: Framed<TcpStream, LinesCodec>,
        addr: SocketAddr,
    },
}

#[derive(Debug)]
pub enum SessionResourceReplyEvent {}
