use super::AuthResourceEvent;
use tokio::sync::mpsc;

pub type AuthResourceSender = mpsc::UnboundedSender<AuthResourceEvent>;
pub type AuthResourceReceiver = mpsc::UnboundedReceiver<AuthResourceEvent>;
