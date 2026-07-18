use std::fmt::Display;
use std::num::ParseIntError;

use thiserror::Error;

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

#[derive(Debug)]
pub enum VersionField {
    Major,
    Minor,
    Patch,
}

impl Display for VersionField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", stringify!(self))
    }
}
