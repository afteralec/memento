use super::AuthResourceReplySender;
use crate::Id;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthResourceEvent {
    #[error("AuthResource::Authenticate raised but channel is closed")]
    Request(AuthRequest, AuthResourceReplySender),
}

#[derive(Debug, Error)]
pub enum AuthResourceReplyEvent {
    #[error("AuthResourceReply::Response raised but channel is closed")]
    Response(AuthResponse),
}

#[derive(Debug)]
pub enum AuthRequest {
    WithCredential(Credential),
}

#[derive(Debug)]
pub enum Credential {
    UserNameAndPassword(String, String),
}

#[derive(Debug)]
pub enum AuthResponse {
    Authenticated {
        id: Id,
        player_id: Id,
        actor_owned: Option<Id>,
        // @TODO: Do I need multiple actors owned by a player?
        // @TODO: Add modeling for initial permissions here
    },
    Forbidden,
}
