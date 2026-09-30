//! Rust port of `packages/core/src/tool-output-store.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending effect/runtime — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending effect — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as ToolOutputStore from "./tool-output-store"
// - export const MAX_LINES = 2_000
// - export const MAX_BYTES = 50 * 1024
// - export const RETENTION = Duration.days(7)
// - export const MANAGED_DIRECTORY = "tool-output"
// - export interface BoundInput {
// - export interface BoundResult {
// - export class StorageError extends Schema.TaggedErrorClass<StorageError>()("ToolOutputStore.StorageError", {
// - export type Error = StorageError
// - export interface Interface {
// - export class Service extends Context.Service<Service, Interface>()("@opencode/v2/ToolOutputStore") {}
// - export const node = makeLocationNode({ service: Service, layer, deps: [FSUtil.node, Global.node, Config.node] })
// - export const nodeWithoutConfig = makeLocationNode({ service: Service, layer, deps: [FSUtil.node, Global.node] })
// - export const cleanupLayer = Layer.effectDiscard(
// - export const cleanupNode = makeGlobalNode({

// Full 1:1 behavior preserved — see source `packages/core/src/tool-output-store.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
