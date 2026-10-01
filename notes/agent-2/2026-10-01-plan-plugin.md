# Lane Plan — Assignment 01 (plugin/*) 1:1 port

branch: agent-2/plugin-api
task: Assignment 01 (plugin/api, slots, command_shim, adapters, runtime)
status: planned (awaiting ack to code)
files changed: (plan only — no source touched)
verify: (not run yet — CI gate on implementation)
needs leader: ack to start coding

> Planner-lane fallback: @planner delegation failed in this env (free-tier task tool), so orchestrator authored this plan directly from disk-verified reads. Same 1:1 doctrine, same scope. No code written.

## 1. Research gate (pins — final)

- Source: `anomalyco/opencode` (ex-`sst/opencode`) tag `v1.18.30` @ `3104c1428ec91f809e5ab86631300de41eb6952e`. Pinned mirror for this lane: `vendor/tui-src/plugin/` (read-only, never edit): `api.ts` 52 / `slots.tsx` 65 / `command-shim.ts` 109 / `adapters.tsx` 355 / `runtime.tsx` 81 = 662 lines.
- Source runtime: Bun 1.3.14 / TS 5.8.2. Target: Rust stable, workspace `version 1.18.30`, edition 2021.
- Deps: `serde 1 + serde_json 1 + tokio 1` ONLY where source is async. NO new deps without leader approval in a note.
- Target stubs (attendance baseline): `crates/tui/src/plugin/{api,slots,command_shim,adapters,runtime}.rs` each 16-line `struct Stub`, plus `mod.rs` barrel (14 lines, re-exports in source order). Implementation replaces Stub bodies completely, keeps `// source: ...` header, updates line count.

## Seam checklist (verified on master 44791e9 — all present)

| Assignment seam | Found |
|---|---|
| `crate::ui::dialog::{DialogStack, DialogControl, DialogSize}` | `crates/tui/src/ui/dialog.rs` exists |
| `crate::ui::dialog_alert::show_alert`, `dialog_confirm::show_confirm`, `dialog_prompt::{show_prompt, PromptProps}` | all three files exist in `ui/` |
| `dialog_workspace_file_changes` | lives at `crates/tui/src/component/dialog_workspace_file_changes.rs` (not `ui/`) — use component path |
| `crate::ui::dialog_select::{SelectState, SelectOption}` | `ui/dialog_select.rs` exists |
| `COMMAND_PALETTE_COMMAND`, `format_key_bindings`, `format_key_sequence` | `keymap.rs:15,182,195` present |
| `BindingLookup`, `CommandMap` | `config/keybind.rs` `BindingLookup` present; `CommandMap` referenced via `TuiKeybind::CommandMap` — verify exact name at implement time, stop + report if drifted |
| `KvStore`, `RouteStore/Route`, `SdkClient`, `SyncStore`, `ThemeContext`, `EventMetadata` | `context/{kv,route,sdk,sync,theme,event}.rs` all exist |
| `ToastState/ToastInput`, `ToastVariant` | `ui/toast.rs` + `util/selection.rs` exist |
| `error_message_value/error_message_std` | `util/error.rs` exists |
| `isRecord` (slots) | `util/record.rs` exists |
| `getOpencodeModeStack` (adapters mode) | check at implement time; stop + report if missing |

No BLOCKER. One path note (workspace_file_changes under `component/`) recorded above.

## 2. Architecture (exact clone, zero-diversion)

File map (source → target, status = STUB → PORT in this lane):

| # | Source | Target | Contents |
|---|---|---|---|
| 1 | `plugin/api.ts` 52 | `plugin/api.rs` | `RouteEntry{key:u64,render}`, `RouteMap`, `createPluginRoutes→PluginRoutes{register,get}`, `createTuiApi` (+lifecycle) |
| 2 | `plugin/slots.tsx` 65 | `plugin/slots.rs` | `HostSlotPlugin/HostPluginApi/HostSlots`, `isHostSlotPlugin`, `createSlots→{Slot,setup,clear}` |
| 3 | `plugin/command-shim.ts` 109 | `plugin/command_shim.rs` | `COMMAND_PALETTE_SHOW`, `warned` set, `warnCommandShim`, `createCommandShimDialog`, `warnOnce`, `toCommand`, `toBindings`, `createCommandShim` |
| 4 | `plugin/adapters.tsx` 355 | `plugin/adapters.rs` | `Input`, `routeNavigate/routeCurrent`, `mapOption/pickOption/mapOptionCb`, `stateApi`, `appApi`, `createTuiApiAdapters` + re-exports of api |
| 5 | `plugin/runtime.tsx` 81 | `plugin/runtime.rs` | `createPluginRuntime→PluginRuntime{Slot,routes,commands,status,update,clear,setupSlots}`, `PluginRuntimeCommands`, `emptyCommands`, `TuiPluginHost`, `Context/Provider/usePluginRuntime` |
| — | barrel | `plugin/mod.rs` | keep module list + re-export order unchanged (touch only if exports shift) |

