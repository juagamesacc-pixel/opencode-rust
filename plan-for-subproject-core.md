---
# plan-for-subproject-core — @opencode-ai/core → crates/core (porting, exact 1:1 clone)

> Compartment: SOURCE `/root/opencode-src/packages/core` → TARGET `/root/opencode-rust/crates/core` (absent, fresh start). This file is the SOLE plan for this compartment. No code edits by planner.

## 0. Compartment + doctrine

- SOURCE pin: `v1.18.30 @3104c14`, `~479 TS files`, `~68.0k LOC`, package `@opencode-ai/core`. TARGET: `/root/opencode-rust/crates/core`, Rust stable `1.98.1`, workspace version `1.18.30`, edition `2021`.
- Read-only context (never write): assigned source dir above; `/root/opencode-rust/crates/server/src/core_provisional.rs` (116 lines PROVISIONAL core stubs — keep names compatible where source dictates, flag deviations); `/root/opencode-rust/PORTING_MAP.md` (format precedent); `/root/opencode-rust/Cargo.toml` (members list — do NOT edit, member entry below is PLAN TEXT); sibling plans `plan-for-subproject-app.md`, `plan-for-subproject-opencode.md`, `plan-for-codemode.md` (format precedent if present).
- ZERO-DIVERSION GUARANTEE (porting): source is spec. No improvements, no renames for taste, no merges/splits/reorders, no modernization, no UI/UX redesign (core has no UI; CLI/TUI wording, error strings, log lines, API shapes preserved exactly), no dropped 'ugly/dead' code without proof + user approval. Every deviation needs explicit user approval, never silent. When in doubt re-read source.
- Write-scope discipline: write ONLY under `/root/opencode-rust/crates/core/` + this plan file. DO NOT touch workspace `Cargo.toml`, `PORTING_MAP.md`, or any other crate — member entry + map rows are REPORT TEXT. Never tooling dirs, `.git/`, caches.

## 1. Research gate — pinned versions

| Artifact | Pinned version | Source of truth | Note |
|---|---|---|---|
| Source repo | v1.18.30 @3104c14 | task brief + source dir | lanes re-verify `git rev-parse HEAD` read-only before inventory |
| Source package | @opencode-ai/core, ~479 files, ~68.0k LOC | brief | lanes produce exhaustive count; brief counts are floor, not ceiling |
| Source runtime | Bun 1.3.14 | brief | behavior reference only |
| Source language | TypeScript 5.8.2 | brief | strict semantics preserved |
| Source framework | Effect (Layer/LayerNode, Schema, Context/Service) | sampled `export const layer/node/Service` | map §3 |
| Target toolchain | Rust stable 1.98.1 (`/root/.cargo/bin`) | brief | CHECKUPS ONLY per §7 |
| Workspace | version 1.18.30, edition 2021 | brief | member entry §6, do not edit Cargo.toml |
| Async | tokio 1 (async only where source async) | doctrine | no new concurrency |
| Schema/JSON | serde 1 + serde_json 1 | doctrine | Effect Schema → serde derives; exact field names/defaults |
| DB | rusqlite-equivalent pending (see provisional) | to confirm | any substitution needs approval |
| Time of plan | ~10:06 AM IST 2026-09-12 Asia/Kolkata | operator | zero downloads expected; downloads need consent outside 12–6AM |

No new research downloads. If a lane needs a version fact, it records local time first and asks via report (never `wait_for_user`, never silent download).

## 2. Architecture for exact clone (mandatory)

