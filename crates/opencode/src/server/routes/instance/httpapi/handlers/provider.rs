// source: src/server/routes/instance/httpapi/handlers/provider.ts — exports: [providerHandlers]
// PROVISIONAL pending crates/core: `@opencode-ai/core/models-dev`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/http` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/httpapi` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/provider`
/// verbatim strings (source order, quoted for V2 audit):
/// - "@/provider/auth"
/// - "BadRequest"
/// - "provider"
/// - "ProviderHttpApi.list"
/// - "ProviderHttpApi.auth"
/// - "ProviderHttpApi.authorize"
/// - "ProviderHttpApi.authorizeRaw"
/// - "ProviderHttpApi.callback"
/// source: `export const providerHandlers` — shape as JSON value; CI verifies.
pub type providerHandlers = serde_json::Value;
