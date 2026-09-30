//! Rust port of `packages/core/src/plugin/provider/zenmux.ts`.

use std::collections::HashMap;

pub const ID: &str = "zenmux";
pub const PACKAGE: &str = "@ai-sdk/openai-compatible";
pub const ZENMUX_URL: &str = "https://zenmux.ai/api/v1";

pub fn headers_if_zenmux(
    current_url: &str,
    mut h: HashMap<String, String>,
) -> HashMap<String, String> {
    if current_url == ZENMUX_URL {
        h.entry("HTTP-Referer".to_string())
            .or_insert("https://opencode.ai/".to_string());
        h.entry("X-Title".to_string())
            .or_insert("opencode".to_string());
    }
    h
}
