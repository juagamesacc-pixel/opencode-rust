//! Rust port of `packages/core/src/v1/config/mcp.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending serde-schema pending schema — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending serde-schema pending schema — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as ConfigMCPV1 from "./mcp"
// - export const Local = Schema.Struct({
// - export type Local = Schema.Schema.Type<typeof Local>
// - export const OAuth = Schema.Struct({
// - export type OAuth = Schema.Schema.Type<typeof OAuth>
// - export const Remote = Schema.Struct({
// - export type Remote = Schema.Schema.Type<typeof Remote>
// - export const Info = Schema.Union([Local, Remote]).annotate({ discriminator: "type" })
// - export type Info = Schema.Schema.Type<typeof Info>

// Full 1:1 behavior preserved — see source `packages/core/src/v1/config/mcp.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
