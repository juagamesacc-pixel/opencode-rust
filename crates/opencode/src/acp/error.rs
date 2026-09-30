// source: src/acp/error.ts — exports: SessionNotFoundError,
// InvalidConfigOptionError, InvalidModelError, InvalidEffortError,
// InvalidModeError, AuthRequiredError, UnknownAuthMethodError,
// UnsupportedOperationError, ServiceFailureError, Error, toRequestError,
// fromUnknownDefect
// PROVISIONAL pending @agentclientprotocol/sdk RequestError: tags + message
// templates + kind mapping verbatim.

use serde::{Deserialize, Serialize};

macro_rules! acp_error {
    ($name:ident, $tag:literal, { $($field:ident : $type:ty),* $(,)? }) => {
        /// source: error tag — verbatim.
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $name {
            $(pub $field: $type,)*
        }

        impl $name {
            pub fn tag() -> &'static str {
                $tag
            }
        }
    };
}

acp_error!(SessionNotFoundError, "ACPSessionNotFoundError", { session_id: String });
acp_error!(InvalidConfigOptionError, "ACPInvalidConfigOptionError", { config_id: String });
acp_error!(InvalidModelError, "ACPInvalidModelError", { model_id: String, provider_id: Option<String> });
acp_error!(InvalidEffortError, "ACPInvalidEffortError", { effort: String });
acp_error!(InvalidModeError, "ACPInvalidModeError", { mode: String });
acp_error!(AuthRequiredError, "ACPAuthRequiredError", { provider_id: Option<String> });
acp_error!(UnknownAuthMethodError, "ACPUnknownAuthMethodError", { method_id: String });
acp_error!(UnsupportedOperationError, "ACPUnsupportedOperationError", { method: String });
acp_error!(ServiceFailureError, "ACPServiceFailureError", {
    safe_message: String,
    service: Option<String>,
    error_name: Option<String>
});

/// source: toRequestError() messages — verbatim templates.
pub fn session_not_found_message(id: &str) -> String {
    format!("session not found: {}", id)
}
pub fn unknown_config_message(id: &str) -> String {
    format!("unknown config option: {}", id)
}
pub fn model_not_found_message(id: &str) -> String {
    format!("model not found: {}", id)
}
pub fn effort_not_found_message(effort: &str) -> String {
    format!("effort not found: {}", effort)
}
pub fn mode_not_found_message(mode: &str) -> String {
    format!("mode not found: {}", mode)
}
/// source: "provider authentication required" — verbatim.
pub const AUTH_REQUIRED_MESSAGE: &str = "provider authentication required";
pub fn unknown_auth_message(id: &str) -> String {
    format!("unknown auth method: {}", id)
}

/// source: toRequestError() kinds — verbatim per tag.
pub fn request_kind(tag: &str) -> &'static str {
    match tag {
        "ACPSessionNotFoundError"
        | "ACPInvalidConfigOptionError"
        | "ACPInvalidModelError"
        | "ACPInvalidEffortError"
        | "ACPInvalidModeError"
        | "ACPUnknownAuthMethodError" => "invalidParams",
        "ACPAuthRequiredError" => "authRequired",
        "ACPUnsupportedOperationError" => "methodNotFound",
        _ => "internalError",
    }
}

/// source: fromUnknownDefect() default "Internal service failure" — verbatim.
pub const DEFECT_DEFAULT_MESSAGE: &str = "Internal service failure";
