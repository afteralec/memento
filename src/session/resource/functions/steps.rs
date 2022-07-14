use crate::{
    auth::{
        AuthResourceReplyEvent, AuthResponse, AuthResourceReplyReceiver
    },
};
use anyhow::{Result, Error};
use tokio::sync::oneshot::error::TryRecvError;

pub fn auth_step(mut receiver: AuthResourceReplyReceiver) -> Result<AuthResponse> {
    loop {
        match receiver.try_recv() {
            Ok(reply) => {
                match reply {
                    AuthResourceReplyEvent::Response(auth_response) => {
                        return Ok(auth_response);
                    }
                }
            }
            Err(err) => {
                match err {
                    TryRecvError::Closed => {
                        return Err(Error::new(err));
                    }
                    TryRecvError::Empty => { continue; },
                }
            }
        }
    }
}
