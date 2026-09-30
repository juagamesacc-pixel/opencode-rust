//! Rust port of `packages/core/src/github-copilot/chat/openai-compatible-chat-language-model.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

pub fn is_parsable_json(s: &str) -> bool {
    serde_json::from_str::<serde_json::Value>(s).is_ok() && !s.trim().is_empty()
}

pub fn map_finish_reason(raw: Option<&str>) -> &'static str {
    match raw {
        Some("stop") => "stop",
        Some("length") => "length",
        Some("tool_calls") => "tool-calls",
        Some("content_filter") => "content-filter",
        Some(s) if s.starts_with("function") => "tool-calls",
        _ => "other",
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChunk {
    pub id: Option<String>,
    pub choices: Vec<ChatChoice>,
    pub usage: Option<serde_json::Value>,
    pub error: Option<ChatError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChoice {
    pub delta: Option<ChatDelta>,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatDelta {
    pub role: Option<String>,
    pub content: Option<String>,
    pub reasoning_text: Option<String>,
    pub reasoning_opaque: Option<String>,
    pub tool_calls: Option<Vec<ToolCallDelta>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallDelta {
    pub index: usize,
    pub id: Option<String>,
    pub function: Option<FunctionDelta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDelta {
    pub name: Option<String>,
    pub arguments: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatError {
    pub message: String,
}

pub fn parse_chunk(line: &str) -> Option<ChatChunk> {
    let data = line.strip_prefix("data: ")?;
    if data == "[DONE]" {
        return None;
    }
    serde_json::from_str(data).ok()
}

pub fn extract_text_from_delta(delta: &ChatDelta) -> Option<&str> {
    delta.content.as_deref().filter(|s| !s.is_empty())
}

pub fn extract_reasoning(delta: &ChatDelta) -> Option<&str> {
    delta.reasoning_text.as_deref().filter(|s| !s.is_empty())
}
