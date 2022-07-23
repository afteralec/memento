use super::{input::ServerInput, model::Server};
use crate::{
    actor::{model::Actor, resource::ActorResource},
    auth::{resource::AuthResource, traits::AuthClient},
    player::{model::Player, resource::PlayerResource},
    room::{model::Room, resource::RoomResource},
};
use anyhow::Result;
use std::{default::Default, fmt::Debug};

pub fn init<C, A, P, R>(server_input: ServerInput<C, A, P, R>) -> Result<Server<C>>
where
    C: 'static + Send + Sync + Debug + Default + AuthClient,
    A: 'static + Send + Sync + Iterator<Item = Actor>,
    P: 'static + Send + Sync + Iterator<Item = Player>,
    R: 'static + Send + Sync + Iterator<Item = Room>,
{
    let auth_resource = AuthResource::<C>::new(server_input.auth_client);
    let actor_resource = ActorResource::new(server_input.actors);
    let player_resource = PlayerResource::new(server_input.players);
    let room_resource = RoomResource::new(server_input.rooms);

    let mut server = Server::builder()
        .actor_resource(actor_resource)
        .auth_resource(auth_resource)
        .player_resource(player_resource)
        .room_resource(room_resource)
        .build();

    server.detach_all()?;
    server.listen()?;

    Ok(server)
}
