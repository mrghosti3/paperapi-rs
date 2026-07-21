//! Typed identifiers for projects, versions, and builds
use std::fmt::Display;

use crate::error::{PaperApiError, VersionField};

#[derive(Debug, Default)]
pub enum Project {
    #[default]
    Paper,
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Paper => "paper",
        };

        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct VersionId {
    pub major: u16,
    pub minor: u8,
    pub patch: u8,
    pub stable: bool,
}

impl<'a> TryFrom<&'a str> for VersionId {
    type Error = PaperApiError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let major: u16;
        let minor: u8;
        let patch: u8;
        let stable: bool;

        // splits iteration into: 1) version; 2) rc segment
        let sp1 = &mut value.splitn(2, '-');
        // for iterating the version itself
        let sp2 = &mut sp1.next().unwrap().splitn(3, '.');

        major = sp2
            .next()
            .map(|v| {
                v.parse()
                    .map_err(|e| PaperApiError::InvalidVersionField(VersionField::Major, e))
            })
            .unwrap_or(Err(PaperApiError::MissingVersionField(
                VersionField::Major,
                value,
            )))?;

        minor = sp2
            .next()
            .map(|v| {
                v.parse()
                    .map_err(|e| PaperApiError::InvalidVersionField(VersionField::Major, e))
            })
            .unwrap_or(Err(PaperApiError::MissingVersionField(
                VersionField::Major,
                value,
            )))?;

        patch = sp2
            .next()
            .map(|v| v.parse())
            .unwrap_or(Ok(0))
            .map_err(|e| PaperApiError::InvalidVersionField(VersionField::Patch, e))?;

        stable = sp1.next().is_none();

        Ok(Self {
            major,
            minor,
            patch,
            stable,
        })
    }
}

impl Display for VersionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[derive(Debug, Default)]
pub enum BuildId {
    #[default]
    Latest,
    Version(u32),
}

impl From<u32> for BuildId {
    fn from(value: u32) -> Self {
        Self::Version(value)
    }
}

impl std::fmt::Display for BuildId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildId::Latest => write!(f, "latest"),
            BuildId::Version(v) => write!(f, "{}", v),
        }
    }
}
