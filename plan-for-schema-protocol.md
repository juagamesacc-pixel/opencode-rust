# Sub-Project Plan — schema + protocol Port (opencode TS → Rust)

## 0. Meta / Doctrine / Download rule

- **Source:** `anomalyco/opencode` v1.18.30, commit `3104c1428ec91f809e5ab86631300de41eb6952e`, Bun 1.3.14 / TS 5.8.2.
- **Target:** Rust 1.98.1. Crates: `/root/opencode-rust/crates/schema`, `/root/opencode-rust/crates/protocol`.
- **DOCTRINE (hard):** 1:1 exact translation — same names/signatures/behavior/edge cases/error strings/status codes/keys/defaults/ordering. Source is spec, re-read before choosing. Never improve/modernize/rename/merge/split/reorder. No scope changes without approval — flag and stop, route to orchestrator.
- **Scope READ only:** `/root/opencode-src/packages/schema` (74 files) + `/root/opencode-src/packages/protocol` (26 files). Reference (patterns only, no logic copy without verification): `/root/opencode-rust/crates/cli`, `/root/opencode-rust/crates/codemode`, `/root/opencode-rust/PORTING_MAP.md`.
- **Scope WRITE only:** `/root/opencode-rust/crates/schema`, `/root/opencode-rust/crates/protocol`, this plan file, + ONLY schema/protocol rows in `PORTING_MAP.md`.
- **NEVER read:** `~/.config/opencode`, `~/.opencode`, `bin/`, global `node_modules`, `.git` content, caches, other packages.
- **Workspace manifest:** `/root/opencode-rust/Cargo.toml` — may edit `members` list only to add both crates. Nothing else.
- **Crates (pinned, no fetch, no new deps):** `serde 1` + `serde_json 1` (+ `thiserror 1` ONLY if error taxonomy needs it — otherwise hand-rolled; choice flagged in §7).
- **Workflow:** plan → todo list → translate → leaf `@auditor` self-audit → report. NO build/test (no toolchain; CI covers it later — deferred, see §6).
- **DOWNLOAD RULE (hard, propagated to every delegation):** Asia/Kolkata time checked 2026-09-11 (Fri Sep 11 08:23 UTC 2026 → 13:53 IST, restricted window 06:00–23:59). CODE-WRITE ONLY — zero downloads/installs/builds/network fetches. Between 00:00–06:00 IST downloads need no consent; at any other time stop and get explicit user consent first. This run: zero downloads performed. Pass this rule down in every future delegation.

## 1. Pinned versions (research gate)

| Artifact | Pinned version | Source of truth |
|---|---|---|
| Source repo | `anomalyco/opencode` v1.18.30 commit `3104c1428ec91f809e5ab86631300de41eb6952e` | task spec |
| Source runtime | Bun 1.3.14 / TS 5.8.2 / Effect Schema | task spec |
| Target toolchain | Rust 1.98.1 | task spec |
| `serde` | 1.x (already in workspace lock; no fetch, no upgrade) | workspace `Cargo.lock` |
| `serde_json` | 1.x (already in workspace lock; no fetch) | workspace `Cargo.lock` |
| `thiserror` | 1.x ONLY if approved; default NOT added | §7 flag |
| No other deps | Forbidden | task spec |

Research gate: versions above are final. Do not pin floating versions. Do not `cargo add`/`npm i`. If workspace lock lacks `serde`/`serde_json`, STOP and flag to orchestrator (do not fetch).

## 2. Full artifact inventory (survival contract)

Translator MUST re-`ls` + re-read each file before translating; this table is the attendance checklist. Any file/artifact missing in Rust = parity failure. Counts verified by sampling: `export const` 301 hits/58 files (schema), `HttpApiEndpoint.*` 61 hits/18 files, `export type` 130 hits/33 files, `Schema.brand` 26 hits/21 files, `export class … extends` 16 hits/4 files (13 TaggedErrors + ProjectCopyError + 2 middleware).

### 2a. `packages/schema` (74 files) — role + key artifacts

