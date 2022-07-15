pub(crate) mod actor {
    pub(crate) mod model {
        pub(crate) mod error;
        pub(crate) mod interface;

        pub use interface::Actor;
    }

    pub(crate) mod resource {
        pub(crate) mod error;
        pub(crate) mod event;
        pub(crate) mod interface;
        pub(crate) mod resolver;
        pub(crate) mod types;

        pub use super::Actor;
        pub(crate) use error::ActorResourceError;
        pub(crate) use event::{ActorResourceEvent, ActorResourceReplyEvent};
        pub use interface::ActorResource;
        pub(crate) use resolver::ActorResourceResolver;
        pub use types::{ActorResourceReceiver, ActorResourceSender};
    }

    pub use model::Actor;
    pub use resource::{ActorResource, ActorResourceSender};
}

pub(crate) mod auth {
    pub(crate) mod resource {
        pub(crate) mod client;
        pub(crate) mod error;
        pub(crate) mod event;
        pub(crate) mod interface;
        pub(crate) mod resolver;
        pub(crate) mod types;

        pub use client::AuthClient;
        pub(crate) use error::AuthResourceError;
        pub use event::{
            AuthRequest, AuthResourceEvent, AuthResourceReplyEvent, AuthResponse, Credential,
        };
        pub use interface::AuthResource;
        pub(crate) use resolver::AuthResourceResolver;
        pub use types::{
            AuthResourceReceiver, AuthResourceReplyReceiver, AuthResourceReplySender,
            AuthResourceSender,
        };
    }

    pub use resource::{
        AuthClient, AuthRequest, AuthResource, AuthResourceEvent, AuthResourceReplyEvent,
        AuthResourceReplyReceiver, AuthResourceSender, AuthResponse, Credential,
    };
}

pub(crate) mod delay {
    pub(crate) mod state;
}

pub(crate) mod keywords {
    pub(crate) mod util;

    pub(crate) use util::Keywords;
}

pub mod messaging {
    pub mod functions;
    pub mod traits;

    pub use functions::resolve_receiver;
    pub use traits::{ResolverMut, Spawn};
}

pub(crate) mod player {
    pub(crate) mod model {
        pub(crate) mod error;
        pub(crate) mod interface;

        pub use interface::Player;
    }

    pub(crate) mod resource {
        pub(crate) mod error;
        pub(crate) mod event;
        pub(crate) mod interface;
        pub(crate) mod resolver;
        pub(crate) mod types;

        pub(crate) use super::Player;
        pub(crate) use error::PlayerResourceError;
        pub use event::{PlayerResourceEvent, PlayerResourceReplyEvent};
        pub use interface::PlayerResource;
        pub(crate) use resolver::PlayerResourceResolver;
        pub use types::{
            PlayerResourceReceiver, PlayerResourceReplyReceiver, PlayerResourceReplySender,
            PlayerResourceSender,
        };
    }

    pub use model::Player;
    pub use resource::{
        PlayerResource, PlayerResourceEvent, PlayerResourceReceiver, PlayerResourceReplyEvent,
        PlayerResourceReplyReceiver, PlayerResourceReplySender, PlayerResourceSender,
    };
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
        pub use event::SessionEvent;
        pub use interface::Session;
        pub use resolver::SessionResolver;
        pub use types::{SessionReceiver, SessionSender};
    }

    pub(crate) mod resource {
        pub(crate) mod error;
        pub(crate) mod event;
        pub(crate) mod functions {
            pub mod create;
            pub mod steps;

            pub use steps::{auth_step, player_step};
        }
        pub(crate) mod interface;
        pub(crate) mod resolver;
        pub(crate) mod types;

        pub use super::Session;
        pub use error::SessionResourceError;
        pub use event::SessionResourceEvent;
        pub use functions::auth_step;
        pub use interface::SessionResource;
        pub use resolver::SessionResourceResolver;
        pub use types::{SessionResourceReceiver, SessionResourceSender};
    }

    pub use model::{Session, SessionEvent, SessionReceiver, SessionSender};
    pub use resource::{
        SessionResource, SessionResourceEvent, SessionResourceReceiver, SessionResourceSender,
    };
}

use std::fmt::{Display, Formatter, Result};

pub use actor::{Actor, ActorResource, ActorResourceSender};
pub use auth::{
    AuthClient, AuthRequest, AuthResource, AuthResourceEvent, AuthResourceSender, AuthResponse,
    Credential,
};
pub use player::{Player, PlayerResource, PlayerResourceEvent, PlayerResourceSender};
pub use room::{
    Room, RoomEdges, RoomError, RoomEvent, RoomReceiver, RoomResolver, RoomResource,
    RoomResourceSender, RoomSender, RoomSize,
};
pub use server::Server;
pub use session::{SessionResource, SessionResourceEvent, SessionResourceSender};

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
