//! Rust port of `packages/server/src/middleware/authorization.ts` (opencode v1.18.30).
//!
//! Source 58 lines. Re-exports `Authorization` from protocol, defines
//! `AUTH_TOKEN_QUERY="auth_token"`, `WWW_AUTHENTICATE='Basic realm="Secure Area"'`,
//! `emptyCredential`, `decodeCredential`, `credentialFromRequest`, and
//! `authorizationLayer`.
//!
//! 1:1 notes:
//! - `Authorization` service id is `"@opencode/HttpApiAuthorization"` (protocol).
//! - `emptyCredential() => { username:"", password: Redacted.make("") }` verbatim.
//! - `decodeCredential` decodes base64, splits on first ":", else empty credential.
//! - `credentialFromRequest` checks `auth_token` query first, else `Authorization: Basic`.
//! - Layer skips auth when `ServerAuth.required` false; else checks `hasPtyConnectTicketURL`
//!   then credential; on failure appends `www-authenticate` and fails `UnauthorizedError`.

use crate::auth::{authorized, required, Config, DecodedCredentials, Info, Redacted};
use protocol::middleware::authorization::AUTHORIZATION_SERVICE_ID;

/// Query param name verbatim: `"auth_token"`.
pub const AUTH_TOKEN_QUERY: &str = "auth_token";

/// Header verbatim: `'Basic realm="Secure Area"'`.
pub const WWW_AUTHENTICATE: &str = r#"Basic realm="Secure Area""#;

/// Port of `function emptyCredential()`.
pub fn empty_credential() -> DecodedCredentials {
    DecodedCredentials {
        username: String::new(),
        password: Redacted::make(""),
    }
}

const B64_ALPHABET_STD: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn b64_value(b: u8) -> Option<u32> {
    match b {
        b'A'..=b'Z' => Some((b - b'A') as u32),
        b'a'..=b'z' => Some((b - b'a' + 26) as u32),
        b'0'..=b'9' => Some((b - b'0' + 52) as u32),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

fn base64_decode(input: &str) -> Option<Vec<u8>> {
    let s = input.trim();
    // Strip padding '=' for decode; length must be multiple of 4 after padding in standard.
    let stripped = s.trim_end_matches('=');
    // Use a simple decoder that handles missing padding gracefully (Buffer.from behavior).
    let bytes = stripped.as_bytes();
    let mut out = Vec::new();
    let mut buf: u32 = 0;
    let mut bits: u32 = 0;
    for &b in bytes {
        let v = b64_value(b)?;
        buf = (buf << 6) | v;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push(((buf >> bits) & 0xFF) as u8);
        }
    }
    Some(out)
}

/// Port of `function decodeCredential(input)` — base64 decode, split on ":".
pub fn decode_credential(input: &str) -> DecodedCredentials {
    let header_bytes = match base64_decode(input) {
        Some(b) => b,
        None => return empty_credential(),
    };
    let header = match String::from_utf8(header_bytes) {
        Ok(s) => s,
        Err(_) => return empty_credential(),
    };
    match header.find(':') {
        Some(idx) => DecodedCredentials {
            username: header[..idx].to_string(),
            password: Redacted::make(header[idx + 1..].to_string()),
        },
        None => empty_credential(),
    }
}

/// Port of `function credentialFromRequest(request)` — pure over URL + headers.
pub fn credential_from_request(
    url: &str,
    headers: &std::collections::HashMap<String, String>,
) -> DecodedCredentials {
    // Check auth_token query first (case-sensitive param name "auth_token").
    if let Some(token) = parse_query_param(url, AUTH_TOKEN_QUERY) {
        return decode_credential(&token);
    }
    // Then Authorization: Basic header (case-insensitive "Basic" prefix, per /^Basic\s+(.+)$/i)
    if let Some(auth) = headers
        .get("authorization")
        .or_else(|| headers.get("Authorization"))
    {
        if let Some(captured) = capture_basic(auth) {
            return decode_credential(&captured);
        }
    }
    empty_credential()
}

fn capture_basic(header: &str) -> Option<String> {
    let s = header.trim();
    if s.len() < 5 {
        return None;
    }
    let prefix = &s[..5];
    if !prefix.eq_ignore_ascii_case("Basic") {
        return None;
    }
    let rest = s[5..].trim_start();
    if rest.is_empty() {
        return None;
    }
    Some(rest.split_whitespace().next().unwrap_or("").to_string())
}

fn parse_query_param(url: &str, key: &str) -> Option<String> {
    let q_start = url.find('?')?;
    let query = &url[q_start + 1..];
    let query = query.split('#').next().unwrap_or(query);
    for pair in query.split('&') {
        let (k, v) = match pair.find('=') {
            Some(i) => (&pair[..i], &pair[i + 1..]),
            None => (pair, ""),
        };
        if k == key {
            // Query value is percent-decoded in real URL; for auth_token we treat raw base64
            // (percent-decode would be a no-op for base64 chars). Match TS `url.searchParams.get`.
            return Some(percent_decode(v));
        }
    }
    None
}

fn percent_decode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let hi = hex_val(bytes[i + 1]);
            let lo = hex_val(bytes[i + 2]);
            if let (Some(h), Some(l)) = (hi, lo) {
                out.push((h << 4 | l) as char);
                i += 3;
                continue;
            }
        }
        if bytes[i] == b'+' {
            out.push(' ');
        } else {
            out.push(bytes[i] as char);
        }
        i += 1;
    }
    out
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

/// Port of `hasPtyConnectTicketURL` check — import from `protocol::groups::pty`.
///
/// Returns true if the URL contains the PTY connect ticket query param.
pub fn has_pty_connect_ticket_url(url: &str) -> bool {
    // Mirrors `hasPtyConnectTicketURL(new URL(request.url, "http://localhost"))`
    // which checks for `PTY_CONNECT_TICKET_QUERY` ("pty_ticket") presence.
    parse_query_param(url, "pty_ticket").is_some()
}

/// Authorization decision (pure): mirrors the `Authorization.of((effect)=>...)` logic.
///
/// Returns: `Ok(())` means proceed (`yield* effect`), `Err(Unauthorized)` means fail.
#[derive(Debug, Clone, PartialEq)]
pub struct UnauthorizedError {
    pub message: String,
}

pub fn authorization_check(
    url: &str,
    headers: &std::collections::HashMap<String, String>,
    config: &Info,
) -> Result<(), UnauthorizedError> {
    if !required(config) {
        return Ok(());
    }
    if has_pty_connect_ticket_url(url) {
        return Ok(());
    }
    let cred = credential_from_request(url, headers);
    if authorized(&cred, config) {
        Ok(())
    } else {
        Err(UnauthorizedError {
            message: "Authentication required".to_string(),
        })
    }
}

/// Descriptor for `authorizationLayer` — records service IDs and dependency.
pub struct AuthorizationLayer;

impl AuthorizationLayer {
    pub const SERVICE: &str = AUTHORIZATION_SERVICE_ID;
    pub const DEPENDS_ON: &str = crate::auth::CONFIG_SERVICE_ID;
    pub const WWW_AUTHENTICATE: &str = WWW_AUTHENTICATE;
}

/// Re-export `Authorization` descriptor from protocol (mirrors `export { Authorization }`).
pub use protocol::middleware::authorization::Authorization;

/// Phantom config struct to mirror source `ServerAuth.Config` dependency.
/// Kept as type alias for documentation linkage.
pub type ServerAuthConfig = Config;
