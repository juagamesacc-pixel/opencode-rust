//! Rust port of `packages/core/src/github-copilot/responses/openai-responses-language-model.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsesChunk {
    #[serde(rename = "type")]
    pub chunk_type: String,
    pub item_id: Option<String>,
    pub delta: Option<String>,
    pub output_index: Option<u32>,
    pub response: Option<ResponsesResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsesResponse {
    pub id: Option<String>,
    pub incomplete_details: Option<Incomplete>,
    pub usage: Option<Usage>,
    pub service_tier: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Incomplete {
    pub reason: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub total_tokens: Option<u32>,
}

pub const TOP_LOGPROBS_MAX: u32 = 20;

pub fn map_responses_finish(reason: Option<&str>, has_function_call: bool) -> &'static str {
    match reason {
        Some("max_output_tokens") => "length",
        Some("content_filter") => "content-filter",
        None if has_function_call => "tool-calls",
        Some("") | None => "stop",
        _ => "other",
    }
}

pub fn is_text_delta(chunk_type: &str) -> bool {
    chunk_type == "response.output_text.delta"
}
pub fn is_created(chunk_type: &str) -> bool {
    chunk_type == "response.created"
}
pub fn is_finished(chunk_type: &str) -> bool {
    matches!(chunk_type, "response.completed" | "response.incomplete")
}
pub fn is_error(chunk_type: &str) -> bool {
    chunk_type == "error"
}

pub fn parse_responses_sse(body: &str) -> Vec<ResponsesChunk> {
    body.lines()
        .filter_map(|l| l.strip_prefix("data: "))
        .filter(|d| *d != "[DONE]")
        .filter_map(|d| serde_json::from_str(d).ok())
        .collect()
}
