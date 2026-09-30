// source: src/plugin/openai/codex.ts — exports: IdTokenClaims,
// parseJwtClaims, extractAccountIdFromClaims, extractAccountId,
// extractResidency, renderOAuthError, CodexAuthPlugin (+ options)
// PROVISIONAL: PKCE/fetch/callback-server as descriptors; CLIENT_ID,
// ISSUER, endpoint, ports, model sets, provider "ChatGPT" verbatim.

/// source: CLIENT_ID — verbatim.
pub const CLIENT_ID: &str = "app_EMoamEEZ73f0CkXaXp7hrann";
/// source: ISSUER — verbatim.
pub const ISSUER: &str = "https://auth.openai.com";
/// source: CODEX_API_ENDPOINT — verbatim.
pub const CODEX_API_ENDPOINT: &str = "https://chatgpt.com/backend-api/codex/responses";
/// source: OAUTH_PORT = 1455 — verbatim.
pub const OAUTH_PORT: u16 = 1455;
/// source: OAUTH_POLLING_SAFETY_MARGIN_MS = 3000 — verbatim.
pub const OAUTH_POLLING_SAFETY_MARGIN_MS: u64 = 3000;

/// source: parseJwtClaims() — 3 parts else undefined. Verbatim rule.
pub fn jwt_part_count_ok(parts: usize) -> bool {
    parts == 3
}

/// source: renderOAuthError provider "ChatGPT" — verbatim.
pub const OAUTH_PAGE_PROVIDER: &str = "ChatGPT";

/// source: base64UrlEncode() — +/→-_, strip =. Verbatim rule.
pub fn base64_url_encode_char(c: char) -> Option<char> {
    match c {
        '+' => Some('-'),
        '/' => Some('_'),
        '=' => None,
        _ => Some(c),
    }
}
