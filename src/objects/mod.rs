//! Data structures for PaperMC API objects.
//!
//! This module contains the main data types returned by the PaperMC API,
//! including versions, builds, and their associated metadata.
pub mod build;
pub mod error;
pub mod version;

#[cfg(test)]
mod test;
