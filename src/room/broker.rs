use crate::Id;
use super::model::{RoomSize, RoomEdges};
use merchant;
use tokio::sync::mpsc;

#[derive(Debug)]
pub enum RoomEvent {}
pub type RoomSender = mpsc::UnboundedSender<RoomEvent>;
pub type RoomReceiver = mpsc::UnboundedReceiver<RoomEvent>;

pub type RoomBroker = merchant::Broker<RoomEvent>;

#[derive(Debug)]
pub struct RoomState {
    id: Id,
    title: String,
    description: String,
    size: RoomSize,
    edges: RoomEdges<RoomSender>,
}

#[derive(Debug)]
pub struct RoomMatcher {
    state: RoomState,
}

impl merchant::MatcherMut<RoomEvent> for RoomMatcher {
    fn match_on_mut(&mut self, event: RoomEvent) -> merchant::Result<()> {
        Ok(())
    }
}
