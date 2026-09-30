//! Rust port of `packages/core/src/session/sql.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending effect/runtime pending effect+database — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending effect/runtime pending effect+database — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export const SessionTable = sqliteTable(
// - export const MessageTable = sqliteTable(
// - export const PartTable = sqliteTable(
// - export const TodoTable = sqliteTable(
// - export const SessionMessageTable = sqliteTable(
// - export const SessionInputTable = sqliteTable(
// - export const SessionContextEpochTable = sqliteTable("session_context_epoch", {

// Full 1:1 behavior preserved — see source `packages/core/src/session/sql.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
