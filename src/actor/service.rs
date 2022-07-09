use super::model::Actor;
use merchant;
use std::collections::HashMap;
use tokio::{sync, sync::mpsc};

#[derive(Debug)]
pub enum ActorServiceEvent {
    GetRoomById(crate::Id, sync::oneshot::Sender<ActorServiceEvent>),
}

pub type RoomServiceSender = mpsc::UnboundedSender<ActorServiceEvent>;

pub type RoomServiceBroker = merchant::Broker<ActorServiceEvent>;

pub type Actors = HashMap<crate::Id, Actor>;

#[derive(Debug)]
pub struct ActorServiceState {
    actors: Actors,
}

impl ActorServiceState {
    pub fn get_actor_by_id(&self, id: &crate::Id) -> Option<&Actor> {
        self.actors.get(id)
    }
}

#[derive(Debug)]
pub struct ActorServiceMatcher {
    state: ActorServiceState,
}

impl ActorServiceMatcher {
    pub fn new(state: ActorServiceState) -> Self {
        ActorServiceMatcher { state }
    }
}

impl merchant::MatcherMut<merchant::ResourceEvent<Actor>> for ActorServiceMatcher {
    fn match_on_mut(&mut self, event: merchant::ResourceEvent<Actor>) -> merchant::Result<()> {
        match event {
            merchant::ResourceEvent::Get(id, reply_sender) => {
                let id = crate::Id(id);

                if let Some(actor) = self.state.get_actor_by_id(&id) {
                    match reply_sender.send(merchant::ResourceEvent::GetSuccess(actor.clone())) {
                        Ok(_) => (),
                        // @TODO: Error handling here
                        Err(_) => (),
                    }
                } else {
                    match reply_sender.send(merchant::ResourceEvent::GetFail) {
                        Ok(_) => (),
                        // @TODO: Error handling here
                        Err(_) => (),
                    }
                }

                Ok(())
            }
            _ => Ok(()),
        }
    }
}
