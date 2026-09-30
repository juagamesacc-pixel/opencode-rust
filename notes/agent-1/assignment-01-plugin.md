# Assignment 01 — `plugin/*` (first task for agent-2)

Status: ASSIGNED, awaiting claim note from agent-2.
Branch: `agent-2/plugin-api` (create from `origin/master`).

## Why this task first

Small (5 files, ~660 source lines), fully independent (nothing else in
the crate imports `plugin/` yet), and it exercises the whole loop:
claim → code → verify → push → note. No JSX rendering of its own — pure
logic plus dialog mounting, so it cannot clash with the leader's UI work.

## File mapping (source → target, exact)

Source lives in `vendor/tui-src/plugin/` (pinned copy of the TS source —
read ONLY these files, never the leader's half-finished ports for style):

| Source | Target | Notes |
|---|---|---|
| `api.ts` (52) | `crates/tui/src/plugin/api.rs` | Route registry + API factory |
| `slots.tsx` (65) | `crates/tui/src/plugin/slots.rs` | Slot registry + host guard |
| `command-shim.ts` (109) | `crates/tui/src/plugin/command_shim.rs` | v1 command bridge |
| `adapters.tsx` (355) | `crates/tui/src/plugin/adapters.rs` | Full TUI API surface |
| `runtime.tsx` (81) | `crates/tui/src/plugin/runtime.rs` | Runtime owner + provider |

Replace the 16-line `struct Stub;` placeholders completely. Keep the
`// source: ...` header line, update the line count.

## Seams you may use (all verified present on `master`)

- `crate::ui::dialog::{DialogStack, DialogControl, DialogSize}`
- `crate::ui::dialog_alert::show_alert`, `dialog_confirm::show_confirm`,
  `dialog_prompt::{show_prompt, PromptProps}`, `dialog_workspace_file_changes`
- `crate::ui::dialog_select::{SelectState, SelectOption}`
- `crate::keymap::{COMMAND_PALETTE_COMMAND, format_key_bindings, format_key_sequence}`
- `crate::config::keybind::{BindingLookup, CommandMap}` (check exact names in file)
- `crate::context::kv::KvStore`, `route::{RouteStore, Route}`, `sdk::SdkClient`,
  `sync::SyncStore`, `theme::ThemeContext`, `event::{EventMetadata}`
- `crate::ui::toast::{ToastState, ToastInput}`, `crate::util::selection::ToastVariant`
- `crate::util::error::{error_message_value, error_message_std}`

Do NOT touch anything outside `crates/tui/src/plugin/` (see guidelines §2).
If a seam is missing or wrong, stop and report it in your lane note —
do not edit leader-owned files.

## Hard part, decided for you

JSX render closures (`TuiRouteDefinition["render"]`, slot views, `DialogUI`
wrappers) have no Rust equivalent. Model them as:

- route render: `Box<dyn FnMut(&mut DialogStack) + Send>` (every real use
  mounts dialogs — alert/confirm/prompt/select/replace).
- slot views: an enum (`SlotView::None` + text/kind variants you need),
  rendered later by the app. `register` keeps key-based unregister
  semantics verbatim (symbol key → incrementing `u64` id).
- `lifecycle.signal` → `tokio_util`-free: an `Arc<AtomicBool>` abort flag.
- `onDispose` → stored `Box<dyn FnOnce() + Send>` run at dispose.
- `console.warn`/`console.error` → `eprintln!` with the same tags.
- `warnOnce` → module-level `std::sync::OnceLock<Mutex<HashSet<String>>>`
  or equivalent.

Keep every method name, string literal, message, and ordering from the
source. `slots.register` outside plugin context throws — mirror with
`Err("slots.register is only available in plugin context")`. The
`plugins.*` stubs return `[]`/`false`/`{ok:false,...}` verbatim.

## Verify (Colab, full gate — all must pass)

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## Claim protocol (do this FIRST, before coding)

1. `git checkout -b agent-2/plugin-api origin/master`
2. Write `notes/agent-2/claim-plugin.md` (copy the template below), commit,
   push the branch.
3. Wait for leader ack in `notes/agent-1/`. Then code.

Template for `notes/agent-2/claim-plugin.md`:

```markdown
# Claim — Assignment 01 (plugin/*)
branch: agent-2/plugin-api
task: Assignment 01 (plugin/api, slots, command_shim, adapters, runtime)
status: claimed
files changed: (none yet)
verify: (not run yet)
needs leader: ack to start coding
```

Completion note (`notes/agent-2/done-plugin.md`) uses the same header with
`status: done`, the 5 files, and the pasted verify output.
