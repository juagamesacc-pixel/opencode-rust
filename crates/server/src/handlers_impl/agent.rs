//! Rust port of `packages/server/src/handlers/agent.ts` (opencode v1.18.30).
//!
//! Source 13 lines: `AgentHandler = HttpApiBuilder.group(Api, "server.agent", h => h.handle("agent.list", () => response(AgentV2.Service.use(agent=>agent.all()))))`
//!
//! PROVISIONAL: `AgentV2.Service` pending `crates/core`.

/// Group name verbatim.
pub const GROUP: &str = "server.agent";
/// Operation ID verbatim.
pub const OPERATION: &str = "agent.list";
/// Descriptor: location-wrapped response (mirrors `response(...)`).
pub const USES_LOCATION_RESPONSE: bool = true;
pub const SERVICE_ID: &str = "@opencode/Agent";
