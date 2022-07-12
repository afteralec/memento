pub(crate) mod actor {
    pub(crate) mod error;
    pub(crate) mod model;
    pub(crate) mod service;

    pub use model::Actor;
}

pub(crate) mod auth {
    pub(crate) mod resource {
        pub(crate) mod error;
        pub(crate) mod event;
        pub(crate) mod interface;
        pub(crate) mod resolver;
        pub(crate) mod types;

        pub(crate) use error::AuthResourceError;
        pub(crate) use event::AuthResourceEvent;
        pub(crate) use interface::AuthResource;
        pub(crate) use resolver::AuthResourceResolver;
        pub(crate) use types::{AuthResourceReceiver, AuthResourceSender};
    }
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
        pub(crate) mod types;

        pub use error::RoomError;
        pub use event::RoomEvent;
        pub use interface::Room;
        pub use resolver::RoomResolver;
        pub use types::{RoomEdges, RoomReceiver, RoomSender, RoomSize};
    }

    pub(crate) mod resource {
        pub(crate) mod error;
        pub(crate) mod event;
        pub(crate) mod interface;
        pub(crate) mod resolver;
        pub(crate) mod types;

        pub(crate) use super::Room;
        pub use error::RoomResourceError;
        pub use event::{RoomResourceEvent, RoomResourceReplyEvent};
        pub use interface::RoomResource;
        pub use resolver::RoomResourceResolver;
        pub use types::{RoomResourceReceiver, RoomResourceSender};
    }

    pub use model::{
        Room, RoomEdges, RoomError, RoomEvent, RoomReceiver, RoomResolver, RoomSender, RoomSize,
    };
    pub use resource::{
        RoomResource, RoomResourceEvent, RoomResourceReceiver, RoomResourceReplyEvent,
        RoomResourceSender,
    };
}

pub(crate) mod server;

pub(crate) mod session {
    pub(crate) mod model {
        pub(crate) mod error;
        pub(crate) mod event;
        pub(crate) mod interface;
        pub(crate) mod resolver;
        pub(crate) mod types;

        pub(crate) use error::SessionError;
        pub(crate) use event::SessionEvent;
        pub(crate) use resolver::SessionResolver;
        pub(crate) use types::{SessionReceiver, SessionSender};
    }

    pub(crate) mod resource {
        pub(crate) mod error;
        pub(crate) mod event;
        pub(crate) mod interface;
        pub(crate) mod resolver;
        pub(crate) mod types;

        pub(crate) use event::SessionResourceEvent;
        pub(crate) use resolver::SessionResourceResolver;
        pub(crate) use types::{SessionResourceReceiver, SessionResourceSender};
    }

    pub(crate) use model::{SessionEvent, SessionReceiver, SessionSender};
    pub(crate) use resource::{
        SessionResourceEvent, SessionResourceReceiver, SessionResourceSender,
    };
}

use std::fmt::{Display, Formatter, Result};

pub use actor::Actor;
pub use room::{
    Room, RoomEdges, RoomError, RoomEvent, RoomReceiver, RoomResolver, RoomResource, RoomSender,
    RoomSize,
};
pub use server::Server;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct Id(u64);

impl Id {
    pub fn new(id: u64) -> Self {
        Id(id)
    }

    pub fn is_valid(&self) -> bool {
        self.0 > 0
    }

    pub fn val(&self) -> u64 {
        self.0
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}
