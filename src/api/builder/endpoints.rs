use url::Url;

use crate::api::ids::{BuildId, Project, VersionId};

use super::Builder;
use super::api_fields::{UnsetBuild, UnsetProject, UnsetVersion};

#[derive(Debug, Default)]
pub struct UnsetApi;

#[derive(Debug)]
pub struct ProjectVersions<P> {
    project: P,
}

impl ProjectVersions<UnsetProject> {
    pub fn new() -> Self {
        Self {
            project: Default::default(),
        }
    }

    pub fn set_project(self, project: Project) -> ProjectVersions<Project> {
        ProjectVersions { project }
    }
}

impl Builder for ProjectVersions<Project> {
    fn build(self) -> Result<Url, url::ParseError> {
        let url = format!("{}/v3/projects/{}/versions", crate::DOMAIN, self.project);

        Url::parse(&url)
    }
}

#[derive(Debug)]
pub struct ProjectVersion<P, V> {
    project: P,
    version: V,
}

impl ProjectVersion<UnsetProject, UnsetVersion> {
    pub fn new() -> Self {
        Self {
            project: Default::default(),
            version: Default::default(),
        }
    }
}

impl<V> ProjectVersion<UnsetProject, V> {
    pub fn set_project(self, project: Project) -> ProjectVersion<Project, V> {
        ProjectVersion {
            project,
            version: self.version,
        }
    }
}

impl<P> ProjectVersion<P, UnsetVersion> {
    pub fn set_version(self, version: VersionId) -> ProjectVersion<P, VersionId> {
        ProjectVersion {
            project: self.project,
            version,
        }
    }
}

impl Builder for ProjectVersion<Project, VersionId> {
    fn build(self) -> Result<Url, url::ParseError> {
        let url = format!(
            "{}/v3/projects/{}/versions/{}",
            crate::DOMAIN,
            self.project,
            self.version
        );

        Url::parse(&url)
    }
}

#[derive(Debug)]
pub struct ProjectBuilds<P, V> {
    project: P,
    version: V,
}

impl ProjectBuilds<UnsetProject, UnsetVersion> {
    pub fn new() -> Self {
        Self {
            project: Default::default(),
            version: Default::default(),
        }
    }
}

impl<V> ProjectBuilds<UnsetProject, V> {
    pub fn set_project(self, project: Project) -> ProjectBuilds<Project, V> {
        ProjectBuilds {
            project,
            version: self.version,
        }
    }
}

impl<P> ProjectBuilds<P, UnsetVersion> {
    pub fn set_version(self, version: VersionId) -> ProjectBuilds<P, VersionId> {
        ProjectBuilds {
            project: self.project,
            version,
        }
    }
}

impl Builder for ProjectBuilds<Project, VersionId> {
    fn build(self) -> Result<Url, url::ParseError> {
        let url = format!(
            "{}/v3/projects/{}/versions/{}/builds",
            crate::DOMAIN,
            self.project,
            self.version
        );

        Url::parse(&url)
    }
}

#[derive(Debug)]
pub struct ProjectBuild<P, V, B> {
    project: P,
    version: V,
    build: B,
}

impl ProjectBuild<UnsetProject, UnsetVersion, UnsetBuild> {
    pub fn new() -> Self {
        Self {
            project: Default::default(),
            version: Default::default(),
            build: Default::default(),
        }
    }
}

impl<V, B> ProjectBuild<UnsetProject, V, B> {
    pub fn set_project(self, project: Project) -> ProjectBuild<Project, UnsetVersion, UnsetBuild> {
        ProjectBuild {
            project,
            version: Default::default(),
            build: Default::default(),
        }
    }
}

impl<P, B> ProjectBuild<P, UnsetVersion, B> {
    pub fn set_version(self, version: VersionId) -> ProjectBuild<P, VersionId, B> {
        ProjectBuild {
            project: self.project,
            version,
            build: self.build,
        }
    }
}

impl<P, V> ProjectBuild<P, V, UnsetBuild> {
    pub fn set_build(self, build: BuildId) -> ProjectBuild<P, V, BuildId> {
        ProjectBuild {
            project: self.project,
            version: self.version,
            build,
        }
    }
}

impl Builder for ProjectBuild<Project, VersionId, BuildId> {
    fn build(self) -> Result<Url, url::ParseError> {
        let url = format!(
            "{}/v3/projects/{}/versions/{}/builds/{}",
            crate::DOMAIN,
            self.project,
            self.version,
            self.build
        );

        Url::parse(&url)
    }
}
