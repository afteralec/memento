use crate::{
    actor::resource::interface::ActorResource, auth::resource::interface::AuthResource,
    player::resource::interface::PlayerResource, room::resource::interface::RoomResource,
    session::resource::interface::SessionResource,
};

#[derive(Debug, Clone)]
pub struct Resources {
    pub actor: ActorResource,
    pub auth: AuthResource,
    pub player: PlayerResource,
    pub room: RoomResource,
    pub session: SessionResource,
}

impl Resources {
    pub fn new(
        actor: ActorResource,
        auth: AuthResource,
        player: PlayerResource,
        room: RoomResource,
        session: SessionResource,
    ) -> Self {
        Resources {
            actor,
            auth,
            player,
            room,
            session,
        }
    }

    pub fn builder() -> ResourcesBuilder {
        ResourcesBuilder::default()
    }
}

#[derive(Debug)]
pub struct ResourcesBuilder {
    actor: Option<ActorResource>,
    auth: Option<AuthResource>,
    player: Option<PlayerResource>,
    room: Option<RoomResource>,
    session: Option<SessionResource>,
}

impl Default for ResourcesBuilder {
    fn default() -> Self {
        ResourcesBuilder {
            actor: None,
            auth: None,
            player: None,
            room: None,
            session: None,
        }
    }
}

impl ResourcesBuilder {
    pub fn builder() -> Self {
        ResourcesBuilder {
            ..Default::default()
        }
    }

    pub fn actor(mut self, actor: ActorResource) -> Self {
        let _ = self.actor.insert(actor);
        self
    }

    pub fn auth(mut self, auth: AuthResource) -> Self {
        let _ = self.auth.insert(auth);
        self
    }

    pub fn player(mut self, player: PlayerResource) -> Self {
        let _ = self.player.insert(player);
        self
    }

    pub fn room(mut self, room: RoomResource) -> Self {
        let _ = self.room.insert(room);
        self
    }

    pub fn session(mut self, session: SessionResource) -> Self {
        let _ = self.session.insert(session);
        self
    }

    pub fn build(self) -> Resources {
        Resources::new(
            self.actor.unwrap(),
            self.auth.unwrap(),
            self.player.unwrap(),
            self.room.unwrap(),
            self.session.unwrap(),
        )
    }
}
