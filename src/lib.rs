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

        pub(crate) use error::ActorResourceError;
        pub(crate) use resolver::ActorResourceResolver;

        pub use event::{ActorResourceEvent, ActorResourceReplyEvent};
        pub use interface::ActorResource;
        pub use types::{
            ActorResourceReceiver, ActorResourceReplyReceiver, ActorResourceReplySender,
            ActorResourceSender,
        };
    }
}

pub(crate) mod auth {
    pub mod resource {
        pub(crate) mod client;
        pub(crate) mod error;
        pub(crate) mod event;
        pub(crate) mod interface;
        pub(crate) mod resolver;
        pub(crate) mod types;

        pub(crate) use error::AuthResourceError;
        pub(crate) use resolver::AuthResourceResolver;

        pub use client::AuthClient;
        pub use event::{
            AuthRequest, AuthResourceEvent, AuthResourceReplyEvent, AuthResponse, Credential,
        };
        pub use interface::AuthResource;
        pub use types::{
            AuthResourceReceiver, AuthResourceReplyReceiver, AuthResourceReplySender,
            AuthResourceSender,
        };
    }

    pub(crate) mod traits;
}

pub(crate) mod delay {
    pub(crate) mod state;
}

pub(crate) mod keywords {
    pub(crate) mod util;
}

pub mod messaging {
    pub mod functions;
    pub mod traits;
}

pub(crate) mod player {
    pub(crate) mod model {
        pub(crate) mod error;
        pub(crate) mod interface;
        pub(crate) mod types;

        pub use interface::Player;
    }

    pub(crate) mod resource {
        pub(crate) mod error;
        pub(crate) mod event;
        pub(crate) mod interface;
        pub(crate) mod resolver;
        pub(crate) mod types;

        pub(crate) use error::PlayerResourceError;
        pub(crate) use resolver::PlayerResourceResolver;

        pub use event::{PlayerResourceEvent, PlayerResourceReplyEvent};
        pub use interface::PlayerResource;
        pub use types::{
            PlayerResourceReceiver, PlayerResourceReplyReceiver, PlayerResourceReplySender,
            PlayerResourceSender,
        };
    }
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

        pub use error::RoomResourceError;
        pub use event::{RoomResourceEvent, RoomResourceReplyEvent};
        pub use interface::RoomResource;
        pub use resolver::RoomResourceResolver;
        pub use types::{
            RoomResourceReceiver, RoomResourceReplyReceiver, RoomResourceReplySender,
            RoomResourceSender,
        };
    }
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
            pub(crate) mod create;
            pub(crate) mod steps;

            pub use steps::{actor_step, auth_step, player_step, room_step};
        }
        pub(crate) mod interface;
        pub(crate) mod resolver;
        pub(crate) mod types;

        pub use error::SessionResourceError;
        pub use event::SessionResourceEvent;
        pub use interface::SessionResource;
        pub use resolver::SessionResourceResolver;
        pub use types::{SessionResourceReceiver, SessionResourceSender};
    }
}

pub(crate) mod tools {
    pub(crate) mod actor_toml;
    pub(crate) mod room_toml;
}

// Define the public interface for the package
pub mod core {
    pub use crate::auth::{
        resource::{AuthResponse, Credential},
        traits::AuthClient,
    };

    pub use crate::server::Server;
}
pub mod model {
    pub use crate::actor::model::Actor;
    pub use crate::player::model::Player;
    pub use crate::room::model::Room;
    pub use crate::session::model::Session;
}

pub mod resource {
    pub use crate::actor::resource::ActorResource;
    pub use crate::auth::resource::AuthResource;
    pub use crate::player::resource::PlayerResource;
    pub use crate::room::resource::RoomResource;
    pub use crate::session::resource::SessionResource;
}

pub mod tooling {
    pub use crate::tools::actor_toml::read as read_actor_toml;
    pub use crate::tools::room_toml::read as read_room_toml;
}

use std::fmt::{Display, Formatter, Result};

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
