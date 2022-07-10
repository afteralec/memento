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
    pub(crate) mod util;

    pub(crate) use util::Keywords;
}

pub(crate) mod messaging {
    pub(crate) mod functions;
    pub(crate) mod traits;

    pub(crate) use functions::match_receiver;
    pub(crate) use traits::{MatcherMut, Spawn};
}

pub(crate) mod player {
    pub(crate) mod error;
    pub(crate) mod model;

    pub use model::Player;
}

pub(crate) mod room {
    pub(crate) mod broker;
    pub(crate) mod delay;
    pub(crate) mod error;
    pub(crate) mod model;
    pub(crate) mod resource;
    pub(crate) mod service;

    pub use broker::{RoomReceiver, RoomSender};
    pub use model::{Room, RoomEdges};
    pub use resource::RoomResource;
}

pub(crate) mod server;

pub(crate) mod session {
    pub(crate) mod broker;
    pub(crate) mod model;

    pub(crate) use broker::{SessionEvent, SessionSender};
}

use std::{error, fmt, result};

pub use actor::Actor;
pub use room::{Room, RoomEdges, RoomReceiver, RoomResource, RoomSender};
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

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// @TODO: Add `thiserror` and `anyhow` crates for error and Result handling
