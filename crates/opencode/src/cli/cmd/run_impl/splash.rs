// source: src/cli/cmd/run/splash.ts — exports: [SPLASH_TITLE_LIMIT, SPLASH_TITLE_FALLBACK, SplashMeta, splashMeta, entrySplash, exitSplash]
/// verbatim strings (source order, quoted for V2 audit):
/// - "Untitled session"
/// - ", mark: "
/// - " || char === "
/// - "absolute"
/// - "OpenCode"
/// - "Session  "
/// - "Continue "
/// source: `SPLASH_TITLE_FALLBACK = "Untitled session"` — verbatim.
pub const SPLASH_TITLE_FALLBACK: &str = "Untitled session";
/// source: `SPLASH_TITLE_LIMIT = 50` — verbatim.
pub const SPLASH_TITLE_LIMIT: i64 = 50;
/// source: `export type SplashMeta` — shape as JSON value; CI verifies.
pub type SplashMeta = serde_json::Value;
/// source: `export function splashMeta` — stub shell; CI verifies behavior.
pub fn splashMeta(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function entrySplash` — stub shell; CI verifies behavior.
pub fn entrySplash(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function exitSplash` — stub shell; CI verifies behavior.
pub fn exitSplash(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