| # | Source file | Role | Namespaces / consts / branded IDs / unions / functions |
|---|---|---|---|
| S01 | `src/schema.ts` | Primitives/helpers | `PositiveInt`, `NonNegativeInt`, `RelativePath` (brand), `AbsolutePath` (brand), `optional()`, `statics()`, `DateTimeUtcFromMillis` |
| S02 | `src/event.ts` | Event core | `ID` (brand `Event.ID`, prefix `evt_`), `define()`, `inventory()`, `latest()`, `versionedType()`, `Definition`, `Latest` |
| S03 | `src/identifier.ts` | ID gen | `ascending()`, `descending()`, `create(descending, timestamp)` |
| S04 | `src/session-id.ts` | Branded ID | `SessionID` (brand `SessionID`, prefix `ses`) |
| S05 | `src/project-id.ts` | Branded ID | `ProjectID` (brand `Project.ID`) |
| S06 | `src/workspace-id.ts` | Branded ID | `WorkspaceID` (brand `WorkspaceV2.ID`, prefix `wrk`) |
| S07 | `src/integration-id.ts` | Branded IDs | `IntegrationID` (brand), `IntegrationMethodID` (brand) |
| S08 | `src/agent.ts` | Agent | `ID` (brand `AgentV2.ID`), `Color` union, `Info` struct |
| S09 | `src/provider.ts` | Provider | `ID` (brand `ProviderV2.ID`), `AISDK`, `Native`, `Api` tagged union, `Request`, `Info` |
| S10 | `src/model.ts` | Model | `ID` (brand `ModelV2.ID`), `VariantID`, `Ref`, `Family`, `Capabilities`, `Cost`, `Api` union, `Info` |
| S11 | `src/plugin.ts` | Plugin | `ID` (brand `Plugin.ID`), `Event Added/Definitions` |
| S12 | `src/credential.ts` | Credential | `ID` (brand), `OAuth`, `Key`, `Value` tagged union |
| S13 | `src/connection.ts` | Connection | `CredentialInfo`, `EnvInfo`, `Info` tagged union |
| S14 | `src/location.ts` | Location | `Ref` struct |
| S15 | `src/workspace.ts` | Re-export | `ID=WorkspaceID`, `Event=WorkspaceEvent` |
| S16 | `src/workspace-event.ts` | Events | `ConnectionStatus`, `Ready`, `Failed`, `Status`, `Definitions` |
| S17 | `src/project.ts` | Project | `ID`, `Vcs`, `Icon`, `Commands`, `Time`, `Info`, `Event Updated/Definitions` |
| S18 | `src/project-directories.ts` | Dirs | `Event Updated/Definitions` |
| S19 | `src/project-copy.ts` | Copy | `StrategyID` (brand), `CreateInput`, `RemoveInput`, `Copy`; type `StrategyID` |
| S20 | `src/session.ts` | Session facade | `ID=SessionID`, `Event=SessionEvent`, `Info`, `ListAnchor` |
| S21 | `src/session-message.ts` | Messages | `ID` (brand `Session.Message.ID`, prefix `msg_`), `UnknownError`, `AgentSwitched`, `ModelSwitched`, `User`, `Synthetic`, `System`, `Shell`, `ToolState{Pending,Running,Completed,Error}`, `ToolState` union, `AssistantTool/Text/Reasoning`, `AssistantContent` union, `Assistant`, `Compaction`, `Message` union, `Type` |
| S22 | `src/session-event.ts` | Live events (8 namespaces) | `Source`, `UnknownError`, `AgentSwitched`, `ModelSwitched`, `Moved`, `Prompted`, `PromptAdmitted`, `ContextUpdated`, namespaces `Shell{Started,Ended}`, `Step{Started,Ended,Failed}`, `Text{Started,Delta,Ended}`, `Reasoning{Started,Delta,Ended}`, `Tool{Called,Progress,Success,Failed, namespace Input{Started,Delta,Ended}}`, `RetryError`, `Retried`, `Compaction{Started,Delta,Ended}`, `RevertEvent{Staged,Cleared,Committed}`, `DurableDefinitions`, `Definitions`, `Durable` oneOf tagged union, `All` oneOf tagged union, types `DurableEvent/Event/Type` |
| S23 | `src/session-status-event.ts` | Status | `Info` union (idle/...), `Status`, `Idle`, `Definitions` |
| S24 | `src/session-compaction-event.ts` | Compaction | `Compacted`, `Definitions` |
| S25 | `src/session-delivery.ts` | Delivery | `Delivery` literals `steer/queue` |
| S26 | `src/session-input.ts` | Input | `Delivery`, `Admitted` struct |
| S27 | `src/session-todo.ts` | Todo | `Info`, `Event Updated/Definitions` |
| S28 | `src/permission.ts` | Permission v2 | `ID` (brand `PermissionV2.ID`, prefix `per`), `Source` union, `Request`, `Reply` literals, `Event Asked/Replied/Definitions`, `Effect`, `Rule`, `Ruleset` |
| S29 | `src/permission-saved.ts` | Saved | `ID` (brand), `Info` |
| S30 | `src/question.ts` | Question v2 | `ID` (brand `QuestionV2.ID`, prefix `que`), `Option`, `Info`, `Prompt`, `Tool`, `Request`, `Answer`, `Reply`, `Event Asked/Replied/Rejected/Definitions` |
| S31 | `src/prompt.ts` | Prompt | `Source`, `FileAttachment`, `AgentAttachment`, `Prompt` |
| S32 | `src/prompt-input.ts` | Prompt input | `FileAttachment`, `Prompt` |
| S33 | `src/llm.ts` | LLM | `ProviderMetadata` record, `ToolTextContent`, `ToolFileContent`, `ToolContent` tagged union |
| S34 | `src/command.ts` | Command | `Info` struct |
| S35 | `src/skill.ts` | Skill | `DirectorySource`, `UrlSource`, `Info`, `EmbeddedSource`, `Source` tagged union |
| S36 | `src/integration.ts` | Integration | `ID`, `MethodID`, `When`, `TextPrompt`, `SelectPrompt`, `Prompt` union, `OAuthMethod`, `KeyMethod`, `EnvMethod`, `Method` tagged union, `Inputs`, `Event Updated/ConnectionUpdated/Definitions`, `Ref`, `AttemptID` (brand), `AttemptStatus` union |
| S37 | `src/pty.ts` | Pty | `ID` (brand `PtyID`), `Info`, `Event Created/Updated/Exited/Deleted/Definitions`, `CreateInput`, `UpdateInput` |
| S38 | `src/pty-ticket.ts` | Ticket | `ConnectToken` |
| S39 | `src/file-diff.ts` | Diff | `Info` |
| S40 | `src/filesystem.ts` | FS | `Event Edited/Definitions`, `Entry`, `Submatch`, `Match` |
| S41 | `src/filesystem-watcher.ts` | Watcher | `Event Updated/Definitions` |
| S42 | `src/reference.ts` | Reference | `Event Updated/Definitions`, `LocalSource`, `GitSource`, `Source` tagged union |
| S43 | `src/revert.ts` | Revert | `FileDiff`, `State` |
| S44 | `src/catalog.ts` | Catalog | `Event Updated/Definitions inventory` |
| S45 | `src/models-dev.ts` | Models.dev | `Event Refreshed/Definitions` |
| S46 | `src/installation-event.ts` | Install | `Updated`, `UpdateAvailable`, `Definitions` |
| S47 | `src/worktree-event.ts` | Worktree | `Ready`, `Failed`, `Definitions` |
| S48 | `src/lsp-event.ts` | LSP | `Updated`, `Definitions` |
| S49 | `src/mcp-event.ts` | MCP | `ToolsChanged`, `BrowserOpenFailed`, `Definitions` |
| S50 | `src/ide-event.ts` | IDE | `Installed`, `Definitions` |
| S51 | `src/server-event.ts` | Server | `Connected`, `Disposed`, `Definitions` |
| S52 | `src/tui-event.ts` | TUI | `PromptAppend`, `CommandExecute`, `ToastShow`, `SessionSelect`, `Definitions` |
| S53 | `src/vcs-event.ts` | VCS | `BranchUpdated`, `Definitions` |
| S54 | `src/event-manifest.ts` | Manifest | `ServerDefinitions`, `Definitions`, `Latest` |
| S55 | `src/durable-event-manifest.ts` | Durable manifest | `SessionDurable`, `Durable` |
| S56–S59 | `src/v1/session.ts` | V1 compat session/parts | `MessageID` (brand, prefix `msg`), `PartID` (brand, prefix `prt`), errors `OutputLengthError/AuthError/AbortedError/StructuredOutputError/APIError/ContextOverflowError/ContentFilterError` (via `namedError`), `Format` union, `SnapshotPart/PatchPart/TextPart/ReasoningPart/Range/FileSource/SymbolSource/ResourceSource/FilePartSource/FilePart/AgentPart/CompactionPart/SubtaskPart/RetryPart/StepStartPart/StepFinishPart/ToolState*/ToolState/ToolPart/User/Part/TextPartInput/FilePartInput/AgentPartInput/SubtaskPartInput/Assistant/Info/WithParts/SessionInfo/PartDelta/Diff/Error/Event` — full union + inputs preserved |
| S60 | `src/v1/permission.ts` | V1 permission | `ID` (brand `PermissionID`, prefix `per`), `Action`, `Rule`, `Ruleset`, `Request`, `Reply`, `ReplyBody`, `Approval`, `AskInput`, `ReplyInput`, `Event Asked/Replied/Definitions` |
| S61 | `src/v1/question.ts` | V1 question | `ID` (brand `QuestionID`, prefix `que`), `Option`, `Info`, `Prompt`, `Tool`, `Request`, `Answer`, `Reply`, `Replied/Rejected`, `Event` |
| S62 | `src/v1/legacy-event.ts` | V1 legacy | `CommandExecuted`, `Definitions` |
| S63–S74 | index/re-export barrels + any remaining per-package files to reach 74 (e.g. `src/index.ts`, per-domain `index` if present) | Barrels only, no new types | Translator MUST `ls` and list them; each barrel maps to `mod.rs` re-export only, zero logic |

