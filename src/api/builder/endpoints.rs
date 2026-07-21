use std::fmt::Display;

use crate::Str;
use crate::api::ids::{BuildId, Project, VersionId};

use super::Builder;

/// Marker type indicating that the API version has not been set yet.
#[derive(Debug)]
pub struct UnsetV;

/// Represents PaperMC API version 3.
#[derive(Debug)]
pub struct V3;

impl Display for V3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("v3")
    }
}

#[derive(Debug, Default)]
pub struct UnsetProject;

#[derive(Debug, Default)]
pub struct UnsetVersion;

#[derive(Debug, Default)]
pub struct UnsetBuild;

#[derive(Debug, Default)]
pub struct UnsetApi;

/// Builder for PaperMC API v3 projects versions endpoint.
///
/// Constructs URLs for: `/v3/projects/{project}/versions`
///
/// # Type Parameters
/// - `V`: API version state (UnsetV or V3)
/// - `P`: Project state (UnsetProject or Project)
///
/// # Example
///
/// ```rust
/// use paper_api::api::builder::{endpoints::ProjectVersions, Builder};
/// use paper_api::api::ids::Project;
///
/// let url = ProjectVersions::new()
///     .v3()                    // Must set API version first
///     .set_project(Project::Paper)
///     .build();
///
/// assert_eq!(url, "v3/projects/paper/versions".into());
/// ```
#[derive(Debug)]
pub struct ProjectVersions<V, P> {
    version: V,
    project: P,
}

impl ProjectVersions<UnsetV, UnsetProject> {
    /// Creates a new `ProjectVersions` builder with unset version and project.
    pub fn new() -> Self {
        Self {
            version: UnsetV,
            project: UnsetProject,
        }
    }

    /// Sets the API version to v3.
    ///
    /// This must be called first before setting the project.
    pub fn v3(self) -> ProjectVersions<V3, UnsetProject> {
        ProjectVersions {
            version: V3,
            project: UnsetProject,
        }
    }
}

impl ProjectVersions<V3, UnsetProject> {
    /// Sets the project for this endpoint.
    ///
    /// # Arguments
    /// * `project` - The PaperMC project (e.g., Project::Paper)
    pub fn set_project(self, project: Project) -> ProjectVersions<V3, Project> {
        ProjectVersions {
            version: self.version,
            project,
        }
    }
}

impl Builder for ProjectVersions<V3, Project> {
    fn build(self) -> Str {
        format!("{}/projects/{}/versions", self.version, self.project).into()
    }
}

/// Builder for PaperMC API v3 project version endpoint.
///
/// Constructs URLs for: `/v3/projects/{project}/versions/{version}`
///
/// # Type Parameters
/// - `V`: API version state (UnsetV or V3)
/// - `P`: Project state (UnsetProject or Project)
/// - `MC`: Minecraft version state (UnsetVersion or VersionId)
///
/// # Example
///
/// ```rust
/// use paper_api::api::builder::{endpoints::ProjectVersion, Builder};
/// use paper_api::api::ids::{Project, VersionId};
///
/// let version: VersionId = "1.20.1".try_into().unwrap();
/// let url = ProjectVersion::new()
///     .v3()                    // Must set API version first
///     .set_project(Project::Paper)
///     .set_version(version)
///     .build();
///
/// assert_eq!(url, "v3/projects/paper/versions/1.20.1".into());
/// ```
#[derive(Debug)]
pub struct ProjectVersion<V, P, MC> {
    version: V,
    project: P,
    mc_version: MC,
}

impl ProjectVersion<UnsetV, UnsetProject, UnsetVersion> {
    pub fn new() -> Self {
        Self {
            version: UnsetV,
            project: UnsetProject,
            mc_version: UnsetVersion,
        }
    }

    /// Sets the API version to v3.
    ///
    /// This must be called first before setting the project.
    pub fn v3(self) -> ProjectVersion<V3, UnsetProject, UnsetVersion> {
        ProjectVersion {
            version: V3,
            project: UnsetProject,
            mc_version: UnsetVersion,
        }
    }
}

