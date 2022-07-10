// use super::broker::{RoomBroker, RoomReceiver, RoomSender};
// use super::model::Room;
// use crate::Id;

// use merchant;
// use std::collections::HashMap;
// use tokio::sync::mpsc;

// pub type RoomServiceSender = mpsc::UnboundedSender<merchant::ResourceEvent<RoomBroker>>;
// pub type RoomServiceReceiver = mpsc::UnboundedReceiver<merchant::ResourceEvent<RoomBroker>>;
// pub type RoomService = merchant::ResourceBroker<RoomBroker>;
// pub type Rooms = HashMap<crate::Id, RoomBroker>;

// #[derive(Debug)]
// pub struct RoomServiceState {
//     room_brokers: Rooms,
// }

// impl RoomServiceState {
//     pub fn from_list(room_list: &[Room]) -> Self {
//         let room_brokers = room_list
//             .into_iter()
//             .map(|room| {
//                 let (room_sender, room_receiver): (RoomSender, RoomReceiver) =
//                     mpsc::unbounded_channel();

//                 (room.id(), RoomBroker::new(room_sender, room_receiver))
//             })
//             .fold(Rooms::new(), |mut rooms, (room_id, room_broker)| {
//                 rooms.insert(room_id, room_broker);
//                 rooms
//             });

//         RoomServiceState { room_brokers }
//     }

//     pub fn get_room_broker_by_id(&self, id: &crate::Id) -> Option<&RoomBroker> {
//         self.room_brokers.get(id)
//     }
// }

// #[derive(Debug)]
// pub struct RoomServiceMatcher {
//     state: RoomServiceState,
// }

// impl RoomServiceMatcher {
//     pub fn new(state: RoomServiceState) -> Self {
//         RoomServiceMatcher { state }
//     }
// }

// impl merchant::MatcherMut<merchant::ResourceEvent<RoomSender>> for RoomServiceMatcher {
//     fn match_on(&mut self, event: merchant::ResourceEvent<RoomSender>) -> merchant::Result<()> {
//         match event {
//             merchant::ResourceEvent::Get(id, reply_sender) => {
//                 let id = crate::Id(id);

//                 if let Some(room_broker) = self.state.get_room_broker_by_id(&id) {
//                     match reply_sender
//                         .send(merchant::ResourceEvent::GetSuccess(room_broker.sender()))
//                     {
//                         Ok(_) => (),
//                         // @TODO: Error handling here
//                         Err(_) => (),
//                     }
//                 } else {
//                     match reply_sender.send(merchant::ResourceEvent::GetFail) {
//                         Ok(_) => (),
//                         // @TODO: Error handling here
//                         Err(_) => (),
//                     }
//                 }

//                 Ok(())
//             }
//             _ => Ok(()),
//         }
//     }
// }
