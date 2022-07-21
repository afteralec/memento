use super::types::PlayerSink;

#[derive(Debug)]
pub enum PlayerEvent {
    Write(String),
    AttachSink(PlayerSink),
}
