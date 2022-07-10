use crate::Id;

use merchant;
use std::{collections::HashMap, default::Default};
use tokio::sync::mpsc;
use unique_id::sequence;

pub type RoomDelays = HashMap<Id, merchant::DelayBroker>;
pub type DelaySender = mpsc::UnboundedSender<merchant::DelayEvent>;
pub type DelayReceiver = mpsc::UnboundedReceiver<merchant::DelayEvent>;

#[derive(Debug)]
pub struct RoomDelayState {
    id_generator: sequence::SequenceGenerator,
    delays: RoomDelays,
}

impl RoomDelayState {
    pub fn new() -> Self {
        RoomDelayState {
            ..Default::default()
        }
    }

    pub fn remove_delay(&mut self, id: &Id) {
        self.delays.remove(id);

        // When the delay table is empty, reset the SequenceGenerator
        if self.delays.is_empty() {
            self.id_generator = sequence::SequenceGenerator::default();
        }
    }
}

impl Default for RoomDelayState {
    fn default() -> Self {
        RoomDelayState {
            id_generator: sequence::SequenceGenerator::default(),
            delays: RoomDelays::new(),
        }
    }
}
