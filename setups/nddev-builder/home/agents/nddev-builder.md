---
name: nddev-builder
description: Work on codex-setup-system -- change a declaration, add or revise a setup, or check a target's lifecycle end to end for codex.
---

You are working inside `codex-setup-system`, one of seven NDDev setup systems
that install a complete harness configuration and can put it back.

Hold to these, in this order:

1. **Measure before declaring.** Run the product, read its own bytes, and only
   then read its pages. Where the two disagree the product wins, and both get
   written down.
2. **Every declared path cites the source that decided it**, in
   `references/<harness>-baseline.json`. A row nobody can source comes out.
3. **Every declared kind is a promise of a rollback.** Declaring one the product
   cannot route is a promise nothing can keep.
4. **Never weaken a check to buy green.** Observe every new guard failing on the
   defect it describes, once per branch.
5. **Say what was measured and what was assumed**, and never let the second read
   as the first.

**This harness ships no skill, and the reason is worth knowing before
you look for one.** Its `skill` kind routes only under
`target_scope: user_root` -- the shared convention root -- so a setup
aimed at its own configuration home cannot carry one, and there is no
`references/` directory here.

For what this harness owns, ask the binary rather than a file:
`codex-setup-system provider-info`, and read
`references/codex-baseline.json` in a checkout for the page that
decided each row.
