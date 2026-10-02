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
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.160.0-linux-arm64.tgz",
        bytes: 155_716_078,
        sha256: "sha256:9286a7e01d500ab224c9e5b4b223b7adf5dd901fba23efd6a438316426798b17",
        shape: Shape::GzipTar,
        member: "package/vendor/aarch64-unknown-linux-musl/bin/codex",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.160.0-linux-x64.tgz",
        bytes: 163_273_896,
        sha256: "sha256:37a41d61c3399182b8c727b77090cc7a1566bd849d0f09070a0bbc6fec4c58dc",
        shape: Shape::GzipTar,
        member: "package/vendor/x86_64-unknown-linux-musl/bin/codex",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.160.0-darwin-arm64.tgz",
        bytes: 134_311_083,
        sha256: "sha256:fc789bcd655d903f92e1a23c8dc5315ba38f43b3586eafb7bd3b195970b57466",
        shape: Shape::GzipTar,
        member: "package/vendor/aarch64-apple-darwin/bin/codex",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.160.0-darwin-x64.tgz",
        bytes: 143_411_935,
        sha256: "sha256:d90ef1be135b605c88af1b7595bac768a02c7ea410688240961899655ccb5d2e",
        shape: Shape::GzipTar,
        member: "package/vendor/x86_64-apple-darwin/bin/codex",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.160.0-win32-arm64.tgz",
        bytes: 151_948_410,
        sha256: "sha256:918d4677c7ccdbbac524de51d3c2f5a46d6bff8d2a596bda668bc231c48c927e",
        shape: Shape::GzipTar,
        member: "package/vendor/aarch64-pc-windows-msvc/bin/codex.exe",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.160.0-win32-x64.tgz",
        bytes: 162_643_563,
        sha256: "sha256:1f4c46470317bf5fabb308ed3e39a80ff544069cedc2cb477005545e694058b9",
        shape: Shape::GzipTar,
        member: "package/vendor/x86_64-pc-windows-msvc/bin/codex.exe",
    },
];

/// The artifacts 0.159.2 was published as, kept so
/// `software_update` has a version to move from and `rollback` a tree to
/// return to. Measured from bytes when it was the current pin.
pub(crate) const PREVIOUS_ARTIFACTS: &[Artifact] = &[
    Artifact {
        platform: "linux/arm64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.159.2-linux-arm64.tgz",
        bytes: 155_020_250,
        sha256: "sha256:1c4756c2f67f1310ffb03234d61d540ad85916cef5383e46e0e7851f1721c142",
        shape: Shape::GzipTar,
        member: "package/vendor/aarch64-unknown-linux-musl/bin/codex",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.159.2-linux-x64.tgz",
        bytes: 162_475_310,
        sha256: "sha256:84a6b35fb45bdcb94cef9fcb329438045a8911f53876cc9a9a7e6f9bb1382eba",
        shape: Shape::GzipTar,
        member: "package/vendor/x86_64-unknown-linux-musl/bin/codex",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.159.2-darwin-arm64.tgz",
        bytes: 133_844_825,
        sha256: "sha256:4e491428fef0a676f8ed1c6b392183c24793940c958fa1e7bd99dbcb982d7cc6",
        shape: Shape::GzipTar,
        member: "package/vendor/aarch64-apple-darwin/bin/codex",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.159.2-darwin-x64.tgz",
        bytes: 142_911_155,
        sha256: "sha256:4d69b1ad1425ff9267d718ffcc0289cd04ccf66bc187ade2267ea89e3ba4c4c5",
        shape: Shape::GzipTar,
        member: "package/vendor/x86_64-apple-darwin/bin/codex",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.159.2-win32-arm64.tgz",
        bytes: 151_227_532,
        sha256: "sha256:fa699ac9829b98ac8ffae168ff4f2a87c930a773888b444e42ad3f3b9bbda33d",
        shape: Shape::GzipTar,
        member: "package/vendor/aarch64-pc-windows-msvc/bin/codex.exe",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.159.2-win32-x64.tgz",
        bytes: 161_768_064,
        sha256: "sha256:a71d5560d56189969350cf42c0121b57d69ae88405fcdab89fc3fa3d704f6bb6",
        shape: Shape::GzipTar,
        member: "package/vendor/x86_64-pc-windows-msvc/bin/codex.exe",
    },
];

/// Codex's program, and where its bytes come from.
pub(crate) const SOFTWARE: Software = Software {
    version: "0.160.0",
    command: "codex",
    delivery: Delivery::Artifacts(ARTIFACTS),
    unsupported: &[],
    previous: Some(Previous {
        version: "0.159.2",
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
