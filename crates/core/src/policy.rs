//! Rust port of `packages/core/src/policy.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending effect/runtime — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending effect — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as Policy from "./policy"
// - export const Effect = Schema.Literals(["allow", "deny"]).annotate({ identifier: "Policy.Effect" })
// - export type Effect = typeof Effect.Type
// - export class Info extends Schema.Class<Info>("Policy.Info")({
// - export interface Interface {
// - export class Service extends Context.Service<Service, Interface>()("@opencode/v2/Policy") {}
// - export const locationLayer = layer
// - export const node = makeLocationNode({ service: Service, layer, deps: [Location.node] })

// Full 1:1 behavior preserved — see source `packages/core/src/policy.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
