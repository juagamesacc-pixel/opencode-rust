// source: packages/enterprise/src/routes/api/[...path].ts — Hono app verbatim structure
//! 1:1 port — route table, OpenAPI identity, validators, header names, and response shapes
//! preserved verbatim. Hono runtime is PROVISIONAL — modeled as route descriptors.

/// source: OpenAPI documentation identity verbatim
pub const API_TITLE: &str = "Opencode Enterprise API";
pub const API_VERSION: &str = "1.0.0";
pub const API_DESCRIPTION: &str = "Opencode Enterprise API endpoints";
pub const OPENAPI_VERSION: &str = "3.1.1";

/// source: `Cache-Control` header value on share data verbatim
pub const SHARE_CACHE_CONTROL: &str =
    "public, max-age=30, s-maxage=300, stale-while-revalidate=86400";

/// source: support remove-share responses verbatim
pub const SUPPORT_UNAUTHORIZED: &str = "Unauthorized";
pub const SUPPORT_INVALID_REQUEST: &str = "Invalid request";
pub const SUPPORT_SHARE_REMOVED: &str = "Share removed";

/// source: `operationId`s verbatim
pub const OP_SHARE_CREATE: &str = "share.create";
pub const OP_SHARE_SYNC: &str = "share.sync";
pub const OP_SHARE_DATA: &str = "share.data";
pub const OP_SHARE_REMOVE: &str = "share.remove";

/// source: Hono route table — (method, path, operation) verbatim, in declaration order
pub const ROUTES: &[(&str, &str, Option<&str>)] = &[
    ("GET", "/api/doc", None),
    ("POST", "/api/share", Some(OP_SHARE_CREATE)),
    ("POST", "/api/share/:shareID/sync", Some(OP_SHARE_SYNC)),
    ("GET", "/api/share/:shareID/data", Some(OP_SHARE_DATA)),
    ("DELETE", "/api/share/:shareID", Some(OP_SHARE_REMOVE)),
    ("DELETE", "/api/support/actions/remove-share", None),
];

/// source: `GET/POST/PUT/DELETE` all delegate to `app.fetch` verbatim
pub const EXPORTED_METHODS: &[&str] = &["GET", "POST", "PUT", "DELETE"];

/// source: share URL construction — `${protocol}://${host}/share/${share.id}` verbatim.
/// Protocol prefers `x-forwarded-proto`, then `x-forwarded-protocol`, else `"https"`.
pub fn share_url(protocol: Option<&str>, host: Option<&str>, share_id: &str) -> String {
    format!(
        "{}://{}/share/{share_id}",
        protocol.unwrap_or("https"),
        host.unwrap_or("")
    )
}

/// source: support-route bearer check — length compare + `timingSafeEqual` semantics verbatim
pub fn bearer_authorized(header: Option<&str>, expected_key: &str) -> bool {
    let expected = format!("Bearer {expected_key}");
    let actual = header.unwrap_or("");
    actual.len() == expected.len()
        && actual
            .bytes()
            .zip(expected.bytes())
            .fold(0u8, |acc, (a, b)| acc | (a ^ b))
            == 0
}
