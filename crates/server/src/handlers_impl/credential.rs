//! Rust port of `packages/server/src/handlers/credential.ts` (opencode v1.18.30).
//!
//! Source 23 lines: two handlers `credential.update` and `credential.remove` via `Integration.Service.connection`.

pub const GROUP: &str = "server.credential";
pub const OPERATIONS: &[&str] = &["credential.update", "credential.remove"];
pub const SERVICE_ID: &str = "@opencode/Integration";
/// Both return `HttpApiSchema.NoContent`.
pub const RETURNS_NO_CONTENT: bool = true;
