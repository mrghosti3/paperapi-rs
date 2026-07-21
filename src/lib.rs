//! # PaperMC API utility crate
//!
//! A Rust library for interacting with the PaperMC API.
//! Provides type-safe structures and builders for working with Minecraft versions and builds.
//!
//! ## Main Modules
//!
//! - `api`: Endpoint builders and ID parsing for PaperMC API
//! - `error`: Custom error types for API interactions
//! - `objects`: Data structures for versions, builds, and related objects
//!
//! ## Quick Start
//!
//! ```rust
//! use paper_api::api::Endpoint;
//! use paper_api::api::builder::endpoints::ProjectVersions;
//! use paper_api::api::ids::Project;
//!
//! // Build a PaperMC API endpoint
//! let endpoint = Endpoint::builder()
//!     .set_domain(Default::default())
//!     .set_endpoint(ProjectVersions::new()
//!         .v3()
//!         .set_project(Project::Paper))
//!     .build()
//!     .unwrap();
//! ```

pub mod api;
pub mod error;
pub mod objects;

/// Boxed string type used throughout the library for efficient string handling.
type Str = Box<str>;
