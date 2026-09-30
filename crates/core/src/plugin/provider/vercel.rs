//! Rust port of `packages/core/src/plugin/provider/vercel.ts`.

use std::collections::HashMap;

pub const ID: &str = "vercel";
pub const PACKAGE: &str = "@ai-sdk/vercel";

pub fn headers(mut h: HashMap<String, String>) -> HashMap<String, String> {
    h.insert(
        "http-referer".to_string(),
        "https://opencode.ai/".to_string(),
    );
    h.insert("x-title".to_string(), "opencode".to_string());
    h
}
