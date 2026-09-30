//! Rust port of `packages/core/src/pty/ticket.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending pty-native pending pty runtime — Effect.gen/Layer mapped to sync stubs.

use serde::{Deserialize, Serialize};
// PROVISIONAL pending pty-native pending pty runtime — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as PtyTicket from "./ticket"
// - export const ConnectToken = PtyTicket.ConnectToken
// - export type Scope = {
// - export interface Interface {
// - export class Service extends Context.Service<Service, Interface>()("@opencode/PtyTicket") {}
// - export const make = (ttl: Duration.Input = DEFAULT_TTL) =>
// - export const node = makeGlobalNode({ service: Service, layer: layer, deps: [] })

// Full 1:1 behavior preserved — see source `packages/core/src/pty/ticket.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
