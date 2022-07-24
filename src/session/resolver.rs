use super::event::SessionEvent;
use crate::{messaging::traits::Resolver, player::interface::Player, room::interface::Room};
use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug)]
pub struct SessionResolver {
    state: SessionState,
}

#[async_trait]
impl Resolver<SessionEvent> for SessionResolver {
    fn resolve_on(&mut self, event: SessionEvent) -> Result<()> {
        match event {
            SessionEvent::NewRoom(room) => {
                self.state.set_room(room);

                Ok(())
            }
            SessionEvent::AttachPlayer(player) => {
                self.state.set_player(player);

                Ok(())
            }
            SessionEvent::Input(input) => {
                tracing::debug!("got input: {:?}", input);

                Ok(())
            }
        }
    }

    async fn resolve_async(&mut self, _: SessionEvent) -> Result<()> {
        unimplemented!(
            "Async resolution not supported for SessionResolver, use resolve_on instead."
        );
    }
}

impl SessionResolver {
    pub fn new() -> Self {
        SessionResolver {
            state: SessionState::new(),
        }
    }
}

#[derive(Debug)]
pub struct SessionState {
    room: Option<Room>,
    player: Option<Player>,
}

impl SessionState {
    pub fn new() -> Self {
        SessionState { room: None, player: None, }
    }

    pub fn set_room(&mut self, room: Room) {
        let _ = self.room.insert(room);
    }

    pub fn set_player(&mut self, player: Player) {
        let _ = self.player.insert(player);
    }
}
