use std::collections::HashMap;

use crate::session;
use crate::Id;
use crate::Keywords;

pub type Names = HashMap<Id, String>;

pub struct Player {
    id: Id,
    // @TODO: Collapse this type to be session::SessionSender
    session_sender: session::broker::SessionSender,
    // @TODO: Move this type to the server module
    writer: session::broker::StreamWriter,
    names: Names,
    keywords: Keywords,
}
