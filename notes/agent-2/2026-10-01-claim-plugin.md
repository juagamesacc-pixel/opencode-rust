# Claim — Assignment 01 (plugin/*)

branch: agent-2/plugin-api
task: Assignment 01 (plugin/api, slots, command_shim, adapters, runtime)
status: claimed
files changed: (none yet)
verify: (not run yet)
needs leader: ack to start coding

## Agent-2 context (Colab worker)

- Role: agent-2 worker per `guidelines.md`. Sticking to lane 1 only: `crates/tui/src/plugin/` (5 stubs). No leader-owned files will be touched.
- Source pin: `vendor/tui-src/plugin/` (v1.18.30 @ 3104c14) is spec. Target: Rust stable (local toolchain absent, see below).
- Branch: `agent-2/plugin-api` created from `origin/master` (commit 44791e9), clean, tracking `origin/master`.
- Scope (write): `crates/tui/src/plugin/api.rs, slots.rs, command_shim.rs, adapters.rs, runtime.rs` (+ `mod.rs` barrel only if needed to keep exports). Read-only: `vendor/tui-src/plugin/*`, assignment + guidelines. Off-limits: everything in guidelines §2 leader-owned.
- Plan: claim → await ack → planner lane (PLAN for plugin 1:1) → implement artifact-by-artifact → self-audit → full gate → `done-plugin.md` note → push branch for leader merge. No push to `master`, no force-push, append-only notes.

## Environment note (IST download rule)

- Local time check: 06:09 AM IST (just outside 12–6 AM free window). No Rust toolchain on this VM (`cargo` absent).
- Per AGENTS.md download rule + PORTING_MAP CI policy (no local toolchain, verify in GitHub Actions), I will NOT install Rust locally without explicit consent. Verification will be via pushed-branch CI unless leader approves a toolchain download in-window.
- Next: planner lane writes lane plan, then implementation starts only after ack. If leader (agent-1) acks via `notes/agent-1/` or via direct approval, I proceed immediately.

## Claim request

Awaiting ack to start coding lane 1. If any seam listed in assignment is stale on current `master`, I will stop and report in a new lane note instead of editing leader files.
