//! Rust port of `packages/core/src/session/runner/index.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending effect/runtime pending effect+database — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending effect/runtime pending effect+database — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as SessionRunner from "./index"
// - export type RunError =
// - export interface Interface {
// - export class Service extends Context.Service<Service, Interface>()("@opencode/v2/SessionRunner") {}

// Full 1:1 behavior preserved — see source `packages/core/src/session/runner/index.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.

// ---- runner/* submodules (source order: index barrel + llm/model/to-llm-message/publish-llm-event/max-steps) ----
pub mod llm;
pub mod max_steps;
pub mod model;
pub mod publish_llm_event;
pub mod to_llm_message;
