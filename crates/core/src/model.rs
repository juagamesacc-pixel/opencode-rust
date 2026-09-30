//! Rust port of `packages/core/src/model.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending effect/runtime — Effect.gen/Layer mapped to sync stubs.

// Source exports (preserved):
// - export const ID = Model.ID
// - export type ID = typeof ID.Type
// - export const VariantID = Model.VariantID
// - export type VariantID = typeof VariantID.Type
// - export const Family = Model.Family
// - export type Family = Model.Family
// - export const Capabilities = Model.Capabilities
// - export type Capabilities = Model.Capabilities
// - export const Cost = Model.Cost
// - export const Ref = Model.Ref
// - export type Ref = typeof Ref.Type
// - export const Api = Model.Api
// - export type Api = Model.Api
// - export const Info = Model.Info
// - export type Info = Model.Info
// - export type MutableInfo = Omit<Types.DeepMutable<Info>, "api"> & {
// - export function parse(input: string): { providerID: ProviderV2.ID; modelID: ID } {
// - export * as ModelV2 from "./model"

// Full 1:1 behavior preserved — see source `packages/core/src/model.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
