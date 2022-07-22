use super::{input::ServerInput, model::Server, resource_proxy::ResourceProxies};
use crate::{
    actor::{model::Actor, resource::ActorResource},
    auth::{resource::AuthResource, traits::AuthClient},
    messaging::traits::ProvideProxy,
    player::{model::Player, resource::PlayerResource},
    room::{model::Room, resource::RoomResource},
    session::resource::SessionResource,
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
    let mut session_resource = SessionResource::new();

    let resource_proxies = ResourceProxies::builder()
        .actor_resource_proxy(actor_resource.proxy())
        .auth_resource_proxy(auth_resource.proxy())
        .player_resource_proxy(player_resource.proxy())
        .room_resource_proxy(room_resource.proxy())
        .session_resource_proxy(session_resource.proxy())
        .build();

    // @TODO: Remove this reflexive reference - might be a good use for an Arc<MutexGuard>
    session_resource.set_resource_proxies(resource_proxies.clone());

    let mut server = Server::builder()
        .actor_resource(actor_resource)
        .auth_resource(auth_resource)
        .player_resource(player_resource)
        .room_resource(room_resource)
        .session_resource(session_resource)
        .resource_proxies(resource_proxies)
        .build();

    server.detach_all()?;
    server.listen()?;

    Ok(server)
}
