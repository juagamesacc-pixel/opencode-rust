# Two-Agent Collaboration Guidelines — opencode-rust

Leader: agent-1 (this device, Motorola Moto G35, Termux proot Ubuntu).
Worker: agent-2 (Google Colab — ephemeral VM, fast network, no
preinstalled Rust, disk dies on session timeout).

Sync medium is GitHub. There is no shared memory between systems, so this
file plus the lane-note protocol below are the entire coordination channel.

## 0.0 Pinned source mirror (read this)

Agent-2 has no access to the original TypeScript repo. A pinned copy of
the exact port source lives in `vendor/tui-src/` (1.6 MB, mirrors
`packages/tui/src/` at v1.18.30). Read ONLY this — never reconstruct
behavior from the leader's half-finished Rust ports. Do not edit, move,
or delete anything under `vendor/`.

## 0. Colab session setup (run at the start of EVERY Colab session)

The VM is fresh each time. Paste this first:

```bash
# toolchain (~10 min, once per VM)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
rustup target add aarch64-unknown-linux-gnu 2>/dev/null
# repo
git clone https://github.com/juagamesacc-pixel/opencode-rust.git
cd opencode-rust
git checkout -b agent-2/<lane>   # or: git fetch origin && git checkout agent-2/<lane>
```

Optional persistence: mount Google Drive and keep a bare mirror there, but
GitHub remains the source of truth — Drive is backup only.

## 0.1 Ephemerality rule (Colab-specific, non-negotiable)

- Push WIP to your branch every 30–60 minutes, finished or not
  (`git commit -am "wip: ..." && git push`). A dead VM must never cost
  more than an hour of work.
- Session end checklist: push branch, push a status note (§4), then shut
  down. Next session starts with `git fetch origin && git checkout
  agent-2/<lane> && git pull --rebase`.
- Never `git push --force`, never rebase commits already on GitHub —
  stack new commits instead (cheap insurance against losing work).

## 1. Branch rule (non-negotiable)

- `master` belongs to the leader. Agent-2 NEVER pushes to `master`.
- Agent-2 works on one branch per task: `agent-2/<topic>` (e.g. `agent-2/feature-plugins`).
- When a task is done and green, agent-2 pushes the branch and writes a
  completion note (see §4). The leader merges after CI passes.
- Agent-2 rebases onto `origin/master` before pushing (`git pull --rebase`).
  If `master` moved under a task, rebase, re-verify (§5), then push.
- Verify on Colab with the FULL gate (fast network, real toolchain):
  `cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test`.
  This is stronger than the leader's device check — a green Colab gate is
  the merge precondition.

## 2. Ownership map (do-not-touch zones)

Leader-owned (agent-2: read-only, never edit):
- `.cargo/`, `.github/workflows/`, `Cargo.toml`, `Cargo.lock`
- `crates/tui/src/app.rs`, `crates/tui/src/lib.rs`
- `crates/tui/src/context/sync.rs`, `data.rs`, `local.rs`, `theme.rs`, `sdk.rs`
- `crates/tui/src/ui/dialog.rs`, `dialog_select.rs`
- `crates/tui/src/prompt/history.rs` (shared `PromptInfo` type lives here)
- `guidelines.md` (this file) and `notes/agent-1/`

Agent-2 lanes (pick whole lanes, in order):
1. `crates/tui/src/plugin/` (5 stubs) — ASSIGNED NOW, see
   `notes/agent-1/assignment-01-plugin.md`. Start here, nothing else,
   until the leader merges it.
2. `crates/tui/src/feature_plugins/` (17 stubs)
3. `crates/tui/src/component/prompt/` (autocomplete, index, history, stash, frecency, cwd)
4. `crates/tui/src/routes/session/` dialogs (question, permission, sidebar, footer, subagent_footer, dialog_*)
5. `crates/tui/src/context/editor.rs`, `crates/tui/src/routes/home.rs`

Everything else stays with the leader (routes/session/index, routes/home,
component/prompt/move+workspace, app wiring, final audit).

Rule: one lane at a time. Finish + merge lane N before starting lane N+1,
so rebases stay trivial.

## 3. Claim protocol (prevents double work)

1. Agent-2 writes a claim note FIRST (`notes/agent-2/claim-<lane>.md`),
   pushes the branch (empty commit allowed), and waits for the leader's
   ack note in `notes/agent-1/`.
2. Leader acks by replying in `notes/agent-1/` (or edits this file's lane
   state). No ack = no coding.
3. If a file you need is leader-owned, stop and request it in the lane
   note instead of editing it. The leader will expose a seam or take the
   subtask.

## 4. Lane-note protocol (the update channel)

- Agent-2 writes ONLY under `notes/agent-2/`, one file per update:
  `notes/agent-2/YYYY-MM-DD-<topic>.md`.
- Notes are APPEND-ONLY. Never edit or force-push a note that is already
  on GitHub — push a new file. (This kills merge conflicts by design.)
- Every note has the same header:
  `branch:`, `task:`, `status: (claimed|in-progress|done|blocked)`,
  `files changed:`, `verify: (fmt/clippy/test results)`, `needs leader:`.
- The leader pulls (`git pull`) to read updates and replies under
  `notes/agent-1/` (same rules, mirrored).

## 5. Definition of done (every task, no exceptions)

- 1:1 parity with `opencode-src/packages/tui/src/...` (same behavior,
  same strings, same ordering; SolidJS reactivity → explicit state).
- `cargo fmt --check` clean.
- `cargo clippy --all-targets -- -D warnings` clean.
- `cargo test` green, no regressions.
- No new crate dependencies without leader approval in a note.
- No `unimplemented!` / `todo!` left behind; no dead placeholder structs.

## 6. Merge process (leader side)

1. Pull agent-2's branch + lane notes.
2. Check CI on the branch (or run it). Review the diff for parity.
3. Merge (`--no-ff`) into `master`, push. CI on master must stay green.
4. Reply in `notes/agent-1/` with merge status / follow-ups.

## 7. Conflict + failure rules

- Same-file collision: leader's version wins; the worker re-applies on top.
- Red CI on a worker branch: worker fixes on the same branch, pushes again,
  writes a new note. Never merge red.
- Blocked > 1 day with no path: write a `blocked` note, park the branch,
  claim nothing new until the leader unblocks.
- Secrets, tokens, personal data: never committed, never in notes.

## 8. Why this shape (leader's assessment)

The proposed GitHub-plus-notes plan is sound — git is the only reliable
bridge between two systems. The adjustments above exist to eliminate the
failure modes that kill uncoordinated collaboration: master clobbering
(§1), note merge conflicts (§4), duplicate edits (§2–§3), and — Colab's
own special risk — losing hours of work to VM timeouts (§0.1). Colab's
real advantages are a genuine local toolchain and fast downloads, so
agent-2 owns the most verification-heavy independent lanes and its green
gate is the merge precondition, while the leader keeps the entangled
core (sync/data/app) and the merge gate.
