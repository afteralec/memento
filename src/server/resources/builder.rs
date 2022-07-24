use super::interface::Resources;
use crate::{
    actor::data::ActorData, auth::traits::AuthClient, player::data::PlayerData,
    room::data::RoomData,
};
use std::{default::Default, fmt::Debug};

#[derive(Debug)]
pub struct ResourceBuilder<C, R, A, P>
where
    C: 'static + Send + Sync + Debug + Default + AuthClient,
    R: Iterator<Item = RoomData>,
    A: Iterator<Item = ActorData>,
    P: Iterator<Item = PlayerData>,
{
    pub auth_client: Option<C>,
    pub rooms: Option<R>,
    pub actors: Option<A>,
    pub players: Option<P>,
}

impl<C, R, A, P> Default for ResourceBuilder<C, R, A, P>
where
    C: 'static + Send + Sync + Debug + Default + AuthClient,
    R: Iterator<Item = RoomData>,
    A: Iterator<Item = ActorData>,
    P: Iterator<Item = PlayerData>,
{
    fn default() -> Self {
        ResourceBuilder {
            auth_client: Option::default(),
            rooms: Option::default(),
            actors: Option::default(),
            players: Option::default(),
        }
    }
}

impl<C, R, A, P> ResourceBuilder<C, R, A, P>
where
    C: 'static + Send + Sync + Debug + Default + AuthClient,
    R: Iterator<Item = RoomData>,
    A: Iterator<Item = ActorData>,
    P: Iterator<Item = PlayerData>,
{
    pub fn build(self) -> Resources {
        Resources::new(
            self.auth_client.unwrap(),
            self.rooms.unwrap(),
            self.actors.unwrap(),
            self.players.unwrap(),
        )
    }

    pub fn auth_client(mut self, client: C) -> Self {
        let _ = self.auth_client.insert(client);
        self
    }

    pub fn rooms(mut self, rooms: R) -> Self {
        let _ = self.rooms.insert(rooms);
        self
    }

    pub fn actors(mut self, actors: A) -> Self {
        let _ = self.actors.insert(actors);
        self
    }

    pub fn players(mut self, players: P) -> Self {
        let _ = self.players.insert(players);
        self
    }
}
