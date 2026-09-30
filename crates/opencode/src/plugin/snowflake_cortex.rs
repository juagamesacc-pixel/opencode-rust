// source: src/plugin/snowflake-cortex.ts — exports: oauthScope,
// SnowflakeCortexAuthPlugin (+ consts per source; PROVISIONAL: PKCE/fetch/
// callback-server as descriptors). Scope rule + Basic header verbatim.

/// source: oauthScope() — none → "refresh_token"; safe → role; else
/// role-encoded. Verbatim.
pub fn oauth_scope(role: Option<&str>) -> String {
    match role {
        None => "refresh_token".to_string(),
        Some(r)
            if r.chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') =>
        {
            format!("refresh_token session:role:{}", r)
        }
        Some(r) => format!("refresh_token session:role-encoded:{}", percent_encode(r)),
    }
}

fn percent_encode(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{:02X}", b));
        }
    }
    out
}

/// source: response_type "code", code_challenge_method "S256" — verbatim.
pub const RESPONSE_TYPE: &str = "code";
pub const CHALLENGE_METHOD: &str = "S256";

/// source: token-request path — verbatim suffix.
pub const TOKEN_PATH: &str = "/oauth/token-request";
/// source: authorize path — verbatim suffix.
pub const AUTHORIZE_PATH: &str = "/oauth/authorize";

/// source: Basic header client_id:client_id base64 — verbatim construction rule.
pub const BASIC_PREFIX: &str = "Basic ";
