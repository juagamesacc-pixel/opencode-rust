//! Rust port of `packages/server/src/handlers/health.ts` (opencode v1.18.30).
//!
//! Source 7 lines: `HealthHandler.handle("health.get", ()=>succeed({healthy:true}))`.

pub const GROUP: &str = "server.health";
pub const OPERATION: &str = "health.get";

/// Response shape verbatim: `{ healthy: true }`.
#[derive(Clone, Debug, PartialEq)]
pub struct HealthResponse {
    pub healthy: bool,
}

pub fn health_response() -> HealthResponse {
    HealthResponse { healthy: true }
}
