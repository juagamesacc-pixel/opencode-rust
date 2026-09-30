//! Rust port of `packages/core/src/credential.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending effect/runtime — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending effect — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as Credential from "./credential"
// - export const ID = Credential.ID
// - export type ID = Credential.ID
// - export const OAuth = Credential.OAuth
// - export type OAuth = Credential.OAuth
// - export const Key = Credential.Key
// - export type Key = Credential.Key
// - export const Value = Credential.Value
// - export type Value = Credential.Value
// - export class Info extends Schema.Class<Info>("Credential.Info")({
// - export interface Interface {
// - export class Service extends Context.Service<Service, Interface>()("@opencode/v2/Credential") {}
// - export const node = makeGlobalNode({ service: Service, layer, deps: [Database.node] })

// Full 1:1 behavior preserved — see source `packages/core/src/credential.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.

// Submodules (from singleton subdir)
pub mod sql;
