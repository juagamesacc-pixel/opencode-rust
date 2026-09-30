// source: packages/session-ui/src/components/markdown-worker-protocol.ts
// 1:1 port — worker request/response/message-key protocol, fully ported as real Rust.

use serde::{Deserialize, Serialize};

use crate::components::markdown_stream::Projection;

/// 1:1 port of `MarkdownToken = [content: string, style: string]`.
pub type MarkdownToken = (String, String);

/// 1:1 port of `MarkdownWorkerRequest`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum MarkdownWorkerRequest {
    Parse {
        id: u64,
        text: String,
    },
    Project {
        id: u64,
        key: String,
        text: String,
        live: bool,
    },
    Highlight {
        id: u64,
        key: String,
        text: String,
        language: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        complete: Option<bool>,
    },
    Dispose {
        key: String,
    },
}

/// 1:1 port of `MarkdownWorkerResponse`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum MarkdownWorkerResponse {
    Parse {
        id: u64,
        html: String,
    },
    Project {
        id: u64,
        key: String,
        projection: Projection,
    },
    Highlight {
        id: u64,
        key: String,
        language: String,
        reset: bool,
        stable: Vec<MarkdownToken>,
        unstable: Vec<MarkdownToken>,
    },
    Error {
        id: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        key: Option<String>,
        message: String,
    },
    Superseded {
        id: u64,
        key: String,
    },
}

/// 1:1 port of `MarkdownWorkerState`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MarkdownWorkerState {
    pub id: u64,
    pub generation: u64,
    pub language: String,
    pub stable: Vec<MarkdownToken>,
    pub unstable: Vec<MarkdownToken>,
}

/// 1:1 port of `shouldReleaseMarkdownWorkerState(complete, latestID, responseID)`.
pub fn should_release_markdown_worker_state(
    complete: bool,
    latest_id: Option<u64>,
    response_id: u64,
) -> bool {
    complete && latest_id == Some(response_id)
}

/// 1:1 port of `markdownBlockKey(owner, cacheKey, index, mode)`.
pub fn markdown_block_key(
    owner: &str,
    cache_key: Option<&str>,
    index: usize,
    mode: &str,
) -> String {
    match cache_key {
        Some(cache_key) => format!("{owner}:{cache_key}:{index}:{mode}"),
        None => format!("{owner}:block:{index}"),
    }
}

/// 1:1 port of `applyMarkdownWorkerResponse(state, response)`.
pub fn apply_markdown_worker_response(
    state: Option<MarkdownWorkerState>,
    response: &MarkdownWorkerHighlightResponse,
) -> MarkdownWorkerState {
    if let Some(state) = state {
        if response.id <= state.id {
            return state;
        }
        let mut stable = state.stable;
        if response.reset {
            stable = response.stable.clone();
        } else {
            stable.extend(response.stable.iter().cloned());
        }
        return MarkdownWorkerState {
            id: response.id,
            generation: state.generation + u64::from(response.reset),
            language: response.language.clone(),
            stable,
            unstable: response.unstable.clone(),
        };
    }
    MarkdownWorkerState {
        id: response.id,
        generation: u64::from(response.reset),
        language: response.language.clone(),
        stable: response.stable.clone(),
        unstable: response.unstable.clone(),
    }
}

/// The `{ type: "highlight" }` response projection consumed by
/// `applyMarkdownWorkerResponse` (matches `Extract<MarkdownWorkerResponse, { type: "highlight" }>`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MarkdownWorkerHighlightResponse {
    pub id: u64,
    pub key: String,
    pub language: String,
    pub reset: bool,
    pub stable: Vec<MarkdownToken>,
    pub unstable: Vec<MarkdownToken>,
}

impl From<&MarkdownWorkerHighlightResponse> for MarkdownWorkerResponse {
    fn from(response: &MarkdownWorkerHighlightResponse) -> Self {
        MarkdownWorkerResponse::Highlight {
            id: response.id,
            key: response.key.clone(),
            language: response.language.clone(),
            reset: response.reset,
            stable: response.stable.clone(),
            unstable: response.unstable.clone(),
        }
    }
}
