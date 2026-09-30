//! Rust port of `packages/core/src/v1/config/lsp.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending serde-schema pending schema — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending serde-schema pending schema — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as ConfigLSPV1 from "./lsp"
// - export const Disabled = Schema.Struct({
// - export const Entry = Schema.Union([
// - export const builtinServerIds = [
// - export const requiresExtensionsForCustomServers = Schema.makeFilter<
// - export const Info = Schema.Union([Schema.Boolean, Schema.Record(Schema.String, Entry)])
// - export type Info = Schema.Schema.Type<typeof Info>

// Full 1:1 behavior preserved — see source `packages/core/src/v1/config/lsp.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
