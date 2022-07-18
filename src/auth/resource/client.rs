use super::{AuthResponse, Credential};

pub trait AuthClient {
    fn validate(&self, credential: Credential) -> AuthResponse;
}
