use serde::Deserialize;

use crate::Str;

/// Enum representing possible error codes from the PaperMC API.
/// 
/// Variants are ordered by:
/// 1. Server-side errors (highest severity)
/// 2. Not found errors (ordered by typical API call sequence)
/// 3. Generic/unknown errors (catch-all)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorType {
    /// Server-side error occurred (e.g., internal server error, service unavailable)
    /// This indicates a problem with the API server itself.
    ServerError,

    /// The request was invalid (malformed parameters, bad syntax, validation failed)
    /// This occurs when client input doesn't meet API requirements.
    InvalidRequest,

    /// The requested API version has been sunset and is no longer available
    /// This occurs when trying to access deprecated API versions (e.g., v2 endpoints)
    /// Example: "sunset" error when calling /v2/projects endpoints.
    Sunset,

    // Not found errors - ordered by typical API call sequence:
    // 1. Project-level errors (called first)
    // 2. Version-level errors (called after project lookup)
    // 3. Build-level errors (called after version lookup)
    // 4. Generic not found (fallback)

    /// The project does not exist (API returns "project_not_found")
    /// Typically occurs when calling /v3/projects/{project} with invalid project name.
    ProjectNotFound,

    /// The requested version was not found (API returns "version_not_found")
    /// Typically occurs when calling /v3/projects/{project}/versions/{version} with invalid version.
    VersionNotFound,

    /// The requested build does not exist (API returns "build_not_found")
    /// Typically occurs when calling /v3/projects/{project}/versions/{version}/builds/{build} with invalid build number.
    BuildNotFound,

    /// Generic not found error for resources that don't exist
    /// Used when the specific resource type isn't known or covered by more specific variants.
    NotFound,

    /// Generic error for cases not covered by other variants
    /// This captures any unexpected error codes returned by the API.
    /// Maintains forward compatibility with future API changes.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ErrorResponse {
    /// Machine-readable error code
    error: ErrorType,
    /// Human-readable error message
    message: Str,
    /// Indicates if the request was successful. Always `false` for errors.
    ok: bool,
}

impl ErrorResponse {
    /// Returns the machine-readable error code from the `error` field.
    /// 
    /// This corresponds to the `error` field in the JSON response and indicates
    /// the specific type of error that occurred (e.g., "version_not_found", "sunset").
    pub fn error(&self) -> ErrorType {
        self.error
    }

    /// Returns the human-readable error message from the `message` field.
    /// 
    /// This corresponds to the `message` field in the JSON response and provides
    /// a detailed description of what went wrong.
    pub fn msg(&self) -> &str {
        self.message.as_ref()
    }

    /// Returns the success status from the `ok` field.
    /// 
    /// This corresponds to the `ok` field in the JSON response. For error responses,
    /// this will always be `false`. Success responses use different struct types.
    pub fn ok(&self) -> bool {
        self.ok
    }
}
