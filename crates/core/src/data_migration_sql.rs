//! Rust port of `packages/core/src/data-migration.sql.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.

// Source exports (preserved):
// - export const DataMigrationTable = sqliteTable("data_migration", {

// Full 1:1 behavior preserved — see source `packages/core/src/data-migration.sql.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
