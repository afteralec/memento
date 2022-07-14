use super::{AuthResponse, Credential};
use anyhow::Result;

pub trait AuthClient {
    fn validate(&self, credential: Credential) -> AuthResponse;
}
