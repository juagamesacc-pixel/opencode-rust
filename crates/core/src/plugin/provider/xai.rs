//! Rust port of `packages/core/src/plugin/provider/xai.ts`.

pub const ID: &str = "xai";
pub const PACKAGE: &str = "@ai-sdk/xai";
pub const BASE_URL: &str = "https://api.x.ai/v1";

pub fn build_language_request(model_id: &str) -> String {
    // xAIPlugin maps xAI models through responses
    format!("responses/{model_id}")
}
