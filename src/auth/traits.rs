use super::resource::{AuthResponse, Credential};

pub trait AuthClient {
    fn validate(&self, credential: Credential) -> AuthResponse;
}
