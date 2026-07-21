use std::fmt::Display;

use self::builder::{endpoints::UnsetApi, UnsetDomain};

pub mod ids;
pub mod builder;

#[cfg(test)]
mod test;

#[repr(transparent)]
pub struct Endpoint(url::Url);

impl Endpoint {
    pub fn builder() -> builder::EndpointBuilder<UnsetDomain, UnsetApi> {
        builder::EndpointBuilder::new()
    }
}

impl Into<url::Url> for Endpoint {
    fn into(self) -> url::Url {
        self.0
    }
}

impl Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
