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

    pub(crate) use functions::resolve_receiver;
    pub(crate) use traits::{ResolverMut, Spawn};
}

pub(crate) mod player {
    pub(crate) mod error;
    pub(crate) mod model;

    pub use model::Player;
}

pub(crate) mod room {
    pub(crate) mod model {
        pub(crate) mod delay;
        pub(crate) mod error;
        pub(crate) mod event;
        pub(crate) mod interface;
        pub(crate) mod resolver;

        pub(crate) use super::{RoomReceiver, RoomSender};
        pub use error::RoomError;
        pub use event::RoomEvent;
        pub use interface::Room;
        pub use resolver::RoomResolver;
    }

    pub(crate) mod resource {
        pub(crate) mod error;
        pub(crate) mod event;
        pub(crate) mod interface;
        pub(crate) mod resolver;

        pub(crate) use super::{RoomResourceReceiver, RoomResourceSender};
        pub use error::RoomResourceError;
        pub use event::RoomResourceEvent;
        pub use interface::RoomResource;
        pub use resolver::RoomResourceResolver;
    }

    pub(crate) mod types;

    pub use model::{Room, RoomError, RoomEvent, RoomResolver};
    pub use resource::{RoomResource, RoomResourceEvent};
    pub use types::{RoomEdges, RoomReceiver, RoomSender, RoomSize, RoomResourceReceiver, RoomResourceSender};
}

pub(crate) mod server;

pub(crate) mod session {
    pub(crate) mod broker;
    pub(crate) mod model;

    pub(crate) use broker::{SessionEvent, SessionSender};
}

use std::{error, fmt, result};

pub use actor::Actor;
pub use room::{
    Room, RoomEdges, RoomError, RoomEvent, RoomResolver, RoomResource, RoomReceiver, RoomSender,
    RoomSize,
};
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
