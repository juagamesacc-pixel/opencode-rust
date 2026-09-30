# Plan-for-desktop — 1:1 Rust port of `@opencode-ai/desktop` v1.18.30 (Electron main/preload/renderer)

> Project root: `/root/opencode-rust` | Crate: `crates/desktop` (partially populated — **continue, do not restart**)
> Source spec: `anomalyco/opencode v1.18.30`, `packages/desktop/src` (Electron 41.2 / Node 24.14.1 / Bun 1.3.x / TS 5.8.2) | Target: `Rust 1.98.1`, edition 2021
> Doctrine: ZERO-DIVERSION 1:1 exact translation — same names/signatures/behavior/edge-cases/error-strings/keys/defaults/ordering. Source is spec. No improvement/modernization/rename-for-taste/merge/split/reorder. No scope change without approval.
> Allowed deps: `serde 1` + `serde_json 1` only (both already present in the workspace `Cargo.lock`; **no downloads, no new crates**).
> **Download rule (recorded):** no downloads performed by this lane. Nothing added to `/root/opencode-rust/Cargo.toml` (workspace membership is the orchestrator's job). Build/test deferred to CI (local link toolchain is broken).
> Lane: DESKTOP PORT (`crates/desktop`). Not a `@designer` task: no layout/styling/UX work — CSS/HTML are byte-passthrough assets.

---

## 1. Research gate (no web research needed)

1.1 The source tree is the spec. Every file was re-read in full before mapping (63 non-i18n files + `renderer/i18n/index.ts` + 60 locale tables + 13 `*.test.ts`).
1.2 Normative boundaries treated as out-of-scope-by-dependency (see §5 PROVISIONAL registry and §6 deps):
- **Electron runtime** (`electron`, `electron-store`, `electron-log`, `electron-updater`, `electron-window-state`, `electron-context-menu`, `@zip.js/zip.js`).
- **Node runtime** (`node:fs`, `node:path`, `node:os`, `node:child_process`, `node:crypto`, `node:url`, `node:tls`, `node:net`, `node:http`, `node:sqlite`).
- **Native addons** (`@lydell/node-pty`).
- **Effect runtime** (`Deferred`/`Effect`/`Fiber`).
- **Cross-package app types** (`@opencode-ai/app/{updater,desktop-menu,wsl/types,i18n/desktop-native}`) — defined in the `app` package, not in `packages/desktop/src`.
- **DOM/Solid runtime** (`solid-js`, `solid-js/web`, `@solidjs/router`, `@sentry/solid`, `@solid-primitives/i18n`).
1.3 Pre-implementation re-read checklist (mandatory before each file): header, exports, error strings, defaults, ordering. Recorded in each generated file's `// source: packages/desktop/<path>` header.

## 2. Full artifact inventory FIRST (attendance baseline)

`packages/desktop/src` = **125 files** (62 `.ts`/`.tsx` code+test, 1 `.ts` i18n index, 60 locale `.ts`, 1 `.css`, 1 `.html`, 2 `env.d.ts`).

### 2.1 `src/main/` — 47 files

| # | Source file | LOC | Status |
|---|---|---|---|
| 1 | `main/apps.ts` | 136 | done (pure helpers) + PROVISIONAL (fs/exec probes) |
| 2 | `main/attachment-picker.ts` | 57 | done (budget + authorizations) + PROVISIONAL (`readAttachment` fs) |
| 3 | `main/background-cli.ts` | 125 | done (pure helpers) + PROVISIONAL (exec/fs) |
| 4 | `main/constants.ts` | 7 | done (`Channel`, `CHANNEL`) + PROVISIONAL (`UPDATER_ENABLED`) |
| 5 | `main/debug.ts` | 95 | done (`focusableSelector`, response readers) + PROVISIONAL (CDP send) |
| 6 | `main/desktop-menu-actions.ts` | 84 | done (full action switch over window trait) |
| 7 | `main/draft-store.ts` | 82 | done (pending/flush/GC algorithm) + PROVISIONAL (sqlite) |
| 8 | `main/env.d.ts` | 19 | done (ambient decls → doc'd type mirrors, no runtime code) |
| 9 | `main/external-url.ts` | 19 | done |
| 10 | `main/index.ts` | 424 | PROVISIONAL (Effect + Electron bootstrap) |
| 11 | `main/initialization.ts` | 6 | PROVISIONAL (Effect `Deferred` combinator) |
| 12 | `main/install-state.ts` | 8 | done |
| 13 | `main/ipc.ts` | 308 | done (channel names, `pickerFilters`) + PROVISIONAL (handlers) |
| 14 | `main/logging.ts` | 210 | done (constants, `safeLogName`, manifest fields) + PROVISIONAL (transports/zip) |
| 15 | `main/menu.ts` | 69 | done (template build) + PROVISIONAL (`Menu.setApplicationMenu`) |
| 16 | `main/migrate.ts` | 91 | done (`TAURI_*`, `tauriDir`) + PROVISIONAL (fs) |
| 17 | `main/native-translations.ts` | 24 | PROVISIONAL (bundle owned by `app` package) |
| 18 | `main/onboarding.ts` | 45 | PROVISIONAL (store/fs/`app.getPath`) |
| 19 | `main/server.ts` | 239 | done (constants, URL construction) + PROVISIONAL (spawn/fetch) |
| 20 | `main/shell-env.ts` | 101 | done (parse/resolve/nushell/merge) + PROVISIONAL (`probe` spawnSync) |
| 21 | `main/sidecar.ts` | 157 | done (`parseCommand`, `serializeError`) + PROVISIONAL (parent port) |
| 22 | `main/store-cleanup.ts` | 94 | done (kind/empty/retention/recency algorithm) + PROVISIONAL (fs) |
| 23 | `main/store-keys.ts` | 7 | done (7 consts) |
| 24 | `main/store.ts` | 35 | PROVISIONAL (`electron-store`) |
| 25 | `main/unresponsive.ts` | 70 | done (constants, sample message) + PROVISIONAL (timers/CDP) |
| 26 | `main/updater-controller.ts` | 97 | done (full state machine) |
| 27 | `main/updater-subscriptions.ts` | 20 | done |
| 28 | `main/updater.ts` | 94 | PROVISIONAL (`electron-updater` + `dialog`) |
| 29 | `main/window-registry.ts` | 47 | done |
| 30 | `main/window-state.ts` | 29 | done |
| 31 | `main/windows.ts` | 564 | done (pure helpers: overlay/clamp/paths/URL policy/header upsert) + PROVISIONAL (`BrowserWindow`) |
| 32–38 | `main/wsl/{ipc,policy,runtime,servers,sidecar,startup}.ts` | 105/33/405/523/134/37 | see §2.2 |
| 39–47 | 9 × `main/*.test.ts` | — | done as `#[cfg(test)]` modules |

### 2.2 `src/main/wsl/` — 7 files

| # | Source file | LOC | Status |
|---|---|---|---|
| 38a | `wsl/ipc.ts` | 104 | done (channel names, unavailable state) + PROVISIONAL (`ipcMain`) |
| 38b | `wsl/policy.ts` | 33 | done |
| 38c | `wsl/runtime.ts` | 405 | done (`wslArgs`, `shellEscape`, `firstLine`, `summarize`, `parseInstalledDistros`, `parseOnlineDistros`, `detectOutputEncoding`, all script literals) + PROVISIONAL (`spawn`, `pty.spawn`, Powershell exec) |
| 38d | `wsl/servers.ts` | 523 | done (`wslServerIdForDistro`, `initialState`, `normalizePersistedServer`, `opencodeCheck`, `distroProbeReady`, `startupFailure`, store key read/write shape) + PROVISIONAL (async controller) |
| 38e | `wsl/sidecar.ts` | 134 | done (bash script builder, `startupFailure`) + PROVISIONAL (`spawn`, port alloc, health poll) |
| 38f | `wsl/startup.ts` | 37 | done (`wslServerIdsToStartOnInitialize`, `expectOpencodeVersion`, `pendingRestartAfterWslInstall`) + PROVISIONAL (`pollWslHealth` timers) |
| 38g | `wsl/servers.test.ts` | 232 | done (pure assertions; async controller cases PROVISIONAL-skipped) |

### 2.3 `src/preload/` — 2 files

| # | Source file | LOC | Status |
|---|---|---|---|
| 39 | `preload/types.ts` | 116 | done (all types mirrored; cross-package types mirrored locally) |
| 40 | `preload/index.ts` | 138 | done (updater callback registry semantics) + PROVISIONAL (`ipcRenderer` channel wiring) |

### 2.4 `src/renderer/` — 10 files + `i18n/`

| # | Source file | LOC | Status |
|---|---|---|---|
| 41 | `renderer/env.d.ts` | 10 | done (ambient decls → doc'd mirror) |
| 42 | `renderer/index.html` | — | **already present** `src/assets/index.html` (byte-passthrough) |
| 43 | `renderer/styles.css` | — | **already present** `src/assets/styles.css` (byte-passthrough) |
| 44 | `renderer/index.tsx` | 452 | done (pure helpers: `windowLastActiveUrlKey`, `getLastActiveUrl` validation, `setLastActiveUrl` guard, `createPlatform` shape, `emitDeepLinks` semantics) + PROVISIONAL (Solid render tree, Sentry, `window.api`) |
| 45 | `renderer/onboarding.tsx` | 54 | PROVISIONAL (Solid context/tabs/server) |
| 46 | `renderer/initialization.ts` | 22 | done (full) |
| 47 | `renderer/webview-zoom.ts` | 138 | done (constants, `clamp`, `normalizeWheelDelta`, pinch state machine) + PROVISIONAL (DOM listeners, `window.api`) |
| 48 | `renderer/window-fullscreen.ts` | 8 | done (signal state) + PROVISIONAL (`window.api` wiring) |
| 49 | `renderer/cli.ts` | 12 | PROVISIONAL (`window.alert`, `window.api.installCli`) |
| 50 | `renderer/wsl/connections.ts` | 28 | done (full) |
| 51 | `renderer/html.test.ts` | 63 | done (asset assertions against `src/assets/index.html`) |
| 52 | `renderer/initialization.test.ts` | 74 | done |
| 53 | `renderer/wsl/connections.test.ts` | 48 | done |
| 54 | `renderer/i18n/index.ts` | 211 | done (`Locale`, `parse_locale`, `parse_record`, `parse_stored`, `pick_locale`, `build`, `detect_locale`, `t` template resolution) + PROVISIONAL (`navigator`, `window.api.storeGet`) |
| 55 | `renderer/i18n/{60 locale}.ts` | — | **already present** `src/renderer/i18n/{60}.rs` — verified byte-faithful (60/60 key+value match vs `.ts`); **kept unchanged** |

### 2.5 Out-of-`src` (recorded, not ported as code)

`packages/desktop/{package.json,tsconfig.json,scripts/utils.ts,resources/linux/*.desktop,icons/**,electron.vite.config.ts}` — build/packaging/assets only; recorded in the inventory, no `.rs` generated. `html.test.ts`'s `publicDir` assertion is therefore PROVISIONAL-skipped with the reason recorded in the test module.

## 3. 1:1 TS→Rust mapping (file-path mirror)

`packages/desktop/src/<a>/<b>.ts` → `crates/desktop/src/<a>/<b>.rs`; `-` in a TS filename becomes `_` in the Rust filename (Rust module-name rule only), module name keeps the source base name in its `//!` header.

```
packages/desktop/src/
├── main/index.ts                    → src/main/index.rs                    (PROVISIONAL)
├── main/initialization.ts           → src/main/initialization.rs           (PROVISIONAL)
├── main/constants.ts                → src/main/constants.rs
├── main/store-keys.ts               → src/main/store_keys.rs
├── main/store.ts                    → src/main/store.rs                    (PROVISIONAL)
├── main/store-cleanup.ts            → src/main/store_cleanup.rs
├── main/draft-store.ts              → src/main/draft_store.rs
├── main/install-state.ts            → src/main/install_state.rs
├── main/external-url.ts             → src/main/external_url.rs
├── main/shell-env.ts                → src/main/shell_env.rs
├── main/apps.ts                     → src/main/apps.rs
├── main/onboarding.ts               → src/main/onboarding.rs               (PROVISIONAL)
├── main/migrate.ts                  → src/main/migrate.rs
├── main/server.ts                   → src/main/server.rs
├── main/sidecar.ts                  → src/main/sidecar.rs
├── main/background-cli.ts           → src/main/background_cli.rs
├── main/logging.ts                  → src/main/logging.rs
├── main/menu.ts                     → src/main/menu.rs
├── main/ipc.ts                      → src/main/ipc.rs
├── main/native-translations.ts      → src/main/native_translations.rs      (PROVISIONAL)
├── main/updater.ts                  → src/main/updater.rs                  (PROVISIONAL)
├── main/updater-controller.ts       → src/main/updater_controller.rs
├── main/updater-subscriptions.ts    → src/main/updater_subscriptions.rs
├── main/window-registry.ts          → src/main/window_registry.rs
├── main/window-state.ts             → src/main/window_state.rs
├── main/windows.ts                  → src/main/windows.rs
├── main/unresponsive.ts             → src/main/unresponsive.rs
├── main/desktop-menu-actions.ts     → src/main/desktop_menu_actions.rs
├── main/attachment-picker.ts        → src/main/attachment_picker.rs
├── main/debug.ts                    → src/main/debug.rs
├── main/env.d.ts                    → (no runtime artifact) doc'd mirrors in src/main/mod.rs
├── main/wsl/{mod→mod.rs, ipc, policy, runtime, servers, sidecar, startup}.ts → src/main/wsl/*.rs
├── preload/index.ts                 → src/preload/index.rs
├── preload/types.ts                 → src/preload/types.rs
├── renderer/index.tsx               → src/renderer/index.rs
├── renderer/onboarding.tsx          → src/renderer/onboarding.rs           (PROVISIONAL)
├── renderer/initialization.ts       → src/renderer/initialization.rs
├── renderer/webview-zoom.ts         → src/renderer/webview_zoom.rs
├── renderer/window-fullscreen.ts    → src/renderer/window_fullscreen.rs
├── renderer/cli.ts                  → src/renderer/cli.rs                  (PROVISIONAL)
├── renderer/wsl/connections.ts      → src/renderer/wsl/connections.rs
├── renderer/env.d.ts                → (no runtime artifact) doc'd mirrors in src/renderer/mod.rs
├── renderer/index.html              → src/assets/index.html                (byte-passthrough, present)
├── renderer/styles.css              → src/assets/styles.css                (byte-passthrough, present)
└── renderer/i18n/index.ts           → src/renderer/i18n/mod.rs
    renderer/i18n/<locale>.ts        → src/renderer/i18n/<locale>.rs         (present ×60, kept)
```

`*.test.ts` never becomes a separate file: each becomes a `#[cfg(test)] mod tests` block at the bottom of the module it tests (same `describe`/`test` names, same assertions), so the 1:1 file mapping above stays exact. `main/wsl/servers.test.ts` covers three source modules, so its assertions are placed in the module each assertion targets (`policy` / `startup` / `servers`) — recorded here so the mapping is auditable.

### 3.1 Naming convention (recorded once, not a per-file decision)

- Exported items keep the source identifier in `snake_case` (`hasExistingAppState` → `has_existing_app_state`, `CHANNEL` → `CHANNEL`, `MAX_ATTACHMENT_BYTES` → `MAX_ATTACHMENT_BYTES`, `Type` → `Type`).
- Types that collide with a Rust primitive keep the source name with a `State`/`Config` suffix **only** where the source itself distinguishes them; otherwise the source name is kept verbatim (`WslServerRuntime`, `TitlebarTheme`, `ServerReadyData`).
- IPC/channel strings, i18n keys, error message strings, env var names, store keys, and file-name templates are **byte-identical** to the source.

### 3.2 Cross-package type mirrors (no dependency on `crates/app`)

`packages/app/src/{updater,desktop-menu,wsl/types,i18n/desktop-native}.ts` are outside this lane's READ/WRITE scope, so their shapes that `packages/desktop/src` consumes are mirrored as local types in `src/preload/types.rs` (`UpdaterState`, `DesktopMenuAction`, `WslServerConfig`, `WslServerItem`, `WslServerRuntime`, `WslServersState`, `WslServersEvent`, `WslJob`, `WslRuntimeCheck`, `WslInstalledDistro`, `WslOnlineDistro`, `WslDistroProbe`, `WslOpencodeCheck`, `DesktopNativeBundle`). Each carries a `// PROVISIONAL(packages/app/src/...): canonical definition lives in the app package` header. The `nativeT` message *templates* are owned by the app package and are therefore **not** duplicated here.

## 4. Implementation sequence (dependency order)

- **Phase 0** — `Cargo.toml` + `src/lib.rs` barrel + directory `mod.rs` files + `src/renderer/i18n/mod.rs`. Gate G0: every file in §3 exists.
- **Phase 1 — pure logic / app state (no platform):** `store_keys` → `install_state` → `window_state` → `window_registry` → `updater_subscriptions` → `updater_controller` → `external_url` → `shell_env` → `store_cleanup` → `attachment_picker` → `draft_store` → `desktop_menu_actions` → `apps` → `constants` → `migrate` → `background_cli` → `debug` → `sidecar` → `unresponsive` → `server`. Gate G1: 0 unresolved `crate::` paths (grep).
- **Phase 2 — WSL:** `wsl/policy` → `wsl/startup` → `wsl/runtime` → `wsl/servers` → `wsl/sidecar` → `wsl/ipc` → `renderer/wsl/connections`. Gate G2: parsers + policy/state assertions 1:1.
- **Phase 3 — i18n + renderer:** `renderer/i18n/mod.rs` → `renderer/initialization` → `renderer/webview_zoom` → `renderer/window_fullscreen` → `renderer/cli` → `renderer/index` → `renderer/onboarding`. Gate G3: locale table count = 60, `build()` chain order preserved.
- **Phase 4 — Electron main + preload:** `main/native_translations` → `main/logging` → `main/menu` → `main/store` → `main/onboarding` → `main/updater` → `main/windows` → `main/ipc` → `preload/types` → `preload/index` → `main/initialization` → `main/index`. Gate G4: IPC channel strings diff 1:1 against `main/ipc.ts` + `preload/index.ts` + `wsl/ipc.ts`.
- **Phase 5 — tests** inlined per §3, then audit + `rustfmt`.

## 5. PROVISIONAL registry

Every entry is a module-level or item-level `// PROVISIONAL(<source path>): <reason>` comment. Bodies are `unimplemented!()` (or return the source's zero value) so signatures and names stay 1:1.

### 5.1 Runtime bindings (no in-workspace Rust equivalent)

| Tag reason | Affected modules |
|---|---|
| `no in-workspace Rust binding for the Electron runtime` | `main/index.rs` (all `app`/`BrowserWindow`/`Effect` bootstrap), `main/ipc.rs` (`ipcMain.handle/on`), `main/windows.rs` (`BrowserWindow`, `protocol`, `net`, `nativeTheme`, `nativeImage`, `shell`, `dialog`), `main/menu.rs` (`Menu.setApplicationMenu/buildFromTemplate`), `main/unresponsive.rs` (CDP `collectJavaScriptCallStack`), `main/debug.rs` (`contents.debugger.sendCommand`), `main/updater.rs` (`autoUpdater`, `dialog.showMessageBox`), `main/logging.rs` (`crashReporter`, `netLog`, `electron-log` transports), `main/servers.ts`→`main/server.rs` (`utilityProcess.fork`), `main/sidecar.rs` (`process.parentPort`), `main/wsl/ipc.rs`, `preload/index.rs` (`ipcRenderer`, `contextBridge`, `webUtils`) |
| `no in-workspace Rust binding for the Node filesystem/process runtime` | `main/store.rs`, `main/store_cleanup.rs` (fs half), `main/onboarding.rs`, `main/migrate.rs` (fs half), `main/apps.rs` (fs/exec half), `main/background_cli.rs` (fs/exec half), `main/draft_store.rs` (sqlite half), `main/attachment_picker.rs` (`readAttachment` fs half), `main/shell_env.rs` (`probe` spawnSync half), `main/wsl/runtime.rs` (spawn/pty/PowerShell half), `main/wsl/sidecar.rs` (spawn/port/health half), `main/wsl/servers.rs` (store + async controller half), `main/server.rs` (`checkHealth` HTTP half) |
| `no in-workspace Rust binding for the Effect runtime` | `main/initialization.rs`, `main/index.rs` (`Effect.gen`/`Deferred`/`Fiber`) |
| `no in-workspace Rust binding for the native node-pty addon` | `main/wsl/runtime.rs` (`runInteractiveCommand`) |
| `no in-workspace Rust binding for the Solid/DOM runtime` | `renderer/index.rs` (render tree, Sentry), `renderer/onboarding.rs`, `renderer/webview_zoom.rs` (DOM listeners), `renderer/window_fullscreen.rs` (signal wiring), `renderer/cli.rs` (`window.alert`) |
| `no in-workspace Rust binding for @solid-primitives/i18n` | `renderer/i18n/mod.rs` (`flatten`/`translator`/`resolveTemplate` — behaviour reproduced with `{{name}}` substitution, see §5.2) |
| `canonical definition lives in the app package (out of this lane's scope)` | `main/native_translations.rs`, cross-package type mirrors in `preload/types.rs` |
| `build/packaging configuration, not runtime code` | `renderer/html.test.rs` `publicDir` assertion, `main/env.d.ts`, `renderer/env.d.ts`, `package.json`, `tsconfig.json`, `scripts/utils.ts`, `resources/`, `icons/`, `electron.vite.config.ts` |
| `spawns child processes / allocates ports; needs an async runtime` | `wsl/servers.rs` controller methods, `wsl/startup.rs` `poll_wsl_health` |

### 5.2 Documented reproductions (not PROVISIONAL, but boundary-noted)

- `external_url.rs` — `URL.canParse` + `new URL(...).href` are reproduced by a focused WHATWG-subset parser (scheme split, http/https empty-path → `/` normalization, `mailto:` passthrough, `file:` host check, percent-decoding of the file pathname). This covers the source's whole reachable surface in `external-url.ts`; the source's `fileURLToPath` Windows drive-letter branch is reproduced. Noted in the module header.
- `updater_controller.rs` — the source's `Promise` chains are sequential, so the port is a synchronous state machine with the identical transition order (`idle → checking → downloading → ready`), identical `pending` coalescing, identical `.finally` reset, and identical `install()` `stop → quitAndInstall → ready` / `stop-failure → ready + rethrow` behavior. Noted in the module header.
- `draft_store.rs` — the pending-write map, 500 ms debounce scheduling decision, GC of unreferenced blobs (sha256 ids collected by walking `document` values) are reproduced; only the SQL execution is PROVISIONAL.
- `store_cleanup.rs` — the retention/recency/emptiness decision logic is reproduced over an injected entry snapshot so it is unit-testable without `fs`; only the readdir/stat/read/rm calls are PROVISIONAL.
- `main/windows.rs` — `overlay`, `clamp_zoom`, `window_state_file`, `window_data_file`, `is_renderer_url`, `upsert_key_value`, `add_renderer_headers` reproduced; `BrowserWindow` wiring PROVISIONAL.

## 6. Required-deps registry

**None added.** `crates/desktop/Cargo.toml` declares only `serde = "1"` (with `derive`) and `serde_json = "1"`, both already resolved in `/root/opencode-rust/Cargo.lock`. Serialization is `serde`/`serde_json` only, per doctrine.

Deps that *would* be required for full de-PROVISIONALization — **listed, not added, no download requested**:

| Needed dep | Unblocks | Present in workspace lock? |
|---|---|---|
| `rusqlite` (bundled) | `main/draft_store.rs` sqlite half | yes (locked) but feature set not enabled by the workspace → **not added** |
| `tokio` (rt, macros, process, fs, time, net) | async controller/HTTP/subprocess ports (`wsl/servers`, `wsl/sidecar`, `wsl/runtime`, `main/server`, `main/shell_env`) | yes (locked) → **not added** |
| `sha2` | `draft_store` `putBlob` id (currently sha256 is PROVISIONAL) | no → would require a download |
| `tauri` | the intended replacement runtime for BrowserWindow/ipcMain/dialog/shell/menu/tray | no → would require a download |
| `portable-pty` / `ptyprocess` | `wsl/runtime.rs` `runInteractiveCommand` | no → would require a download |
| `zip` | `main/logging.rs` `writeZip` | no → would require a download |
| `chrono` / `time` | `main/logging.rs` `stamp()` ISO-8601 formatting | no → would require a download |
| `url` | `external_url.rs` WHATWG parser | yes (locked) → **not added** (focused std parser used, §5.2) |
| `regex` | `store_cleanup`/`install_state` `window-state-*.json` pattern | no → would require a download (std matcher used) |

No download is genuinely required to finish this lane: every module is either fully ported with std or explicitly PROVISIONAL. If the orchestrator approves any of the above, they must be added to the **workspace** `Cargo.toml`/lock by the orchestrator, not by this lane.

## 7. Verification gates

- **V1 attendance** — every file in §2 exists at its §3 path (automated: file-list diff against the source tree; 125 source artifacts accounted for).
- **V2 string preservation** — grep-diff of IPC channel names (`kill-sidecar` … `run-desktop-menu-action`, `wsl-servers-*`), store keys (`opencode.settings`, `defaultServerUrl`, `firstLaunchOnboardingComplete`, `oldLayoutEligible`, `wslServers`, `pinchZoomEnabled`, `windowIds`), env vars (`OPENCODE_*`, `XDG_STATE_HOME`, `NO_PROXY`/`no_proxy`, `APPDATA`, `SystemRoot`/`windir`, `ELECTRON_RENDERER_URL`), i18n keys, file-name templates (`window-state-…json`, `opencode.window.…dat`, `opencode.draft.…dat`, `opencode.workspace.…dat`, `tauriMigrated`, `ready`), numeric defaults (`20 * 1024 * 1024`, `5_000`, `6_000`, `60_000`, `10 * 60 * 1000`, `128`, `30 * 24 * 60 * 60 * 1000`, `100`, `0.2`, `10`, `40`, `14`, `1280`, `800`, `1000`, `15000`, `12`, `20000`, `900000`, `16`, `20`, `160`, `5 MiB`, `50 MiB`, `20 MiB`, `7 days`, `24 h`, `24 h`), and error strings (`Invalid ${name}`, `Invalid native translation sender`, `Invalid native translation bundle`, `Window not found`, `Window ID not found`, `Update is not ready to install`, `Invalid DOM.getDocument response`, `Invalid DOM.querySelectorAll response`, `Sidecar did not become ready within …`, `Sidecar exited before ready with code …`, `Sidecar exited before health check passed with code …`, `Failed to get port`, `Sidecar parent port unavailable`) 1:1.
- **V3 behavior equivalence** — 13 `*.test.ts` ported as `#[cfg(test)]` modules with the same `describe`/`test` names and assertions; locale tables byte-compared against the `.ts` sources.
- **V4 doctrine audit** — no rename/merge/split/reorder beyond the recorded §3.1 convention; no invented functionality; every deviation carries a PROVISIONAL tag or a note `diversion`.
- **V5 code consistency** — zero unresolved `crate::`/`super::` references; every `pub mod` in a `mod.rs` resolves to a real file; no `unimplemented!()` reachable from a non-PROVISIONAL pure function.
- **V6 formatter** — `rustfmt --edition 2021 --check` over `crates/desktop/src`. Build/test are **not** run by this lane (broken local linker; CI owns builds).

## 8. Deferred build/test-and-fix gate (NOT executed)

- When authorized: `cargo build -p desktop` / `cargo test -p desktop`; fix-and-retest while preserving 1:1; record versions/log/failures→fixes.
- Self-audit gate on that pass: re-run V1–V6 and confirm `git status` shows ONLY `crates/desktop/**` + this plan file (workspace `Cargo.toml` and `PORTING_MAP.md` belong to the orchestrator).

## 9. PORTING_MAP + attestation (orchestrator-owned, deferred)

- The `desktop` row of `/root/opencode-rust/PORTING_MAP.md` (`| packages/desktop/src | crates/desktop/ | pending — Electron native |`) is **not** edited by this lane.
- Attestation to hand up: source commit + `1.18.30`, artifact counts (§2), module map (§3), PROVISIONAL count (§5), deps list (§6), formatter result (V6), residual risks (§10).

## 10. Residual risks (flagged, not silently resolved)

- **R1** Electron surface parity: `BrowserWindow`/`ipcMain`/`dialog`/`shell`/`Menu`/`protocol`/`net`/`nativeTheme`/`crashReporter`/`netLog`/`utilityProcess`/`clipboard`/`webUtils`/`contextBridge` are PROVISIONAL. Any future non-Electron runtime must reproduce: window restore, titlebar overlay, `oc://` privileged protocol, CSP-ish header injection, zoom clamping, permission allow-list, and the single-instance/deep-link flow.
- **R2** Effect semantics: `main/initialization.ts` (`Deferred` failure forwarding) and the `Effect.gen` bootstrap in `main/index.ts` are PROVISIONAL; the two `index.test.ts` cases (failure forwarded before/at renderer wait) therefore have no executable Rust equivalent — recorded as skipped-with-reason, not silently dropped.
- **R3** Async ordering: `wsl/servers.ts` job/attempt bookkeeping (start-attempt invalidation, stale background checks, parallel `probeAddable`) is inherently concurrent; the pure helpers are ported 1:1 but the controller's interleaving is PROVISIONAL. The four async `servers.test.ts` cases are recorded as skipped-with-reason.
- **R4** Native-copy fidelity: `nativeT` message templates (WSL errors, recovery dialogs, picker errors, updater dialogs) are owned by the app package. The desktop lane consumes them through the `DesktopNativeBundle` mirror; **if the app-side bundle keys drift, desktop strings drift silently.** The exact 44-key set desktop references is enumerated as `DESKTOP_NATIVE_KEYS` in `main/native_translations.rs` so drift is detectable by diffing that list.
- **R5** `URL.canParse` subset: a pathological non-WHATWG input could parse differently in the std subset parser than in Bun's `URL`. The source's own tests are covered; the boundary is documented in `external_url.rs`.
- **R6** Build wiring: `crates/desktop` is **not** yet a workspace member (orchestrator adds it) and has no `[[bin]]`; `src/main/index.rs` is the app entry in the source, so the crate ships a `lib` (library) target and leaves the binary decision to the orchestrator's runtime choice.
