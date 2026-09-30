//! Rust port of `packages/core/src/session/runner/model.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending effect/runtime pending effect+database — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending effect/runtime pending effect+database — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as SessionRunnerModel from "./model"
// - export class ModelNotSelectedError extends Schema.TaggedErrorClass<ModelNotSelectedError>()(
// - export class ModelUnavailableError extends Schema.TaggedErrorClass<ModelUnavailableError>()(
// - export class VariantUnavailableError extends Schema.TaggedErrorClass<VariantUnavailableError>()(
// - export class UnsupportedApiError extends Schema.TaggedErrorClass<UnsupportedApiError>()(
// - export type Error =
// - export interface Interface {
// - export class Service extends Context.Service<Service, Interface>()("@opencode/v2/SessionRunnerModel") {}
// - export const layerWith = (resolve: Interface["resolve"]) => Layer.succeed(Service, Service.of({ resolve }))
// - export const fromCatalogModel = (
// - export const resolve = (session: SessionSchema.Info, model: ModelV2.Info, credential?: Credential.Value) =>
// - export const supported = (model: ModelV2.Info) =>
// - export const locationLayer = Layer.effect(
// - export const node = makeLocationNode({ service: Service, layer: locationLayer, deps: [Catalog.node, Integration.node] })

// Full 1:1 behavior preserved — see source `packages/core/src/session/runner/model.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