Note: 62 logical files observed via grep; remaining ~12 are barrels/config. Translator must enumerate all 74 on disk and extend table without renaming.

### 2b. `packages/protocol` (26 files)

| # | Source file | Role | Artifacts |
|---|---|---|---|
| P01 | `src/api.ts` | Api assembly | `makeApi()`, `makeDefaultApi()` (generic over LocationId/LocationService/Session… middleware) |
| P02 | `src/errors.ts` | 13 TaggedErrors (httpApiStatus preserved verbatim) | `InvalidRequestError`, `UnauthorizedError`, `ConflictError`, `ServiceUnavailableError`, `UnknownError`, `ProviderNotFoundError`, `SessionNotFoundError`, `MessageNotFoundError`, `InvalidCursorError`, `PermissionNotFoundError`, `QuestionNotFoundError`, `ForbiddenError`, `PtyNotFoundError` — each `Schema.TaggedErrorClass(tag, {fields})` + `httpApiStatus` code; translator must copy codes/fields/error strings verbatim |
| P03 | `src/middleware/authorization.ts` | Middleware | `Authorization extends HttpApiMiddleware.Service("@opencode/HttpApiAuth…")` |
| P04 | `src/middleware/schema-error.ts` | Middleware | `SchemaErrorMiddleware extends HttpApiMiddleware.Service("@op…")` |
| P05 | `src/groups/health.ts` | HttpApiGroup `server.health` | `HealthGroup`: `GET health.get /api/health` |
| P06 | `src/groups/provider.ts` | `server.provider` | `ProviderGroup`: `GET provider.list /api/provider`, `GET provider.get /api/provider/:providerID` |
| P07 | `src/groups/model.ts` | `server.model` | `ModelGroup`: `GET model.list /api/model` |
| P08 | `src/groups/agent.ts` | `server.agent` | `AgentGroup`: `GET agent.list /api/agent` |
| P09 | `src/groups/command.ts` | `server.command` | `CommandGroup`: `GET command.list /api/command` |
| P10 | `src/groups/skill.ts` | `server.skill` | `SkillGroup`: `GET skill.list /api/skill` |
| P11 | `src/groups/reference.ts` | `server.reference` | `ReferenceGroup`: `GET reference.list /api/reference` |
| P12 | `src/groups/credential.ts` | `server.credential` | `CredentialGroup`: `PATCH credential.update /api/credential/:credentialID`, `DELETE credential.remove …` |
| P13 | `src/groups/location.ts` | `server.location` | `LocationQuery`, `locationQueryOpenApi`, `LocationGroup GET location.get /api/location` |
| P14 | `src/groups/fs.ts` | `server.fs` | `FileSystemGroup`: `GET fs.read /api/fs/read/*`, `GET fs.list /api/fs/list`, `GET fs.find /api/fs/find` (+ `ListQuery/FindQuery`) |
| P15 | `src/groups/message.ts` | `server.message` | `SessionMessagesQuery`, `MessageGroup GET session.messages /api/session/:sessionID/message` |
| P16 | `src/groups/session.ts` | session (factory) | `SessionsCursor` (brand), `SessionHistoryQuery`, `SessionsQuery`, `makeSessionGroup(middleware)`: `GET session.list /api/session`, `POST session.create`, `GET session.active`, `GET session.get`, `POST session.switchAgent`, `POST session.switchModel`, `POST session.prompt`, `POST session.compact`, `POST session.wait`, `POST session.revert.stage/clear/commit`, `GET session.context/history/events`, `POST session.interrupt`, `GET session.message` |
| P17 | `src/groups/permission.ts` | permission (factory) | `makePermissionGroup()`: `GET permission.request.list`, `GET permission.saved.list`, `DELETE permission.saved.remove`, `POST session.permission.create/list/get/reply` |
| P18 | `src/groups/question.ts` | question (factory) | `makeQuestionGroup()`: `GET question.request.list`, `GET session.question.list`, `POST session.question.reply/reject` |
| P19 | `src/groups/integration.ts` | `server.integration` | `IntegrationGroup`: `GET integration.list/get`, `POST integration.connect.key/oauth`, `GET integration.attempt.status`, `POST integration.attempt.complete`, `DELETE integration.attempt.cancel` |
| P20 | `src/groups/pty.ts` | `server.pty` | `PTY_CONNECT_TICKET_QUERY`, `PTY_CONNECT_TOKEN_HEADER`, `PTY_CONNECT_TOKEN_HEADER_VALUE`, `PtyGroup`: `GET pty.list/get`, `POST pty.create`, `PUT pty.update`, `DELETE pty.remove`, `POST pty.connectToken`, `GET pty.connect` |
| P21 | `src/groups/project-copy.ts` | `server.projectCopy` | `ProjectCopyError` (ErrorClass), `ProjectCopyGroup`: `POST projectCopy.create/delete→remove/refresh` |
| P22 | `src/groups/event.ts` | event (factory) | `makeEventGroup(definitions)`, `EventGroup`, `OpenCodeEvent`, types `OpenCodeEvent/OpenCodeEventEncoded`; `GET event.subscribe /api/event` (SSE) |
| P23–P26 | barrels (`src/index.ts`, `src/groups/index.ts`, `src/middleware/index.ts`, etc.) | Barrels | Re-exports only |

