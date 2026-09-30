//! Rust port of `packages/server/src/handlers/model.ts` (opencode v1.18.30).
//!
//! Source 14 lines: `ModelHandler.handle("model.list", fn* => response(catalog.model.available()))`
//!
//! PROVISIONAL: `Catalog.Service` pending `crates/core`.

pub const GROUP: &str = "server.model";
pub const OPERATION: &str = "model.list";
pub const SERVICE_ID: &str = "@opencode/Catalog";
pub const USES_LOCATION_RESPONSE: bool = true;
