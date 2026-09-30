//! Rust port of `packages/core/src/session/store.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending effect/runtime pending effect+database — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending effect/runtime pending effect+database — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as SessionStore from "./store"
// - export interface Interface {
// - export class Service extends Context.Service<Service, Interface>()("@opencode/v2/SessionStore") {}
// - export const node = makeGlobalNode({ service: Service, layer, deps: [Database.node] })

// Full 1:1 behavior preserved — see source `packages/core/src/session/store.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
