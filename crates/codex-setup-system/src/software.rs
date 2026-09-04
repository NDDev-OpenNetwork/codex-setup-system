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
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.153.2-linux-arm64.tgz",
        bytes: 121_675_259,
        sha256: "sha256:df2f8f764bf86aeae51d71cc37686dcbd2c24e4bce9a9d1e8f249141fd99fac3",
        shape: Shape::GzipTar,
        member: "package/vendor/aarch64-unknown-linux-musl/bin/codex",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.153.2-linux-x64.tgz",
        bytes: 129_259_793,
        sha256: "sha256:1fd52113294979c9936a110cab7cb9d5e9d1e28086f112fd6748a05826a9e5a0",
        shape: Shape::GzipTar,
        member: "package/vendor/x86_64-unknown-linux-musl/bin/codex",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.153.2-darwin-arm64.tgz",
        bytes: 115_669_249,
        sha256: "sha256:151ecdd90b96af823a2828146bff056c8a5f42d2ab4da63a198fe8a4ad84990b",
        shape: Shape::GzipTar,
        member: "package/vendor/aarch64-apple-darwin/bin/codex",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.153.2-darwin-x64.tgz",
        bytes: 123_497_670,
        sha256: "sha256:ec139bb87cdd25d8038069cef7ff2bdd2d119c7fcf0ac6e26dcc1020573cd112",
        shape: Shape::GzipTar,
        member: "package/vendor/x86_64-apple-darwin/bin/codex",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.153.2-win32-arm64.tgz",
        bytes: 132_143_077,
        sha256: "sha256:71ba906090143a310354ae5c864d5c677e05842b373cadcf84ef2872f8ddcfa1",
        shape: Shape::GzipTar,
        member: "package/vendor/aarch64-pc-windows-msvc/bin/codex.exe",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.153.2-win32-x64.tgz",
        bytes: 141_510_231,
        sha256: "sha256:e674587ea13f7051aeae697b1425ec028a236f54fa1dd1a15661ef7c063f2874",
        shape: Shape::GzipTar,
        member: "package/vendor/x86_64-pc-windows-msvc/bin/codex.exe",
    },
];

/// The artifacts 0.153.1 was published as, kept so
/// `software_update` has a version to move from and `rollback` a tree to
/// return to. Measured from bytes when it was the current pin.
pub(crate) const PREVIOUS_ARTIFACTS: &[Artifact] = &[
    Artifact {
        platform: "linux/arm64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.153.1-linux-arm64.tgz",
        bytes: 121_697_093,
        sha256: "sha256:3e050251b40b334e74a27c52128b86f8468fa8242d5c8205b04f4b31f85690f7",
        shape: Shape::GzipTar,
        member: "package/vendor/aarch64-unknown-linux-musl/bin/codex",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.153.1-linux-x64.tgz",
        bytes: 129_241_027,
        sha256: "sha256:33b4ae16df4530361a651bc1540ae8de325ccc789622a812c94585bdf205f7f4",
        shape: Shape::GzipTar,
        member: "package/vendor/x86_64-unknown-linux-musl/bin/codex",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.153.1-darwin-arm64.tgz",
        bytes: 115_673_114,
        sha256: "sha256:33babfe9db041ff18100775bf34df673c1baf9a5d7b1d09fdcafefc58501aa87",
        shape: Shape::GzipTar,
        member: "package/vendor/aarch64-apple-darwin/bin/codex",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.153.1-darwin-x64.tgz",
        bytes: 123_566_510,
        sha256: "sha256:bafcac30746296be53a183ff907b9ae59cef9f8e65fa511abcb7661bd247bf80",
        shape: Shape::GzipTar,
        member: "package/vendor/x86_64-apple-darwin/bin/codex",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.153.1-win32-arm64.tgz",
        bytes: 132_174_290,
        sha256: "sha256:5ee2fab96e2ba898ea7e4c0c8f4222f4dfd08d0ea9b4c9b849323ebb1e90ef14",
        shape: Shape::GzipTar,
        member: "package/vendor/aarch64-pc-windows-msvc/bin/codex.exe",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://registry.npmjs.org/@openai/codex/-/codex-0.153.1-win32-x64.tgz",
        bytes: 141_469_181,
        sha256: "sha256:fa9e2ac23e905fdcba1fb4f396c57da65fac04c1a3ee4eebbbb85b5ad6b2dc10",
        shape: Shape::GzipTar,
        member: "package/vendor/x86_64-pc-windows-msvc/bin/codex.exe",
    },
];

/// Codex's program, and where its bytes come from.
pub(crate) const SOFTWARE: Software = Software {
    version: "0.153.2",
    command: "codex",
    delivery: Delivery::Artifacts(ARTIFACTS),
    unsupported: &[],
    previous: Some(Previous {
        version: "0.153.1",
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
