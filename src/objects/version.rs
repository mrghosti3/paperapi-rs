use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::Str;

/// Represents the support status of a Minecraft version.
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SupportStatus {
    Supported,
    Deprecated,
    Unsupported,
}

/// Represents the support details for a Minecraft version.
#[derive(Debug, Clone, Deserialize)]
pub struct Support {
    /// End-of-life date for the version (nullable).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTime<Utc>>,
    /// Support status of the version.
    pub status: SupportStatus,
}

/// Java requirements (internal)
#[derive(Debug, Clone, Deserialize)]
struct JavaVersion {
    minimum: u32,
}

/// Java flags (internal)
#[derive(Debug, Clone, Deserialize)]
struct JavaFlags {
    recommended: Box<[Str]>,
}

/// Java requirements
#[derive(Debug, Clone, Deserialize)]
pub struct Java {
    version: JavaVersion,
    flags: JavaFlags,
}

impl Java {
    /// Get the minimum Java version required
    pub fn minimum(&self) -> u32 {
        self.version.minimum
    }

    /// Get the recommended JVM flags
    pub fn recommended(&self) -> &[Str] {
        &self.flags.recommended
    }
}

/// Minecraft version
#[derive(Debug, Clone, Deserialize)]
pub struct Version {
    pub id: Str,
    pub java: Java,
    pub support: Support,
}

/// Represents VersionResponse object from Paper API
#[derive(Debug, Clone, Deserialize)]
pub struct VersionResponse {
    /// The Minecraft version.
    pub version: Version,
    /// List of build IDs for this version.
    pub builds: Box<[u32]>,
}
