use std::fmt::Display;

use url::{ParseError, Url};

use crate::Str;

use self::endpoints::UnsetApi;

use super::Endpoint;

pub mod endpoints;

#[derive(Debug)]
pub struct UnsetDomain;

#[derive(Debug, Default)]
pub enum Domain {
    #[default]
    Default,
    Custom(Str),
}

impl Display for Domain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Default => "https://fill.papermc.io",
            Self::Custom(d) => d.as_ref(),
        })
    }
}

#[derive(Debug)]
pub struct EndpointBuilder<D, A> {
    domain: D,
    endpoint: A,
}

impl EndpointBuilder<UnsetDomain, UnsetApi> {
    pub(crate) fn new() -> Self {
        Self {
            domain: UnsetDomain,
            endpoint: UnsetApi,
        }
    }
}

impl<A> EndpointBuilder<UnsetDomain, A> {
    pub fn set_domain(self, domain: Domain) -> EndpointBuilder<Domain, A> {
        EndpointBuilder {
            domain,
            endpoint: self.endpoint,
        }
    }
}

impl<D> EndpointBuilder<D, UnsetApi> {
    pub fn set_endpoint<A>(self, endpoint: A) -> EndpointBuilder<D, A> {
        EndpointBuilder {
            domain: self.domain,
            endpoint,
        }
    }
}

impl<T: Builder> EndpointBuilder<Domain, T> {
    pub fn build<'a>(self) -> Result<Endpoint, ParseError> {
        let url =
            Url::parse(format!("{}/{}", self.domain, self.endpoint.build().as_ref()).as_str())?;
        Ok(Endpoint(url))
    }
}

pub trait Builder {
    fn build(self) -> Str;
}

pub mod api_fields {
    #[derive(Debug, Default)]
    pub struct UnsetProject;

    #[derive(Debug, Default)]
    pub struct UnsetVersion;

    #[derive(Debug, Default)]
    pub struct UnsetBuild;
}
