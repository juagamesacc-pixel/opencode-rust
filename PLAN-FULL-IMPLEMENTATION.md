# PLAN-FULL-IMPLEMENTATION — real logic behind stubs (opencode v1.18.30 → Rust)

> Status: CI GREEN baseline (run 36065495555). This plan replaces PROVISIONAL stub bodies and assert!(true) placeholders with REAL ported behavior. 1:1 fidelity stays: source is spec, no new features, no renames/merges. User approved full implementation.
> Planner lane errored on infra; plan authored directly by orchestrator from disk-verified triage (2026-09-24).

## 0. Pins (research gate — restated, final)

- Source: anomalyco/opencode v1.18.30 @ 3104c1428ec91f809e5ab86631300de41eb6952e, Bun 1.3.14 / TS 5.8.2.
- Target: Rust stable 1.98.1, workspace version 1.18.30, edition 2021.
- Deps: serde 1 + serde_json 1 + tokio 1 (async only where source is async). NO new deps without explicit user approval.

## 1. Triage inventory (measured 2026-09-24, read-only)

| Crate | unimplemented!() | PROVISIONAL markers | assert!(true) in tests |
|---|---|---|---|
| crates/app (src) | 0 / 0 files | 530 / 239 files | 570 / 80 files (tests/) |
| crates/core (src) | 6 / 3 files | 634 / 245 files | 1050 / 145 files (tests/) |
| crates/opencode (src) | 0 / 0 files | 721 / 293 files | 0 (descriptor-style tests) |
| TOTAL | 6 | ~1885 | ~1620 |

Read: app/opencode stubs are descriptor-shaped (constants/tables present, behavior bodies provisional); core has 6 hard unimplemented!() sites (fix first — they panic if hit). Test placeholders use the established `#![allow(clippy::all)]` pattern: bodies become real assertions only where implementation is real.

## 2. Architecture / dependency order

L1 core-foundation → L2 core-domain → L3 core-runtime → L4 opencode wiring → L5 app state machines. Later lanes consume earlier lanes' real types; no lane writes outside its scope; every lane ends with workspace CI green (never break the baseline).

- Effect mapping (unchanged doctrine): Effect.gen/async → tokio async ONLY where source async; sync Effect → sync fn. Layer/Context.Service → structs + build_* constructors with same dep order. Schema → serde (done). Streams → tokio mpsc/iterators where source streams. Unported natives (sqlite/ripgrep/pty/fff) stay PROVISIONAL behind explicit pending-crate markers until approved.
- Provisional wiring rule: an import marked `PROVISIONAL pending X` may be wired to the real crate ONLY if X is ported AND its API covers the use; otherwise it stays provisional with the marker. crates/core now exists → opencode's 293 core-pending sites are the first wiring candidates (verify API-by-API, never assume).

## 3. Lanes (max 4 parallel, DISJOINT write scopes)

| Lane | Scope (write) | Objective |
|---|---|---|
| L1 core-foundation | crates/core/src/{util,effect,id,flag,observability,installation}/** | Real logic for pure utils (glob/which/flock/memo/keyed/app-node); clear the 6 unimplemented!() sites. Port pure unit coverage to real asserts where now testable. |
| L2 core-domain | crates/core/src/{config,v1,database,event,account,credential,project,share,control_plane,catalog,integration,location*}** | Real config schemas/migrate, persistence tables, project/copy logic. |
| L3 core-runtime | crates/core/src/{session,tool,plugin,provider*,models_dev,aisdk,github_copilot,pty,filesystem,ripgrep,shell,process*,git,file*,snapshot,image,oauth,question,skill,system_context,reference}/** | Session lifecycle/runner, all tools, providers — the behavioral heart. Largest lane; may split into L3a/L3b if a single lane stalls. |
| L4 opencode wiring | crates/opencode/src/** | Wire core-pending imports to real crates/core API-by-API; implement pure logic bodies; keep unported-sibling stubs marked. |
| L5 app machines | crates/app/src/** | Explicit new()/update()/transition state machines for SolidJS reactive parts; keep Solid/tauri/wasm surfaces PROVISIONAL. |

Dependency graph: L1 → {L2, L3} → L4 → L5. L1 runs first alone; L2+L3 parallel after L1 green; L4 after L2+L3 green; L5 last. Each lane: inventory → implement in source order → rustfmt → self-audit → report; orchestrator commits+pushes per lane and watches CI to green before next lane starts.

## 4. Test strategy

- Placeholder test bodies become REAL assertions only for newly-real behavior in the same lane (same commit scope).
- Tests for still-provisional areas keep the `#![allow(clippy::all)]` + PROVISIONAL-marker pattern (CI stays green).
- No test is deleted or weakened: a case the lane cannot yet implement stays placeholder with its source-quoted expectation comment intact.

## 5. Gates per lane

(i) attendance unchanged (no files added/removed except lane-owned); (ii) behavior parity spot-checks vs source (3+ per lane, quoted in report); (iii) rustfmt clean on touched files; (iv) push → CI green before next lane. Lane done = (i)–(iv) attested with CI run link.

## 6. Open questions (via report, never wait_for_user; defaults = NO)

1. New crates (rusqlite/sqlx, reqwest, ignore/globset, etc.)? Default NO — stubs stay until approved.
2. Ripgrep/fff/pty/sqlite native bindings: stub or approved crate?
3. tokio scope: async only where source async (unchanged)? Confirm.
4. opencode→core wiring: source-wins where core_provisional.rs diverges from source? (Assumed yes.)
5. OnceLock for Layer memo-map globals? (Assumed yes, pending confirmation.)

## 7. Todo list (phase tracker)

- [x] triage inventory (this file §1)
- [ ] L1 core-foundation → CI green
- [ ] L2 core-domain → CI green
- [ ] L3 core-runtime → CI green
- [ ] L4 opencode wiring → CI green
- [ ] L5 app machines → CI green
- [ ] final attestation (behavior parity + map update)
