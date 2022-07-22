use crate::{
    actor::resource::proxy::ActorResourceProxy, auth::resource::proxy::AuthResourceProxy,
    player::resource::proxy::PlayerResourceProxy, room::resource::proxy::RoomResourceProxy,
    session::resource::proxy::SessionResourceProxy,
};

#[derive(Debug, Clone)]
pub struct ResourceProxies {
    pub actor_resource_proxy: ActorResourceProxy,
    pub auth_resource_proxy: AuthResourceProxy,
    pub player_resource_proxy: PlayerResourceProxy,
    pub room_resource_proxy: RoomResourceProxy,
    pub session_resource_proxy: SessionResourceProxy,
}

impl ResourceProxies {
    pub fn new(
        actor_resource_proxy: ActorResourceProxy,
        auth_resource_proxy: AuthResourceProxy,
        player_resource_proxy: PlayerResourceProxy,
        room_resource_proxy: RoomResourceProxy,
        session_resource_proxy: SessionResourceProxy,
    ) -> Self {
        ResourceProxies {
            actor_resource_proxy,
            auth_resource_proxy,
            player_resource_proxy,
            room_resource_proxy,
            session_resource_proxy,
        }
    }

    pub fn builder() -> ResourceProxiesBuilder {
        ResourceProxiesBuilder::default()
    }
}

#[derive(Debug)]
pub struct ResourceProxiesBuilder {
    actor_resource_proxy: Option<ActorResourceProxy>,
    auth_resource_proxy: Option<AuthResourceProxy>,
    player_resource_proxy: Option<PlayerResourceProxy>,
    room_resource_proxy: Option<RoomResourceProxy>,
    session_resource_proxy: Option<SessionResourceProxy>,
}

impl Default for ResourceProxiesBuilder {
    fn default() -> Self {
        ResourceProxiesBuilder {
            actor_resource_proxy: None,
            auth_resource_proxy: None,
            player_resource_proxy: None,
            room_resource_proxy: None,
            session_resource_proxy: None,
        }
    }
}

impl ResourceProxiesBuilder {
    pub fn builder() -> Self {
        ResourceProxiesBuilder {
            ..Default::default()
        }
    }

    pub fn actor_resource_proxy(mut self, actor_resource_proxy: ActorResourceProxy) -> Self {
        let _ = self.actor_resource_proxy.insert(actor_resource_proxy);
        self
    }

    pub fn auth_resource_proxy(mut self, auth_resource_proxy: AuthResourceProxy) -> Self {
        let _ = self.auth_resource_proxy.insert(auth_resource_proxy);
        self
    }

    pub fn player_resource_proxy(mut self, player_resource_proxy: PlayerResourceProxy) -> Self {
        let _ = self.player_resource_proxy.insert(player_resource_proxy);
        self
    }

    pub fn room_resource_proxy(mut self, room_resource_proxy: RoomResourceProxy) -> Self {
        let _ = self.room_resource_proxy.insert(room_resource_proxy);
        self
    }

    pub fn session_resource_proxy(mut self, session_resource_proxy: SessionResourceProxy) -> Self {
        let _ = self.session_resource_proxy.insert(session_resource_proxy);
        self
    }

    pub fn build(self) -> ResourceProxies {
        ResourceProxies::new(
            self.actor_resource_proxy.unwrap(),
            self.auth_resource_proxy.unwrap(),
            self.player_resource_proxy.unwrap(),
            self.room_resource_proxy.unwrap(),
            self.session_resource_proxy.unwrap(),
        )
    }
}
