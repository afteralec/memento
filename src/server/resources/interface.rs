use super::builder::ResourceBuilder;
use crate::{
    actor::{
        data::ActorData,
        resource::{
            interface::ActorResource, resolver::ActorResourceResolver,
            types::ActorResourceMessenger,
        },
    },
    auth::{
        resource::{
            interface::AuthResource, resolver::AuthResourceResolver, types::AuthResourceMessenger,
        },
        traits::AuthClient,
    },
    messaging::traits::{Detach, Provide},
    player::{
        data::PlayerData,
        resource::{
            interface::PlayerResource, resolver::PlayerResourceResolver,
            types::PlayerResourceMessenger,
        },
    },
    room::{
        data::RoomData,
        resource::{
            interface::RoomResource, resolver::RoomResourceResolver, types::RoomResourceMessenger,
        },
    },
    session::resource::{
        interface::SessionResource, resolver::SessionResourceResolver,
        types::SessionResourceMessenger,
    },
};
use std::fmt::Debug;

#[readonly::make]
#[derive(Debug, Clone)]
pub struct Resources {
    pub actor: ActorResource,
    pub auth: AuthResource,
    pub player: PlayerResource,
    pub room: RoomResource,
    pub session: SessionResource,
}

impl Resources {
    pub fn new<C, R, A, P>(client: C, rooms: R, actors: A, players: P) -> Self
    where
        C: 'static + Send + Sync + Debug + Default + AuthClient,
        R: Iterator<Item = RoomData>,
        A: Iterator<Item = ActorData>,
        P: Iterator<Item = PlayerData>,
    {
        let mut auth_resource_messenger =
            AuthResourceMessenger::<C>::new("auth resource", AuthResourceResolver::new(client));
        let mut room_resource_messenger =
            RoomResourceMessenger::new("room resource", RoomResourceResolver::new(rooms));
        let mut actor_resource_messenger =
            ActorResourceMessenger::new("actor resource", ActorResourceResolver::new(actors));
        let mut player_resource_messenger =
            PlayerResourceMessenger::new("player resource", PlayerResourceResolver::new(players));
        let mut session_resource_messenger =
            SessionResourceMessenger::new("session resource", SessionResourceResolver::new());

        auth_resource_messenger.detach().unwrap();
        room_resource_messenger.detach().unwrap();
        actor_resource_messenger.detach().unwrap();
        player_resource_messenger.detach().unwrap();
        session_resource_messenger.detach().unwrap();

        Resources {
            actor: actor_resource_messenger.provide(),
            auth: auth_resource_messenger.provide(),
            player: player_resource_messenger.provide(),
            room: room_resource_messenger.provide(),
            session: session_resource_messenger.provide(),
        }
    }

    pub fn builder<C, R, A, P>() -> ResourceBuilder<C, R, A, P>
    where
        C: 'static + Send + Sync + Debug + Default + AuthClient,
        R: Iterator<Item = RoomData>,
        A: Iterator<Item = ActorData>,
        P: Iterator<Item = PlayerData>,
    {
        ResourceBuilder::default()
    }
}
