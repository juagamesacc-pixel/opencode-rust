// source: src/plugin/xai.ts — exports: accessTokenIsExpiring,
// DeviceCodeResponse, requestDeviceCode, pollDeviceCodeToken, XaiAuthPlugin
// (+ options/consts per source; PROVISIONAL: fetch/device-flow as descriptors)
// Unsigned JWT expiry check + refresh-skew default verbatim.

use serde::{Deserialize, Serialize};

/// source: DeviceCodeResponse — verbatim snake_case fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCodeResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_uri_complete: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<u64>,
}

/// source: accessTokenIsExpiring() — non-string/opaque → false; exp*1000 <=
/// now + max(0, skew). Verbatim.
pub fn access_token_is_expiring(exp_ms: Option<i64>, now_ms: i64, skew_ms: i64) -> bool {
    match exp_ms {
        None => false,
        Some(exp) => exp <= now_ms + skew_ms.max(0),
    }
}

/// source: `xAI token refresh failed (${status})${detail}` — verbatim template.
pub fn refresh_failed_message(status: u16, detail: Option<&str>) -> String {
    match detail.filter(|d| !d.is_empty()) {
        Some(d) => format!("xAI token refresh failed ({})： {}", status, d),
        None => format!("xAI token refresh failed ({})", status),
    }
}

/// source: grant_type "refresh_token" — verbatim.
pub const GRANT_REFRESH: &str = "refresh_token";
