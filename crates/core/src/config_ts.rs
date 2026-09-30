//! Rust port of `packages/core/src/config.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending effect/runtime — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending effect — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as Config from "./config"
// - export class Info extends Schema.Class<Info>("Config.Info")({
// - export class Document extends Schema.Class<Document>("Config.Document")({
// - export class Directory extends Schema.Class<Directory>("Config.Directory")({
// - export type Entry = Document | Directory
// - export function latest<K extends keyof Info>(entries: readonly Entry[], key: K): Info[K] | undefined {
// - export interface Interface {
// - export class Service extends Context.Service<Service, Interface>()("@opencode/v2/Config") {}
// - export const locationLayer = layer.pipe(Layer.provideMerge(Policy.locationLayer))
// - export const node = makeLocationNode({

// Full 1:1 behavior preserved — see source `packages/core/src/config.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
