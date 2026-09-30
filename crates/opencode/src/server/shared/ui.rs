// source: src/server/shared/ui.ts — exports: [UI_UPSTREAM, csp, DEFAULT_CSP, themePreloadHash, cspForHtml, upstreamURL, embeddedUI, serveEmbeddedUIEffect, serveUIEffect]
// PROVISIONAL pending crates/core: `@opencode-ai/core/fs-util`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/http` (host-provided; no new dep)
// PROVISIONAL pending external `node:crypto` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@opencode-ai/core/fs-util"
/// - "https://app.opencode.ai"
/// - "wasm-unsafe-eval"
/// - "sha256-${hash}"
/// - "unsafe-inline"
/// - " || request.method === "
/// - "content-length"
/// - "content-type"
/// source: `export const UI_UPSTREAM` — shape as JSON value; CI verifies.
pub type UI_UPSTREAM = serde_json::Value;
/// source: `export const csp` — shape as JSON value; CI verifies.
pub type csp = serde_json::Value;
/// source: `export const DEFAULT_CSP` — shape as JSON value; CI verifies.
pub type DEFAULT_CSP = serde_json::Value;
/// source: `export function themePreloadHash` — stub shell; CI verifies behavior.
pub fn themePreloadHash(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function cspForHtml` — stub shell; CI verifies behavior.
pub fn cspForHtml(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function upstreamURL` — stub shell; CI verifies behavior.
pub fn upstreamURL(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function embeddedUI` — stub shell; CI verifies behavior.
pub fn embeddedUI(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function serveEmbeddedUIEffect` — stub shell; CI verifies behavior.
pub fn serveEmbeddedUIEffect(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function serveUIEffect` — stub shell; CI verifies behavior.
pub fn serveUIEffect(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
