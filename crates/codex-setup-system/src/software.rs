//! Codex's own program, as measured rather than as described.
//!
//! Generated from the `software_artifacts` block of
//! `references/codex-baseline.json`. Every member path below was read out
//! of the archive it names, not assumed: codex's carries the target triple and
//! so genuinely differs per platform.
//!
//! Where a `previous_software_artifacts` block is present, it is transcribed
//! too. It is not a second choice: the outgoing current pin is stored there on
//! a bump, so the pair is always two consecutive real releases and there is
//! still exactly one value to keep fresh.
//!
//! Do not edit. The test at the bottom re-reads that baseline and compares it
//! field by field, so an edit here fails rather than silently installing bytes
//! nobody measured.

use harness_runtime::{Artifact, Delivery, Previous, Shape, Software};

/// The artifacts codex is published as.
pub(crate) const ARTIFACTS: &[Artifact] = &[
    Artifact {
        platform: "linux/arm64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.157.0-linux-arm64.tgz",
        bytes: 142_965_067,
        sha256: "sha256:e94d36849541bb0e07aacbbf3edc99af7eba22ff813e6878eabd3e4c328309eb",
        shape: Shape::GzipTar,
        member: "package/vendor/aarch64-unknown-linux-musl/bin/codex",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.157.0-linux-x64.tgz",
        bytes: 150_316_314,
        sha256: "sha256:8cc62b1a3b0d7fd2c3227c2b82a8c442bedff52a7b30b69850686e6f5129abd3",
        shape: Shape::GzipTar,
        member: "package/vendor/x86_64-unknown-linux-musl/bin/codex",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.157.0-darwin-arm64.tgz",
        bytes: 133_011_685,
        sha256: "sha256:8ed1854086533492068d2c103e4b13c627b0def7716d038874f05474cb4fc718",
        shape: Shape::GzipTar,
        member: "package/vendor/aarch64-apple-darwin/bin/codex",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.157.0-darwin-x64.tgz",
        bytes: 141_684_247,
        sha256: "sha256:e147dcf9670f9ffa8134247c6db14a7ad3452899c23311dcd46e5d208c013307",
        shape: Shape::GzipTar,
        member: "package/vendor/x86_64-apple-darwin/bin/codex",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.157.0-win32-arm64.tgz",
        bytes: 150_580_314,
        sha256: "sha256:bba499caac6a226dd1d05a934901734065708ae8e253361b1100831384db101d",
        shape: Shape::GzipTar,
        member: "package/vendor/aarch64-pc-windows-msvc/bin/codex.exe",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.157.0-win32-x64.tgz",
        bytes: 160_985_645,
        sha256: "sha256:fe71b497453d7a5372842f50a8b737668dc0755624713ba8a9093f09419b79b8",
        shape: Shape::GzipTar,
        member: "package/vendor/x86_64-pc-windows-msvc/bin/codex.exe",
    },
];

/// The artifacts 0.155.1 was published as, kept so
/// `software_update` has a version to move from and `rollback` a tree to
/// return to. Measured from bytes when it was the current pin.
pub(crate) const PREVIOUS_ARTIFACTS: &[Artifact] = &[
    Artifact {
        platform: "linux/arm64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.155.1-linux-arm64.tgz",
        bytes: 135_126_766,
        sha256: "sha256:7da3e7bea6db4751d1b837f046c9cec08918ac7f05427dd0b5bcf64ea6c85964",
        shape: Shape::GzipTar,
        member: "package/vendor/aarch64-unknown-linux-musl/bin/codex",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.155.1-linux-x64.tgz",
        bytes: 142_140_011,
        sha256: "sha256:f110cccdd50b0be8130b84f45b3144ea775c233f1c8bd8226da6ee719d63d206",
        shape: Shape::GzipTar,
        member: "package/vendor/x86_64-unknown-linux-musl/bin/codex",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.155.1-darwin-arm64.tgz",
        bytes: 127_465_533,
        sha256: "sha256:93cc218b25b71c8da3edb50a013fbd22acf8f39058fb64083fefff638a084976",
        shape: Shape::GzipTar,
        member: "package/vendor/aarch64-apple-darwin/bin/codex",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.155.1-darwin-x64.tgz",
        bytes: 135_811_357,
        sha256: "sha256:819db6dbb57a56cc21383a30331fc4115696e02f4c47ba7f686a285a2173fadd",
        shape: Shape::GzipTar,
        member: "package/vendor/x86_64-apple-darwin/bin/codex",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.155.1-win32-arm64.tgz",
        bytes: 135_509_012,
        sha256: "sha256:72525256ac769a23381236a374c71679927c988dd10b7a19d342c98193216b7f",
        shape: Shape::GzipTar,
        member: "package/vendor/aarch64-pc-windows-msvc/bin/codex.exe",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.155.1-win32-x64.tgz",
        bytes: 145_165_338,
        sha256: "sha256:727fd5bfaeed16fe8fdeb57d1c0faa688f48edda4a0541323a103a3138925350",
        shape: Shape::GzipTar,
        member: "package/vendor/x86_64-pc-windows-msvc/bin/codex.exe",
    },
];

