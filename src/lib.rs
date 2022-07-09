pub(crate) mod actor {
    pub(crate) mod model;
    pub(crate) mod service;
}

pub(crate) mod delay {
    pub(crate) mod broker;
}

pub(crate) mod keywords {
    pub(crate) mod model;

    pub(crate) use model::Keywords;
}

pub(crate) mod player {
    pub(crate) mod model;
}

pub(crate) mod room {
    pub(crate) mod broker;
    pub(crate) mod model;
    pub(crate) mod service;
}

pub(crate) mod server;

pub(crate) mod session {
    pub(crate) mod broker;
    pub(crate) mod model;

    pub(crate) use broker::SessionSender;
}

use std::{error, result};

pub use server::Server;
pub(crate) use keywords::Keywords;

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct Id(pub i64);
