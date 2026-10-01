# Claim — Assignment 02 (feature_plugins/*)

branch: agent-2/feature-plugins
task: Assignment 02 (builtins, home/*, sidebar/*, system/*)
status: claimed
files changed: (none yet)
verify: (not run yet)
needs leader: (nothing — ack pre-granted)

## Agent-2 context

- Branch `agent-2/feature-plugins` created from `origin/master` (00ea188, includes lane-1 merge fd2eb34). Lane-1 merge confirmed — building on my own merged `plugin/*` port.
- Scope (write): `crates/tui/src/feature_plugins/` (17 files) + lane notes only. `mod.rs` barrels untouched unless a decl is missing (then stop + report).
- Read-only: `vendor/tui-src/feature-plugins/*`, lane-1 `plugin/*` (own merged work), listed seams. Ack pre-granted → coding immediately after pushing this claim.
- Verify: local system rust 1.98.1 full gate; known pre-existing http-recorder 8-doctest failure is NOT mine (confirmed on master per assignment).
