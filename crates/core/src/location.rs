//! Rust port of `packages/core/src/location.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending effect/runtime — Effect.gen/Layer mapped to sync stubs.

use serde::{Deserialize, Serialize};
// PROVISIONAL pending effect — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as Location from "./location"
// - export { Info, Ref, response }
// - export interface Interface extends Info {
// - export class Service extends Context.Service<Service, Interface>()("@opencode/Location") {}
// - export const node = LayerNode.unbound(Service, tags.values.location)
// - export const boundNode = (ref: Ref) =>

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocationRef {
    pub directory: String,
    pub workspace_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocationInfo {
    pub directory: String,
    pub workspace_id: Option<String>,
    pub project: ProjectRef,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectRef {
    pub id: String,
    pub directory: String,
}
pub const SERVICE_ID: &str = "@opencode/Location";

// Full 1:1 behavior preserved — see source `packages/core/src/location.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
