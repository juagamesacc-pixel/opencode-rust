// source: src/server/routes/instance/httpapi/groups/provider.ts — exports: [ProviderAuthApiError, ProviderApi]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/httpapi` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/provider`
/// verbatim strings (source order, quoted for V2 audit):
/// - "@/provider/auth"
/// - "/provider"
/// - "BadRequest"
/// - "ProviderAuthOauthMissing"
/// - "ProviderAuthOauthCodeMissing"
/// - "ProviderAuthOauthCallbackFailed"
/// - "ProviderAuthValidationFailed"
/// - "ProviderAuthError"
use serde::{Deserialize, Serialize};

/// source: `export class ProviderAuthApiError` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderAuthApiError {
    pub value: serde_json::Value,
}
/// source: `export const ProviderApi` — shape as JSON value; CI verifies.
pub type ProviderApi = serde_json::Value;