Translator must confirm 26 files on disk; any extra group file = add row, no merging.

## 3. Architecture for exact clone

### 3.1 Source-file → Rust-module mirror (1:1, no merge/split/rename)

```text
crates/schema/src/lib.rs              ← barrels (index.ts)
crates/schema/src/schema_primitives.rs← schema.ts (named *_primitives to avoid clash with crate name; module path documented in map — NOT a rename of types)
crates/schema/src/event.rs            ← event.ts
crates/schema/src/identifier.rs       ← identifier.ts
crates/schema/src/session_id.rs       ← session-id.ts
... one .rs per source .ts, kebab→snake, same stem ...
crates/schema/src/v1/session.rs       ← v1/session.ts
crates/schema/src/v1/permission.rs    ← v1/permission.ts
crates/schema/src/v1/question.rs      ← v1/question.ts
crates/schema/src/v1/legacy_event.rs  ← v1/legacy-event.ts
crates/schema/src/v1/mod.rs           ← v1 barrel
crates/schema/src/mod.rs              ← root barrel (if lib.rs needs it; keep lib.rs as sole barrel if cli/codemode pattern prefers — follow established pattern, document)

crates/protocol/src/lib.rs
crates/protocol/src/api.rs            ← api.ts
crates/protocol/src/errors.rs         ← errors.ts (13 TaggedErrors)
crates/protocol/src/middleware/authorization.rs
crates/protocol/src/middleware/schema_error.rs
crates/protocol/src/middleware/mod.rs
crates/protocol/src/groups/{health,provider,model,agent,command,skill,reference,credential,location,fs,message,session,permission,question,integration,pty,project_copy,event}.rs
crates/protocol/src/groups/mod.rs
```

