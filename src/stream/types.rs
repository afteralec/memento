use futures::stream::SplitStream;
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LinesCodec};

pub type Stream = SplitStream<Framed<TcpStream, LinesCodec>>;
