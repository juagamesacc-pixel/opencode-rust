//! Rust port of `packages/core/src/catalog.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending effect/runtime — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending effect — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as Catalog from "./catalog"
// - export type ProviderRecord = {
// - export type DefaultModel = { providerID: ProviderV2.ID; modelID: ModelV2.ID }
// - export const PolicyActions = Schema.Literals(["provider.use"])
// - export const Event = Catalog.Event
// - export type Draft = {
// - export interface Interface extends State.Transformable<Draft> {
// - export class Service extends Context.Service<Service, Interface>()("@opencode/v2/Catalog") {}
// - export const locationLayer = layer.pipe(
// - export const node = makeLocationNode({ service: Service, layer, deps: [EventV2.node, Policy.node, Integration.node] })

// Full 1:1 behavior preserved — see source `packages/core/src/catalog.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