- Mirror: `src/<subdir>/*.ts` → `crates/core/src/<subdir_snake>/*.rs` + `mod.rs` per dir; top-level `src/*.ts` → same-named `*.rs` (+ `lib.rs` re-exports in source order). No regrouping. Example: `src/session/runner/llm.ts` → `src/session/runner/llm.rs`; `src/tool/write.ts` → `src/tool/write.rs`; `src/database/migration/*.ts` → `src/database/migration/*.rs`. File order, symbol order, error-string order preserved.
- Effect mapping: `Effect.gen/function*` → explicit Rust state machine: `async fn` on tokio 1 ONLY where source `async`/`Effect` async; sync Effect → sync fn. `Layer`/`LayerNode.makeGlobalNode/makeLocationNode` → explicit structs + `build_*_layer()` constructors carrying same `deps:[...]` ordering (no DI framework invention). `Context.Service` → trait + struct pair with identical method names. `Schema.Struct/Union/Literals/brand` → `#[derive(Serialize,Deserialize)]` structs/enums with `#[serde(rename=...)]` preserving wire names, defaults, optionality. `Effect.fail/NamedError` → typed `Error` enums preserving message strings + status codes. `Stream` → `tokio::sync::mpsc` / iterator only where source streams. `SynchronizedRef/MemoMap/KeyedMutex/Flock` → `tokio::sync::{Mutex,RwLock,Semaphore}` + file-lock equivalent flagged PROVISIONAL if unported dep.
- Provisional stubs: `crates/server/src/core_provisional.rs` (116 lines) ANTICIPATES core interfaces. Lanes open it read-only first; where source dictates a name, keep stub name (adjust implementation, not the contract); where stub diverges from source, keep SOURCE, flag deviation in lane report + map row `PROVISIONAL-DEVIATION: <stub> vs <source> → source wins, approval requested`. Unported deps (sqlite native, ripgrep binding, fff, pty native, Effect platform layers) stay `PROVISIONAL + pending crate` with `todo!()`-free explicit `unimplemented!(source-ref)` + compile-safe stub, never invented behavior.
- Naming: snake_case files/modules per Rust convention ONLY at file/module level; item names stay source-identical modulo case conversion, recorded in map. No taste renames. Error/log/config strings byte-identical. Ordering of match arms, schema fields, table columns identical.
- UI/UX: core package has no screens/components; parity obligation reduces to: no change to user-visible strings, CLI help text, prompt templates (`MAX_STEPS_PROMPT`, tool descriptions), markdown rendering, question prompts. Any TUI/CLI surface owned elsewhere is out of scope — flag cross-compartment strings, do not duplicate.

## 3. Artifact inventory strategy (contract)

Each lane produces BEFORE translating: `inventory-<lane>.md` under `crates/core/docs/` (lane-owned path) listing EVERY assigned source file with role (1 line) + inside it every `function/method/class/handler/const/enum/table/schema/layer/node` + notable variables. Method: read-only walk of assigned `src` subdirs (no sampling), cross-checked with `export function/export const/export class` ast_grep counts + manual read for `Effect.fn/Layer/Schema/sqliteTable` artifacts missed by naive patterns. Counts roll up to plan table: files/classes/functions/handlers/variables/tables/schemas/nodes. Inventory is the attendance contract for §5 verification. Include build/schema/config assets (`*.sql.ts`, `migration.gen.ts`, `v1/config/*`, `config/*`) and `test/*` fixtures touching assigned scope (tests ported 1:1, fixtures referenced not duplicated).

## 4. Bidirectional map strategy

`PORTING_MAP.md` is NOT edited by lanes. Each lane maintains `map-<lane>.md` (lane-owned, under `crates/core/docs/`) with rows: `source path::symbol (kind) → target path::symbol (kind) | status [mapped|provisional|blocked] | notes`. Every source artifact gets exactly one target counterpart before translation of that artifact begins. Rename-for-compiler cases recorded (`Type` → `Type_` style minimal + reason). Provisional rows tagged `PROVISIONAL + pending crate: <name>`. On lane completion, lane report pastes NEW/CHANGED map rows as REPORT TEXT for the map keeper; never edits the central map directly.

## 5. Implementation sequence — 7 parallel lanes, DISJOINT write scopes

Dependency graph: L1 foundation → L2 config/policy → L3 persistence → L4 session-runtime → L5 tools-execution → L6 providers/models; L7 infra-misc depends on L1 only, runs anytime. L2–L7 all depend on L1 types; L4 depends on L3 tables; L5 depends on L3+L4 types; L6 depends on L1+L2. Lanes run parallel after L1 `lib.rs` + shared error/schema skeleton lands (L1 day-1 skeleton, reviewed via CI, others stub against it with PROVISIONAL imports — no cross-lane writes).