- Kebab→snake is mechanical transliteration only, not a rename. Every TS export name keeps exact camelCase/PascalCase as Rust `struct/enum/fn/const` name (Rust allows non-snake for parity? Use `#[allow(non_snake_case)]` if needed to preserve exact names — parity over lint).
- `mod.rs`/`lib.rs` only re-export; no logic moves.
- Add both crates to workspace `members` only.

### 3.2 Effect Schema → serde (identical JSON shape)

| TS | Rust | Notes |
|---|---|---|
| `Schema.Struct({…})` | `#[derive(Serialize,Deserialize,Clone,Debug,PartialEq)] pub struct` with `#[serde(rename="…")]` per field to keep exact keys | Field order in struct follows source order (serde preserves); do not reorder. |
| `Schema.Union + toTaggedUnion("type")` | `#[serde(tag="type")] pub enum` with `#[serde(rename="…")]` per variant | e.g. `Credential.Value`, `Integration.Method`, `SessionMessage.Message`, `v1.Part`. Preserve `oneOf` mode semantics (untagged vs tagged verified per file). |
| `Schema.Literal("x")` / `Literals([...])` | Unit variants or `&'static str` consts + custom Deserialize that rejects other values | Preserve exact strings. |
| `optional(S)` / `Schema.optional` | `Option<T>` with `#[serde(skip_serializing_if="Option::is_none")]` ONLY if TS omits key when absent; else plain `Option` (null). Verify per struct from source — default: skip-if-none for `optional()` helper. |
| `Schema.Record(String, T)` | `BTreeMap<String,T>` (deterministic; JSON shape same as object). If cli/codemode uses `HashMap`, still use `BTreeMap` and flag — ordering parity for tests. | |
| `Schema.Array` | `Vec<T>` | |
| `Schema.String.pipe(brand("…"))` | `#[derive(...)] pub struct BrandName(pub String)` with `#[serde(transparent)]` + `Deref/AsRef/Display/From` + `create()` static where source has `statics(create…)` (ProjectID, Pty.ID, SessionID, etc.) | All 26 brands become newtypes; wire format stays bare string. Preserve prefix checks (`ses`, `msg_`, `prt`, `per`, `que`, `evt_`, `wrk`) as `is_*`/validation fns, not as serde change. |
| `Schema.Int` checks (`PositiveInt`, `NonNegativeInt`) | `i64` (or `u64` where non-negative proven? NO — use `i64` to preserve negative-input error behavior; validate in constructor) | Do not narrow type and change error path. |
| `Schema.Finite` / `NumberFromString` | `f64` / `String`-deserializing wrapper preserving query semantics (`SessionsQuery`, `SessionMessagesQuery`) | Keep `NumberFromString` decode (string→number) via custom deserializer. |
| `Schema.DateTimeUtc`, `Finite→Date` | `chrono` NOT allowed (new dep). Use `i64` millis + `String` RFC3339 passthrough matching `DateTimeUtcFromMillis` encode/decode exactly; do not add dep. | Flagged in §7 but no substitution — shape preserved with primitives. |
| `Schema.Unknown`, `JsonValue` | `serde_json::Value` | |
| `Event.define({type, schema})` | `pub const TYPE: &str` + `pub struct` for payload + `definition()` fn returning `{type, schema}` descriptor; `inventory()` → `pub const DEFINITIONS: &[…]`; `latest()`/`versionedType()` ported as fns in `event.rs` | Preserve `type` strings verbatim (`session.next.*`, `mcp.*`, etc.). |
| `namedError(name, fields)` (v1) | Same pattern as TaggedError but in `schema::v1` (preserve `MessageOutputLengthError`, `ProviderAuthError`, `MessageAbortedError`, `StructuredOutputError`, `APIError`, `ContextOverflowError`, `ContentFilterError` with exact fields) | |
| Constants (`PTY_CONNECT_*`, `Delivery`, `Vcs`) | `pub const …: &str` / literal-union enums with exact values | |

