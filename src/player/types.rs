use super::{event::PlayerEvent, resolver::PlayerResolver};
use crate::messaging::messenger::Messenger;
use futures::stream::SplitSink;
use tokio::{net::TcpStream, sync::mpsc};
use tokio_util::codec::{Framed, LinesCodec};

pub type PlayerMessenger = Messenger<PlayerEvent, PlayerResolver>;
pub type PlayerSink = SplitSink<Framed<TcpStream, LinesCodec>, String>;
pub type PlayerSender = mpsc::UnboundedSender<PlayerEvent>;
