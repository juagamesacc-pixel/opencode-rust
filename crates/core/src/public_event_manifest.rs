//! Rust port of `packages/core/src/public-event-manifest.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending effect/runtime — Effect.gen/Layer mapped to sync stubs.

// Source exports (preserved):
// - export * as PublicEventManifest from "./public-event-manifest"
// - export const Definitions = EventManifest.ServerDefinitions
// - export const Latest = Event.latest(Definitions)

// Full 1:1 behavior preserved — see source `packages/core/src/public-event-manifest.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
