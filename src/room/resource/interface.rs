use crate::{messaging::Spawn, Room};
use super::{RoomResourceSender, RoomResourceReceiver, RoomResourceResolver};

use std::default::Default;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct RoomResource {
    sender: RoomResourceSender,
    receiver: Option<RoomResourceReceiver>,
    resolver: Option<RoomResourceResolver>,
}

impl Default for RoomResource {
    fn default() -> Self {
        let (room_resource_sender, room_resource_receiver) = mpsc::unbounded_channel();

        RoomResource {
            sender: room_resource_sender,
            receiver: Some(room_resource_receiver),
            resolver: Some(RoomResourceResolver::default()),
        }
    }
}

impl Spawn for RoomResource {}

impl RoomResource {
    pub fn new(room_iter: impl Iterator<Item = Room>) -> Self {
        RoomResource {
            resolver: Some(RoomResourceResolver::new(room_iter)),
            ..Default::default()
        }
    }

    pub fn sender(&self) -> RoomResourceSender {
        self.sender.clone()
    }
}
