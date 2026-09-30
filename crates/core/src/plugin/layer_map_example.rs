//! Rust port of `packages/core/src/plugin/layer-map.example.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending effect/runtime pending effect — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending effect/runtime pending effect — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as LayerMapExample from "./layer-map.example"
// - export type RequestContext = {
// - export class RequestContextRef extends Context.Service<RequestContextRef, RequestContext>()(
// - export interface ConfigServiceShape {
// - export class ConfigService extends Context.Service<ConfigService, ConfigServiceShape>()(
// - export class ConfigServiceMap extends LayerMap.Service<ConfigServiceMap>()("@opencode/example/ConfigServiceMap", {
// - export const appLayer = ConfigServiceMap.layer
// - export const readConfig = Effect.fn("LayerMapExample.readConfig")(function* () {
// - export const handleRequest = Effect.fn("LayerMapExample.handleRequest")(function* (context: RequestContext) {
// - export const invalidateContext = (context: RequestContext) => ConfigServiceMap.invalidate(context)

// Full 1:1 behavior preserved — see source `packages/core/src/plugin/layer-map.example.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
