//! The Codex CLI setup system.
//!
//! This file is the harness's *facts*. Every command over them lives in
//! [`harness_runtime`], shared with every other setup system, so a change to
//! behaviour lands once and a change to Codex CLI's surface lands here.
//!
//! Codex installs itself, so this provider owns the configuration only.

use std::process::ExitCode;

mod software;

use harness_runtime::{Harness, Scoped};
use provider_v3::{ComponentKind, ProjectionKind, TargetScope};

/// Everything specific to Codex CLI, verified against `codex-baseline.json`.
pub const CODEX: Harness = Harness {
    harness_id: "codex",
    provider_id: "codex-setup-system",
    version: env!("CARGO_PKG_VERSION"),
    product: "Codex CLI",
    vendor: "OpenAI",
    documented_config_home: "~/.codex",
    config_home_env: "CODEX_HOME",
    control_directory: ".codex-setup-system",
    state_file: "NDDEV-CODEX-PROVIDER.json",
    predecessor_state_file: "NDDEV-CODEX-SETUP.json",
    profile_id: "codex/native-and-plugins/1",
    // Everything outside this list is a sibling overlay preserved verbatim.
    // `skills` and `.agents/skills` were here and are gone. Codex searches
    // `$HOME/.agents/skills` -- a *sibling* of `~/.codex`, not a child --
    // so declared against this target the second resolved to
    // `~/.codex/.agents/skills`, which the product never reads. The same
    // shape as the pi `managed_paths` defect, one level up.
    native_namespaces: &["AGENTS.md", "config.toml", "hooks.json", "prompts"],
    // The product's own: credentials, session history and runtime caches. Never
    // read, never written, and never copied into a backup slot.
    never_touch: &[
        "auth.json",
        "sessions",
        "history.jsonl",
        "cache",
        "shell_snapshots",
    ],
    // No near neighbour measured for this product. A marker listed here is a
    // refusal waiting to happen, so nothing is listed without evidence.
    foreign_homes: &[],
    permission_profiles: &["default"],
    // `prompts` are Codex's slash commands, so `Command` is declared. MCP
    // servers live under `[mcp_servers.<name>]` inside `config.toml`: a key
    // of a file this provider owns is not a surface it can install, observe
    // and restore on its own, so `Mcp` is not declared. `Skill` and `Plugin`
    // went with the namespaces that turned out not to exist.
    component_kinds: &[
        ComponentKind::Instruction,
        ComponentKind::Setting,
        ComponentKind::Hook,
        ComponentKind::Command,
    ],
    projection_kinds: &[
        ProjectionKind::NativeFiles,
        ProjectionKind::Marketplace,
        ProjectionKind::Plugin,
    ],
    // One scope. Codex's project surfaces live under `.codex/` in a workspace, which is a
    // different root rather than a second scope of this target.
    //
    // Empty rather than absent: a harness that owns one target says so.
    // The one root in this estate that belongs to a convention rather than to a
    // product. `learn.chatgpt.com/docs/build-skills` names `$HOME/.agents/skills`
    // as the user-level skills directory -- a *sibling* of `~/.codex`, not a
    // child, so nothing declared against this provider's own target can reach
    // it. That is what `user_root` exists for, and this is the only scope in
    // the seven that uses it.
    //
    // **Owning a shared root, weighed rather than assumed.** `.agents` is named
    // for being shared, and an owned namespace is removed whole: a `remove`
    // here takes `~/.agents/skills` entirely, not only what a setup put there.
    // Measured 2026-08-28 across all seven baselines, the user-level `.agents`
    // root looked uncontested: only Codex *documents* reading from it, and
    // Antigravity's `.agents` surfaces are all workspace-level with its global
    // configuration at `~/.gemini/config/`. That sentence closed with *if a
    // second product adopts the user-level root, this declaration is the first
    // thing to re-read*.
    //
    // **Four of the seven do, and the sweep that found them read products
    // rather than pages.** Measured 2026-08-28 from pinned artifacts, digests
    // verified before reading:
    //
    // | product | evidence |
    // | --- | --- |
    // | codex | documented: `learn.chatgpt.com/docs/build-skills` |
    // | grok | its own embedded reference: scans `.agents/skills/` *at each tier*, and the tier table names User |
    // | opencode | vendor lists *Global agent-compatible: `~/.agents/skills/<name>/SKILL.md`*; the binary carries the literal |
    // | pi | source, and no page says it: `package-manager.js:2017` |
    //
    // The same sweep found a second shared surface, in the other direction:
    // `~/.claude/skills` is read by grok (*User tier, Lowest, configurable*) and
    // opencode (*Global Claude-compatible*) as well as by Claude Code. This
    // provider's neighbour owns it, and a namespace is removed whole -- so a
    // remove of the Claude setup changes what two other products see.
    //
    // Pi's is the one worth keeping the method for. Its pinned `0.84.3` bundle,
    // `package/dist/core/package-manager.js`: line 1976 builds
    // `userAgentsSkillsDir = join(getHomeDir(), ".agents", "skills")`, line 2012
    // names the root itself as its `dirname`, and line 2017 loads from it --
    // `addResources("skills", collectAutoSkillEntries(userAgentsSkillsDir,
    // "agents"), ...)`. No Pi page says so. (A neighbouring use in
    // `trust-manager.js:160` *excludes* that directory while walking up for a
    // project-scoped one, so the variable name alone would have misread it; the
    // line that matters is 2017.)
    //
    // **What does not change: this declaration stands.** It is measured, it is
    // Codex's own documented root, and nothing routes to `user_root` yet.
    //
    // **What is now open: who owns a root two products read.** Pi is
    // deliberately *not* given the same scoped profile, recorded in
    // `pi-baseline.json` under `$HOME/.agents/skills`. Two providers declaring
    // one path are not two owners -- a namespace is removed whole, so either
    // one's `remove` under this scope takes the other's skills. Recoverable, and
    // still a question a shared root deserves one answer to. Raised with the
    // consumer, whose scope this is.
    scoped_projections: &[Scoped {
        target_scope: TargetScope::UserRoot,
        // Distinct from the global identity, because the digest binds a
        // declaration together with the scope it owns.
        profile_id: "codex/native-files/user-root/1",
        component_kinds: &[ComponentKind::Skill],
        projection_kinds: &[ProjectionKind::NativeFiles],
        // Relative to `~/.agents`, which is the target this scope names -- so a
        // skill is `skills/<name>` rather than `.agents/skills/<name>`. Writing
        // the root into the path would be the eighth face of one sentence: a
        // path is only a path together with what it is relative to.
        native_namespaces: &["skills"],
    }],
    max_files: 8192,
    max_bytes: 64 * 1024 * 1024,
    kit_identity: include_str!("../../../provider-kit/v3/KIT-IDENTITY.json"),
    // Generated by `build.rs` from this harness's `setups/` directory, so the
    // binary carries the catalog it is named after instead of hoping to find
    // one on a disk it was never shipped to.
    embedded_setups: include!(concat!(env!("OUT_DIR"), "/embedded_setups.rs")),
    software: Some(software::SOFTWARE),
};

