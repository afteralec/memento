use crate::{
    actor::data::ActorData, auth::traits::AuthClient, player::data::PlayerData,
    room::data::RoomData,
};
use std::{default::Default, fmt::Debug};

#[derive(Debug)]
pub struct ServerInput<C, R, A, P>
where
    C: 'static + Send + Sync + Debug + Default + AuthClient,
    R: Iterator<Item = RoomData>,
    A: Iterator<Item = ActorData>,
    P: Iterator<Item = PlayerData>,
{
    pub ip: String,
    pub port: String,
    pub auth_client: C,
    pub actors: A,
    pub players: P,
    pub rooms: R,
}

impl<C, R, A, P> ServerInput<C, R, A, P>
where
    C: 'static + Send + Sync + Debug + Default + AuthClient,
    A: Iterator<Item = ActorData>,
    P: Iterator<Item = PlayerData>,
    R: Iterator<Item = RoomData>,
{
    pub fn builder() -> ServerInputBuilder<C, R, A, P> {
        ServerInputBuilder::default()
    }
}

#[derive(Debug)]
pub struct ServerInputBuilder<C, R, A, P>
where
    C: 'static + Send + Sync + Debug + Default + AuthClient,
    A: Iterator<Item = ActorData>,
    P: Iterator<Item = PlayerData>,
    R: Iterator<Item = RoomData>,
{
    ip: Option<String>,
    port: Option<String>,
    auth_client: Option<C>,
    actors: Option<A>,
    players: Option<P>,
    rooms: Option<R>,
}

impl<C, R, A, P> Default for ServerInputBuilder<C, R, A, P>
where
    C: 'static + Send + Sync + Debug + Default + AuthClient,
    A: Iterator<Item = ActorData>,
    P: Iterator<Item = PlayerData>,
    R: Iterator<Item = RoomData>,
{
    fn default() -> Self {
        ServerInputBuilder {
            ip: Some("127.0.0.1".to_owned()),
            port: Some("8080".to_owned()),
            auth_client: None,
            actors: None,
            players: None,
            rooms: None,
        }
    }
}

impl<C, R, A, P> ServerInputBuilder<C, R, A, P>
where
    C: 'static + Send + Sync + Debug + Default + AuthClient,
    A: Iterator<Item = ActorData>,
    P: Iterator<Item = PlayerData>,
    R: Iterator<Item = RoomData>,
{
    pub fn ip(mut self, ip: &str) -> Self {
        let _ = self.ip.insert(ip.to_owned());
        self
    }

    pub fn port(mut self, port: &str) -> Self {
        let _ = self.port.insert(port.to_owned());
        self
    }

    pub fn auth_client(mut self, auth_client: C) -> Self {
        let _ = self.auth_client.insert(auth_client);
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

    pub fn rooms(mut self, rooms: R) -> Self {
        let _ = self.rooms.insert(rooms);
        self
    }

    pub fn build(self) -> ServerInput<C, R, A, P> {
        ServerInput {
            ip: self.ip.unwrap(),
            port: self.port.unwrap(),
            auth_client: self.auth_client.unwrap(),
            actors: self.actors.unwrap(),
            players: self.players.unwrap(),
            rooms: self.rooms.unwrap(),
        }
    }
}
