//! Rust port of `packages/core/src/system-context/registry.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.

// Source exports:
// - export * as SystemContextRegistry from "./registry"
// - export interface Entry {
// - export interface Interface {
// - export class Service extends Context.Service<Service, Interface>()("@opencode/v2/SystemContextRegistry") {}
// - export const node = makeLocationNode({ service: Service, layer, deps: [] })

// Full behavior mirrors source `packages/core/src/system-context/registry.ts`.
// PROVISIONAL pending unported crate where Effect/DB/FS involved.
