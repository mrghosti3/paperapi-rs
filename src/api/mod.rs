//! API module for constructing PaperMC API endpoints.
//!
//! This module provides the main `Endpoint` type and builder pattern
//! for constructing URLs to interact with the PaperMC API.
use std::fmt::Display;

use self::builder::{UnsetDomain, endpoints::UnsetApi};

pub mod builder;
pub mod ids;

#[cfg(test)]
mod test;

/// A constructed PaperMC API endpoint URL.
///
/// This type represents a fully constructed URL for making requests
/// to the PaperMC API. It can be converted to a `url::Url` or displayed
/// as a string.
///
/// ## Example
/// ```rust
/// use paper_api::api::Endpoint;
/// use paper_api::api::builder::endpoints::ProjectVersions;
/// use paper_api::api::ids::Project;
/// use paper_api::api::builder::Domain;
///
/// let endpoint = Endpoint::builder()
///     .set_domain(Domain::Default)
///     .set_endpoint(ProjectVersions::new()
///         .v3()
///         .set_project(Project::Paper))
///     .build()
///     .unwrap();
///
/// println!("URL: {}", endpoint); // Display as string
/// let url: url::Url = endpoint.into(); // Convert to url::Url
/// ```
#[repr(transparent)]
pub struct Endpoint(url::Url);

impl Endpoint {
    /// Create a new endpoint builder.
    ///
    /// Returns a builder that starts with unset domain and API endpoint.
    /// Use the builder methods to configure the desired endpoint.
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
