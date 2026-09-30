//! Rust port of `packages/core/src/plugin/provider/sap-ai-core.ts`.

pub const ID: &str = "sap-ai-core";

pub fn resolve_service_key(options: Option<&str>) -> Option<String> {
    std::env::var("AICORE_SERVICE_KEY")
        .ok()
        .or_else(|| options.map(|s| s.to_string()))
}
