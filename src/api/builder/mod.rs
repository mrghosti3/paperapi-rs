use url::{ParseError, Url};

use self::endpoints::UnsetApi;

use super::Endpoint;

pub mod endpoints;

#[derive(Debug)]
pub struct EndpointBuilder<T> {
    endpoint: T,
}

impl EndpointBuilder<UnsetApi> {
    pub(crate) fn new() -> Self {
        Self {
            endpoint: Default::default(),
        }
    }

    pub fn set_endpoint<T>(self, endpoint: T) -> EndpointBuilder<T> {
        EndpointBuilder { endpoint }
    }
}

impl<T: Builder> EndpointBuilder<T> {
    pub fn build<'a>(self) -> Result<Endpoint, ParseError> {
        Ok(Endpoint(self.endpoint.build()?))
    }
}

pub trait Builder {
    fn build(self) -> Result<Url, ParseError>;
}

pub mod api_fields {
    #[derive(Debug, Default)]
    pub struct UnsetProject;

    #[derive(Debug, Default)]
    pub struct UnsetVersion;

    #[derive(Debug, Default)]
    pub struct UnsetBuild;
}
