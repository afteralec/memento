use super::event::PlayerEvent;
use futures::stream::SplitSink;
use tokio::{net::TcpStream, sync::mpsc};
use tokio_util::codec::{Framed, LinesCodec};

pub type PlayerSink = SplitSink<Framed<TcpStream, LinesCodec>, String>;
pub type PlayerSender = mpsc::UnboundedSender<PlayerEvent>;
pub type PlayerReceiver = mpsc::UnboundedReceiver<PlayerEvent>;