fn main() -> ExitCode {
    harness_runtime::run(&CODEX, std::env::args().skip(1).collect())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic)]

    use super::*;

    /// The directory name this harness's setups live under in the workspace.
    const TOOL: &str = "codex";
    /// The declaration under test, named once so the shared test below reads
    /// the same in all seven crates.
    const HARNESS: Harness = CODEX;

    /// `build.rs` put the whole catalog in, under the paths it will be read by.
    ///
    /// This does **not** test for staleness, and an earlier version of this
    /// comment claimed it did. It cannot: `build.rs` declares
    /// `rerun-if-changed` on the catalog directory, so editing a setup rebuilds
    /// the table before this runs, and the test would be comparing the tree
    /// with itself. Observed — a deliberately edited setup left it green.
    ///
    /// What it does test is the build script, against a walk written
    /// independently of it: every file present, none invented, bytes exact, and
    /// paths relative and slash-separated. That last one is the one that would
    /// really break — `join("/")` is the only reason these keys are usable on
    /// Windows, and a path built with the platform separator would still look
    /// perfectly correct in the generated source.
    /// The bytes this harness ships, pinned so they cannot change unseen.
    ///
    /// A setup's `definition_digest` is what makes two setups the same setup,
    /// and it appears in `list`, in a plan and in provider state -- and until
    /// this, nothing compared it to anything. A stray character in a setup file
    /// changed what the estate installs and every test stayed green.
    ///
    /// One aggregate rather than one per setup, because the claim is about the
    /// catalogue: sorted definition digests, joined by a newline, hashed. A
    /// deliberate change to a setup updates the line in the baseline, which is
    /// the point -- the peer calls this a golden and it earns itself the first
    /// time a row moves without anyone meaning it to.
    ///
    /// **And it is the three-OS check nothing else makes.** The setups are
    /// embedded with `include_bytes!`, so whatever the checkout holds is what
    /// ships; `.gitattributes` pins `eol=lf` to keep a Windows checkout from
    /// rewriting them, and this is the assertion that would notice if it ever
    /// stopped working. The matrix runs it on all three systems, so a digest
    /// that differed by platform could not stay hidden.
    #[test]
    fn the_catalogue_this_harness_ships_is_the_one_the_baseline_records() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let mut digests: Vec<String> = catalog
            .list()
            .unwrap()
            .iter()
            .map(|setup| setup.definition_digest.clone())
            .collect();
        digests.sort();
        let joined = digests.join("\n");
        let aggregate = harness_runtime::digest_of_bytes(&joined);
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../references")
            .join(format!("{TOOL}-baseline.json"));
        let baseline: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        let recorded = baseline["setup_catalogue_digest"].as_str().unwrap_or("");
        assert_eq!(
            aggregate, recorded,
            "the setups this binary ships are not the ones {TOOL}-baseline.json \
             records; if the change was meant, put this digest there"
        );
    }

    #[test]
    fn the_catalog_this_binary_carries_is_the_one_in_the_tree() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        // The workspace holds one directory per harness; a rendered public tree
        // ships one harness and holds it flat. Same two candidates `build.rs`
        // chooses between, asked the same way.
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };

        // Only the setup directories, which is what the reader lists and what
        // `build.rs` embeds. A rendered public tree also carries a
        // `setups/README.md` at the catalog root, which belongs to no setup.
        let mut on_disk = Vec::new();
        let mut stack: Vec<std::path::PathBuf> = std::fs::read_dir(&root)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.join("setup.json").is_file())
            .collect();
        while let Some(directory) = stack.pop() {
            for entry in std::fs::read_dir(&directory).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    stack.push(path);
                } else {
                    on_disk.push(path);
                }
            }
        }

        assert_eq!(
            HARNESS.embedded_setups.len(),
            on_disk.len(),
            "the binary carries {} files and the tree holds {}",
            HARNESS.embedded_setups.len(),
            on_disk.len()
        );

        for (relative, bytes) in HARNESS.embedded_setups {
            assert!(
                !relative.contains('\\') && !relative.starts_with('/'),
                "{relative:?} is not a relative slash path; a key built with the \
                 platform separator reads correctly on Unix and finds nothing on Windows"
            );
            let path = root.join(relative);
            let found = std::fs::read(&path)
                .unwrap_or_else(|e| panic!("{relative} is compiled in but not in the tree: {e}"));
            assert_eq!(
                &found, bytes,
                "{relative} differs between the binary and the tree"
            );
        }
    }

    #[test]
    fn the_declaration_is_valid_and_names_this_host() {
        let info = CODEX.provider_info().unwrap();
        assert_eq!(info.provider_id, env!("CARGO_PKG_NAME"));
        assert_eq!(info.harness_id, "codex");
        assert_eq!(info.protocol_version, 3);
        assert!(info.supports_this_host());
    }

    #[test]
    fn no_namespace_is_both_owned_and_disclaimed() {
        for name in CODEX.never_touch {
            assert!(
                !CODEX.native_namespaces.contains(name),
                "{name} is claimed and disclaimed"
            );
        }
    }

    /// Everything this harness claims to own, against the vendor page that
    /// decided it.
    ///
    /// What this replaced only checked that the baseline parsed. The block it
    /// reads now is hand-authored beside the rest of the baseline, and this is
    /// what keeps that block from being decoration: a namespace no vendor
    /// document names, or a declared kind no owned surface routes, is red here.
    ///
    /// Both directions, because the defect it was written for ran both ways --
    /// `~/.cursor/rules` was owned and does not exist, `~/.pi/agent/prompts`
    /// exists and was not owned. Conformance caught neither: its
    /// `declared_native_route_is_compilable` case asks for **one** route, not
    /// every one.
    #[test]
    fn every_surface_this_harness_owns_is_one_the_vendor_documents() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../references")
            .join(format!("{TOOL}-baseline.json"));
        let baseline: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        let problems = harness_runtime::surfaces::disagreements(&HARNESS, &baseline);
        assert!(
            problems.is_empty(),
            "the declaration and {TOOL}-baseline.json disagree:
  {}",
            problems.join(
                "
  "
            )
        );
    }

    #[test]
    fn the_control_directory_and_state_file_are_provider_owned_not_product_owned() {
        assert!(CODEX.control_directory.contains("setup-system"));
        assert!(CODEX.state_file.starts_with("NDDEV-"));
        assert!(!CODEX.native_namespaces.contains(&CODEX.state_file));
    }
    /// A setup that writes a configuration file says where its format came from.
    ///
    /// The release before this one made the *surfaces* sourced: a path this
    /// provider owns cites the page that documents it. This is the same rule
    /// one level down, and it was written because two of the seven failed it.
    ///
    /// opencode's baseline set `"permission": "ask"` where the product
    /// documents an object of tool names, and antigravity's set
    /// `toolPermissions` where the product reads `toolPermission` with four
    /// values, none of them the one written. Both were valid JSON in the right
    /// file at the right path. Both installed, verified and restored cleanly.
    /// Neither changed anything about the product — a target that looks
    /// configured and is not, which is the failure this estate refuses one
    /// level up and had been shipping one level down.
    #[test]
    fn a_setup_that_writes_configuration_says_where_its_format_came_from() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems = harness_runtime::catalog::unsourced(&catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }
    /// Three postures, on every one of the seven.
    ///
    /// `baseline` is a working floor, `minimal` is the product's own defaults,
    /// and `full-auto` asks nothing and sandboxes nothing. A caller who learns
    /// them on one product knows them on all seven, which is the whole reason
    /// the names are the estate's rather than each harness's.
    ///
    /// The second half of the check is the one worth having: two setups with
    /// the same bytes mean one of them is a posture in name only, and it would
    /// still read as offered in `list`.
    #[test]
    fn the_three_postures_are_offered_and_are_actually_different() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems = harness_runtime::catalog::asymmetric(&catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }
    /// Nothing this setup ships tells a reader to run something that is not here.
    ///
    /// A setup carries documents an agent reads and acts on -- a skill, a rule,
    /// a command file -- and nothing was checking them. One shipped
    /// `software-status --target <dir> --json` and `list --json` for six
    /// releases; the binary refuses both, and says so in those words.
    ///
    /// Two refusals: a name belonging to the frozen estate, and any line naming
    /// this provider followed by a verb `into_command` does not accept. English
    /// is not judged -- `install` in a sentence is a word, and only
    /// `<provider> install` is an instruction.
    #[test]
    fn nothing_this_harness_ships_names_a_command_it_refuses() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems =
            harness_runtime::catalog::misdirecting(HARNESS.provider_id, &catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }
}
