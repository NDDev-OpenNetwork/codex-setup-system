# Support

## Before opening anything

`--help` states what this build does and does not do. `status --target <dir>
--json` reports what it found in a target without changing it, and its output is
safe to share: it carries identities and digests, never secret values.

## Where to go

| You have | Go to |
| --- | --- |
| A defect | [Issues](../../issues) — use the defect template |
| A question about behaviour | [Issues](../../issues) — a blank issue is fine |
| A vulnerability | [Security advisories](../../security/advisories/new), privately |

Never open a public issue for a vulnerability, and never paste credentials,
tokens, or the contents of a backup slot anywhere in this repository. A backup
slot holds whatever the target held when it was captured.

## What this build does, and what it does not

The software lifecycle — installing, updating and removing the product
itself — is declared and does work. `plan` names the exact bytes offline,
whoever holds the network fetches them, and `apply` verifies and installs
with the network gone.

`launch` is declared. It starts the exact executable a software install
placed under `--prefix`, never a name found on `PATH`, and points the
product at `--target` through the environment variable its own
documentation names.

A provider that advertised an operation it cannot perform would let a caller ask
for something that cannot be honoured, which is worse than not offering it.

All five core operations do work: `backup`, `restore`, `remove`, `install` and
`replace`, both from the local setup catalog and from an `ai-stp-bundle/1`
arriving over the wire.

## Using this against a home you already have

**An owned namespace is removed whole.** The table below says what this build
owns; `remove` deletes each of those paths entirely, and a backup slot holds
what was there first. That includes content this build never wrote -- if the
product itself put a key in a configuration file this provider owns, `remove`
takes the file, not the keys this provider added to it.

Measured, with the real product: launching Codex through `launch` and running
`mcp add` writes `~/.codex/config.toml` with an `[mcp_servers.*]` entry; a
later `install` captures that file into a slot and replaces it; a later
`remove` deletes it. The entry is not lost -- `backups` lists the slot as
*before install, setup none*, and restoring it returns the file byte for byte
-- but it is not in the target either.

So: point `--target` at a home you are willing to have managed. `backups
--target <dir>` names every earlier state and which setup each preceded, and
`restore --backup <ref>` returns any of them exactly.

## When conformance says this provider is malformed

`ai-stp provider conformance --protocol-version 3` reports each case by name.
If the one that fails is `provider_info_v3_closed`, with a detail about fields
differing from the closed schema, **check the version of the checker before
suspecting this build**.

The v3 capability schema is compared as an exact field set, so a provider that
declares a field the checker predates is reported as malformed rather than as
newer. `scoped_projection_profiles` (`ADR-0125`) is the field this applies to,
and it is omitted entirely when empty -- so a build that declares no scope
satisfies an older checker by accident, and a build that declares one does not.

Two versions, two different answers, both measured:

| checker | result |
| --- | --- |
| `ai-stp-cli` 0.0.3 | five pass; Codex and Antigravity report `conforms=false`, detail *fields differ from the closed v3 schema* |
| `ai-stp-cli` 0.0.7 | six pass 23 of 23; Codex reports `conforms=false`, detail *a scoped projection profile names an unknown target scope* |
| `ai-stp-cli` 0.0.8 | **all seven pass**, 27 to 29 cases each |

The middle row was never a defect in this build, and the third row is how that
was settled: **it closed with no change on this side.** `0.0.7` carried the
field but its scope enum was `["project"]` alone, while the provider kit this
program vendors and verifies byte-for-byte gave `["project", "user_root"]`. The
kit is the artifact a provider is told to build against, so a build declaring
`user_root` was right by the document it was handed and wrong by the checker
shipped beside it. `0.0.8` shipped the enum, and a declaration that had been
correct for a month started being read as correct.

**Withdrawing a correct declaration to make a lagging instrument print green is
never the answer here.** The three rows above are the argument for that, and
they are also the argument for the rule this section exists for.

Which is the general rule this section exists for: **check the version of the
checker before suspecting this build**, and prefer the newest, because an older
one reports a wider failure than the one it found.

## What `status` reports, and what it does not

`state` answers **who manages this target**, and never *whether a setup is
installed*. Three values, and the distinction matters most for the fourth
situation, which is not a fourth value:

| | |
| --- | --- |
| `missing` | the directory is empty |
| `unmanaged` | it holds content, none of it this provider's |
| `managed` | this provider's state file is present and current |

