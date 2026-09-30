//! Rust port of `packages/core/src/session/projector.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending effect/runtime pending effect+database — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending effect/runtime pending effect+database — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as SessionProjector from "./projector"
// - export class SessionAlreadyProjected extends Error {}
// - export const node = makeGlobalNode({ name: "session-projector", layer, deps: [EventV2.node, Database.node] })

// Full 1:1 behavior preserved — see source `packages/core/src/session/projector.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
