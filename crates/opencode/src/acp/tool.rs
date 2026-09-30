// source: src/acp/tool.ts — exports: ToolInput, ToolAttachment,
// CompletedToolState, RunningToolState, ErrorToolState, ImageAttachment,
// toToolKind (+mapToolKind), toLocations (+extractLocations),
// completedToolContent (+buildCompletedToolContent), pendingToolCall
// (+buildPendingToolCall), runningToolUpdate (+build), duplicateRunningToolUpdate
// (+build), completedToolUpdate (+build), errorToolUpdate (+build),
// completedToolRawOutput (+build), imageContents, extractImageAttachments,
// shellOutputSnapshot (+extract)
// PROVISIONAL pending ACP sdk types: kind table, location branches, content
// envelope {"type":"content"} verbatim.

use serde::{Deserialize, Serialize};

/// source: toToolKind() — verbatim mapping (lowercased).
pub fn to_tool_kind(tool_name: &str) -> &'static str {
    match tool_name.to_lowercase().as_str() {
        "bash" | "shell" => "execute",
        "webfetch" => "fetch",
        "edit" | "apply_patch" | "patch" | "write" => "edit",
        "grep"
        | "glob"
        | "context"
        | "context7_resolve_library_id"
        | "context7_get_library_docs" => "search",
        "read" => "read",
        "task" => "think",
        _ => "other",
    }
}

/// source: toLocations() tool branches — verbatim key sets.
pub fn location_keys(tool_name: &str) -> &'static str {
    match tool_name.to_lowercase().as_str() {
        "bash" | "shell" => "workdir",
        "read" | "edit" | "write" => "file",
        "external_directory" => "file+parent+dirs",
        "grep"
        | "glob"
        | "context"
        | "context7_resolve_library_id"
        | "context7_get_library_docs" => "path",
        _ => "none",
    }
}

/// source: completedToolContent() — read uses displayText ?? output; edit
/// appends diff content; attachments appended. Verbatim rule.
pub fn read_uses_display_text(tool_name: &str) -> bool {
    tool_name.to_lowercase() == "read"
}

/// source: update kinds — verbatim status strings.
pub const STATUS_COMPLETED: &str = "completed";
pub const STATUS_RUNNING: &str = "running";
pub const STATUS_ERROR: &str = "error";

/// source: content envelope type "content" — verbatim.
pub const CONTENT_TYPE: &str = "content";

/// source: ImageAttachment { mimeType, data } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageAttachment {
    pub mime_type: String,
    pub data: String,
}