`missing` used to be looser -- it asked whether this provider owned anything,
so a directory full of another product's files reported `missing`. A consumer
reads this to decide what it is looking at, and being told a populated
directory is empty invites it to treat the place as free. Emptiness is about
the directory, not about us.

**After a `remove`, `state` stays `managed`, and that is the honest answer.**
The setup is gone -- no file a product reads survives it -- but the control
directory and a backup slot remain, and that slot is what makes the removal
reversible: `restore` brings the setup back. A target reported as `missing`
while a restore is pending would be a lie in the direction that costs someone
their data.

Whether a setup is installed is carried by `setup_stable_id`, which is `null`
exactly when none is. That is the field to test, not this word.
`target_identity_digest` corroborates it -- after a remove it is the digest of
an empty tree -- but the field is the direct answer and the digest is not.

## The network, stated exactly

**This artifact does not link the network, and no local phase can spawn
anything that could.** Two lints hold it rather than a promise: `std::net` is
refused outright, and `std::process::Command` is refused everywhere but two
named places -- the `launch` command, which is declared in `provider-info` and
absent from builds that do not declare it, and a lifecycle probe that drives
this binary's own executable. Adding a `tar` shell-out to ordinary code fails
the build with *only `launch` may spawn, and it is declared*. Every crate that
may be linked is named in `deny.toml`, so a transitive dependency cannot arrive
unread.

Those are claims about the source, and a lint can be wrong, bypassed, or simply
disbelieved. So `ci` reads the shipped binary too: a `boundary` job asks the
import table of the artifact this build produces whether any network symbol is
present, and whether a build declaring no `launch` imports anything that could
spawn. You can run it yourself against a downloaded release --
`nm -D --undefined-only <binary>` on Linux, `nm -u` on macOS -- and it needs no
part of this repository to be trusted.

**What that does not buy, said plainly because the stronger claim is the
tempting one.** This is a dynamically linked program: it imports `syscall` from
libc like any other, so no property of the binary can prove a socket is
unreachable to code that is determined to open one. What is proven is narrower
and still worth having: no code path here reaches for the network, none can be
added without the build refusing, and no local phase can hand the job to a
child process. If your threat model needs the guarantee rather than the
absence, run `plan` and `apply` under whatever sandbox you already trust; both
phases are offline by design, and `apply` verifies the digests it was given
with the network gone.

## What this build owns inside a target

Everything else in the target is a sibling overlay and is preserved
verbatim. Each row cites the vendor page it was read from, and the same
table is bound to the declaration by a test, so this cannot drift from
what `provider-info` publishes.

Configuration home as the product documents it: `~/.codex`.

