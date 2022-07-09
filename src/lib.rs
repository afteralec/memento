pub(crate) mod actor {
    pub(crate) mod error;
    pub(crate) mod model;
    pub(crate) mod service;

    pub use model::Actor;
}

pub(crate) mod delay {
    pub(crate) mod state;
}

pub(crate) mod keywords {
    pub(crate) mod model;

    pub(crate) use model::Keywords;
}

pub(crate) mod player {
    pub(crate) mod error;
    pub(crate) mod model;

    pub use model::Player;
}

pub(crate) mod room {
    pub(crate) mod broker;
    pub(crate) mod delay;
    pub(crate) mod model;
    pub(crate) mod service;

    pub use broker::{RoomReceiver, RoomSender};
    pub use model::{Room, RoomEdges};
}

pub(crate) mod server;

pub(crate) mod session {
    pub(crate) mod broker;
    pub(crate) mod model;

    pub(crate) use broker::{SessionEvent, SessionSender};
}

use std::{error, result};

pub use actor::Actor;
pub(crate) use keywords::Keywords;
pub use room::{Room, RoomEdges, RoomReceiver, RoomSender};
pub use server::Server;

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct Id(u64);

impl Id {
    pub fn new(id: u64) -> Self {
        Id(id)
    }

    pub fn val(&self) -> u64 {
        self.0
    }
}
