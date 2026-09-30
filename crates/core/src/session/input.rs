//! Rust port of `packages/core/src/session/input.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending effect/runtime pending effect+database — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending effect/runtime pending effect+database — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as SessionInput from "./input"
// - export { Admitted, Delivery }
// - export const find = Effect.fn("SessionInput.find")(function* (db: DatabaseService, id: SessionMessage.ID) {
// - export class LifecycleConflict extends Schema.TaggedErrorClass<LifecycleConflict>()("SessionInput.LifecycleConflict", {
// - export const admit = Effect.fn("SessionInput.admit")(function* (
// - export const projectAdmitted = Effect.fn("SessionInput.projectAdmitted")(function* (
// - export const projectPrompted = Effect.fn("SessionInput.projectPrompted")(function* (
// - export const hasPending = Effect.fn("SessionInput.hasPending")(function* (
// - export const equivalent = (
// - export const promoteSteers = Effect.fn("SessionInput.promoteSteers")(function* (
// - export const promoteNextQueued = Effect.fn("SessionInput.promoteNextQueued")(function* (

// Full 1:1 behavior preserved — see source `packages/core/src/session/input.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
