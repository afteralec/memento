use super::model::Server;
use crate::{
    actor::resource::ActorResource,
    core::AuthClient,
    resource::{AuthResource, PlayerResource, RoomResource, SessionResource},
};
use std::{default::Default, fmt::Debug};

#[derive(Debug)]
pub struct ServerBuilder<T>
where
    T: 'static + Send + Sync + Debug + Default + AuthClient,
{
    ip: Option<String>,
    port: Option<String>,
    actor_resource: Option<ActorResource>,
    auth_resource: Option<AuthResource<T>>,
    player_resource: Option<PlayerResource>,
    room_resource: Option<RoomResource>,
    session_resource: Option<SessionResource>,
}

impl<T> Default for ServerBuilder<T>
where
    T: 'static + Send + Sync + Debug + Default + AuthClient,
{
    fn default() -> Self {
        ServerBuilder {
            ip: Some("127.0.0.1".to_owned()),
            port: Some("8080".to_owned()),
            actor_resource: None,
            auth_resource: None,
            player_resource: None,
            room_resource: None,
            session_resource: Some(SessionResource::new()),
        }
    }
}

impl<T> ServerBuilder<T>
where
    T: 'static + Send + Sync + Debug + Default + AuthClient,
{
    pub fn ip(mut self, ip: &str) -> Self {
        let _ = self.ip.insert(ip.to_owned());
        self
    }

    pub fn port(mut self, port: &str) -> Self {
        let _ = self.port.insert(port.to_owned());
        self
    }

    pub fn actor_resource(mut self, actor_resource: ActorResource) -> Self {
        let _ = self.actor_resource.insert(actor_resource);
        self
    }

    pub fn auth_resource(mut self, auth_resource: AuthResource<T>) -> Self {
        let _ = self.auth_resource.insert(auth_resource);
        self
    }

    pub fn player_resource(mut self, player_resource: PlayerResource) -> Self {
        let _ = self.player_resource.insert(player_resource);
        self
    }

    pub fn room_resource(mut self, room_resource: RoomResource) -> Self {
        let _ = self.room_resource.insert(room_resource);
        self
    }

    pub fn session_resource(mut self, session_resource: SessionResource) -> Self {
        let _ = self.session_resource.insert(session_resource);
        self
    }

    pub fn build(self) -> Server<T> {
        Server {
            ip: self.ip.unwrap(),
            port: self.port.unwrap(),
            actor_resource: self.actor_resource.unwrap(),
            auth_resource: self.auth_resource.unwrap(),
            player_resource: self.player_resource.unwrap(),
            room_resource: self.room_resource.unwrap(),
            session_resource: self.session_resource.unwrap(),
        }
    }
}
