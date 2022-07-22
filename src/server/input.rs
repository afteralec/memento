use crate::{
    actor::model::Actor, auth::traits::AuthClient, player::model::Player, room::model::Room,
};
use std::{default::Default, fmt::Debug};

#[derive(Debug)]
pub struct ServerInput<C, A, P, R>
where
    C: 'static + Send + Sync + Debug + Default + AuthClient,
    A: Iterator<Item = Actor>,
    P: Iterator<Item = Player>,
    R: Iterator<Item = Room>,
{
    pub ip: String,
    pub port: String,
    pub auth_client: C,
    pub actors: A,
    pub players: P,
    pub rooms: R,
}

impl<C, A, P, R> ServerInput<C, A, P, R>
where
    C: 'static + Send + Sync + Debug + Default + AuthClient,
    A: Iterator<Item = Actor>,
    P: Iterator<Item = Player>,
    R: Iterator<Item = Room>,
{
    pub fn builder() -> ServerInputBuilder<C, A, P, R> {
        ServerInputBuilder::default()
    }
}

#[derive(Debug)]
pub struct ServerInputBuilder<C, A, P, R>
where
    C: 'static + Send + Sync + Debug + Default + AuthClient,
    A: Iterator<Item = Actor>,
    P: Iterator<Item = Player>,
    R: Iterator<Item = Room>,
{
    ip: Option<String>,
    port: Option<String>,
    auth_client: Option<C>,
    actors: Option<A>,
    players: Option<P>,
    rooms: Option<R>,
}

impl<C, A, P, R> Default for ServerInputBuilder<C, A, P, R>
where
    C: 'static + Send + Sync + Debug + Default + AuthClient,
    A: Iterator<Item = Actor>,
    P: Iterator<Item = Player>,
    R: Iterator<Item = Room>,
{
    fn default() -> Self {
        ServerInputBuilder {
            ip: Some("127.0.0.1".to_owned(),),
            port: Some("8080".to_owned()),
            auth_client: None,
            actors: None,
            players: None,
            rooms: None,
        }
    }
}

impl<C, A, P, R> ServerInputBuilder<C, A, P, R>
where
    C: 'static + Send + Sync + Debug + Default + AuthClient,
    A: Iterator<Item = Actor>,
    P: Iterator<Item = Player>,
    R: Iterator<Item = Room>,
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

    pub fn build(self) -> ServerInput<C, A, P, R> {
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