### 3.3 TaggedError → Rust errors (httpApiStatus verbatim)

- `errors.rs`: `pub enum ProtocolError { InvalidRequest{…}, Unauthorized{…}, … }` with `#[serde(tag="_tag")]`-compatible shape matching `TaggedErrorClass` JSON (`_tag` + fields). Each variant exposes `fn http_status(&self)->u16` returning the verbatim `httpApiStatus` from source, `fn tag(&self)->&'static str`, `fn message(&self)->&str` where applicable.
- `ProjectCopyError` stays in `groups/project_copy.rs` as its own error type (do not merge into `errors.rs`).
- Do NOT use `thiserror` unless approved — default hand-rolled `impl Display/Error` with exact `error strings` from source. Rationale in §7.
- Middleware: `authorization.rs` → `pub struct Authorization` descriptor with same service id string; `schema_error.rs` → `pub struct SchemaErrorMiddleware` descriptor. No behavior invention — types + constants only (HTTP serving lives elsewhere).

### 3.4 v1-compat

- `schema::v1::*` mirrors `v1/*` exactly, including legacy names (`QuestionID` vs `QuestionV2.ID`, `PermissionID` vs `PermissionV2.ID`, `MessageID/PartID`, `SessionInfo`, `WithParts`, `*Input` types). Do not unify v1 and v2 types. `event-manifest`/`durable-event-manifest` unions reference both; keep cross-refs as `crate::…` paths with identical variant sets.

## 4. Bidirectional artifact map (source → target; reverse by reading column right→left)

