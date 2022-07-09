use super::broker::{RoomBroker, RoomReceiver, RoomSender};
use super::model::Room;
use crate::Id;

use merchant;
use std::collections::HashMap;
use tokio::sync::mpsc;

pub type RoomServiceSender = mpsc::UnboundedSender<merchant::ResourceEvent<RoomBroker>>;
pub type RoomServiceReceiver = mpsc::UnboundedReceiver<merchant::ResourceEvent<RoomBroker>>;
pub type Rooms = HashMap<crate::Id, RoomBroker>;

#[derive(Debug)]
pub struct RoomServiceState {
    rooms: Rooms,
}

impl RoomServiceState {
    pub fn from_list(room_list: &[Room]) -> Self {
        let room_brokers = room_list
            .into_iter()
            .map(|room| {
                let (room_sender, room_receiver): (RoomSender, RoomReceiver) =
                    mpsc::unbounded_channel();

                (room.id(), RoomBroker::new(room_sender, Some(room_receiver)))
            })
            .collect::<Vec<(Id, RoomBroker)>>();

        let rooms = Rooms::new();

        RoomServiceState { rooms }
    }

    pub fn get_room_by_id(&self, id: &crate::Id) -> Option<&RoomBroker> {
        self.rooms.get(id)
    }
}

#[derive(Debug)]
pub struct RoomServiceMatcher {
    state: RoomServiceState,
}

impl RoomServiceMatcher {
    pub fn new(state: RoomServiceState) -> Self {
        RoomServiceMatcher { state }
    }
}

impl merchant::MatcherMut<merchant::ResourceEvent<RoomBroker>> for RoomServiceMatcher {
    fn match_on_mut(&mut self, event: merchant::ResourceEvent<RoomBroker>) -> merchant::Result<()> {
        match event {
            merchant::ResourceEvent::Get(id, reply_sender) => {
                let id = crate::Id(id);

                if let Some(room_broker) = self.state.get_room_by_id(&id) {
                    match reply_sender.send(merchant::ResourceEvent::GetSuccess(RoomBroker::new(
                        room_broker.sender(),
                        None,
                    ))) {
                        Ok(_) => (),
                        // @TODO: Error handling here
                        Err(_) => (),
                    }
                } else {
                    match reply_sender.send(merchant::ResourceEvent::GetFail) {
                        Ok(_) => (),
                        // @TODO: Error handling here
                        Err(_) => (),
                    }
                }

                Ok(())
            }
            _ => Ok(()),
        }
    }
}
