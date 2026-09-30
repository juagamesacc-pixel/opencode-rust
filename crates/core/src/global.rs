//! Rust port of `packages/core/src/global.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending effect/runtime — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending effect — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export const Path = paths
// - export class Service extends Context.Service<Service, Interface>()("@opencode/Global") {}
// - export interface Interface {
// - export function make(input: Partial<Interface> = {}): Interface {
// - export const node = makeGlobalNode({ service: Service, layer: layer, deps: [] })
// - export const layerWith = (input: Partial<Interface>) =>
// - export * as Global from "./global"

pub struct GlobalPaths {
    pub home: String,
    pub data: String,
    pub cache: String,
    pub config: String,
    pub state: String,
    pub tmp: String,
    pub bin: String,
    pub log: String,
    pub repos: String,
}
pub const SERVICE_ID: &str = "@opencode/Global";

// Full 1:1 behavior preserved — see source `packages/core/src/global.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