| Source artifact | Target module/type |
|---|---|
| `schema/schema.ts: PositiveInt, NonNegativeInt, RelativePath, AbsolutePath, optional, statics, DateTimeUtcFromMillis` | `schema::schema_primitives::{PositiveInt(alias i64), NonNegativeInt, RelativePath(newtype), AbsolutePath(newtype), optional_as_opt, statics_trait, datetime_utc_from_millis_fns}` |
| `schema/event.ts: ID, define, inventory, latest, versionedType` | `schema::event::{EventId, define, inventory, latest, versioned_type}` |
| `schema/identifier.ts: ascending/descending/create` | `schema::identifier::{ascending,descending,create}` |
| `schema/session-id.ts: SessionID` | `schema::session_id::SessionID` |
| `schema/project-id.ts: ProjectID` | `schema::project_id::ProjectID` |
| `schema/workspace-id.ts: WorkspaceID` | `schema::workspace_id::WorkspaceID` |
| `schema/integration-id.ts: IntegrationID, IntegrationMethodID` | `schema::integration_id::{IntegrationID, IntegrationMethodID}` |
| `schema/agent.ts: ID, Color, Info` | `schema::agent::{ID,Color,Info}` |
| `schema/provider.ts: ID, AISDK, Native, Api, Request, Info` | `schema::provider::{…}` |
| `schema/model.ts: ID, VariantID, Ref, Family, Capabilities, Cost, Api, Info` | `schema::model::{…}` |
| `schema/plugin.ts: ID, Added, Event` | `schema::plugin::{…}` |
| `schema/credential.ts: ID, OAuth, Key, Value` | `schema::credential::{…}` |
| `schema/connection.ts: CredentialInfo, EnvInfo, Info` | `schema::connection::{…}` |
| `schema/location.ts: Ref` | `schema::location::Ref` |
| `schema/workspace.ts: ID, Event` | `schema::workspace::{ID,Event}` (re-export) |
| `schema/workspace-event.ts: ConnectionStatus, Ready, Failed, Status, Definitions` | `schema::workspace_event::{…}` |
| `schema/project.ts: ID, Vcs, Icon, Commands, Time, Info, Event` | `schema::project::{…}` |
| `schema/project-directories.ts: Event` | `schema::project_directories::Event` |
| `schema/project-copy.ts: StrategyID, CreateInput, RemoveInput, Copy` | `schema::project_copy::{…}` |
| `schema/session.ts: ID, Event, Info, ListAnchor` | `schema::session::{…}` |
| `schema/session-message.ts: all Message parts/unions` | `schema::session_message::{…}` |
| `schema/session-event.ts: all live events + 8 namespaces + Durable/All` | `schema::session_event::{…, shell, step, text, reasoning, tool, compaction, revert_event modules}` |
| `schema/session-status-event.ts: Info, Status, Idle, Definitions` | `schema::session_status_event::{…}` |
| `schema/session-compaction-event.ts: Compacted, Definitions` | `schema::session_compaction_event::{…}` |
| `schema/session-delivery.ts: Delivery` | `schema::session_delivery::Delivery` |
| `schema/session-input.ts: Delivery, Admitted` | `schema::session_input::{…}` |
| `schema/session-todo.ts: Info, Event` | `schema::session_todo::{…}` |
| `schema/permission.ts: ID, Source, Request, Reply, Event, Effect, Rule, Ruleset` | `schema::permission::{…}` |
| `schema/permission-saved.ts: ID, Info` | `schema::permission_saved::{…}` |
| `schema/question.ts: ID, Option, Info, Prompt, Tool, Request, Answer, Reply, Event` | `schema::question::{…}` |
| `schema/prompt.ts + prompt-input.ts` | `schema::prompt::{…}`, `schema::prompt_input::{…}` (kept separate) |
| `schema/llm.ts` | `schema::llm::{ProviderMetadata, ToolTextContent, ToolFileContent, ToolContent}` |
| `schema/command.ts: Info` | `schema::command::Info` |
| `schema/skill.ts` | `schema::skill::{DirectorySource,UrlSource,Info,EmbeddedSource,Source}` |
| `schema/integration.ts` | `schema::integration::{… AttemptID, AttemptStatus …}` |
| `schema/pty.ts + pty-ticket.ts` | `schema::pty::{…}`, `schema::pty_ticket::{ConnectToken}` |
| `schema/file-diff.ts, filesystem.ts, filesystem-watcher.ts, reference.ts, revert.ts, catalog.ts, models-dev.ts, installation-event.ts, worktree-event.ts, lsp-event.ts, mcp-event.ts, ide-event.ts, server-event.ts, tui-event.ts, vcs-event.ts, event-manifest.ts, durable-event-manifest.ts` | Same-stem `schema::…` modules, same const names |
| `schema/v1/session.ts` (all parts/messages/errors) | `schema::v1::session::{…}` (no merging with v2) |
| `schema/v1/permission.ts`, `v1/question.ts`, `v1/legacy-event.ts` | `schema::v1::{permission, question, legacy_event}` |
| `protocol/api.ts: makeApi, makeDefaultApi` | `protocol::api::{make_api, make_default_api}` descriptors (route tables, not server) |
| `protocol/errors.ts: 13 TaggedErrors` | `protocol::errors::ProtocolError` enum + per-variant structs, `http_status()` verbatim |
| `protocol/middleware/*` | `protocol::middleware::{authorization::Authorization, schema_error::SchemaErrorMiddleware}` |
| `protocol/groups/* (18 groups)` | `protocol::groups::{…}` one module per group, `*Group` consts, query/param structs, `make_*_group` fns, preserved operation IDs (`health.get`, `provider.list`, …, `session.prompt`, `pty.connect`, `event.subscribe`), paths (`/api/…`), methods (GET/POST/PUT/PATCH/DELETE) |
| `protocol/groups/project-copy.ts: ProjectCopyError` | `protocol::groups::project_copy::ProjectCopyError` (stays here) |
| `protocol/groups/event.ts: makeEventGroup, EventGroup, OpenCodeEvent` | `protocol::groups::event::{make_event_group, EVENT_GROUP, OpenCodeEvent}` |
| `protocol/groups/pty.ts: PTY_CONNECT_* consts` | `protocol::groups::pty::{PTY_CONNECT_TICKET_QUERY, PTY_CONNECT_TOKEN_HEADER, PTY_CONNECT_TOKEN_HEADER_VALUE, PtyGroup}` |

## 5. Implementation sequence (parallel lanes + gates)

