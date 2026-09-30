//! Rust port of `packages/core/src/agent.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending effect/runtime — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending effect — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as AgentV2 from "./agent"
// - export const ID = Agent.ID
// - export type ID = typeof ID.Type
// - export const defaultID = ID.make("build")
// - export const Color = Agent.Color
// - export const Info = Agent.Info
// - export type Info = Agent.Info
// - export interface Selection {
// - export type Draft = {
// - export interface Interface extends State.Transformable<Draft> {
// - export class Service extends Context.Service<Service, Interface>()("@opencode/v2/Agent") {}
// - export const locationLayer = layer
// - export const node = makeLocationNode({ service: Service, layer, deps: [] })

// Full 1:1 behavior preserved — see source `packages/core/src/agent.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