impl<MC> ProjectVersion<V3, UnsetProject, MC> {
    /// Sets the project for this endpoint.
    ///
    /// # Arguments
    /// * `project` - The PaperMC project (e.g., Project::Paper)
    pub fn set_project(self, project: Project) -> ProjectVersion<V3, Project, MC> {
        ProjectVersion {
            version: self.version,
            project,
            mc_version: self.mc_version,
        }
    }
}

impl<V, P> ProjectVersion<V, P, UnsetVersion> {
    /// Sets the version for this endpoint.
    ///
    /// # Arguments
    /// * `version` - Minecraft version
    pub fn set_version(self, version: VersionId) -> ProjectVersion<V, P, VersionId> {
        ProjectVersion {
            version: self.version,
            project: self.project,
            mc_version: version,
        }
    }
}

impl Builder for ProjectVersion<V3, Project, VersionId> {
    fn build(self) -> Str {
        format!(
            "{}/projects/{}/versions/{}",
            self.version, self.project, self.mc_version
        )
        .into()
    }
}

/// Builder for PaperMC API v3 project builds endpoint.
///
/// Constructs URLs for: `/v3/projects/{project}/versions/{version}/builds`
///
/// This endpoint returns a list of all builds available for a specific
/// Minecraft version of a project.
///
/// # Type Parameters
///
/// - `V`: API version state (UnsetV or V3)
/// - `P`: Project state (UnsetProject or Project)
/// - `MC`: Minecraft version state (UnsetVersion or VersionId)
///
/// # Example
///
/// ```rust
/// use paper_api::api::builder::{endpoints::ProjectBuilds, Builder};
/// use paper_api::api::ids::{Project, VersionId};
///
/// let version: VersionId = "1.20.1".try_into().unwrap();
/// let url = ProjectBuilds::new()
///     .v3()                    // Must set API version first
///     .set_project(Project::Paper)
///     .set_version(version)
///     .build();
///
/// assert_eq!(url, "v3/projects/paper/versions/1.20.1/builds".into());
/// ```
#[derive(Debug)]
pub struct ProjectBuilds<V, P, MC> {
    version: V,
    project: P,
    mc_version: MC,
}

impl ProjectBuilds<UnsetV, UnsetProject, UnsetVersion> {
    pub fn new() -> Self {
        Self {
            version: UnsetV,
            project: UnsetProject,
            mc_version: UnsetVersion,
        }
    }

    /// Sets the API version to v3.
    ///
    /// This must be called first before setting the project.
    pub fn v3(self) -> ProjectBuilds<V3, UnsetProject, UnsetVersion> {
        ProjectBuilds {
            version: V3,
            project: self.project,
            mc_version: self.mc_version,
        }
    }
}

impl<MC> ProjectBuilds<V3, UnsetProject, MC> {
    /// Sets the project for this endpoint.
    ///
    /// # Arguments
    /// * `project` - The PaperMC project (e.g., Project::Paper)
    pub fn set_project(self, project: Project) -> ProjectBuilds<V3, Project, MC> {
        ProjectBuilds {
            version: self.version,
            project,
            mc_version: self.mc_version,
        }
    }
}

impl<V, P> ProjectBuilds<V, P, UnsetVersion> {
    /// Sets the version for this endpoint.
    ///
    /// # Arguments
    /// * `version` - Minecraft version
    pub fn set_version(self, version: VersionId) -> ProjectBuilds<V, P, VersionId> {
        ProjectBuilds {
            version: self.version,
            project: self.project,
            mc_version: version,
        }
    }
}

impl Builder for ProjectBuilds<V3, Project, VersionId> {
    fn build(self) -> Str {
        format!(
            "{}/projects/{}/versions/{}/builds",
            self.version, self.project, self.mc_version
        )
        .into()
    }
}

