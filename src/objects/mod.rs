//! Data structures for PaperMC API objects.
//!
//! This module contains the main data types returned by the PaperMC API,
//! including versions, builds, and their associated metadata.
//!
//! ## Submodules
//!
//! - `error`: Error response structures from the PaperMC API
//! - `version`: Minecraft version information and support status
//! - `build`: Build information, downloads, and checksums
pub mod build;
pub mod error;
pub mod version;

#[cfg(test)]
mod test;
