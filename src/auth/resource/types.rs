use super::AuthResourceEvent;
use tokio::sync::mpsc;

pub type AuthResourceSender = mpsc::UnboundedSender<AuthResourceEvent>;
pub type AuthResourceReceiver = mpsc::UnboundedReceiver<AuthResourceEvent>;

#[derive(Debug)]
pub enum AuthRequest {
    WithCredential(Credential),
}

#[derive(Debug)]
pub enum Credential {
    UserNameAndPassword(String, String),
}

pub enum AuthResponse {
    Authenticated,
    Forbidden,
}
