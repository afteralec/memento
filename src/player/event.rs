use super::types::PlayerSink;
use crate::session::event::SessionEvent;

#[derive(Debug)]
pub enum PlayerEvent {
    Write(String),
    SendToSession(SessionEvent),
    AttachSink(PlayerSink),
}
