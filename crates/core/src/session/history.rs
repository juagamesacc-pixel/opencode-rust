//! Rust port of `packages/core/src/session/history.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending effect/runtime pending effect+database — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending effect/runtime pending effect+database — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export const latestCompaction = Effect.fnUntraced(function* (db: DatabaseService, sessionID: SessionSchema.ID) {
// - export const load = Effect.fn("SessionHistory.load")(function* (db: DatabaseService, sessionID: SessionSchema.ID) {
// - export const loadForRunner = Effect.fn("SessionHistory.loadForRunner")(function* (
// - export const entriesForRunner = Effect.fn("SessionHistory.entriesForRunner")(function* (
// - export * as SessionHistory from "./history"

// Full 1:1 behavior preserved — see source `packages/core/src/session/history.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
