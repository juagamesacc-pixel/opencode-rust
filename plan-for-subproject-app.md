# Plan for subproject: app (packages/app -> crates/app) — porting 1:1, zero diversion
Source pin: v1.18.30 @3104c14, Bun 1.3.14, TS 5.8.2. Target: /root/opencode-rust/crates/app/ (ABSENT greenfield, version 1.18.30, edition 2021). Doctrine: source is spec; no improvements/renames/merges/reordering; mirror 1:1 (one Rust module per source file/dir, same names snake_cased only where Rust forbids identifier — record each rename).

## 1. Research gate (pinned + verified)
- Pins: opencode v1.18.30 @3104c14; Bun 1.3.14; TS 5.8.2 (from compartment brief, no re-pin).
- Crate versions: use ONLY std + crates already in workspace lockfiles (verify via read-only inspection of /root/opencode-rust/Cargo.lock + crates/server/Cargo.toml dependency style; e.g. serde/serde_json if already locked — DO NOT introduce new versions without lockfile evidence + user consent).
- Precedent verified read-only: /root/opencode-rust/crates/server/src/cors.rs exists (lib.rs + per-module + tests/ + provisional stubs pattern); /root/opencode-rust/crates/server/Cargo.toml = dependency-style reference; /root/opencode-rust/Cargo.toml workspace members + /root/opencode-rust/PORTING_MAP.md L1-60 row format flagged for implementer read-only verification (TOML/MD not parseable via AST tools in-session).
- Local time Asia/Kolkata: 2026-09-12 ~09:26 IST (outside 12AM-6AM free window) => ZERO downloads expected; no crates.io fetch, no bun/npm fetch, no new tools. Explicit user consent required before any download; never wait_for_user — questions to main orchestrator.
- Package metadata (read for mapping only): package.json, vite.config.ts, index.html — not ported as code; record entry points, path alias @/ -> crate root, build-only shims.

