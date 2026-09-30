// source: packages/web/src/pages/[...slug].md.ts and packages/web/src/pages/s/[id].astro
//
// Descriptor of the two non-docs routes. Fetching, SSR rendering and the Solid island need the
// Astro runtime, so live behavior is out of scope:
// PROVISIONAL: Astro route handlers, `fetch` of share data and Solid rendering need the Astro runtime.

/// Route pattern of the markdown endpoint: `src/pages/[...slug].md.ts`.
pub const MARKDOWN_ROUTE: &str = "[...slug].md";

/// HTTP method served by the markdown endpoint.
pub const MARKDOWN_METHOD: &str = "GET";

/// `Content-Type` of a successful markdown response.
pub const MARKDOWN_CONTENT_TYPE: &str = "text/plain; charset=utf-8";

/// Fallback body (and `statusText`) when a markdown slug or its translator is missing.
pub const NOT_FOUND_KEY: &str = "share.not_found";

/// Slug served when the `[...slug]` param is empty.
pub const INDEX_SLUG: &str = "index";

/// Route pattern of the share page: `src/pages/s/[id].astro`.
pub const SHARE_ROUTE: &str = "s/[id]";

/// Share page template, sidebar and table-of-contents flags.
pub const SHARE_TEMPLATE: &str = "splash";

/// Share pages are excluded from pagefind indexing.
pub const SHARE_PAGEFIND: bool = false;

/// Share pages opt out of search-engine indexing.
pub const SHARE_ROBOTS: &str = "noindex, nofollow, noarchive, nosnippet";

/// API path the share page fetches, relative to `VITE_API_URL`.
pub const SHARE_DATA_PATH: &str = "/share_data";

/// Fallback version label when the shared session reports no version.
pub const FALLBACK_VERSION: &str = "v0.0.1";

/// Maximum title characters fed into the social-card URL.
pub const TITLE_TRUNCATE_AT: usize = 700;

/// Maximum description characters fed into the social-card URL.
pub const DESCRIPTION_TRUNCATE_AT: usize = 400;

/// Message keys the share page resolves, verbatim from `src/pages/s/[id].astro`.
///
/// Every entry except `locale` is looked up as `share.<name>` in the `i18n` collection.
pub const SHARE_MESSAGE_KEYS: [&str; 46] = [
    "locale",
    "link_to_message",
    "copied",
    "copy",
    "show_more",
    "show_less",
    "show_results",
    "hide_results",
    "show_details",
    "hide_details",
    "show_preview",
    "hide_preview",
    "show_contents",
    "hide_contents",
    "show_output",
    "hide_output",
    "error",
    "waiting_for_messages",
    "status_connected_waiting",
    "status_connecting",
    "status_disconnected",
    "status_reconnecting",
    "status_error",
    "status_unknown",
    "error_id_not_found",
    "error_api_url_not_found",
    "error_connection_failed",
    "opencode_version",
    "opencode_name",
    "models",
    "cost",
    "input_tokens",
    "output_tokens",
    "reasoning_tokens",
    "scroll_to_bottom",
    "attachment",
    "thinking",
    "thinking_pending",
    "creating_plan",
    "completing_plan",
    "updating_plan",
    "match_one",
    "match_other",
    "result_one",
    "result_other",
    "debug_key",
];

/// Resolve the not-found text the way the markdown endpoint does: a translator function wins,
/// an empty or missing translator falls back to the `share.not_found` key.
pub fn not_found_text(translated: Option<&str>) -> String {
    match translated {
        Some(text) if !text.is_empty() => text.to_string(),
        _ => NOT_FOUND_KEY.to_string(),
    }
}

/// Look up the doc body served by the markdown endpoint for `slug`.
pub fn markdown_body(locale: &str, slug: &str) -> Option<&'static crate::manifest::Page> {
    crate::manifest::find(locale, slug)
}

/// Version label shown on the share page: `v<version>`, or the fallback when absent.
pub fn version_label(version: Option<&str>) -> String {
    match version {
        Some(version) if !version.is_empty() => format!("v{}", version),
        _ => FALLBACK_VERSION.to_string(),
    }
}

/// `model` query parameter of the share social-card URL, verbatim from the share page.
pub fn model_param(models: &[&str]) -> String {
    match models {
        [] => String::new(),
        [only] => only.to_string(),
        [first, second] => encode_uri_component(&format!("{} & {}", first, second)),
        [first, ..] => encode_uri_component(&format!("{} & {} others", first, models.len() - 1)),
    }
}

/// Encoded share title for the social-card URL: the title truncated to 700 characters,
/// URI-encoded, base64-encoded, then URI-encoded again.
pub fn encode_share_title(title: &str) -> String {
    let truncated: String = title.chars().take(TITLE_TRUNCATE_AT).collect();
    encode_uri_component(&base64_encode(encode_uri_component(&truncated).as_bytes()))
}

/// Encoded docs description for the social-card URL, truncated to 400 characters.
pub fn encode_docs_description(description: &str) -> String {
    let truncated: String = description.chars().take(DESCRIPTION_TRUNCATE_AT).collect();
    encode_uri_component(&truncated)
}

/// Standard base64 encoding with padding, as produced by `js-base64`.
pub fn base64_encode(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let a = chunk[0] as u32;
        let b = *chunk.get(1).unwrap_or(&0) as u32;
        let c = *chunk.get(2).unwrap_or(&0) as u32;
        let triple = (a << 16) | (b << 8) | c;
        out.push(ALPHABET[((triple >> 18) & 63) as usize] as char);
        out.push(ALPHABET[((triple >> 12) & 63) as usize] as char);
        out.push(if chunk.len() > 1 {
            ALPHABET[((triple >> 6) & 63) as usize] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            ALPHABET[(triple & 63) as usize] as char
        } else {
            '='
        });
    }
    out
}

/// `encodeURIComponent`, byte-for-byte compatible for the inputs used here.
pub fn encode_uri_component(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for byte in input.bytes() {
        if byte.is_ascii_alphanumeric() || b"-_.!~*'()".contains(&byte) {
            out.push(byte as char);
        } else {
            out.push_str(&format!("%{:02X}", byte));
        }
    }
    out
}