| Path | Component kinds routed here | Decided by |
| --- | --- | --- |
| `AGENTS.md` | `instruction` | [source](https://learn.chatgpt.com/docs/agent-configuration/agents-md) |
| `config.toml` | `setting` | [source](https://learn.chatgpt.com/docs/config-file/config-reference) -- anchored literal measured in the pinned artifact by scripts/evidence.py |
| `hooks.json` | `hook` | [source](https://learn.chatgpt.com/docs/config-file/config-reference) |
| `prompts` | `command` | [source](https://learn.chatgpt.com/docs/custom-prompts) |
| `agents` | `agent` | [source](https://github.com/openai/codex/blob/rust-v0.151.0/codex-rs/agent-roles/src/discovery.rs) -- routing measured by running the 0.151.0 binary against a temporary CODEX_HOME |

A path routing no component kind is owned so a setup can carry it;
nothing compiles a component to it.

### A second target: `target_scope: user_root`

Rooted at `~/.agents`, which is not the configuration home
above. A consumer reaches it by naming the scope on the request, and
every path below is relative to that root.

| Path | Component kinds routed here | Decided by |
| --- | --- | --- |
| `skills` | `skill` | [source](https://learn.chatgpt.com/docs/build-skills) -- and measured by running the 0.150.1 product against a temporary HOME, 2026-08-29 |

This root is read by several products at once, so under this scope
`remove`, the backup and a restore act on the files this program
recorded writing rather than on the directory whole. A neighbour's
files are never captured into a backup slot here, and never reverted
by a restore.

### A second target: `target_scope: project`

Rooted at `project root`, which is not the configuration home
above. A consumer reaches it by naming the scope on the request, and
every path below is relative to that root.

| Path | Component kinds routed here | Decided by |
| --- | --- | --- |
| `AGENTS.md` | `instruction` | [source](https://developers.openai.com/codex/guides/agents-md) -- codex-0.153.0-linux-x64.tgz sha256:856f408ea61b44a381b7d6fb7c82365dfcef649ae2a340fc01282cf63c30cd8a, run 2026-09-03 |
| `.codex/hooks.json` | `hook` | [source](https://developers.openai.com/codex/hooks) -- path and project scope carried by the Codex 0.153.0 hook configuration implementation |

This root is read by several products at once, so under this scope
`remove`, the backup and a restore act on the files this program
recorded writing rather than on the directory whole. A neighbour's
files are never captured into a backup slot here, and never reverted
by a restore.

### Considered and not owned

Everything named here is left exactly as it was found, like any
other file beside a target.

**`skills`** -- Codex does read personal skills from here, and this record used to say it did not. Measured 2026-08-28 by installing the product through this provider and driving it with `debug prompt-input`, which renders the model-visible skills list: a SKILL.md at $CODEX_HOME/skills/<name>/ is listed, with that locator. The earlier reason -- "Codex has no skills directory under its own home" -- was read off the page and the page does not say it; it lists where skills are found and is silent about this one. A negative taken from a document that never made it. Declined all the same, for two reasons that compound. First, no page says so: learn.chatgpt.com/docs/build-skills names the working directory and its parents to the repository root, $REPO_ROOT/.agents/skills, $HOME/.agents/skills, /etc/codex/skills, and skills bundled by OpenAI -- not this; and github.com/openai/codex/blob/main/docs/skills.md links to developers.openai.com/codex/skills, which redirects back to that same page, so there is one source reached two ways rather than two sources. Second, the product manages this directory: its own bundled skills materialise at skills/.system/, and deleting that directory entirely made the product rebuild all six on the next start. Owning `skills` would make `remove` delete the product's own along with ours, and the only evidence that this survives is the product's willingness to recover -- behaviour rather than a promise, true until the product changes. The documented user-level path is $HOME/.agents/skills, outside any product's configuration home, which is what the consumer's ADR-0127 proposes a scope for. (measured through launch; learn.chatgpt.com/docs/build-skills lists six locations and not this one)

**`.agents/skills`** -- The user-scope skills directory is $HOME/.agents/skills -- a sibling of ~/.codex, not a child of it. Declared relative to this provider's target it resolves to ~/.codex/.agents/skills, which Codex never reads. Same shape as the pi managed_paths defect. ([source](https://learn.chatgpt.com/docs/build-skills))

**`plugins`** -- Codex plugins are drawn from remote marketplaces shared with ChatGPT, not from a folder under the Codex home. Re-checked against the official rust-v0.153.0 release notes on 2026-09-03: the new plugin CLI can list, install and remove plugins from remote marketplaces. That is an acquisition command and product-managed state, not a new provider-owned projection path, so the decline remains correct and the nddev-builder must not manufacture a local plugins namespace. ([source](https://learn.chatgpt.com/docs/codex/cli))

**`AGENTS.override.md`** -- Codex reads this **before** AGENTS.md at global scope and takes only the first non-empty file at that level, so a home holding a non-empty one ignores the instruction file this provider installs. An empty override does not silence the floor -- Codex skips empty files -- which is worth knowing before concluding that a present override is the reason instructions are not applying. Deliberately not owned: an override exists so a person can escape a managed floor, and a provider that owned it could remove the escape with `remove`. ([source](https://learn.chatgpt.com/docs/agent-configuration/agents-md))

**`NDDEV-CODEX-PROVIDER.json`** -- This provider's own state file: which setup is applied, the identity it recorded, and which slot reverses the last operation. Written by every operation and excluded from target identity, because counting it would leave a target different from the identity the operation just wrote. Not a projection surface and never ownable as one. (this provider's own contract; no vendor page is involved)

**`.codex-setup-system`** -- This provider's own control directory: the target lock, the backup slots and their payloads. Kept out of the declaration for the same reason as the state file, and recorded here because the declined list is where a reader looks before opening a file to find out what it is. (this provider's own contract; no vendor page is involved)

**`managed-config`** -- Not a path in the target, and named without an extension for that reason: this product's managed policy lives at a **system** path, and every recorded path here is relative to the target. Three literals in the 0.149.1 binary:

  * `/etc/codex/managed_config.toml` — the administrator's policy
  * `/etc/codex/requirements.toml` — the clamp above it
  * `/etc/codex/config.toml` — a system-wide layer below both

The same build carries `legacy_managed_config_file`, `legacy_managed_config_mdm` and `allow_managed_hooks_only`, so the policy also arrives by MDM and can restrict which hooks may run at all. No macOS or Windows equivalent appears as a literal in this build, and none is claimed here: absence of a literal is not absence of a path, and the honest record is what was measured.

**It bears on the `full-auto` posture**, exactly as the same surface does on the two harnesses that already record one. That setup writes a permissive approval policy and sandbox mode into the owned `config.toml`; under a managed policy those keys are correct, at a correct path, in a file the product reads — and a higher layer overrides them. Install, verify and restore all succeed and nothing about what the product permits has changed.

Recorded and never touched. It needs root to write, it is outside the configuration home this provider is given, and owning an organisation's policy is the defect this estate has already shipped once: on the harness next door, owning a signed policy deleted it and kept its signatures, which is the one state that product's own gate refuses.

**Corrected 2026-08-30: the sentence above measured one build and spoke about the product.** It said *"no macOS or Windows equivalent appears as a literal in this build, and none is claimed here"*, which was true and was read as a fact about codex rather than about a linux artifact. Windows paths are compiled out of a linux binary, so that search could only ever return zero.

Asked of the Windows artifact instead -- `codex-0.151.0-win32-x64.tgz`, sha256 `9044e644…d7355` checked against this baseline's own table before extracting, `package/vendor/x86_64-pc-windows-msvc/bin/codex.exe`:

  OpenAI\Codex\requirements.toml   6
  OpenAI\Codex                      6
  ProgramData                       7
  OpenAI\Codex\config.toml          0
  NddevInventedDir                  0   (control)
  nddev-invented                    0   (control)

So the managed **requirements** path is in the shipped Windows bytes as a joined literal, the config path beside it is composed at runtime and does not appear as one -- the same reason every runtime-joined path in this record reads `page` -- and the invented controls return zero, so the search discriminates. The pinned source agrees: `config/src/loader/mod.rs` at `rust-v0.151.0` documents `%ProgramData%\OpenAI\Codex\requirements.toml` and `…\config.toml` as the Windows system layers, `/etc/codex/…` as the Unix ones, and carries a `#[cfg(target_os = "macos")]` branch reading a managed requirements payload delivered by MDM.

Still declined, and for the reason it always was: these are **system** paths, outside any target this provider is given. What changes is that the record no longer implies the product lacks them. (measured in the 0.149.1 binary; https://learn.chatgpt.com/docs/config-file/config-reference)

**`mcp.json`** -- Codex keeps MCP servers under `[mcp_servers.<name>]` in `config.toml`, which this provider owns and writes and restores whole -- so MCP is already covered by the `setting` kind. **A key inside a file is not a projection surface**: declaring `mcp` here would promise to install, observe and roll back a fragment of a document, which nothing in this program does.

There is no `mcp.json` under this home. The 0.150.1 binary carried the literal twenty-two times and every one was about a *plugin's* servers -- `selected_executor_plugin_mcp.mcp.json` beside `.codex-plugin/plugin.json`, `.claude-plugin/plugin.json` and `.cursor-plugin/plugin.json`. Recorded because the string is there and a reader who greps for it will find it.

**Re-asked of the 0.151.0 linux/x86_64 bytes on 2026-08-30, because a decline is only as current as the release it was taken against.** Twenty-three occurrences now, still every one a plugin context; `.codex/mcp.json` returns zero, and two invented paths searched in the same run return zero, so the search discriminates. `mcp_servers` appears forty times. The decline stands and is now measured against what this tree pins.

Re-measured deliberately rather than left alone: the `agents` row above declined a kind on a 0.149.1 reading that 0.151.0 had already made false, and this is the other kind this harness withholds. A decline nobody re-asks is the shape that defect had.

One false lead resolved while doing it, recorded so the next reader does not chase it: the strings table places `agents/openai.yaml` next to these literals, which would be alarming if it were a path under this home -- `agents` is a namespace this provider removes whole. It is not. It is `skill-creator/agents/openai.yaml`, an asset inside the product's own bundled sample skill, plus a capability-discovery message. Nothing writes into `$CODEX_HOME/agents` but a person and this provider. (measured from the pinned artifact, digest verified before reading (codex 0.150.1); https://learn.chatgpt.com/docs/config-file/config-reference)

## Response

One maintainer. Defects are triaged as time allows; security reports are
acknowledged first.