TS → Rust (assignment decisions, verbatim):
- Route `render` closure → `Box<dyn FnMut(&mut DialogStack) + Send>`; slot views → `enum SlotView::None + text/kind variants`, app renders later; `register` key `symbol` → incrementing `u64` id with identical unregister semantics.
- `lifecycle.signal` (AbortSignal) → `Arc<AtomicBool>` abort flag; `onDispose` → `Box<dyn FnOnce() + Send>` run at dispose; `console.warn/error` → `eprintln!` same tags (`[tui.plugin] deprecated TUI plugin API`, `[tui.slot] plugin error` with plugin/slot/phase/source/message fields in order).
- `warnOnce` → `OnceLock<Mutex<HashSet<String>>>` (module-level `warned`).
- `createSignal` revision/view/commands/status → explicit struct + `new()/update()` + revision counter; `get(name)` returns LAST registered render (`.at(-1)` semantics).
- `slots.register` outside plugin context → `Err("slots.register is only available in plugin context")`; `plugins.*` → `[]/false/{ok:false,...}` verbatim; `theme.install` throws `"theme.install is only available in plugin context"`; `plugins.install` message `"plugins.install is only available in plugin context"`; runtime `install` message `"Plugin runtime is not available."`; `usePluginRuntime` outside provider throws `"usePluginRuntime must be used within PluginRuntimeProvider"`.
- `COMMAND_PALETTE_SHOW = "command.palette.show"`; shim `register/trigger/show` warn strings `"api.command.register"→"api.keymap.registerLayer({ commands, bindings })"`, `"api.command.trigger"→"api.keymap.dispatchCommand(name)"`, `"api.command.show"→api.keymap.dispatchCommand("command.palette.show")`; `toCommand` namespace `"palette"` + field order namespace/name/title/desc/category/suggested/hidden/enabled/slashName/slashAliases/run; toast default `variant "info"`.

Zero-diversion statement: no functionality changes, no redesign, no renames for taste (nesting flattened `command-shim.ts→command_shim.rs` only where Rust forbids `-`; recorded here), no reordering, no dropped branches. Every deviation is a bug unless leader approves in writing.

## 3. Implementation sequence (single writer, sequential)

Order (dependencies first): `api.rs` → `slots.rs` → `command_shim.rs` → `runtime.rs` (uses api+slots) → `adapters.rs` (uses api+command_shim + all seams) → `mod.rs` iff needed.
No parallel writers — one lane, disjoint from leader. Each file: inventory (below) → translate in source order → `cargo fmt` on file → self-audit vs source → next file.

Per-file artifact inventory (must all survive):
- `api.ts`: `RouteEntry`, `RouteMap`, `createPluginRoutes` (+ inner `register/unregister-closure/get`), `PluginRoutes`, `createTuiApi` (+ inner `lifecycle{signal,onDispose}`).
- `slots.tsx`: `RuntimeSlotMap`, `SlotView`, `HostSlotPlugin`, `HostPluginApi`, `HostSlots{register,dispose}`, `isHostSlotPlugin`, `createSlots` (+ `Slot/setup/register/dispose/clear`), `empty` view, `onPluginError` payload keys.
- `command-shim.ts`: `COMMAND_PALETTE_SHOW`, `warned`, `Warn/LegacyDialog/CommandShimDialog/LegacyKeybinds` types, `warnCommandShim`, `createCommandShimDialog` (+ `replace/clear/setSize/size/depth/open`), `warnOnce`, `toCommand`, `toBindings`, `createCommandShim` (+ `register/trigger/show`).
- `runtime.tsx`: `createPluginRuntime` (+ `Slot/routes/commands/status/update/clear/setupSlots`), `PluginRuntimeCommands{activate,deactivate,add,install}`, `emptyCommands` (3×false + install `{ok:false}`), `PluginRuntime`, `TuiPluginHost{start,dispose}`, `Context/Provider/usePluginRuntime`.
- `adapters.tsx`: `Input` (15 fields in order), `routeNavigate` (home/session/plugin branches), `routeCurrent` (home/session/plugin shapes), `mapOption/pickOption/mapOptionCb`, `stateApi` (+ `ready/config/provider/path/vcs/session{count,get,diff,todo,messages,status,permission,question}/part/lsp/mcp` + mcp sort + failed-error rule), `appApi`, `createTuiApiAdapters` (all 15+ keys in source order incl. `slots.register` throw, `plugins.*`, `theme.*`).

## 4. Parity verification

- Attendance: 5/5 files ported, every inventory item has a target counterpart; `mod.rs` exports unchanged order; no extra `pub` beyond source.
- Behavior spot-checks (quoted, verify verbatim at audit): `at(-1)` last-wins route; unregister removes only matching key, deletes name when empty; `slots.register` + `theme.install` + `plugins.install` error strings; `install→{ok:false,"Plugin runtime is not available."}`; `COMMAND_PALETTE_SHOW` + 3 warn-once pairs; `namespace:"palette"`; dialog `depth/open` = `stack.length`; toast `variant ?? "info"`; mcp sorted by name + `error` only when `failed`; `session.diff` skips `file===undefined`; `routeCurrent` session `{sessionID,prompt}` shape.
- Tests: source `plugin/` has zero `*.test.*` (confirm at implement time with listing); verification = verbatim spot-checks + `mod.rs` order check + full-gate green. No test invented, none weakened.

## 5. Build/test and fix gate (CI-driven)

- Local VM has NO toolchain and IST is outside free-download window → NO `curl rustup` without explicit consent. Gate runs in GitHub Actions on pushed branch: `cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test` must all pass (guidelines §5 + §1 merge precondition).
- Fix budget: ≤10 CI iterations per PORTING_MAP policy, then lane parks with failures logged in map. Never merge red. No `unimplemented!/todo!` left; no dead Stub structs; no new deps.
- Handoff: write `notes/agent-2/2026-10-01-done-plugin.md` (same header, `status: done`, 5 files + pasted CI links/output), commit, push `agent-2/plugin-api` (never `master`), leader merges `--no-ff` after CI green + parity review.

## Todo tracker

- [x] claim pushed
- [x] lane plan (this file)
- [ ] ack received → implement api→slots→command_shim→runtime→adapters (source order)
- [ ] self-audit (attendance + spot-checks quoted above)
- [ ] push → CI green → done-note → merge request
