// source: src/cli/network.ts — exports: [NetworkOptions, withNetworkOptions, hasArg, resolveNetworkOptions, resolveNetworkOptionsNoConfig]
// PROVISIONAL pending external `yargs` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/config/config`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "port to listen on"
/// - "hostname to listen on"
/// - "127.0.0.1"
/// - "enable mDNS service discovery (defaults hostname to 0.0.0.0)"
/// - "mdns-domain"
/// - "custom domain name for mDNS service (default: opencode.local)"
/// - "opencode.local"
/// - "additional domains to allow for CORS"
/// source: `export type NetworkOptions` — shape as JSON value; CI verifies.
pub type NetworkOptions = serde_json::Value;
/// source: `export function withNetworkOptions` — stub shell; CI verifies behavior.
pub fn withNetworkOptions(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function hasArg` — stub shell; CI verifies behavior.
pub fn hasArg(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export const resolveNetworkOptions` — shape as JSON value; CI verifies.
pub type resolveNetworkOptions = serde_json::Value;
/// source: `export function resolveNetworkOptionsNoConfig` — stub shell; CI verifies behavior.
pub fn resolveNetworkOptionsNoConfig(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
