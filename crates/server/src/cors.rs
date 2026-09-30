//! Rust port of `packages/server/src/cors.ts` (opencode v1.18.30).
//!
//! Source 34 lines. Exports: `CorsOptions`, `CorsConfig` (`"@opencode/ServerCorsConfig"`),
//! `isAllowedCorsOrigin`, `isAllowedRequestOrigin`, private `sameHost`.
//!
//! 1:1 notes:
//! - `opencodeOrigin = /^https:\/\/([a-z0-9-]+\.)*opencode\.ai$/` copied verbatim.
//! - All string literals and early-return order preserved.
//! - `sameHost` parses origin with URL host compare, fallback false on parse error.

/// Mirrors `type CorsOptions = { cors?: string[] }`.
#[derive(Clone, Debug, PartialEq)]
pub struct CorsOptions {
    pub cors: Option<Vec<String>>,
}

/// Service id verbatim: `"@opencode/ServerCorsConfig"`. `Context.Reference` default is `undefined`.
pub const CORS_CONFIG_SERVICE_ID: &str = "@opencode/ServerCorsConfig";

/// Descriptor for the reference (mirrors `Context.Reference<CorsOptions|undefined>`).
#[derive(Clone, Debug, PartialEq)]
pub struct CorsConfig(pub Option<CorsOptions>);

impl CorsConfig {
    pub fn service_id() -> &'static str {
        CORS_CONFIG_SERVICE_ID
    }
}

fn is_opencode_origin(input: &str) -> bool {
    // Regex: ^https://([a-z0-9-]+\.)*opencode\.ai$
    if !input.starts_with("https://") {
        return false;
    }
    let rest = &input[8..];
    if rest == "opencode.ai" {
        return true;
    }
    if let Some(stripped) = rest.strip_suffix("opencode.ai") {
        if !stripped.ends_with('.') {
            return false;
        }
        let prefix = &stripped[..stripped.len() - 1];
        if prefix.is_empty() {
            return false;
        }
        // Each label in prefix must be [a-z0-9-]+ and labels dot-separated.
        // The regex `([a-z0-9-]+\.)*` means zero or more labels each ending with dot.
        for label in prefix.split('.') {
            if label.is_empty() {
                return false;
            }
            if !label
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
            {
                return false;
            }
        }
        return true;
    }
    false
}

/// Port of `isAllowedCorsOrigin(input, opts)` — early-return order verbatim.
pub fn is_allowed_cors_origin(input: Option<&str>, opts: Option<&CorsOptions>) -> bool {
    let input = match input {
        None => return true,
        Some(s) => s,
    };
    if input.starts_with("http://localhost:") {
        return true;
    }
    if input.starts_with("http://127.0.0.1:") {
        return true;
    }
    if input.starts_with("oc://renderer") {
        return true;
    }
    if input == "tauri://localhost"
        || input == "http://tauri.localhost"
        || input == "https://tauri.localhost"
    {
        return true;
    }
    if is_opencode_origin(input) {
        return true;
    }
    match opts.and_then(|o| o.cors.as_ref()) {
        Some(list) => list.iter().any(|s| s == input),
        None => false,
    }
}

/// Port of `isAllowedRequestOrigin(input, host, opts)` — host compare via `sameHost`.
pub fn is_allowed_request_origin(
    input: Option<&str>,
    host: Option<&str>,
    opts: Option<&CorsOptions>,
) -> bool {
    let input = match input {
        None => return true,
        Some(s) => s,
    };
    if let Some(h) = host {
        if same_host(input, h) {
            return true;
        }
    }
    is_allowed_cors_origin(Some(input), opts)
}

fn same_host(origin: &str, host: &str) -> bool {
    match parse_host(origin) {
        Some(h) => h == host,
        None => false,
    }
}

fn parse_host(origin: &str) -> Option<String> {
    // Minimal URL host extraction (mirrors `new URL(origin).host`).
    let scheme_end = origin.find("://")?;
    let rest = &origin[scheme_end + 3..];
    let host_and_rest = rest.split('/').next().unwrap_or(rest);
    let host = host_and_rest.split('?').next().unwrap_or(host_and_rest);
    let host = host.split('#').next().unwrap_or(host);
    if host.is_empty() {
        None
    } else {
        Some(host.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opencode_origin_cases() {
        assert!(is_opencode_origin("https://opencode.ai"));
        assert!(is_opencode_origin("https://foo.opencode.ai"));
        assert!(is_opencode_origin("https://a-b123.opencode.ai"));
        assert!(is_opencode_origin("https://foo.bar.opencode.ai"));
        assert!(!is_opencode_origin("https://opencode.ai.evil.com"));
        assert!(!is_opencode_origin("http://opencode.ai"));
        assert!(!is_opencode_origin("https://OPencode.ai"));
    }

    #[test]
    fn allowed_localhost() {
        assert!(is_allowed_cors_origin(Some("http://localhost:3000"), None));
        assert!(is_allowed_cors_origin(Some("http://127.0.0.1:4000"), None));
        assert!(is_allowed_cors_origin(Some("oc://renderer/foo"), None));
        assert!(is_allowed_cors_origin(Some("tauri://localhost"), None));
        assert!(is_allowed_cors_origin(None, None));
    }
}
