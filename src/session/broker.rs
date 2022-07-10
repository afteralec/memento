use crate::{room, server};

use merchant;
use tokio::sync::mpsc;

#[derive(Debug)]
pub enum SessionEvent {
    NewRoomSender(room::RoomSender),
}

pub type SessionSender = mpsc::UnboundedSender<SessionEvent>;

pub type SessionBroker = merchant::StreamBroker<SessionEvent>;

#[derive(Debug)]
pub struct SessionState {
    room_sender: room::RoomSender,
    writer: server::StreamWriter,
    // @TODO: Action pool here
}

impl SessionState {
    pub fn new(room_sender: room::RoomSender, writer: mpsc::UnboundedSender<String>) -> Self {
        SessionState {
            room_sender,
            writer,
        }
    }

    pub fn new_room_sender(&mut self, room_sender: room::RoomSender) {
        self.room_sender = room_sender;
    }
}

#[derive(Debug)]
pub struct SessionMatcher {
    state: Box<SessionState>,
}

impl SessionMatcher {
    pub fn new(state: Box<SessionState>) -> Self {
        SessionMatcher { state }
    }
}

impl merchant::MatcherMut<SessionEvent> for SessionMatcher {
    fn match_on(&mut self, event: SessionEvent) -> crate::Result<()> {
        match event {
            SessionEvent::NewRoomSender(room_sender) => {
                self.state.new_room_sender(room_sender);
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct SessionStreamMatcher {
    state: Box<SessionState>,
}

impl SessionStreamMatcher {
    pub fn new(state: Box<SessionState>) -> Self {
        SessionStreamMatcher { state }
    }
}

impl merchant::MatcherMut<String> for SessionStreamMatcher {
    fn match_on(&mut self, input: String) -> crate::Result<()> {
        match input {
            _ => self
                .state
                .writer
                .send("That isn't something you can do.".to_string())?,
        }
        Ok(())
    }
}
