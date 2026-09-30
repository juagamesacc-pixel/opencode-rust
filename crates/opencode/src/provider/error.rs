// source: src/provider/error.ts — exports: HeaderTimeoutError,
// ResponseStreamError, ParsedStreamError, parseStreamError,
// ParsedAPICallError, parseAPICallError, ProviderError
// PROVISIONAL pending ai APICallError + @opencode-ai/llm isContextOverflow:
// names, message templates, code table, retry rules verbatim.

use serde::{Deserialize, Serialize};

/// source: HeaderTimeoutError name "ProviderHeaderTimeoutError" — verbatim.
#[derive(Debug, Clone)]
pub struct HeaderTimeoutError {
    pub ms: u64,
}

impl HeaderTimeoutError {
    /// source: `Provider response headers timed out after ${ms}ms` — verbatim.
    pub fn message(&self) -> String {
        format!("Provider response headers timed out after {}ms", self.ms)
    }
}

/// source: ResponseStreamError name "ProviderResponseStreamError" — verbatim.
#[derive(Debug, Clone)]
pub struct ResponseStreamError {
    pub message: String,
}

/// source: isOpenAiErrorRetryable — no status → isRetryable; 404 always retryable. Verbatim.
pub fn openai_retryable(status: Option<u16>, is_retryable: bool) -> bool {
    match status {
        None => is_retryable,
        Some(404) => true,
        Some(_) => is_retryable,
    }
}

/// source: message() — empty→body/status-text/"Unknown error"; body-append
/// rule; HTML gateway 401/403 templates; `${msg}: ${responseBody}` fallthrough.
/// + .trim(). Verbatim.
pub fn api_message(msg: &str, status: Option<u16>, body: Option<&str>) -> String {
    let out = if msg.is_empty() {
        if let Some(b) = body {
            if !b.is_empty() {
                b.to_string()
            } else {
                status_text(status).unwrap_or("Unknown error").to_string()
            }
        } else {
            status_text(status).unwrap_or("Unknown error").to_string()
        }
    } else if body.is_none_or(|b| b.is_empty())
        || status.is_some_and(|s| msg != status_text(Some(s)).unwrap_or(""))
    {
        msg.to_string()
    } else {
        let b = body.unwrap_or("");
        match extract_error_message(b) {
            Some(e) => format!("{}: {}", msg, e),
            None => {
                if is_html_body(b) {
                    if status == Some(401) {
                        return "Unauthorized: request was blocked by a gateway or proxy. Your authentication token may be missing or expired — try running `opencode auth login <your provider URL>` to re-authenticate.".to_string();
                    }
                    if status == Some(403) {
                        return "Forbidden: request was blocked by a gateway or proxy. You may not have permission to access this resource — check your account and provider settings.".to_string();
                    }
                    return msg.to_string();
                }
                format!("{}: {}", msg, b)
            }
        }
    };
    out.trim().to_string()
}

/// source: STATUS_CODES lookup — verbatim behavior (subset used by ports).
pub fn status_text(status: Option<u16>) -> Option<&'static str> {
    Some(match status? {
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        408 => "Request Timeout",
        409 => "Conflict",
        413 => "Content Too Large",
        422 => "Unprocessable Entity",
        429 => "Too Many Requests",
        500 => "Internal Server Error",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        _ => return None,
    })
}

fn extract_error_message(body: &str) -> Option<String> {
    let v: serde_json::Value = serde_json::from_str(body).ok()?;
    let m = v
        .get("message")
        .and_then(|m| m.as_str())
        .map(|s| s.to_string())
        .or_else(|| {
            v.get("error")
                .and_then(|e| e.as_str())
                .map(|s| s.to_string())
        })
        .or_else(|| {
            v.get("error")
                .and_then(|e| e.get("message"))
                .and_then(|m| m.as_str())
                .map(|s| s.to_string())
        });
    m.filter(|s| !s.is_empty())
}

fn is_html_body(body: &str) -> bool {
    let lower = body.to_lowercase();
    let t = lower.trim_start();
    t.starts_with("<!doctype") || t.starts_with("<html")
}

/// source: "Unknown error" — verbatim.
pub const UNKNOWN_MESSAGE: &str = "Unknown error";

/// source: ParsedStreamError — verbatim tags.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ParsedStreamError {
    #[serde(rename = "context_overflow")]
    ContextOverflow {
        message: String,
        #[serde(rename = "responseBody")]
        response_body: String,
    },
    #[serde(rename = "api_error")]
    ApiError {
        message: String,
        #[serde(rename = "isRetryable")]
        is_retryable: bool,
        #[serde(rename = "responseBody")]
        response_body: String,
    },
}

/// source: stream error code table — verbatim messages + retry flags.
pub fn stream_code_message(code: &str, error_message: Option<&str>) -> (String, bool) {
    match code {
        "context_length_exceeded" => ("Input exceeds context window of this model".to_string(), false),
        "insufficient_quota" => ("Quota exceeded. Check your plan and billing details.".to_string(), false),
        "usage_not_included" => (
            "To use Codex with your ChatGPT plan, upgrade to Plus: https://chatgpt.com/explore/plus.".to_string(),
            false,
        ),
        "invalid_prompt" => (error_message.unwrap_or("Invalid prompt.").to_string(), false),
        "server_is_overloaded" | "server_error" => (error_message.unwrap_or("Server error.").to_string(), true),
        _ => (error_message.unwrap_or("Server error.").to_string(), true),
    }
}

/// source: ParsedAPICallError — verbatim tags.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ParsedAPICallError {
    #[serde(rename = "context_overflow")]
    ContextOverflow {
        message: String,
        #[serde(rename = "responseBody")]
        response_body: Option<String>,
    },
    #[serde(rename = "api_error")]
    ApiError {
        message: String,
        #[serde(rename = "statusCode")]
        status_code: Option<u16>,
        #[serde(rename = "isRetryable")]
        is_retryable: bool,
        #[serde(rename = "responseHeaders")]
        response_headers: Option<std::collections::HashMap<String, String>>,
        #[serde(rename = "responseBody")]
        response_body: Option<String>,
        metadata: Option<std::collections::HashMap<String, String>>,
    },
}

/// source: overflow rule — isContextOverflow(m) || 413 || code context_length_exceeded. Verbatim.
pub fn is_overflow(message_overflow: bool, status: Option<u16>, code_overflow: bool) -> bool {
    message_overflow || status == Some(413) || code_overflow
}

/// source: openai retry prefix rule — providerID.startsWith("openai"). Verbatim.
pub fn is_openai_provider(provider_id: &str) -> bool {
    provider_id.starts_with("openai")
}
