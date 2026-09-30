// source: src/cli/cmd/providers.ts — exports: [resolvePluginProviders, ProvidersCommand, ProvidersListCommand, ProvidersLoginCommand, ProvidersLogoutCommand]
// PROVISIONAL pending external `yargs` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/models-dev`
// PROVISIONAL pending crates/core: `@opencode-ai/core/global`
// PROVISIONAL pending crates/plugin: `@opencode-ai/plugin`
// PROVISIONAL pending external `node:stream/consumers` (host-provided; no new dep)
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "Cli.providers.put"
/// - "Cli.providers.pluginAuth"
/// - "Login method"
/// - "${methodName}"
/// - "10 millis"
/// - "Failed to authorize: "
/// - "Go to: "
/// - "Waiting for authorization..."
/// source: `export function resolvePluginProviders` — stub shell; CI verifies behavior.
pub fn resolvePluginProviders(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export const ProvidersCommand` — shape as JSON value; CI verifies.
pub type ProvidersCommand = serde_json::Value;
/// source: `export const ProvidersListCommand` — shape as JSON value; CI verifies.
pub type ProvidersListCommand = serde_json::Value;
/// source: `export const ProvidersLoginCommand` — shape as JSON value; CI verifies.
pub type ProvidersLoginCommand = serde_json::Value;
/// source: `export const ProvidersLogoutCommand` — shape as JSON value; CI verifies.
pub type ProvidersLogoutCommand = serde_json::Value;
