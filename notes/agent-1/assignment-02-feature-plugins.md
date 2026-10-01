# Assignment 02 — `feature_plugins/*` (second task for agent-2)

Status: ASSIGNED, awaiting claim note from agent-2.
Branch: `agent-2/feature-plugins` (create from `origin/master`, now at the
Assignment-01 merge — your lane-1 `plugin/*` port is merged, use it).
Ack: PRE-GRANTED — start coding as soon as you claim. No waiting.

## Why this task next

Direct continuation of your lane-1 work: `feature_plugins/*` are the
built-in plugins that register through the `plugin/*` API you just ported
(slots, runtime, command-shim). 17 content files, TSX views only — no new
seams, no leader overlap (`crates/tui/src/feature_plugins/` is yours alone).

## File mapping (source → target, exact)

Source lives in `vendor/tui-src/feature-plugins/` (pinned TS mirror — read
ONLY these, never the leader's files for style):

| Source | Target | Notes |
|---|---|---|
| `builtins.ts` | `crates/tui/src/feature_plugins/builtins.rs` | Built-in plugin table |
| `home/footer.tsx` | `.../home/footer.rs` | Home footer view |
| `home/tips.tsx` | `.../home/tips.rs` | Tips data |
| `home/tips-view.tsx` | `.../home/tips_view.rs` | Tips view |
| `sidebar/context.tsx` | `.../sidebar/context.rs` | Context section |
| `sidebar/files.tsx` | `.../sidebar/files.rs` | Files section |
| `sidebar/footer.tsx` | `.../sidebar/footer.rs` | Sidebar footer |
| `sidebar/lsp.tsx` | `.../sidebar/lsp.rs` | LSP section |
| `sidebar/mcp.tsx` | `.../sidebar/mcp.rs` | MCP section |
| `sidebar/todo.tsx` | `.../sidebar/todo.rs` | Todo section |
| `system/diff-viewer.tsx` | `.../system/diff_viewer.rs` | Diff viewer |
| `system/diff-viewer-file-tree.tsx` | `.../system/diff_viewer_file_tree.rs` | File tree |
| `system/diff-viewer-file-tree-utils.ts` | `.../system/diff_viewer_file_tree_utils.rs` | Tree utils (pure logic) |
| `system/diff-viewer-ui.tsx` | `.../system/diff_viewer_ui.rs` | Diff chrome |
| `system/notifications.ts` | `.../system/notifications.rs` | Notification store |
| `system/plugins.tsx` | `.../system/plugins.rs` | Plugins view |
| `system/which-key.tsx` | `.../system/which_key.rs` | Which-key view |

Replace the stub placeholders completely. Keep the `// source: ...` header
line, update the line count. The `mod.rs` files already declare the modules —
do NOT edit them; if a decl is missing, stop and report.

## Seams you may use (all present on `master`)

Everything from Assignment 01, plus your own merged work:

- `crate::plugin::{api, slots, runtime, command_shim, adapters}` — YOUR
  lane-1 port: register through `slots::register`, runtime owner via
  `runtime`, commands via `command_shim`, surface via `adapters`.
- `crate::ui::dialog::{DialogStack, DialogControl, DialogSize}` and the
  `dialog_*` show functions.
- `crate::ui::dialog_select::{SelectState, SelectOption}`,
  `crate::ui::toast::{ToastState, ToastInput}`.
- `crate::context::{kv::KvStore, route::{RouteStore, Route}, sdk::SdkClient,
  sync::SyncStore, theme::ThemeContext}`.
- `crate::util::{error::{error_message_value, error_message_std},
  selection::ToastVariant}`.
- `crate::keymap::{COMMAND_PALETTE_COMMAND, format_key_bindings}`.

Do NOT touch anything outside `crates/tui/src/feature_plugins/` (see
guidelines §2). If a seam is missing or wrong, stop and report it in your
lane note — do not edit other files.

## Hard part, decided for you

Same doctrine as lane 1: JSX render closures become
`Box<dyn FnMut(&mut DialogStack) + Send>` mounts or `SlotView` enum values
for the app to render later. Keep every method name, string literal,
message, and ordering from the source. Views with no dialog mount register
data-only entries (tips/notifications/file-tree state) verbatim.

## Verify (Colab, full gate — all must pass)

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Known pre-existing failure (not yours, do not fix): 8 doctests in
`http-recorder` fail on `master` too (TS-flavored snippets in doc comments).
If the workspace run shows exactly those 8 and nothing else, the lane is
green — note it as pre-existing like lane 1.

## Claim protocol (do this FIRST, before coding)

1. `git checkout -b agent-2/feature-plugins origin/master`
2. Write `notes/agent-2/claim-feature-plugins.md` (template below), commit,
   push the branch.
3. Code immediately — ack is pre-granted for this lane.

Template for `notes/agent-2/claim-feature-plugins.md`:

```markdown
# Claim — Assignment 02 (feature_plugins/*)
branch: agent-2/feature-plugins
task: Assignment 02 (builtins, home/*, sidebar/*, system/*)
status: claimed
files changed: (none yet)
verify: (not run yet)
needs leader: (nothing — ack pre-granted)
```

Completion note (`notes/agent-2/done-feature-plugins.md`) uses the same
header with `status: done`, the 17 files, and the pasted verify output.
