//! Rust port of `packages/core/src/plugin/provider/snowflake-cortex.ts`.

pub const ID: &str = "snowflake-cortex";

pub fn cortex_transform_body(body: &str) -> String {
    // replace max_tokens with max_completion_tokens
    if let Ok(mut v) = serde_json::from_str::<serde_json::Value>(body) {
        if let Some(obj) = v.as_object_mut() {
            if let Some(val) = obj.remove("max_tokens") {
                obj.insert("max_completion_tokens".to_string(), val);
            }
        }
        return serde_json::to_string(&v).unwrap_or_else(|_| body.to_string());
    }
    body.to_string()
}

pub fn should_synthesize_stop(error_json: &str) -> bool {
    let lower = error_json.to_lowercase();
    lower.contains("conversation complete")
}

pub fn fix_role_empty(delta: &str) -> String {
    delta
        .replace("\"role\":\"\"", "\"role\":\"assistant\"")
        .replace("\"role\": \"\"", "\"role\":\"assistant\"")
}

pub fn resolve_token(options_token: Option<&str>) -> Option<String> {
    std::env::var("SNOWFLAKE_CORTEX_TOKEN")
        .ok()
        .or_else(|| std::env::var("SNOWFLAKE_CORTEX_PAT").ok())
        .or_else(|| options_token.map(|s| s.to_string()))
}
