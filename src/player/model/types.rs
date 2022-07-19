use futures::stream::SplitSink;
use tokio::{net::TcpStream, sync::mpsc};
use tokio_util::codec::{Framed, LinesCodec};

pub type PlayerSink = SplitSink<Framed<TcpStream, LinesCodec>, String>;
pub type PlayerWriter = mpsc::UnboundedSender<String>;
pub type PlayerReceiver = mpsc::UnboundedReceiver<String>;
