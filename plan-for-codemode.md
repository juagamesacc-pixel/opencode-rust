# Plan-for-codemode — 1:1 Rust port of `@opencode-ai/codemode` v1.18.30

> Project root: `/root/opencode-rust` | New crate: `crates/codemode` (does NOT exist yet — plan creation only)
> Source spec: `anomalyco/opencode v1.18.30 @ 3104c1428ec91f809e5ab86631300de41eb6952e`, `Bun 1.3.14 / TS 5.8.2` | Target: `Rust 1.98.1`
> Package: `@opencode-ai/codemode v1.18.30` — "Effect-native confined code execution over schema-described tools"
> Doctrine: ZERO-DIVERSION 1:1 exact translation — same names/signatures/behavior/edge-cases/error-strings/config-keys/defaults/ordering. Source is spec. No improvement/modernization/rename-for-taste/merge/split/reorder. No scope change without approval.
> Allowed deps ONLY: `serde 1 + serde_json 1`, `tokio 1` only if async needed, `clap` NOT needed. No fetch, no new deps (incl. no parser crate) without approval.
> Reference pattern (read-only, do not copy code): `crates/cli` flat `src/*.rs`, `[[bin]]/lib` convention, pinned-dep `Cargo.toml`; `PORTING_MAP.md` codemode row (read-only here; update deferred).
> **Download rule (recorded):** Local time Asia/Kolkata at planning `2026-09-11 ~11:32 IST` (daytime) — CODE-WRITE ONLY lane: NO downloads, NO `rustup`/`cargo fetch`/`cargo build`, NO `bun`/`npm` installs. Build/test is DEFERRED, not executed.

## 1. Research gate (no web research needed)

1.1 Pinned versions are fixed above; source tree is spec — re-read before choosing any mapping.
1.2 Spec inputs to treat as normative:
- `Acorn 8.15.0` parser boundary (parse TS/JS → Acorn AST `Program` node; `Failed to parse TypeScript: …` / `Failed to parse script as a Program node.` errors; `loc` → `line/column` diagnostics). Do NOT add a Rust parser crate silently — flag parser-equivalence risk (§8).
- `effect` catalog: `Effect.Effect`, `Fiber`, `Schema`, `HttpClient.HttpClient`, `JSON Schema` render-only acceptance. Model as explicit Rust state machine with identical observable semantics (§3.5); do NOT introduce `tokio` unless promise-concurrency lane proves async is required, then request approval.
- Config assets: `package.json` (name/version/desc/deps peer `effect`, `acorn`), `tsconfig` if present, `codemode.md` + `AGENTS.md` (spec context only, not ported as code), `src/openapi/TODO.md` (port as `TODO.md` note / doc comment pointer, not behavior).
1.3 Pre-implementation re-read checklist (mandatory before each lane): file header + exports + error strings + defaults + ordering for that file. Record date/commit in attestation.

## 2. Full artifact inventory FIRST (attendance baseline)

