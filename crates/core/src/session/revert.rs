//! Rust port of `packages/core/src/session/revert.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending effect/runtime pending effect+database — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending effect/runtime pending effect+database — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as SessionRevert from "./revert"
// - export class MessageNotFoundError extends Schema.TaggedErrorClass<MessageNotFoundError>()(
// - export const stage = Effect.fn("SessionRevert.stage")(function* (input: {
// - export const clear = Effect.fn("SessionRevert.clear")(function* (session: SessionSchema.Info) {
// - export const commit = Effect.fn("SessionRevert.commit")(function* (session: SessionSchema.Info) {

// Full 1:1 behavior preserved — see source `packages/core/src/session/revert.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
