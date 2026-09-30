// source: src/server/routes/instance/httpapi/groups/config.ts — exports: [ConfigApi]
// PROVISIONAL pending crates/core: `@opencode-ai/core/v1/config/config`
// PROVISIONAL pending external `effect/unstable/httpapi` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@/config/config"
/// - "/config"
/// - "Get config info"
/// - "config.get"
/// - "Get configuration"
/// - "Retrieve the current OpenCode configuration settings and preferences."
/// - "Successfully updated config"
/// - "config.update"
/// source: `export const ConfigApi` — shape as JSON value; CI verifies.
pub type ConfigApi = serde_json::Value;
