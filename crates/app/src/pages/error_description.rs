//! Rust port of `packages/app/src/pages/error-description.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — same names/behavior/edge-cases/error-strings.
//! Rename: `error-description.ts` -> `error_description.rs` (kebab -> snake_case).

pub fn error_description_key(error: &serde_json::Value) -> &'static str {
    if let Some(obj) = error.as_object() {
        if let Some(v) = obj.get("localServerStartup") {
            if v.as_bool() == Some(true) {
                return "error.page.description.localServerStartup";
            }
        }
    }
    "error.page.description"
}
