use super::event::SessionEvent;
use futures::stream::SplitStream;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_util::codec::{Framed, LinesCodec};

pub type SessionSender = mpsc::UnboundedSender<SessionEvent>;
pub type SessionReceiver = mpsc::UnboundedReceiver<SessionEvent>;

pub type SessionStream = SplitStream<Framed<TcpStream, LinesCodec>>;