2.1 `src/` — 25 × `.ts`:
- Top (7): `codemode.ts` (public `Input/DiagnosticKind/Diagnostic/Success/Failure/Result`, `ExecuteOptions/Options`, `execute/make`), `index.ts` (barrel re-exports → `lib.rs`), `tool.ts` (`Definition`, `isDefinition`, `make`), `tool-schema.ts` (`identifierSegment`, `toTypeScript/jsonSchemaToTypeScript`, `inputProperties/inputTypeScript/outputTypeScript`, `decodeInput/decodeOutput`), `tool-runtime.ts` (`ToolReference`, `ToolCall/Started/Ended`, `ToolDescription/SafeObject/DiscoveryPlan/SearchEntry`, `blockedMemberNames/isBlockedMember`, `copyIn/copyOut`, `catalog/searchIndex/assertValidTools/prepare/make`, `defaultCatalogBudget`, `reserved` root), `tool-error.ts` (`ToolError` + `toolError(message,cause?)` factory), `values.ts` (`SandboxPromise/Date/RegExp/Map/Set/URL/URLSearchParams`, `isSandboxValue`).
- `interpreter/` (2): `model.ts` (`SourcePosition/Location`, `AstNode/ProgramNode/Binding/StatementResult/MemberReference`, `CodeModeFunction/IntrinsicReference/ComputedValue`, `PromiseNamespace/MethodReference/PromiseMethodName`, `GlobalNamespace/MethodReference/GlobalNamespaceName`, `CoercionFunction/UriFunction`, `ProgramThrow/ErrorConstructorReference`, `DiagnosticKind`, `supportedSyntaxMessage/unsupportedSyntax()`, `isRecord/asNode/getArray/getString/getBoolean/getOptionalNode/getNode/sourceLocation/formatLocation`), `runtime.ts` **[3465 lines]** (`executeWithLimits` entry + ~117 `throw` sites: parse boundary, `instanceof`/string/array/console/date/global guards, loop `break/continue/return/throw`, `for await/of/in`, labels, destructuring, `NewExpression`/`RegExp/Map/Set/URL/URLSearchParams`, binary/logical/unary/in/`Unsupported…operator`, assignment/update targets, tools-only callability, spread, `Promise.*`, method dispatch, object/array spread, template literals, tool-path safety, property access, assignment/circularity/`URL.* read-only/TypeError`, scope stack `Unknown identifier/ReferenceError/TypeError`, timeout interruption `Unhandled rejection from an un-awaited tool call`).
- `openapi/` (4): `index.ts` (`fromSpec(options)→Result`), `runtime.ts` (`invoke(plan,input)`), `spec.ts` (`methods`, `isRecord/nonEmptyString/own/resolve/componentDefinitions/operationInput/inputSchema/operationOutput/operationPath/specServerUrl/validateBaseUrl/securityRequirements/operationSecurityRequirements/securitySchemes`), `types.ts` (16 types: `Document/Operation/SecurityScheme/Credential/AuthResolver/Options/Skipped/Tools/Result/InputLocation/InputField/Body/OperationInput/SecurityRequirement/Plan/AppliedAuth`), plus `TODO.md` (doc only).
- `stdlib/` (12): `collections.ts`, `console.ts` (`MAX_CONSOLE_DEPTH=32`), `date.ts`, `json.ts`, `math.ts`, `number.ts`, `object.ts`, `promise.ts` (`TOOL_CALL_CONCURRENCY=8`), `regexp.ts`, `string.ts`, `url.ts`, `value.ts`.
2.2 `test/` — 7 × `.test.ts` + 2 fixtures: `codemode`, `enumeration`, `stdlib`, `openapi`, `promise`, `parity`, `signature` + fixtures passthrough byte-identical.
2.3 Build/config: `package.json` (+ `tsconfig` if present); `codemode.md`, `AGENTS.md`, `src/openapi/TODO.md` — spec context only.

## 3. Architecture — flat crate mirroring `src` 1:1

3.1 Layout (flat `src/*.rs` per pilot `cli`; snake_case file names; NO merge/split/reorder):
- `Cargo.toml` (pinned `serde 1`, `serde_json 1`; `tokio 1` commented-out unless proven need + approval) + `src/lib.rs` ← `src/index.ts` barrel (re-export order = source order).
- `src/codemode.rs` ← `codemode.ts` | `src/tool.rs` ← `tool.ts` | `src/tool_schema.rs` ← `tool-schema.ts` | `src/tool_runtime.rs` ← `tool-runtime.ts` | `src/tool_error.rs` ← `tool-error.ts` | `src/values.rs` ← `values.ts`.
- `src/interpreter_model.rs` ← `interpreter/model.ts` | `src/interpreter_runtime.rs` ← `interpreter/runtime.ts` (single module; sequencing splits WORK not files).
- `src/openapi_index.rs` ← `openapi/index.ts` | `src/openapi_runtime.rs` ← `openapi/runtime.ts` | `src/openapi_spec.rs` ← `openapi/spec.ts` | `src/openapi_types.rs` ← `openapi/types.ts`.
- `src/stdlib_collections.rs, stdlib_console.rs, stdlib_date.rs, stdlib_json.rs, stdlib_math.rs, stdlib_number.rs, stdlib_object.rs, stdlib_promise.rs, stdlib_regexp.rs, stdlib_string.rs, stdlib_url.rs, stdlib_value.rs` ← `stdlib/*.ts` 1:1.
- `tests/` as `#[cfg(test)]` unit modules inside each `.rs` + integration `tests/*.rs` mirroring 7 test files 1:1 + `tests/fixtures/` byte-passthrough (2 files).
3.2 Error taxonomy: preserve `ToolError{message,cause}`, `InterpreterRuntimeError{message,node,diagnosticKind}` + `.as("TypeError"/"ReferenceError")` brand mapping, `ProgramThrow(value)`, `Diagnostic{kind,message,location}` + `DiagnosticKind` union copy, `supportedSyntaxMessage`, `unsupportedSyntax(kind,node)` strings verbatim; `copyIn/copyOut` boundary errors verbatim.
3.3 Value model: `SandboxDate/RegExp/Map/Set/URL/SearchParams/Promise`, `CodeModeFunction`, `IntrinsicReference/ComputedValue/...`, `SafeObject=Map<String,JsonValue>`; `isSandboxValue/boundedData/coerceToString/coerceToNumber/invokeCoercion`, `MAX_CONSOLE_DEPTH=32`, `TOOL_CALL_CONCURRENCY=8`, `copyIn/copyOut` incl. `NaN/Infinity→null` at boundary only, `undefined→null` flag, circular detection.
3.4 OpenAPI split: `openapi_types.rs` = pure types; `openapi_spec.rs` = pure spec parsing/resolution; `openapi_runtime.rs` = `invoke(plan,input)` execution; `openapi_index.rs` = `fromSpec` orchestration only.
3.5 Effect→state-machine: model `Effect`/`Fiber` as explicit enum `ExecState{Ready,BlockedOnTool,AwaitingPromise,Interrupted,Done}` + scope stack + tool-call ledger + log buffer + budget counters; preserve eager un-awaited start, settlement order, timeout interruption, `.then/.catch/.finally` await-inside error; keep `execute/make/prepare/catalog/searchIndex` ordering and `defaultCatalogBudget` verbatim.

