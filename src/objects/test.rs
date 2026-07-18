use super::build::*;
use super::error::*;
use super::version::*;
use chrono::TimeZone;
use chrono::Utc;
use serde_json;

#[test]
fn test_build_response_success() {
    let json_data = r#"
        {
            "id": 74,
            "time": "2026-07-06T16:51:09Z",
            "channel": "STABLE",
            "commits": [],
            "downloads": {
                "server:default": {
                    "name": "paper-26.1.2-74.jar",
                    "checksums": {
                        "sha256": "1d70b1dab9cf4a6de615209a536f3a45a2186240253c428213ce2188ab95e5f7"
                    },
                    "size": 52893229,
                    "url": "https://fill-data.papermc.io/v1/objects/1d70b1dab9cf4a6de615209a536f3a45a2186240253c428213ce2188ab95e5f7/paper-26.1.2-74.jar"
                }
            }
        }
    "#;

    let build: BuildResponse = serde_json::from_str(json_data).expect("Failed to deserialize");

    // Verify fields
    assert_eq!(build.id, 74);
    assert_eq!(build.channel, Channel::Stable);

    // Verify timestamp
    let expected_time = Utc.with_ymd_and_hms(2026, 7, 6, 16, 51, 9).unwrap();
    assert_eq!(build.time, expected_time);

    // Verify downloads
    let download = build
        .downloads
        .get("server:default")
        .expect("Download not found");

    assert_eq!(download.name.as_ref(), "paper-26.1.2-74.jar");
    assert_eq!(download.size, 52893229);
    assert_eq!(
        download.digest(),
        &[
            0x1du8, 0x70, 0xb1, 0xda, 0xb9, 0xcf, 0x4a, 0x6d, 0xe6, 0x15, 0x20, 0x9a, 0x53, 0x6f,
            0x3a, 0x45, 0xa2, 0x18, 0x62, 0x40, 0x25, 0x3c, 0x42, 0x82, 0x13, 0xce, 0x21, 0x88,
            0xab, 0x95, 0xe5, 0xf7
        ]
    );
    assert_eq!(
        download.url.as_ref(),
        "https://fill-data.papermc.io/v1/objects/1d70b1dab9cf4a6de615209a536f3a45a2186240253c428213ce2188ab95e5f7/paper-26.1.2-74.jar"
    );
}

#[test]
fn test_build_response_fail() {
    let invalid_json = r#"
        {
            "id": "not_an_integer",  // Invalid: id must be i32
            "time": "2026-07-06T16:51:09Z",
            "channel": "STABLE",
            "commits": [],
            "downloads": {}
        }
    "#;

    let result = serde_json::from_str::<BuildResponse>(invalid_json);
    assert!(result.is_err(), "Expected deserialization to fail");
}

#[test]
fn test_version_response_success() {
    let json_data = r#"
        {
            "version": {
                "id": "26.1.2",
                "support": {
                    "status": "SUPPORTED"
                },
                "java": {
                    "version": {
                        "minimum": 25
                    },
                    "flags": {
                        "recommended": [
                            "-XX:+AlwaysPreTouch",
                            "-XX:+DisableExplicitGC",
                            "-XX:+ParallelRefProcEnabled",
                            "-XX:+PerfDisableSharedMem",
                            "-XX:+UnlockExperimentalVMOptions",
                            "-XX:+UseG1GC",
                            "-XX:G1HeapRegionSize=8M",
                            "-XX:G1HeapWastePercent=5",
                            "-XX:G1MaxNewSizePercent=40",
                            "-XX:G1MixedGCCountTarget=4",
                            "-XX:G1MixedGCLiveThresholdPercent=90",
                            "-XX:G1NewSizePercent=30",
                            "-XX:G1RSetUpdatingPauseTimePercent=5",
                            "-XX:G1ReservePercent=20",
                            "-XX:InitiatingHeapOccupancyPercent=15",
                            "-XX:MaxGCPauseMillis=200",
                            "-XX:MaxTenuringThreshold=1",
                            "-XX:SurvivorRatio=32"
                        ]
                    }
                }
            },
            "builds": [
                74, 73, 72, 71, 70, 69, 68, 67, 66, 65, 64, 63, 62, 61, 60, 59, 58, 57, 56, 55,
                54, 53, 52, 51, 50, 49, 48, 47, 46, 45, 44, 43, 42, 41, 40, 39, 38, 37, 36, 35,
                34, 33, 32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15,
                13, 12, 11, 9, 8, 7, 6, 5, 4, 3, 2
            ]
        }
    "#;

    let version_response: VersionResponse =
        serde_json::from_str(json_data).expect("Failed to deserialize VersionResponse");

    // Verify version fields
    assert_eq!(version_response.version.id.as_ref(), "26.1.2");
    assert_eq!(
        version_response.version.support.status,
        SupportStatus::Supported
    );
    assert!(version_response.version.support.end.is_none());

    // Verify java fields
    assert_eq!(version_response.version.java.minimum(), 25);
    assert_eq!(version_response.version.java.recommended().len(), 18);
    assert_eq!(
        version_response.version.java.recommended()[0].as_ref(),
        "-XX:+AlwaysPreTouch"
    );

    // Verify builds
    assert_eq!(version_response.builds.len(), 71);
    assert_eq!(version_response.builds[0], 74);
    assert_eq!(*version_response.builds.last().unwrap(), 2);
}

#[test]
fn test_version_response_fail() {
    let invalid_json = r#"
        {
            "version": {
                "id": 123,  // Invalid: id must be a string
                "support": {
                    "status": "SUPPORTED"
                },
                "java": {
                    "version": {
                        "minimum": 25
                    },
                    "flags": {
                        "recommended": []
                    }
                }
            },
            "builds": []
        }
    "#;

    let result = serde_json::from_str::<VersionResponse>(invalid_json);
    assert!(result.is_err(), "Expected deserialization to fail");
}

#[test]
fn test_error_response_success() {
    let json_data = r#"
        {
            "ok": false,
            "error": "version_not_found",
            "message": "No version was found with the given identifier."
        }
    "#;

    let error_response: ErrorResponse =
        serde_json::from_str(json_data).expect("Failed to deserialize ErrorResponse");

    // Verify fields
    assert_eq!(error_response.ok(), false);
    assert_eq!(error_response.error(), ErrorType::VersionNotFound);
    assert_eq!(
        error_response.msg(),
        "No version was found with the given identifier."
    );
}

#[test]
fn test_error_response_fail() {
    let invalid_json = r#"
        {
            "ok": "not_a_boolean",  // Invalid: ok must be boolean
            "error": "version_not_found",
            "message": "No version was found with the given identifier."
        }
    "#;

    let result = serde_json::from_str::<ErrorResponse>(invalid_json);
    assert!(result.is_err(), "Expected deserialization to fail");
}

#[test]
fn test_api_sunset_error_success() {
    let json_data = r#"
        {
            "ok": false,
            "error": "sunset",
            "message": "This API version has been sunset and is no longer available. To continue using the service, please upgrade to a supported API version."
        }
    "#;

    let error_response: ErrorResponse =
        serde_json::from_str(json_data).expect("Failed to deserialize ErrorResponse");

    // Verify fields
    assert_eq!(error_response.ok(), false);
    assert_eq!(error_response.error(), ErrorType::Sunset);
    assert_eq!(
        error_response.msg(),
        "This API version has been sunset and is no longer available. To continue using the service, please upgrade to a supported API version."
    );
}
