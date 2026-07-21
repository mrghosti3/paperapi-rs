//! Builder pattern for constructing API endpoints

use std::fmt::Display;

use url::{ParseError, Url};

use crate::Str;

use self::endpoints::UnsetApi;

use super::Endpoint;

pub mod endpoints;

/// Marker type indicating that the domain has not been set yet.
///
/// This is used as a type parameter in `EndpointBuilder` to enforce
/// that the domain must be set before building the endpoint.
/// The builder starts in this state and transitions to `Domain` state
/// when `set_domain()` is called.
#[derive(Debug)]
pub struct UnsetDomain;

/// Represents the domain configuration for PaperMC API endpoints.
///
/// This enum allows choosing between the default PaperMC API domain
/// or a custom domain for testing or alternative instances.
///
/// ## Variants
///
/// - `Default`: Uses the official PaperMC API domain (`https://fill.papermc.io`)
/// - `Custom(Str)`: Uses a custom domain specified as a boxed string
///
/// ## Example
///
/// ```rust
/// use paper_api::api::builder::Domain;
///
/// // Use the default PaperMC API
/// let default_domain = Domain::Default;
///
/// // Use a custom domain (e.g., for testing)
/// let custom_domain = Domain::Custom("https://custom.papermc.io".into());
/// ```
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

/// Main builder for constructing PaperMC API endpoints.
///
/// This builder uses a two-phase approach:
/// 1. Set the domain (default or custom)
/// 2. Set the specific endpoint builder
///
/// # Type Parameters
/// - `D`: Domain state (UnsetDomain or Domain)
/// - `A`: Endpoint state (UnsetApi or specific endpoint builder)
///
/// # Example - Using Default Domain
///
/// ```rust
/// use paper_api::api::Endpoint;
/// use paper_api::api::builder::endpoints::ProjectVersions;
/// use paper_api::api::ids::Project;
/// use paper_api::api::builder::Domain;
///
/// let endpoint = Endpoint::builder()
///     .set_domain(Domain::Default)
///     .set_endpoint(
///         ProjectVersions::new()
///             .v3()
///             .set_project(Project::Paper)
///     )
///     .build();
/// ```
///
/// # Example - Using Custom Domain
///
/// ```rust
/// use paper_api::api::builder::Domain;
/// use paper_api::api::Endpoint;
/// use paper_api::api::builder::endpoints::ProjectVersions;
/// use paper_api::api::ids::Project;
///
/// let endpoint = Endpoint::builder()
///     .set_domain(Domain::Custom("https://custom.papermc.io".into()))
///     .set_endpoint(
///         ProjectVersions::new()
///             .v3()
///             .set_project(Project::Paper)
///     )
///     .build();
/// ```
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
    /// Sets the domain for the endpoint builder.
    ///
    /// This must be called first to transition from UnsetDomain to Domain state.
    ///
    /// # Arguments
    ///
    /// * `domain` - The domain to use (Domain::Default or Domain::Custom)
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
