use super::{input::ServerInput, interface::Server, resources::interface::Resources};
use crate::{
    actor::data::ActorData, auth::traits::AuthClient, player::data::PlayerData,
    room::data::RoomData,
};
use anyhow::Result;
use std::{default::Default, fmt::Debug};

pub fn init<C, R, A, P>(server_input: ServerInput<C, R, A, P>) -> Result<Server>
where
    C: 'static + Send + Sync + Debug + Default + AuthClient,
    R: 'static + Send + Sync + Iterator<Item = RoomData>,
    A: 'static + Send + Sync + Iterator<Item = ActorData>,
    P: 'static + Send + Sync + Iterator<Item = PlayerData>,
{
    // @TODO: Separate the logic for detaching the resources from the build call
    let resources = Resources::builder()
        .auth_client(server_input.auth_client)
        .rooms(server_input.rooms)
        .actors(server_input.actors)
        .players(server_input.players)
        .build();

    let mut server = Server::builder().resources(resources).build();

    server.listen()?;

    Ok(server)
}
