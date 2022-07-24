use super::{interface::Server, resources::interface::Resources};
use std::{default::Default, fmt::Debug};

#[derive(Debug)]
pub struct ServerBuilder {
    ip: Option<String>,
    port: Option<String>,
    resources: Option<Resources>,
}

impl Default for ServerBuilder {
    fn default() -> Self {
        ServerBuilder {
            ip: Some("127.0.0.1".to_owned()),
            port: Some("8080".to_owned()),
            resources: None,
        }
    }
}

impl ServerBuilder {
    pub fn ip(mut self, ip: &str) -> Self {
        let _ = self.ip.insert(ip.to_owned());
        self
    }

    pub fn port(mut self, port: &str) -> Self {
        let _ = self.port.insert(port.to_owned());
        self
    }

    pub fn resources(mut self, resources: Resources) -> Self {
        let _ = self.resources.insert(resources);
        self
    }

    pub fn build(self) -> Server {
        Server {
            ip: self.ip.unwrap(),
            port: self.port.unwrap(),
            resources: self.resources.unwrap(),
        }
    }
}
