use merchant;
use super::model::RoomEdges;
use tokio::sync::mpsc;

#[derive(Debug)]
pub enum RoomEvent {}
pub type RoomSender = mpsc::UnboundedSender<RoomEvent>;
pub type RoomReceiver = mpsc::UnboundedReceiver<RoomEvent>;

pub type RoomBroker = merchant::Broker<RoomEvent>;

#[derive(Debug)]
pub struct RoomState {
    id: i64,
    title: String,
    description: String,
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