| Lane | Assigned SOURCE subdirs (read-only) | Owned TARGET writes (DISJOINT under crates/core/src/) | Key artifacts |
|---|---|---|---|
| L1 foundation | `src/*.ts` root shared (location.ts, global.ts, fs-util.ts, process.ts, shell.ts, patch.ts, npm.ts, npm-config.ts, workspace.ts, snapshot.ts skeleton), `src/util/*`, `src/id/*`, `src/flag/*`, `src/effect/*`, `src/observability/*`, `src/installation/*` | `lib.rs`, `foundation.rs`, `util/*`, `id/*`, `flag.rs`, `effect/*`, `global.rs`, `observability/*`, `installation/*`, `docs/inventory-l1.md`, `docs/map-l1.md` | Effect/ServiceUse/MemoMap/KeyedMutex/AppNode, runID/loggers, paths, which/slug/wildcard/encode/token/path/module/flock |
| L2 config-policy | `src/config/*` (+`plugin/*`), `src/v1/*`, `src/policy*`, `src/permission/*`, `src/catalog*`, `src/integration/*`, `src/location*` | `config/*`, `v1/*`, `policy.rs`, `permission/*`, `catalog.rs`, `integration/*`, `location*.rs`, `docs/inventory-l2.md`, `docs/map-l2.md` | Schemas (agent/mcp/lsp/formatter/experimental), migrate/isV1, permission Ask/Reply, catalog events, location nodes |
| L3 persistence | `src/database/*` (+`migration/*`), `src/event/*`, `src/account/*`, `src/credential/*`, `src/project/*`, `src/share/*`, `src/control-plane/*`, `src/data-migration.sql.ts` | `database/*`, `event/*`, `account/*`, `credential/*`, `project/*`, `share.rs`, `control_plane/*`, `docs/inventory-l3.md`, `docs/map-l3.md` | sqlite tables, migration apply/applyOnly, event sequences, project copy/directories, move-session |
| L4 session-runtime | `src/session/*` (runner/*, execution/*, schema/sql/store/input/history/compaction/revert/todo/run-coordinator/projector/context-epoch), `src/snapshot*`, `src/background-job*`, `src/question*`, `src/skill/*`, `src/system-context/*`, `src/reference/*`, `src/instruction-context*` | `session/*`, `snapshot.rs`, `background_job.rs`, `question.rs`, `skill/*`, `system_context/*`, `reference/*`, `instruction_context.rs`, `docs/inventory-l4.md`, `docs/map-l4.md` | Session lifecycle, runner llm/model/to-llm-message/publish-llm-event/max-steps, compaction, skill discovery, sysctx builtins |
| L5 tools-execution | `src/tool/*` (~18 files), `src/filesystem/*`, `src/ripgrep/*`, `src/pty/*`, `src/shell*`, `src/process*` (impl), `src/cross-spawn-spawner*`, `src/git*`, `src/file*`, `src/file-mutation*` | `tool/*`, `filesystem/*`, `ripgrep/*`, `pty/*`, `shell.rs`, `process_impl.rs`, `spawner.rs`, `git.rs`, `file*.rs`, `docs/inventory-l5.md`, `docs/map-l5.md` | All tools (bash/read/edit/write/grep/glob/webfetch/websearch/question/skill/todowrite/apply-patch), registry/builtins, search/watcher/ignore, pty protocol/ticket |
| L6 providers-models | `src/provider*`, `src/plugin/*` (host/internal/command/agent/variant/promise/skill/models-dev + `provider/*` ~30 files), `src/models-dev*`, `src/aisdk*`, `src/github-copilot/*`, `src/model*` | `provider.rs`, `plugin/*`, `models_dev.rs`, `aisdk.rs`, `github_copilot/*`, `model.rs`, `docs/inventory-l6.md`, `docs/map-l6.md` | Provider Info/Request/Api, ~30 provider plugins (anthropic/openai/azure/google/…/gateway/dynamic), copilot chat/responses/tools, models-dev catalog |
| L7 infra-misc | `src/image/*`, `src/oauth/*`, `src/tool-output-store*`, `src/location-mutation*`, `src/repository-cache*`, remaining root (`command.ts`, `file.ts` impl, `location-services.ts`, `public-event-manifest.ts`) | `image/*`, `oauth/*`, `tool_output_store.rs`, `location_mutation.rs`, `repository_cache.rs`, `command.rs`, `docs/inventory-l7.md`, `docs/map-l7.md` | OAuth pages, image/photon, tool-output retention, repo cache, location services wiring |

Rules: lanes translate artifact-by-artifact 1:1 in source order; no cross-lane file writes; shared types consumed via L1 skeleton (read-only); any needed cross-lane change requested via report, never direct edit. Each lane ends with §6 gates green on CI.

## 6. Proposed workspace + map text (DO NOT APPLY — report text only)

Cargo member entry (propose to orchestrator):
```toml
# crates/core/Cargo.toml — PROPOSED (plan text only, do not edit workspace Cargo.toml here)
[package]
name = "opencode-core"
version = "1.18.30"
edition = "2021"
[dependencies]
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = { version = "1" }
# + rusqlite/sqlx, reqwest, ignore/globset etc. ONLY with approval — else PROVISIONAL stub
```
Workspace `members` add (propose): `"crates/core",` alongside existing members.
PORTING_MAP rows (propose, abbreviated — full per-artifact rows in lane maps):
```
packages/core/src/util/wildcard.ts::match → crates/core/src/util/wildcard.rs::match_ | mapped
packages/core/src/tool/write.ts::Input/Output/node → crates/core/src/tool/write.rs::Input/Output/build_node | mapped
packages/core/src/database/migration.ts::apply → crates/core/src/database/migration.rs::apply | mapped
packages/core/src/session/runner/llm.ts::* → crates/core/src/session/runner/llm.rs::* | provisional (tokio stream) + pending review
packages/core/src/plugin/provider/*.ts::*Plugin → crates/core/src/plugin/provider/*.rs::*_plugin | mapped
```

## 7. Lane delegation template (paste verbatim into every lane prompt; operator rules verbatim)

> You are a core-port lane operator. SCOPE: <lane id + assigned source subdirs + owned target paths from §5 — write ONLY there + your docs files; read-only elsewhere>. DOCTRINE: exact 1:1 clone per this plan §§0–4; source is spec; no renames/merges/reorders/improvements; provisional stubs flagged PROVISIONAL + pending crate; tests ported 1:1; every deviation needs user approval via report (never silent). SEQUENCE: (1) record local time Asia/Kolkata; (2) read-only inventory → `docs/inventory-<lane>.md`; (3) map → `docs/map-<lane>.md`; (4) translate artifact-by-artifact in source order; (5) per-lane gates §8; (6) report note with counts + map rows + questions. OPERATOR RULES (verbatim): (a) local Rust (/root/.cargo/bin) CHECKUPS ONLY — rustfmt binary on own files, read-only inspection; NEVER cargo build/test/clippy locally (GitHub CI only); (b) downloads: 12AM–6AM Asia/Kolkata free, else explicit user consent first — record local time, zero downloads expected (local time now ~10:06 AM IST 2026-09-12); (c) never wait_for_user — user questions come back through report. WRITE-SCOPE DISCIPLINE: write ONLY under /root/opencode-rust/crates/core/ + your docs files. DO NOT touch workspace Cargo.toml, PORTING_MAP.md, or any other crate — prepare member entry + map rows as REPORT TEXT. Never tooling dirs, .git/, caches. NOTES: use session_id passed in delegation in EVERY note call; log todo/status/finding/risk/report.

## 8. Verification gates per lane (parity/attendance, tests 1:1)

Per lane, ALL required: (i) attendance diff: every inventory artifact has one map row + one target symbol — zero orphans/missing; (ii) behavior parity: same inputs → same outputs/errors/exit codes, error strings/config keys/defaults/ordering identical (spot-check via CI tests, not local run); (iii) ported tests 1:1: matching `test/*` cases for assigned scope translated without strengthening/weakening, run on GitHub CI; (iv) provisional audit: all stubs tagged + pending crate named; (v) rustfmt-clean own files (checkup only). Lane done = gates (i)–(v) attested in lane report with CI run link. Cross-lane full attendance + full test suite at compartment close.

## 9. Build/test-and-fix gate (GitHub CI only)

NEVER `cargo build/test/clippy` locally. Push lane branches; GitHub CI builds/tests; fix-forward on CI red (read logs, patch owned files only, re-push). Compartment green = workspace build + `crates/core` tests + ported core tests green on CI, attendance 100%, zero unapproved deviations, provisional list approved or resolved. No 'done' claim without CI evidence links.

## 10. Open questions (via report, never wait_for_user)

1. Confirm Rust crate name `opencode-core` vs `core` for member entry? 2. Approve DB driver (rusqlite vs sqlx) + sqlite native replacement for `sqlite.node/bun`? 3. Approve HTTP/FileSystem platform layer crates (reqwest + tokio::fs) vs stubs? 4. Keep `core_provisional.rs` stub names where they diverge from source, or rename stubs to source (source-wins assumed)? 5. How to map `Layer.makeMemoMapUnsafe`/global memo state — approved `OnceLock` pattern? 6. Ripgrep/fff/pty native bindings: stub or approved crate?

## 11. Todo list (planner tracking)

- [x] pin + research gate; [x] source sampling + lane subdivision; [x] architecture + inventory/map strategies; [x] lane template + gates + CI policy; [ ] persist this file to disk + verify exists (blocked on write tool — orchestrator action); [ ] delegate L1→L7 with fresh session_ids; [ ] collect lane reports + attendance; [ ] compartment attestation.

---
END PLAN