**Todo list (for tracker):**
- [ ] T0 Confirm 74+26 file list on disk (`ls`), extend §2 if barrels differ
- [ ] T1 Scaffold `crates/schema` + `crates/protocol` (`Cargo.toml` with serde/serde_json only), add to workspace `members`
- [ ] T2 Port `schema` primitives/IDs/events (S01–S07 + event/identifier)
- [ ] T3 Port `schema` domain structs/unions (S08–S55)
- [ ] T4 Port `schema/v1/*` compat
- [ ] T5 Port `protocol/errors + middleware + api`
- [ ] T6 Port `protocol/groups/*`
- [ ] T7 Update `PORTING_MAP.md` schema/protocol rows only
- [ ] T8 Leaf `@auditor` self-audit + report (build/test deferred to CI)

**Lanes (parallelizable after T1):**
- Lane A (schema core): T2 — no deps.
- Lane B (schema domains): T3 — depends on T2 (primitives/IDs).
- Lane C (v1 compat): T4 — depends on T2, parallel with B.
- Lane D (protocol errors/middleware/api): T5 — depends on T2 (IDs for params).
- Lane E (protocol groups): T6 — depends on T5 + B/C (schemas referenced in success/payload/query).
- Lane F (map + audit): T7–T8 after all lanes.

**Dependency graph:** T0→T1→{A,B,C,D}→E→F. B and C and D can run in parallel. E needs B+C+D.

**Verification gates:**
- G1 Attendance: every §2 row has a target module/type; `grep` for each source export name in Rust (allow snake file vs exact type name per §3.1).
- G2 JSON-shape parity: for each Struct/Union compare `serde_json::to_value` keys, `tag="type"` values, literal strings, optional omission vs null, `Record→object`, `Array→array`, branded→bare string. Spot-check `Credential.Value`, `SessionMessage.Message`, `v1.Part`, `SessionEvent.All/Durable`, `Integration.Method`.
- G3 Error/status parity: all 13 `httpApiStatus` codes + tags + field names + `ProjectCopyError` + 7 v1 `namedError`s byte-identical; operation IDs/paths/methods in groups byte-identical; `PTY_CONNECT_*`, `Delivery`, event `type` strings byte-identical.
- G4 No-diversion: no new pub names, no reordered variants/fields, no merged v1/v2, no added deps.

## 6. Parity / audit verification

- **Self-audit via `@auditor` (leaf, required):** after T7, delegate file-by-file attendance + G2/G3 spot checks + forbidden-read/write compliance. Auditor reports pass/fail per file; failures route back to translator, not to scope change.
- **Build/test explicitly DEFERRED to CI (reason):** no Rust toolchain in this environment and download rule forbids installs/fetches. Do NOT run `cargo build/test/check/fmt`. Record as deferred; CI covers it later. Translators must still write idiomatic, compiling-intent code (correct `serde` attrs, no missing imports) but must not attempt verification locally.
- **PORTING_MAP.md:** append/update ONLY schema/protocol rows (source path → crate path → status). No other rows.

## 7. Open questions / substitution requests (default: none)

1. **`thiserror` vs hand-rolled:** RECOMMEND hand-rolled `Display/Error` (zero new deps, exact strings guaranteed). If orchestrator prefers `thiserror 1`, approve explicitly — shape unchanged, only impl macro differs. FLAG awaiting approval; default = hand-rolled.
2. **`BTreeMap` vs `HashMap` for `Record`:** Use `BTreeMap` for deterministic tests; JSON shape identical. Not a substitution (wire format same). Noted for auditor.
3. **`Finite`/`DateTime` without `chrono`:** Use `f64`/`i64`+`String` primitives with custom (de)serializers matching `NumberFromString`/`DateTimeUtcFromMillis`. No new dep, no behavior change. If source reveals RFC3339 edge cases, flag rather than guess.
4. **No other substitutions.** If target language forces anything (e.g. keyword clash like `type`), use raw identifier `r#type` + `#[serde(rename="type")]` and flag in report — parity preserved.

## 8. Zero-diversion guarantee

We guarantee exact 1:1 clone: no functionality changes, no UI/UX redesign (N/A but no API redesign), no modernization, no renaming for taste (only mechanical kebab→snake file stems + `r#` escapes, type names verbatim), no merging/splitting files or types (v1/v2 kept separate, `ProjectCopyError` stays in its group), no reordering fields/variants/endpoints, no new defaults/keys/status codes, no extra dependencies, no scope expansion. Any ambiguity → flag and stop, route to orchestrator. Source is spec.

## Appendix — established Rust patterns (from cli/codemode reads only)

- `#[derive(Serialize,Deserialize,Clone,Debug,PartialEq)]` structs, `#[serde(tag="type")]` enums, `BTreeMap` for maps, `serde_json::Value` for unknowns, `Option<T>` optionals — followed in §3.2. No `thiserror` observed in sampled cli files — supports hand-rolled default.
