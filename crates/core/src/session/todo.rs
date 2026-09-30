//! Rust port of `packages/core/src/session/todo.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending effect/runtime pending effect+database — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending effect/runtime pending effect+database — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as SessionTodo from "./todo"
// - export const Info = SessionTodo.Info
// - export type Info = typeof Info.Type
// - export const Event = SessionTodo.Event
// - export interface Interface {
// - export class Service extends Context.Service<Service, Interface>()("@opencode/v2/SessionTodo") {}
// - export const node = makeLocationNode({ service: Service, layer, deps: [EventV2.node, Database.node] })

// Full 1:1 behavior preserved — see source `packages/core/src/session/todo.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
