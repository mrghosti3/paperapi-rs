//! Error types for the PaperMC API client.
//!
//! This module defines custom error types that can occur when working
//! with the PaperMC API, including version parsing errors and validation errors.
use std::fmt::Display;
use std::num::ParseIntError;

use thiserror::Error;

/// Main error type for PaperMC API operations.
///
/// This enum represents all possible errors that can occur when
/// working with the PaperMC API client.
#[derive(Debug, Error)]
pub enum PaperApiError<'a> {
    #[error("invalid version format: {0}")]
    InvalidVersion(&'a str),

    #[error("invalid version {0} field format: {1}")]
    InvalidVersionField(VersionField, ParseIntError),

    #[error("missing version {0} field: {1}")]
    MissingVersionField(VersionField, &'a str),

    #[error("invalid sha256 value: len = {0}, '{1}'")]
    InvalidSha256Value(usize, &'a str),
}

/// Fields that can be invalid or missing in version strings.
///
/// Used in error reporting when parsing version identifiers.
#[derive(Debug)]
pub enum VersionField {
    /// Major version number (e.g., "1" in "1.20.1")
    Major,
    /// Minor version number (e.g., "20" in "1.20.1")
    Minor,
    /// Patch version number (e.g., "1" in "1.20.1")
    Patch,
}

impl Display for VersionField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", stringify!(self))
    }
}
