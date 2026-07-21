use super::Endpoint;
use super::builder::Builder;
use super::builder::endpoints::*;
use super::ids::{BuildId, Project, VersionId};

#[test]
fn test_example_build_endpoint() {
    let endpoint = Endpoint::builder()
        .set_domain(Default::default())
        .set_endpoint(ProjectVersions::new().v3().set_project(Project::Paper))
        .build()
        .unwrap();

    println!("Endpoint URL: {}", endpoint);
}

#[test]
fn test_version_parse_stable_ok() {
    let ver1: VersionId = "26.2".try_into().unwrap();
    assert_eq!(
        ver1,
        VersionId {
            major: 26,
            minor: 2,
            patch: 0,
            stable: true,
        }
    );
    let ver2: VersionId = "26.1.2".try_into().unwrap();
    assert_eq!(
        ver2,
        VersionId {
            major: 26,
            minor: 1,
            patch: 2,
            stable: true
        }
    );
    let ver3: VersionId = "26.2".try_into().unwrap();
    assert_eq!(
        ver3,
        VersionId {
            major: 26,
            minor: 2,
            patch: 0,
            stable: true
        }
    );
}

#[test]
fn test_version_parse_rc_ok() {
    let ver1: VersionId = "26.2-rc-2".try_into().unwrap();
    assert_eq!(
        ver1,
        VersionId {
            major: 26,
            minor: 2,
            patch: 0,
            stable: false,
        }
    );
    let ver2: VersionId = "1.21.11-rc3".try_into().unwrap();
    assert_eq!(
        ver2,
        VersionId {
            major: 1,
            minor: 21,
            patch: 11,
            stable: false
        }
    );
    let ver3: VersionId = "1.13-pre7".try_into().unwrap();
    assert_eq!(
        ver3,
        VersionId {
            major: 1,
            minor: 13,
            patch: 0,
            stable: false
        }
    );
}

// Builder endpoint tests
#[test]
fn test_project_versions_url() {
    let url = ProjectVersions::new()
        .v3()
        .set_project(Project::Paper)
        .build();

    assert_eq!(url.as_ref(), "v3/projects/paper/versions");
}

#[test]
fn test_project_version_url() {
    let version: VersionId = "1.20.1".try_into().unwrap();
    let url = ProjectVersion::new()
        .v3()
        .set_project(Project::Paper)
        .set_version(version)
        .build();

    assert_eq!(url.as_ref(), "v3/projects/paper/versions/1.20.1");
}

#[test]
fn test_project_builds_url() {
    let version: VersionId = "1.20.1".try_into().unwrap();
    let url = ProjectBuilds::new()
        .v3()
        .set_project(Project::Paper)
        .set_version(version)
        .build();

    assert_eq!(url.as_ref(), "v3/projects/paper/versions/1.20.1/builds");
}

#[test]
fn test_project_build_url_with_version() {
    let version: VersionId = "1.20.1".try_into().unwrap();
    let url = ProjectBuild::new()
        .v3()
        .set_project(Project::Paper)
        .set_version(version)
        .set_build(BuildId::Latest)
        .build();

    assert_eq!(
        url.as_ref(),
        "v3/projects/paper/versions/1.20.1/builds/latest"
    );
}

#[test]
fn test_project_build_url_with_build_number() {
    let version: VersionId = "1.20.1".try_into().unwrap();
    let url = ProjectBuild::new()
        .v3()
        .set_project(Project::Paper)
        .set_version(version)
        .set_build(BuildId::Version(123))
        .build();

    assert_eq!(url.as_ref(), "v3/projects/paper/versions/1.20.1/builds/123");
}