/// Builder for PaperMC API v3 specific build endpoint.
///
/// Constructs URLs for: `/v3/projects/{project}/versions/{version}/builds/{build}`
///
/// This endpoint returns detailed information about a specific build,
/// including download links, checksums, and build metadata.
///
/// # Type Parameters
///
/// - `V`: API version state (UnsetV or V3)
/// - `P`: Project state (UnsetProject or Project)
/// - `MC`: Minecraft version state (UnsetVersion or VersionId)
/// - `B`: Build state (UnsetBuild or BuildId)
///
/// # Example - Latest Build
///
/// ```rust
/// use paper_api::api::builder::{endpoints::ProjectBuild, Builder};
/// use paper_api::api::ids::{Project, VersionId, BuildId};
/// use std::convert::TryInto;
///
/// let version: VersionId = "1.20.1".try_into().unwrap();
/// let url = ProjectBuild::new()
///     .v3()                    // Must set API version first
///     .set_project(Project::Paper)
///     .set_version(version)
///     .set_build(BuildId::Latest)
///     .build();
///
/// assert_eq!(url, "v3/projects/paper/versions/1.20.1/builds/latest".into());
/// ```
///
/// # Example - Specific Build Number
///
/// ```rust
/// use paper_api::api::builder::{endpoints::ProjectBuild, Builder};
/// use paper_api::api::ids::{Project, VersionId, BuildId};
/// use std::convert::TryInto;
///
/// let version: VersionId = "1.20.1".try_into().unwrap();
/// let url = ProjectBuild::new()
///     .v3()
///     .set_project(Project::Paper)
///     .set_version(version)
///     .set_build(BuildId::Version(123))
///     .build();
///
/// assert_eq!(url, "v3/projects/paper/versions/1.20.1/builds/123".into());
/// ```
#[derive(Debug)]
pub struct ProjectBuild<V, P, MC, B> {
    version: V,
    project: P,
    mc_version: MC,
    build: B,
}

impl ProjectBuild<UnsetV, UnsetProject, UnsetVersion, UnsetBuild> {
    pub fn new() -> Self {
        Self {
            version: UnsetV,
            project: UnsetProject,
            mc_version: UnsetVersion,
            build: UnsetBuild,
        }
    }

    /// Sets the API version to v3.
    ///
    /// This must be called first before setting the project.
    pub fn v3(self) -> ProjectBuild<V3, UnsetProject, UnsetVersion, UnsetBuild> {
        ProjectBuild {
            version: V3,
            project: UnsetProject,
            mc_version: UnsetVersion,
            build: UnsetBuild,
        }
    }
}

impl<MC, B> ProjectBuild<V3, UnsetProject, MC, B> {
    /// Sets the project for this endpoint.
    ///
    /// # Arguments
    /// * `project` - The PaperMC project (e.g., Project::Paper)
    pub fn set_project(
        self,
        project: Project,
    ) -> ProjectBuild<V3, Project, MC, B> {
        ProjectBuild {
            version: self.version,
            project,
            mc_version: self.mc_version,
            build: self.build,
        }
    }
}

impl<V, P, B> ProjectBuild<V, P, UnsetVersion, B> {
    /// Sets the version for this endpoint.
    ///
    /// # Arguments
    /// * `version` - Minecraft version
    pub fn set_version(self, version: VersionId) -> ProjectBuild<V, P, VersionId, B> {
        ProjectBuild {
            version: self.version,
            project: self.project,
            mc_version: version,
            build: self.build,
        }
    }
}

impl<V, P, MC> ProjectBuild<V, P, MC, UnsetBuild> {
    /// Sets build_id for this endpoint.
    ///
    /// # Arguments
    /// * `build` - Minecraft Paper build id (e.g., BuildId::Latest)
    pub fn set_build(self, build: BuildId) -> ProjectBuild<V, P, MC, BuildId> {
        ProjectBuild {
            version: self.version,
            project: self.project,
            mc_version: self.mc_version,
            build,
        }
    }
}

impl Builder for ProjectBuild<V3, Project, VersionId, BuildId> {
    fn build(self) -> Str {
        format!(
            "{}/projects/{}/versions/{}/builds/{}",
            self.version, self.project, self.mc_version, self.build
        )
        .into()
    }
}
