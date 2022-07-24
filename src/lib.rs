pub(crate) mod actor {
    pub(crate) mod resource {
        pub(crate) mod event;
        pub(crate) mod interface;
        pub(crate) mod resolver;
        pub(crate) mod types;

        pub use event::{ActorResourceEvent, ActorResourceReplyEvent};
        pub use interface::ActorResource;
        pub use types::{
            ActorResourceReplyReceiver, ActorResourceReplySender, ActorResourceSender,
        };
    }

    pub(crate) mod data;
    pub(crate) mod error;
    pub(crate) mod event;
    pub(crate) mod interface;
    pub(crate) mod resolver;
    pub(crate) mod types;
}

pub(crate) mod auth {
    pub mod resource {
        pub(crate) mod event;
        pub(crate) mod interface;
        pub(crate) mod resolver;
        pub(crate) mod types;

        pub use event::{
            AuthRequest, AuthResourceEvent, AuthResourceReplyEvent, AuthResponse, Credential,
        };
        pub use interface::AuthResource;
        pub use types::{AuthResourceReplyReceiver, AuthResourceReplySender, AuthResourceSender};
    }

    pub(crate) mod traits;
}

pub(crate) mod keywords {
    pub(crate) mod util;
}

pub mod messaging {
    pub mod error;
    pub mod functions;
    pub mod messenger;
    pub mod traits;
    pub mod types;
}

pub(crate) mod player {
    pub(crate) mod resource {
        pub(crate) mod event;
        pub(crate) mod interface;
        pub(crate) mod resolver;
        pub(crate) mod types;
    }

    pub(crate) mod data;
    pub(crate) mod error;
    pub(crate) mod event;
    pub(crate) mod interface;
    pub(crate) mod resolver;
    pub(crate) mod types;
}

pub(crate) mod room {
    pub(crate) mod functions {
        pub(crate) mod hydrate_edges;

        pub(crate) use hydrate_edges::hydrate_edges;
    }
    pub(crate) mod resource {
        pub(crate) mod event;
        pub(crate) mod interface;
        pub(crate) mod resolver;
        pub(crate) mod types;
    }

    pub(crate) mod data;
    pub(crate) mod event;
    pub(crate) mod interface;
    pub(crate) mod resolver;
    pub(crate) mod types;
}

pub mod server {
    pub(crate) mod resources {
        pub(crate) mod builder;
        pub(crate) mod interface;
    }

    pub(crate) mod builder;
    pub(crate) mod init;
    pub(crate) mod input;
    pub(crate) mod interface;
}

pub(crate) mod session {
    pub(crate) mod functions {
        pub(crate) mod create;
        pub(crate) mod steps;
    }
    pub(crate) mod resource {
        pub(crate) mod event;
        pub(crate) mod interface;
        pub(crate) mod resolver;
        pub(crate) mod types;

        pub use event::SessionResourceEvent;
        pub use interface::SessionResource;
        pub use resolver::SessionResourceResolver;
        pub use types::SessionResourceSender;
    }

    pub(crate) mod error;
    pub(crate) mod event;
    pub(crate) mod interface;
    pub(crate) mod resolver;
    pub(crate) mod types;
}

pub(crate) mod stream {
    pub(crate) mod resolve;
    pub(crate) mod resolver;
    pub(crate) mod types;
}

pub(crate) mod tools {
    pub(crate) mod actor_toml;
    pub(crate) mod room_toml;
    pub(crate) mod toml;
}

// Define the public interface for the package
pub mod core {
    pub use crate::auth::{
        resource::{AuthResponse, Credential},
        traits::AuthClient,
    };

    pub mod server {
        pub use crate::server::init::init;
        pub use crate::server::input::ServerInput;
    }
}

pub mod data {
    pub use crate::actor::data::ActorData;
    pub use crate::player::data::PlayerData;
    pub use crate::room::data::RoomData;
}

pub mod model {
    pub use crate::actor::interface::Actor;
    pub use crate::player::interface::Player;
    pub use crate::room::interface::Room;
    pub use crate::session::interface::Session;
}

pub mod resource {
    pub use crate::actor::resource::ActorResource;
    pub use crate::auth::resource::AuthResource;
    pub use crate::player::resource::interface::PlayerResource;
    pub use crate::room::resource::interface::RoomResource;
    pub use crate::session::resource::SessionResource;
}

pub mod tooling {
    pub use crate::tools::actor_toml::read as read_actor_toml;
    pub use crate::tools::room_toml::read as read_room_toml;
}

use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct Id(i64);

impl Id {
    pub fn new(id: i64) -> Self {
        Id(id)
    }

    pub fn set(&mut self, id: i64) {
        self.0 = id;
    }

    pub fn is_valid(&self) -> bool {
        self.0 > 0
    }

    pub fn val(&self) -> i64 {
        self.0
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}