/// Codex's program, and where its bytes come from.
pub(crate) const SOFTWARE: Software = Software {
    version: "0.157.0",
    command: "codex",
    delivery: Delivery::Artifacts(ARTIFACTS),
    unsupported: &[],
    previous: Some(Previous {
        version: "0.155.1",
        artifacts: PREVIOUS_ARTIFACTS,
    }),
};

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic)]

    // Named rather than glob-imported: a product delivered by a package manager
    // has no `Artifact` in scope, and the test is the same text for all seven.
    use harness_runtime::{Delivery, Shape};

    use super::SOFTWARE;

    fn measured() -> serde_json::Value {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../references/codex-baseline.json");
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
    }

    #[test]
    fn every_artifact_compiled_in_is_the_one_the_baseline_measured() {
        let block = &measured()["software_artifacts"];
        assert_eq!(block["version"], SOFTWARE.version);
        assert_eq!(block["command"], SOFTWARE.command);

        let Delivery::Artifacts(compiled) = SOFTWARE.delivery else {
            // A product delivered by a package manager has no artifacts, and
            // the baseline must agree that it has none.
            assert_eq!(block["shape"], "manager");
            assert!(block["platforms"].as_object().unwrap().is_empty());
            return;
        };
        let published = block["platforms"].as_object().unwrap();
        assert_eq!(
            compiled.len(),
            published.len(),
            "the table and the baseline disagree on how many platforms exist"
        );
        for artifact in compiled {
            let entry = &published[artifact.platform];
            assert_eq!(entry["url"], artifact.url, "{}", artifact.platform);
            assert_eq!(entry["bytes"], artifact.bytes, "{}", artifact.platform);
            assert_eq!(entry["sha256"], artifact.sha256, "{}", artifact.platform);
            let member = entry.get("member").and_then(serde_json::Value::as_str);
            assert_eq!(
                member.unwrap_or(""),
                artifact.member,
                "{} names a different member",
                artifact.platform
            );
            assert_eq!(
                artifact.shape == Shape::Raw,
                member.is_none(),
                "{} disagrees about whether the bytes are the program",
                artifact.platform
            );
        }
    }

    /// The second pin is the baseline's, or it is absent in both places.
    ///
    /// Asserted from either side rather than only where it exists: a harness
    /// that has never been bumped must compile in `None`, and a build that
    /// dropped the block while the baseline still carried it would otherwise
    /// pass by having nothing to compare.
    #[test]
    fn the_version_this_build_can_move_between_is_the_one_measured_before_it() {
        let baseline = measured();
        let recorded = baseline.get("previous_software_artifacts");
        let Some(earlier) = SOFTWARE.previous else {
            assert!(
                recorded.is_none(),
                "the baseline records a previous release and this build names none"
            );
            return;
        };
        let block = recorded.unwrap_or_else(|| {
            panic!("this build names a previous release the baseline does not record")
        });
        assert_eq!(block["version"], earlier.version);
        assert_ne!(
            earlier.version, SOFTWARE.version,
            "a second pin equal to the first is one version wearing two names"
        );
        let published = block["platforms"].as_object().unwrap();
        assert_eq!(
            earlier.artifacts.len(),
            published.len(),
            "the previous table and the baseline disagree on how many platforms exist"
        );
        for artifact in earlier.artifacts {
            let entry = &published[artifact.platform];
            assert_eq!(entry["url"], artifact.url, "{}", artifact.platform);
            assert_eq!(entry["bytes"], artifact.bytes, "{}", artifact.platform);
            assert_eq!(entry["sha256"], artifact.sha256, "{}", artifact.platform);
        }
    }

    #[test]
    fn a_platform_the_vendor_does_not_publish_is_listed_rather_than_missing() {
        let block = &measured()["software_artifacts"];
        let unpublished: Vec<&str> = block
            .get("unpublished")
            .and_then(serde_json::Value::as_array)
            .map(|entries| {
                entries
                    .iter()
                    .filter_map(serde_json::Value::as_str)
                    .collect()
            })
            .unwrap_or_default();
        assert_eq!(unpublished, SOFTWARE.unsupported);
    }

    #[test]
    fn no_release_calls_a_platform_both_published_and_unpublished() {
        let baseline = measured();
        for name in ["software_artifacts", "previous_software_artifacts"] {
            let Some(block) = baseline.get(name) else {
                continue;
            };
            let published = block["platforms"].as_object().unwrap();
            let unpublished = block
                .get("unpublished")
                .and_then(serde_json::Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(serde_json::Value::as_str);
            for platform in unpublished {
                assert!(
                    !published.contains_key(platform),
                    "{name}: {platform} is both published and unpublished"
                );
            }
        }
    }
}
