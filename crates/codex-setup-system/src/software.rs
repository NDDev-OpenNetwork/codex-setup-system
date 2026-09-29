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
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.159.1-linux-arm64.tgz",
        bytes: 155_055_348,
        sha256: "sha256:c06c0bed5af67fc8d2815a9f756113855f97a05f7c755b45efea3b12bb060b1a",
        shape: Shape::GzipTar,
        member: "package/vendor/aarch64-unknown-linux-musl/bin/codex",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.159.1-linux-x64.tgz",
        bytes: 162_497_372,
        sha256: "sha256:9d6476abf421afe566c7afade3bb3c4aa1586c6f673c6a9e346b625e6b19561f",
        shape: Shape::GzipTar,
        member: "package/vendor/x86_64-unknown-linux-musl/bin/codex",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.159.1-darwin-arm64.tgz",
        bytes: 133_916_568,
        sha256: "sha256:1dd7f5612d51b8c5347ad0f950f5ccc9cd5fdaf1de57c436616c01d21c4af259",
        shape: Shape::GzipTar,
        member: "package/vendor/aarch64-apple-darwin/bin/codex",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.159.1-darwin-x64.tgz",
        bytes: 142_838_417,
        sha256: "sha256:62375093db474df7067f2f3fbbea772fb00a177caa11683ea6694af718255979",
        shape: Shape::GzipTar,
        member: "package/vendor/x86_64-apple-darwin/bin/codex",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.159.1-win32-arm64.tgz",
        bytes: 151_255_481,
        sha256: "sha256:a0b26da5cb8307bbffae90877a931be542221a460a20122818a34b2bc3e41459",
        shape: Shape::GzipTar,
        member: "package/vendor/aarch64-pc-windows-msvc/bin/codex.exe",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.159.1-win32-x64.tgz",
        bytes: 161_808_975,
        sha256: "sha256:06316f97864f0a0e8a15e564018c90ec158681a51c7ef2e462181d9f2cee38ef",
        shape: Shape::GzipTar,
        member: "package/vendor/x86_64-pc-windows-msvc/bin/codex.exe",
    },
];

/// The artifacts 0.158.0 was published as, kept so
/// `software_update` has a version to move from and `rollback` a tree to
/// return to. Measured from bytes when it was the current pin.
pub(crate) const PREVIOUS_ARTIFACTS: &[Artifact] = &[
    Artifact {
        platform: "linux/arm64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.158.0-linux-arm64.tgz",
        bytes: 154_764_938,
        sha256: "sha256:f4ce42757e10140e0b2f0c916e72941c707d3e5d7d90922465df99268cf53f22",
        shape: Shape::GzipTar,
        member: "package/vendor/aarch64-unknown-linux-musl/bin/codex",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.158.0-linux-x64.tgz",
        bytes: 162_057_445,
        sha256: "sha256:3fe84106aaf2fbfc13299068510d34b3d0157eeb9af4b37be8cf5416f485a6bb",
        shape: Shape::GzipTar,
        member: "package/vendor/x86_64-unknown-linux-musl/bin/codex",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.158.0-darwin-arm64.tgz",
        bytes: 133_524_491,
        sha256: "sha256:7849f8aa87c3956823cef082d83b538980a0ced6e468cca4f39c1cf606ea8dae",
        shape: Shape::GzipTar,
        member: "package/vendor/aarch64-apple-darwin/bin/codex",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.158.0-darwin-x64.tgz",
        bytes: 142_394_998,
        sha256: "sha256:72967f856612168cafcc4259ec0c890ea98483982441b51ec09c1811d386b390",
        shape: Shape::GzipTar,
        member: "package/vendor/x86_64-apple-darwin/bin/codex",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.158.0-win32-arm64.tgz",
        bytes: 151_185_263,
        sha256: "sha256:ae7a6d75eac88486db8c6dd36582578c99843930f61710f94088a12366df4789",
        shape: Shape::GzipTar,
        member: "package/vendor/aarch64-pc-windows-msvc/bin/codex.exe",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.158.0-win32-x64.tgz",
        bytes: 161_635_245,
        sha256: "sha256:fffd7f7ce365029b3f1d3607b53e7a2b7334f43affd776a718d782d5df2b890e",
        shape: Shape::GzipTar,
        member: "package/vendor/x86_64-pc-windows-msvc/bin/codex.exe",
    },
];

/// Codex's program, and where its bytes come from.
pub(crate) const SOFTWARE: Software = Software {
    version: "0.159.1",
    command: "codex",
    delivery: Delivery::Artifacts(ARTIFACTS),
    unsupported: &[],
    previous: Some(Previous {
        version: "0.158.0",
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
