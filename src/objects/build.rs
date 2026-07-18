use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::collections::HashMap;

use crate::Str;
use crate::error::PaperApiError;

const SHA256_LEN: usize = 256 / 8;
const SHA256_STR_LEN: usize = SHA256_LEN * 2;

/// SHA-256 hash represented as a 32-byte array (256 bits).
///
/// This is a transparent wrapper around [u8; 32] for type safety.
/// Deserializes from hexadecimal string format as used in PaperMC API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(try_from = "&str")]
pub struct Sha256([u8; SHA256_LEN]);

/// Parses a SHA-256 hash from a hexadecimal string.
///
/// Accepts 64-character hexadecimal strings (0-9, a-f, A-F).
/// Case-insensitive: converts input to lowercase before parsing.
/// Returns `PaperApiError::InvalidSha256Value` for invalid input.
/// Used by serde for deserialization from PaperMC API JSON responses.
impl<'a> TryFrom<&'a str> for Sha256 {
    type Error = PaperApiError<'a>;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        use PaperApiError::InvalidSha256Value;

        if s.len() < SHA256_STR_LEN {
            return Err(InvalidSha256Value(s.len(), s));
        }

        fn hex_to_int(c: u8) -> Option<u8> {
            if b'0' <= c && c <= b'9' {
                return Some(c - b'0');
            }

            if b'a' <= c && c <= b'f' {
                return Some(c - b'a' + 10);
            }

            None
        }

        let mut buf = [0u8; 32];
        let iter = s
            .chars()
            .step_by(2)
            .zip(s.chars().skip(1).step_by(2))
            .enumerate()
            .map(|(i, (c1, c2))| {
                (
                    i,
                    c1.to_ascii_lowercase() as u8,
                    c2.to_ascii_lowercase() as u8,
                )
            });

        for (i, c1, c2) in iter {
            let c1 = hex_to_int(c1).ok_or(InvalidSha256Value(s.len(), s))?;
            let c2 = hex_to_int(c2).ok_or(InvalidSha256Value(s.len(), s))?;
            buf[i] = c1 << 4 | c2;
        }

        Ok(Self(buf))
    }
}

impl PartialEq<[u8; SHA256_LEN]> for Sha256 {
    fn eq(&self, other: &[u8; SHA256_LEN]) -> bool {
        for (a, b) in self.0.iter().zip(other.iter()) {
            if a != b {
                return false;
            }
        }

        true
    }
}

/// File checksums for verifying download integrity.
///
/// PaperMC provides SHA-256 checksums for all downloadable files to ensure
/// they haven't been tampered with and match the expected content.
#[derive(Debug, Clone, Deserialize)]
pub struct Checksums {
    /// SHA-256 hash of the file content.
    ///
    /// This is a 32-byte (256-bit) cryptographic hash that uniquely identifies
    /// the file content. Can be used to verify downloads match the expected build.
    sha256: Sha256,
}

impl Checksums {
    /// Returns a reference to the SHA-256 hash for file verification.
    ///
    /// This hash can be used to verify that downloaded files match the expected
    /// content and haven't been tampered with.
    pub fn digest(&self) -> &Sha256 {
        &self.sha256
    }
}

/// Downloadable file information for a build artifact.
///
/// Each build provides multiple downloadable files (JARs) for different purposes.
/// This struct contains all the information needed to download and verify a file.
#[derive(Debug, Clone, Deserialize)]
pub struct Download {
    /// Human-readable name of the download file.
    ///
    /// Typically follows the pattern: `{project}-{version}-{build}.jar`
    /// Example: "paper-1.20.4-477.jar"
    pub name: Str,

    /// Size of the file in bytes.
    ///
    /// Useful for progress tracking during downloads and verifying
    /// the complete file was downloaded.
    pub size: usize,

    /// Direct URL to download the file.
    ///
    /// This is a full HTTPS URL pointing to the file on PaperMC's CDN.
    /// Example: "https://fill-data.papermc.io/v1/objects/{hash}/{filename}"
    pub url: Str,

    /// Checksums for verifying the downloaded file.
    ///
    /// Contains cryptographic hashes to verify the file integrity after download.
    checksums: Checksums,
}

impl Download {
    /// Returns a reference to the SHA-256 hash for verifying this download.
    ///
    /// This hash can be used to verify that the downloaded file matches the expected
    /// content and hasn't been tampered with during transfer.
    pub fn digest(&self) -> &Sha256 {
        self.checksums.digest()
    }
}

/// Build release channel indicating stability and recommendation level.
///
/// PaperMC uses different channels to categorize builds based on their
/// stability, testing level, and recommendation status. This helps users
/// choose appropriate builds for their needs (development vs production).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Channel {
    /// Alpha builds - Experimental and potentially unstable.
    ///
    /// These builds contain the latest features and fixes but may have
    /// significant bugs or compatibility issues. Suitable only for testing
    /// and development environments.
    Alpha,

    /// Beta builds - More stable than Alpha but still in testing.
    ///
    /// These builds have undergone some testing and are more stable than
    /// Alpha builds, but may still contain bugs. Suitable for staging
    /// environments and adventurous users.
    Beta,

    /// Stable builds - Generally safe for production use.
    ///
    /// These builds have been tested and are considered stable enough
    /// for production use. They receive regular updates and bug fixes.
    Stable,

    /// Recommended builds - Thoroughly tested and recommended for production.
    ///
    /// These builds have undergone extensive testing and are officially
    /// recommended by the PaperMC team for production servers. They offer
    /// the best balance of features, stability, and performance.
    Recommended,
}

/// Complete information about a specific build of a PaperMC project.
///
/// This struct represents a single build of a PaperMC project (like Paper)
/// for a specific Minecraft version. Each build has a unique ID, timestamp,
/// release channel, and associated downloadable files.
///
/// Obtained from endpoints like:
/// - GET /v3/projects/{project}/versions/{version}/builds/{build}
/// - GET /v3/projects/{project}/versions/{version}/builds/latest
#[derive(Debug, Clone, Deserialize)]
pub struct BuildResponse {
    /// Unique numerical identifier for this build.
    ///
    /// Build IDs are sequential integers starting from 1 for each version.
    /// Higher numbers indicate newer builds.
    pub id: u32,

    /// Timestamp when this build was created.
    ///
    /// ISO 8601 formatted timestamp indicating when the build was compiled
    /// and made available. Useful for determining how recent a build is.
    pub time: DateTime<Utc>,

    /// Release channel of this build.
    ///
    /// Indicates the stability and recommendation level of this build.
    /// See the `Channel` enum documentation for details on each channel.
    pub channel: Channel,

    /// Available download files for this build.
    ///
    /// A map of download keys to `Download` structs. Common keys include:
    /// - "server:default" - The main server JAR file
    /// - "server:mojang-mapped" - Mojang-mapped version for modding
    /// - Other variant-specific downloads
    pub downloads: HashMap<Str, Download>,
}
