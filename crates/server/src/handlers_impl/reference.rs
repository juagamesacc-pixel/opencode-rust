//! Rust port of `packages/server/src/handlers/reference.ts` (opencode v1.18.30).
//!
//! Source 7 lines: `ReferenceHandler.handle("reference.list", ()=>response(Reference.Service.use(ref=>ref.list())))`
//!
//! PROVISIONAL: `Reference.Service` pending `crates/core`.

pub const GROUP: &str = "server.reference";
pub const OPERATION: &str = "reference.list";
pub const USES_LOCATION_RESPONSE: bool = true;
pub const SERVICE_ID: &str = "@opencode/Reference";