## 2. Architecture + full file map source->target
- Rule: packages/app/src/<path>.ts(x) -> crates/app/src/<same_path_snake>.rs (dirs become mod.rs + per-file modules); .css/assets/help -> passthrough (include_str!/static assets, no logic change); i18n/*.ts + *.json -> src/i18n/*.rs data tables (en source of truth; de/hu/hr/tr/zh/zht import en + desktop-native keys preserved); *.test.ts -> crates/app/tests/<same>_test.rs integration tests 1:1.
- Approved SolidJS pattern (no Rust equivalent in workspace): data/state/types -> serde structs + explicit state machines; components/pages (.tsx) -> Rust structs modeling props/state + render-descriptor struct (field-for-field, no invented rendering/DOM); signals/memos/effects/createStore/persisted -> explicit new()/update()/transition fns + Persist keys preserved; router/navigation -> route-key structs + href builders (session-route preserved); workers/wasm/shiki/ghostty-web/@pierre/trees/tanstack-solid-*/solid-primitives/tauri/electron/vite-only -> PROVISIONAL stubs flagged `// PROVISIONAL: pending <crate> — mirrors <src path>` following crates/server provisional-stub pattern.
- Rename log: only snake_casing (e.g. prompt-input -> prompt_input, settings-v2 -> settings_v2, file-tree-v2-model -> file_tree_v2_model) + hyphen->underscore; any other rename requires diversion note (must be zero).
- Map table (implementer MUST verify exhaustive via read-only `find packages/app/src -type f | sort`; below covers brief-mandated + discovered representatives; rule covers ALL):
  src/app.tsx -> src/app.rs | src/entry.tsx -> src/entry.rs | src/index.ts -> src/index.rs | src/updater.ts -> src/updater.rs | src/desktop-menu.ts -> src/desktop_menu.rs | src/theme-preload.test.ts -> tests/theme_preload_test.rs | src/desktop-menu.test.ts -> tests/desktop_menu_test.rs
  src/addons/* (incl serialize.ts+serialize.test.ts) -> src/addons/* | src/assets/help/* -> src/assets/help/* (passthrough) | src/constants/* (file-picker.ts etc) -> src/constants/* | src/components/prompt-input/* (submit, build-request-parts, history, history-store, attachments, files, paste, editor-dom, placeholder, submission-state, transient-state + *.test.ts) -> src/components/prompt_input/* | src/components/server/* -> src/components/server/* | src/components/session/* (session-context-*) -> src/components/session/* | src/components/settings-v2/* (general-controllers, general-controller-behavior + test) -> src/components/settings_v2/* | src/components/ui/* -> src/components/ui/* (descriptor structs) | remaining src/components/* (command-palette, titlebar-*, directory-picker*, file-tree*, updater-action, edit-project, dialog-*, status-popover-*, pierre-tree, virtual-scroll-element + *.test.ts) -> src/components/* 1:1
  src/context/file/* (watcher, view-cache, tree-store, content-cache, path) -> src/context/file/* | src/context/global-sync/* (utils, types, session-trim/load/cache, eviction, queue, bootstrap, event-reducer, child-store, home-session-index, mcp + *.test.ts) -> src/context/global_sync/* | remaining src/context/* (command, language, server, layout*, tabs, sync, sdk, prompt*, permission*, models, mcp, local*, notification, terminal*, etc + *.test.ts incl tabs, server, layout, prompt-state, server-sdk, server-sync, permission-auto-respond, model-variant, settings, local-agent) -> src/context/* 1:1
  src/hooks/* (use-providers, provider-catalog + test) -> src/hooks/* | src/i18n/* (en + de/hu/hr/tr/zh/zht + desktop-native + *.test.ts) -> src/i18n/* | src/pages/home/*, src/pages/layout/*, src/pages/new-session/*, src/pages/session/**/* (composer/*, timeline/*, v2/* + dozens *.test.ts) -> src/pages/* 1:1 | src/utils/* (server, server-scope/health/errors/protocol/compat, session*, persist, id, uuid, base64, draft-store, toast, sound, etc + *.test.ts) -> src/utils/* 1:1 | src/wsl/* (settings-model, add-server-probes + test) -> src/wsl/* | package.json/vite.config.ts/index.html -> docs mapping only (no .rs).
- Required literals (include verbatim in manifests):
  workspace Cargo member entry: "crates/app"
  PORTING_MAP.md replacement row for packages/app/src: `| packages/app/src | crates/app/src | SolidJS+Vite frontend v1.18.30 @3104c14 — 1:1 modules, PROVISIONAL stubs for solid-js/tauri/vite-electron/wasm | |` (adjust columns to match L1-60 format verified read-only; preserve row order, no reorder).

## 3. Phased implementation sequence (parallel leaf lanes, DISJOINT write scopes)
- Phase 0 gate: read-only `find` inventory + precedent re-read (server lib.rs/Cargo.toml, PORTING_MAP L1-60, workspace Cargo.toml); create crates/app/Cargo.toml + src/lib.rs skeleton (mod declarations only) — single writer.
- Lane A (roots+constants+utils+wsl): src/{app,entry,index,updater,desktop_menu}.rs + constants/* + utils/* + wsl/* + package.json/vite mapping doc. Scope disjoint from B/C/D.
- Lane B (components/*): components/{prompt_input,server,session,settings_v2,ui} + remaining components/*. No touch of context/hooks/pages.
- Lane C (context+hooks+i18n): context/{file,global_sync} + context/* + hooks/* + i18n/*. No touch of components/pages.
- Lane D (pages+addons+assets+tests): pages/{home,layout,new-session,session} + addons/* + assets/help passthrough + tests/*. No touch of A/B/C scopes.
- Each lane: port types->structs first, then pure fns (verified by colocated tests), then state machines, then PROVISIONAL stubs for UI/reactive deps; record renames + PROVISIONAL list per file.
- Join gate: lib.rs mod tree compiles conceptually; PORTING_MAP row + workspace member line prepared as text (DO NOT EDIT Cargo.toml/PORTING_MAP.md in lanes — orchestrator applies).
- Dependency graph: A (leaf utils) -> C (context uses utils) -> B/D (components/pages use context); but lanes run parallel as stubs-first then fill (no cross-lane file writes).

## 4. Parity/audit verification checklist
- [ ] Artifact attendance: every src file has 1:1 target (find count match, assets/i18n JSON included, no merges/splits).
- [ ] Behavior equivalence: every exported fn/type/const ported; pure-logic tests (desktop-menu, theme-preload + ALL *.test.ts) ported 1:1 under crates/app/tests/ with same assertions.
- [ ] No diversion: no renames beyond snake_case, no reordering, no UI redesign, no modernization; each PROVISIONAL stub cites source path + pending crate.
- [ ] Rename log + PROVISIONAL list complete; rulebook porting mandates pass.

## 5. Build/test gate (CI owns)
- Local Rust (/root/.cargo/bin) CHECKUPS ONLY: rustfmt on own files, read-only inspection (e.g. `cargo fmt --check`, file reads). NEVER cargo build/test/clippy locally.
- CI owns cargo build/test/clippy + bun tests parity; lanes must leave code rustfmt-clean.

## Operator rules (recorded, do not violate)
(a) local Rust checkups only — rustfmt own files, read-only inspection; NEVER cargo build/test/clippy locally — CI owns it. (b) downloads 12AM-6AM IST free else explicit user consent — now ~09:26 IST so ZERO downloads; never wait_for_user (questions to main orchestrator). Write scope: only plan file /root/opencode-rust/plan-for-subproject-app.md.
