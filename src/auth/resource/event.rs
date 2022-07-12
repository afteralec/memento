use crate::Id;
use thiserror::Error;
use tokio::sync::oneshot;

#[derive(Debug, Error)]
pub enum AuthResourceEvent {
    #[error("AuthResource::Authenticate raised with id {} but channel is closed", .id)]
    Authenticate {
        id: Id,
        reply_sender: oneshot::Sender<AuthResourceReplyEvent>,
    },
}

#[derive(Debug, Error)]
pub enum AuthResourceReplyEvent {
    #[error("AuthResourceReply::Authenticated raised with id {0} but channel is closed")]
    Authenticated(Id),
    #[error("AuthResourceReply::Forbidden raised with id {0} but channel is closed")]
    Forbidden(Id),
    #[error("AuthResourceReply::Allowed raised with id {0} but channel is closed")]
    Allowed(Id),
    #[error("AuthResourceReply::NotAllowed raised with id {0} but channel is closed")]
    NotAllowed(Id),
}