## 4. Implementation sequence — parallel lanes + gates

- Phase 0 — Scaffolding + attendance: `crates/codemode/{Cargo.toml,src/lib.rs}` + empty 1:1 `.rs` stubs. Gate G0: file count match.
- Lane A (pure values/errors): `tool_error.rs` → `values.rs` + `stdlib/value.rs` → `interpreter_model.rs`. Gate GA: error-string grep 1:1.
- Lane B (schema/catalog, parallel with A after `tool_error`): `tool.rs` → `tool_schema.rs` → `tool_runtime.rs`. Gate GB: signature-test vectors match.
- Lane C (stdlib, parallel after A): `stdlib_*` in order `value→collections→string→number→math→date→regexp→json→object→url→console→promise`. Gate GC: stdlib/parity vectors.
- Lane D (OpenAPI, parallel after B): `openapi_types.rs` → `openapi_spec.rs` → `openapi_index.rs` → `openapi_runtime.rs`. Gate GD: openapi generation vectors.
- Lane E (interpreter_runtime — SEQUENCED): E1 parse boundary+`loc`; E2 scope/env/identifiers; E3 statements; E4 expressions; E5 member/call dispatch + tools-only guard; E6 stdlib wiring; E7 promises/concurrency; E8 budgets/logs/limits + `executeWithLimits`/`execute/make` + `codemode.rs` envelope. Gate GE per sub-step.
- Phase F — Test porting 1:1: 7 `tests/*.rs` + 2 fixtures passthrough. Gate GF: 7/7 files mapped.

## 5. Parity/audit verification

- V1 attendance: `25 src .ts + 7 tests + 2 fixtures + package.json/tsconfig + codemode.md/AGENTS.md/TODO.md` checklist.
- V2 string/key/default/order preservation: grep-diff error strings, config keys/defaults (`defaultCatalogBudget`, `MAX_CONSOLE_DEPTH=32`, `TOOL_CALL_CONCURRENCY=8`, `absent maxOutputBytes=no truncation`), export order = source order.
- V3 behavior/test equivalence: 7 tests ported 1:1 incl. fixtures byte-identical.
- V4 doctrine audit: no rename/merge/split/reorder/modernization; `serde/serde_json` only; parser boundary faithful.

## 6. Deferred build/test-and-fix gate (NOT executed — daytime, toolchain absent)

- When authorized: `cargo test -p codemode`; fix-and-retest preserving 1:1; record versions/log/failures→fixes.
- Self-audit gate: re-run V1–V4 + `git status` shows ONLY `crates/codemode/**` + codemode row of `PORTING_MAP.md`.

## 7. PORTING_MAP + attestation (deferred, codemode-row-only)

- Update ONLY the `codemode` row of `/root/opencode-rust/PORTING_MAP.md`; do NOT touch other rows.
- Attestation: source commit `3104c1428ec91f809e5ab86631300de41eb6952e` + `1.18.30`, counts (25+7+2), module map, lane/gate results, deferred build pointer, risks, zero-diversion statement.

## 8. Residual risks (flag for approval, do NOT silently resolve)

- R1 Acorn parser-equivalence: TS dialect/`loc` fidelity, `Program`-node gate — needs golden AST fixtures; NO parser crate without approval.
- R2 Effect→state-machine semantics: eager fibers, `race` cancellation/interruption, `allSettled` catch-no-throw, timeout propagation.
- R3 Unsupported-syntax taxonomy: `UnsupportedSyntax` vs `InterpreterRuntimeError` vs branded `TypeError/ReferenceError` triage must match throw-for-throw.
- R4 Binary boundary: `NaN/Infinity→null`, `undefined→null` flag, circular detection, `copyIn(preserveSandboxValues)` checkpoints, `URL.read-only/TypeError`, tool-ref opacity, `in`-operator guards.
