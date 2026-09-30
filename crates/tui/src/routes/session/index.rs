// source: packages/tui/src/routes/session/index.tsx (2706 lines, v1.18.30)
// 1:1 port — the session route as render descriptors + pure selectors.
// Every message/tool renderer becomes a `*View` struct the app draws;
// all command metadata, reply payloads, toggles, retry text, export
// filenames, and parse helpers are verbatim.

#![allow(dead_code)]

use serde_json::Value;
use std::collections::HashMap;

use super::super::session::dialog_message::{copy_text_from_parts, rebuild_prompt_from_parts};
use crate::prompt::history::PromptInfo;
use crate::routes::subagent_footer as subagent;
use crate::ui::dialog_select::{SelectOption, SelectState};
use crate::util::collapse_tool_output::collapse_tool_output;
use crate::util::error::error_message_value;
use crate::util::locale::{duration, number, time, titlecase, today_time_or_date_time, truncate};
use crate::util::revert_diff::{get_revert_diff_files, RevertDiffFile};
use crate::util::tool_display::web_search_provider_label;

/// Go-upsell KV keys + window verbatim.
pub const GO_UPSELL_FREE_TIER_LAST_SEEN_AT: &str = "go_upsell_last_seen_at";
pub const GO_UPSELL_FREE_TIER_DONT_SHOW: &str = "go_upsell_dont_show";
pub const GO_UPSELL_ACCOUNT_RATE_LIMIT_LAST_SEEN_AT: &str =
    "go_upsell_account_rate_limit_last_seen_at";
pub const GO_UPSELL_ACCOUNT_RATE_LIMIT_DONT_SHOW: &str = "go_upsell_dont_show";
pub const GO_UPSELL_WINDOW_MS: i64 = 86_400_000;
pub const GO_UPSELL_PROVIDERS: &[&str] = &["opencode", "opencode-go"];

/// Session keybinding command lists verbatim.
pub const SESSION_BINDING_COMMANDS: &[&str] = &[
    "session.share",
    "session.rename",
    "session.timeline",
    "session.fork",
    "session.compact",
    "session.unshare",
    "session.undo",
    "session.redo",
    "session.sidebar.toggle",
    "session.toggle.conceal",
    "session.toggle.timestamps",
    "session.toggle.thinking",
    "session.toggle.actions",
    "session.toggle.scrollbar",
    "session.toggle.generic_tool_output",
    "session.first",
    "session.last",
    "session.messages_last_user",
    "session.message.next",
    "session.message.previous",
    "messages.copy",
    "session.copy",
    "session.export",
    "session.child.first",
    "session.parent",
    "session.child.next",
    "session.child.previous",
];
pub const SESSION_GLOBAL_BINDING_COMMANDS: &[&str] = &[
    "session.page.up",
    "session.page.down",
    "session.line.up",
    "session.line.down",
    "session.half.page.up",
    "session.half.page.down",
];
pub const SESSION_GLOBAL_UNFOCUSED_BINDING_COMMANDS: &[&str] = &["session.first", "session.last"];

/// Icon width verbatim.
pub const INLINE_TOOL_ICON_WIDTH: usize = 2;

/// Known tool displays (mirrors `toolDisplays`).
pub const TOOL_DISPLAYS: &[&str] = &[
    "bash",
    "glob",
    "read",
    "grep",
    "webfetch",
    "websearch",
    "write",
    "edit",
    "task",
    "apply_patch",
    "todowrite",
    "question",
    "skill",
    "execute",
];

/// Mirrors `toolDisplay`.
pub fn tool_display(tool: &str) -> &str {
    if TOOL_DISPLAYS.contains(&tool) {
        tool
    } else {
        "generic"
    }
}

/// Mirrors `goUpsellKeys`.
pub fn go_upsell_keys(provider: Option<&str>, reason: Option<&str>) -> Option<(String, String)> {
    let provider = provider?;
    if !GO_UPSELL_PROVIDERS.contains(&provider) {
        return None;
    }
    match reason {
        Some("free_tier_limit") => Some((
            GO_UPSELL_FREE_TIER_LAST_SEEN_AT.to_string(),
            GO_UPSELL_FREE_TIER_DONT_SHOW.to_string(),
        )),
        Some("account_rate_limit") => Some((
            GO_UPSELL_ACCOUNT_RATE_LIMIT_LAST_SEEN_AT.to_string(),
            GO_UPSELL_ACCOUNT_RATE_LIMIT_DONT_SHOW.to_string(),
        )),
        _ => None,
    }
}

/// Value helpers (mirrors `stringValue`/`numberValue`/`recordValue`).
pub fn string_value(value: &Value) -> Option<String> {
    value.as_str().map(str::to_string)
}

pub fn number_value(value: &Value) -> Option<f64> {
    match value {
        Value::Number(number) => number.as_f64(),
        _ => None,
    }
}

pub fn record_value(value: &Value) -> Option<&serde_json::Map<String, Value>> {
    value.as_object()
}

/// Mirrors `input(input, omit?)` — `[k=v, …]` for primitives.
pub fn format_input(input: &Value, omit: &[&str]) -> String {
    let Some(map) = input.as_object() else {
        return String::new();
    };
    let primitives: Vec<String> = map
        .iter()
        .filter(|(key, value)| {
            !omit.contains(&key.as_str())
                && (value.is_string() || value.is_number() || value.is_boolean())
        })
        .map(|(key, value)| {
            let rendered = match value {
                Value::String(text) => text.clone(),
                other => other.to_string(),
            };
            format!("{key}={rendered}")
        })
        .collect();
    if primitives.is_empty() {
        return String::new();
    }
    format!("[{}]", primitives.join(", "))
}

/// Apply-patch file entry (mirrors `parseApplyPatchFiles`).
#[derive(Debug, Clone)]
pub struct ApplyPatchFile {
    pub file_type: String,
    pub relative_path: String,
    pub file_path: String,
    pub patch: String,
    pub deletions: u64,
    pub move_path: Option<String>,
}

pub fn parse_apply_patch_files(value: &Value) -> Vec<ApplyPatchFile> {
    let Some(items) = value.as_array() else {
        return Vec::new();
    };
    items
        .iter()
        .filter_map(|item| {
            let file = item.as_object()?;
            Some(ApplyPatchFile {
                file_type: file.get("type")?.as_str()?.to_string(),
                relative_path: file.get("relativePath")?.as_str()?.to_string(),
                file_path: file.get("filePath")?.as_str()?.to_string(),
                patch: file.get("patch")?.as_str()?.to_string(),
                deletions: file.get("deletions")?.as_u64()?,
                move_path: file
                    .get("movePath")
                    .and_then(|v| v.as_str())
                    .map(str::to_string),
            })
        })
        .collect()
}

/// Todo entry (mirrors `parseTodos`).
#[derive(Debug, Clone)]
pub struct TodoEntry {
    pub status: String,
    pub content: String,
}

pub fn parse_todos(value: &Value) -> Vec<TodoEntry> {
    let Some(items) = value.as_array() else {
        return Vec::new();
    };
    items
        .iter()
        .filter_map(|item| {
            let todo = item.as_object()?;
            Some(TodoEntry {
                status: todo.get("status")?.as_str()?.to_string(),
                content: todo.get("content")?.as_str()?.to_string(),
            })
        })
        .collect()
}

/// Question rows (mirrors `parseQuestions`).
pub fn parse_questions(value: &Value) -> Vec<String> {
    let Some(items) = value.as_array() else {
        return Vec::new();
    };
    items
        .iter()
        .filter_map(|item| {
            item.as_object()?
                .get("question")?
                .as_str()
                .map(str::to_string)
        })
        .collect()
}

/// Question answers (mirrors `parseQuestionAnswers`).
pub fn parse_question_answers(value: &Value) -> Option<Vec<Vec<String>>> {
    let items = value.as_array()?;
    Some(
        items
            .iter()
            .map(|answer| {
                answer
                    .as_array()
                    .map(|list| {
                        list.iter()
                            .filter_map(|item| item.as_str().map(str::to_string))
                            .collect()
                    })
                    .unwrap_or_default()
            })
            .collect(),
    )
}

/// Diagnostic entry (mirrors `parseDiagnostics`: severity 1 only, top 3).
#[derive(Debug, Clone)]
pub struct DiagnosticEntry {
    pub line: i64,
    pub character: i64,
    pub message: String,
}

pub fn parse_diagnostics(value: &Value, file_path: &str) -> Vec<DiagnosticEntry> {
    let Some(list) = value
        .as_object()
        .and_then(|map| map.get(file_path))
        .and_then(|v| v.as_array())
    else {
        return Vec::new();
    };
    list.iter()
        .filter_map(|item| {
            let diagnostic = item.as_object()?;
            if diagnostic.get("severity").and_then(|v| v.as_i64()) != Some(1) {
                return None;
            }
            let start = diagnostic.get("range")?.get("start")?.as_object()?;
            Some(DiagnosticEntry {
                line: start.get("line")?.as_i64()?,
                character: start.get("character")?.as_i64()?,
                message: diagnostic.get("message")?.as_str()?.to_string(),
            })
        })
        .take(3)
        .collect()
}

/// Execute child calls (mirrors `executeCalls`).
#[derive(Debug, Clone)]
pub struct ExecuteCall {
    pub tool: String,
    pub status: String,
    pub input: Option<Value>,
}

pub fn execute_calls(value: &Value) -> Vec<ExecuteCall> {
    let Some(items) = value.as_array() else {
        return Vec::new();
    };
    items
        .iter()
        .filter_map(|call| {
            let item = call.as_object()?;
            let tool = item.get("tool")?.as_str()?.to_string();
            let status = item.get("status")?.as_str()?.to_string();
            if !["running", "completed", "error"].contains(&status.as_str()) {
                return None;
            }
            Some(ExecuteCall {
                tool,
                status,
                input: item
                    .get("input")
                    .and_then(|v| v.as_object())
                    .map(|map| Value::Object(map.clone())),
            })
        })
        .collect()
}

/// Execute content lines (mirrors the `content` memo).
pub fn execute_content(calls: &[ExecuteCall]) -> String {
    let mut lines = vec!["execute".to_string()];
    for call in calls {
        let args = call
            .input
            .as_ref()
            .map(|input| format_input(input, &[]))
            .unwrap_or_default();
        let failed = if call.status == "error" {
            " (failed)"
        } else {
            ""
        };
        lines.push(format!("↳ {}{args}{failed}", call.tool));
    }
    lines.join("\n")
}

/// Subagent formatters (mirrors the exported helpers verbatim).
pub fn format_subagent_toolcalls(count: usize) -> String {
    format!("{count} toolcall{}", if count == 1 { "" } else { "s" })
}

pub fn format_subagent_title(agent: &str, description: &str, background: bool) -> String {
    format!(
        "{agent} Task{} — {description}",
        if background { " (background)" } else { "" }
    )
}

pub fn format_subagent_retry(attempt: u64, message: &str) -> String {
    format!("Retrying (attempt {attempt}) · {message}")
}

pub fn format_completed_subagent_detail(toolcalls: usize, duration_text: &str) -> String {
    if toolcalls == 0 {
        return duration_text.to_string();
    }
    format!("{} · {duration_text}", format_subagent_toolcalls(toolcalls))
}

/// Retry banner text (mirrors the retry memo + `retryText`).
pub fn retry_text(message: &str, attempt: u64, seconds: u64, truncated: bool) -> String {
    let gemini_hot = message.contains("exceeded your current quota") && message.contains("gemini");
    let base = if gemini_hot {
        "gemini is way too hot right now".to_string()
    } else if message.chars().count() > 80 {
        format!("{}…", message.chars().take(80).collect::<String>())
    } else {
        message.to_string()
    };
    let hint = if truncated { " (click to expand)" } else { "" };
    let duration = crate::util::format::format_duration(seconds as f64);
    let retry_info = format!(
        " [retrying {}attempt #{attempt}]",
        if duration.is_empty() {
            String::new()
        } else {
            format!("in {duration} ")
        }
    );
    format!("{base}{hint}{retry_info}")
}

/// Session view flags (kv-backed toggles with verbatim keys/defaults).
#[derive(Debug, Clone)]
pub struct SessionViewFlags {
    pub sidebar: String,
    pub sidebar_open: bool,
    pub conceal: bool,
    pub timestamps: bool,
    pub show_details: bool,
    pub show_assistant_metadata: bool,
    pub show_scrollbar: bool,
    pub diff_wrap_word: bool,
    pub show_generic_tool_output: bool,
}

impl SessionViewFlags {
    pub fn defaults() -> Self {
        Self {
            sidebar: "auto".to_string(),
            sidebar_open: false,
            conceal: true,
            timestamps: false,
            show_details: true,
            show_assistant_metadata: true,
            show_scrollbar: false,
            diff_wrap_word: true,
            show_generic_tool_output: false,
        }
    }

    pub fn sidebar_visible(&self, wide: bool, has_parent: bool) -> bool {
        if has_parent {
            return false;
        }
        if self.sidebar_open {
            return true;
        }
        if self.sidebar == "auto" && wide {
            return true;
        }
        false
    }

    pub fn toggle_sidebar(&mut self) {
        let visible = self.sidebar == "auto";
        self.sidebar = if visible {
            "hide".to_string()
        } else {
            "auto".to_string()
        };
        self.sidebar_open = !visible;
    }
}

/// Session command metadata (mirrors `sessionCommandList` titles/values).
#[derive(Debug, Clone)]
pub struct SessionCommandMeta {
    pub title: String,
    pub value: String,
    pub category: String,
    pub hidden: bool,
    pub slash: Option<String>,
    pub slash_aliases: Vec<String>,
}

pub fn session_command_metas(
    share_url: Option<&str>,
    share_enabled: bool,
    sidebar_visible: bool,
    conceal: bool,
    show_timestamps: bool,
    show_details: bool,
    show_generic: bool,
    has_revert: bool,
    unshare_enabled: bool,
    background_enabled: bool,
    has_parent: bool,
) -> Vec<SessionCommandMeta> {
    let mut commands = vec![
        SessionCommandMeta {
            title: if share_url.is_some() {
                "Copy share link".to_string()
            } else {
                "Share session".to_string()
            },
            value: "session.share".to_string(),
            category: "Session".to_string(),
            hidden: false,
            slash: Some("share".to_string()),
            slash_aliases: Vec::new(),
        },
        SessionCommandMeta {
            title: "Rename session".to_string(),
            value: "session.rename".to_string(),
            category: "Session".to_string(),
            hidden: false,
            slash: Some("rename".to_string()),
            slash_aliases: Vec::new(),
        },
        SessionCommandMeta {
            title: "Jump to message".to_string(),
            value: "session.timeline".to_string(),
            category: "Session".to_string(),
            hidden: false,
            slash: Some("timeline".to_string()),
            slash_aliases: Vec::new(),
        },
        SessionCommandMeta {
            title: "Fork session".to_string(),
            value: "session.fork".to_string(),
            category: "Session".to_string(),
            hidden: false,
            slash: Some("fork".to_string()),
            slash_aliases: Vec::new(),
        },
        SessionCommandMeta {
            title: "Compact session".to_string(),
            value: "session.compact".to_string(),
            category: "Session".to_string(),
            hidden: false,
            slash: Some("compact".to_string()),
            slash_aliases: vec!["summarize".to_string()],
        },
        SessionCommandMeta {
            title: "Unshare session".to_string(),
            value: "session.unshare".to_string(),
            category: "Session".to_string(),
            hidden: !unshare_enabled,
            slash: Some("unshare".to_string()),
            slash_aliases: Vec::new(),
        },
        SessionCommandMeta {
            title: "Undo previous message".to_string(),
            value: "session.undo".to_string(),
            category: "Session".to_string(),
            hidden: false,
            slash: Some("undo".to_string()),
            slash_aliases: Vec::new(),
        },
        SessionCommandMeta {
            title: "Redo".to_string(),
            value: "session.redo".to_string(),
            category: "Session".to_string(),
            hidden: !has_revert,
            slash: None,
            slash_aliases: Vec::new(),
        },
        SessionCommandMeta {
            title: if sidebar_visible {
                "Hide sidebar".to_string()
            } else {
                "Show sidebar".to_string()
            },
            value: "session.sidebar.toggle".to_string(),
            category: "Session".to_string(),
            hidden: false,
            slash: None,
            slash_aliases: Vec::new(),
        },
        SessionCommandMeta {
            title: if conceal {
                "Disable code concealment".to_string()
            } else {
                "Enable code concealment".to_string()
            },
            value: "session.toggle.conceal".to_string(),
            category: "Session".to_string(),
            hidden: false,
            slash: None,
            slash_aliases: Vec::new(),
        },
        SessionCommandMeta {
            title: if show_timestamps {
                "Hide timestamps".to_string()
            } else {
                "Show timestamps".to_string()
            },
            value: "session.toggle.timestamps".to_string(),
            category: "Session".to_string(),
            hidden: false,
            slash: Some("timestamps".to_string()),
            slash_aliases: vec!["toggle-timestamps".to_string()],
        },
        SessionCommandMeta {
            title: "Collapse/Expand thinking".to_string(),
            value: "session.toggle.thinking".to_string(),
            category: "Session".to_string(),
            hidden: false,
            slash: Some("thinking".to_string()),
            slash_aliases: vec!["toggle-thinking".to_string()],
        },
        SessionCommandMeta {
            title: if show_details {
                "Hide tool details".to_string()
            } else {
                "Show tool details".to_string()
            },
            value: "session.toggle.actions".to_string(),
            category: "Session".to_string(),
            hidden: false,
            slash: None,
            slash_aliases: Vec::new(),
        },
        SessionCommandMeta {
            title: "Toggle session scrollbar".to_string(),
            value: "session.toggle.scrollbar".to_string(),
            category: "Session".to_string(),
            hidden: false,
            slash: None,
            slash_aliases: Vec::new(),
        },
        SessionCommandMeta {
            title: if show_generic {
                "Hide generic tool output".to_string()
            } else {
                "Show generic tool output".to_string()
            },
            value: "session.toggle.generic_tool_output".to_string(),
            category: "Session".to_string(),
            hidden: false,
            slash: None,
            slash_aliases: Vec::new(),
        },
        SessionCommandMeta {
            title: "Copy last assistant message".to_string(),
            value: "messages.copy".to_string(),
            category: "Session".to_string(),
            hidden: false,
            slash: None,
            slash_aliases: Vec::new(),
        },
        SessionCommandMeta {
            title: "Copy session transcript".to_string(),
            value: "session.copy".to_string(),
            category: "Session".to_string(),
            hidden: false,
            slash: Some("copy".to_string()),
            slash_aliases: Vec::new(),
        },
        SessionCommandMeta {
            title: "Export session transcript".to_string(),
            value: "session.export".to_string(),
            category: "Session".to_string(),
            hidden: false,
            slash: Some("export".to_string()),
            slash_aliases: Vec::new(),
        },
        SessionCommandMeta {
            title: "Background subagents".to_string(),
            value: "session.background".to_string(),
            category: "Session".to_string(),
            hidden: !background_enabled,
            slash: None,
            slash_aliases: Vec::new(),
        },
        SessionCommandMeta {
            title: "Go to child session".to_string(),
            value: "session.child.first".to_string(),
            category: "Session".to_string(),
            hidden: true,
            slash: None,
            slash_aliases: Vec::new(),
        },
        SessionCommandMeta {
            title: "Go to parent session".to_string(),
            value: "session.parent".to_string(),
            category: "Session".to_string(),
            hidden: !has_parent,
            slash: None,
            slash_aliases: Vec::new(),
        },
        SessionCommandMeta {
            title: "Next child session".to_string(),
            value: "session.child.next".to_string(),
            category: "Session".to_string(),
            hidden: !has_parent,
            slash: None,
            slash_aliases: Vec::new(),
        },
        SessionCommandMeta {
            title: "Previous child session".to_string(),
            value: "session.child.previous".to_string(),
            category: "Session".to_string(),
            hidden: !has_parent,
            slash: None,
            slash_aliases: Vec::new(),
        },
    ];
    commands
}

/// Message selectors (mirrors the session memos over message lists).
pub fn session_children(sessions: &[Value], session_id: &str) -> Vec<Value> {
    let session = sessions
        .iter()
        .find(|s| s.get("id").and_then(|v| v.as_str()) == Some(session_id));
    let parent_id = session
        .and_then(|s| s.get("parentID"))
        .and_then(|v| v.as_str())
        .unwrap_or(session_id);
    let mut children: Vec<Value> = sessions
        .iter()
        .filter(|s| {
            s.get("parentID").and_then(|v| v.as_str()) == Some(parent_id)
                || s.get("id").and_then(|v| v.as_str()) == Some(parent_id)
        })
        .cloned()
        .collect();
    children.sort_by(|a, b| {
        a.get("id")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .cmp(b.get("id").and_then(|v| v.as_str()).unwrap_or(""))
    });
    children
}

/// Messages before the revert point (mirrors `messagesBeforeRevert`).
pub fn messages_before_revert(messages: &[Value], revert_message_id: Option<&str>) -> Vec<Value> {
    let Some(revert_id) = revert_message_id else {
        return messages.to_vec();
    };
    match messages
        .iter()
        .position(|message| message.get("id").and_then(|v| v.as_str()) == Some(revert_id))
    {
        Some(index) => messages[..index].to_vec(),
        None => messages.to_vec(),
    }
}

/// Pending assistant index (mirrors the `pending` memo).
pub fn pending_index(messages: &[Value]) -> Option<usize> {
    let completed = messages.iter().rposition(|message| {
        message.get("role").and_then(|v| v.as_str()) == Some("assistant")
            && message
                .get("time")
                .and_then(|t| t.get("completed"))
                .is_some()
    });
    let pending = messages.iter().rposition(|message| {
        message.get("role").and_then(|v| v.as_str()) == Some("assistant")
            && message
                .get("time")
                .and_then(|t| t.get("completed"))
                .is_none()
    });
    match (completed, pending) {
        (_, Some(index)) if Some(index) != completed => Some(index),
        (Some(_), _) => {
            let after = messages.iter().enumerate().rev().find(|(_, message)| {
                message.get("role").and_then(|v| v.as_str()) == Some("assistant")
                    && message
                        .get("time")
                        .and_then(|t| t.get("completed"))
                        .is_none()
            });
            after
                .map(|(index, _)| index)
                .filter(|index| completed.map(|done| *index > done).unwrap_or(true))
        }
        _ => pending,
    }
}

/// User message view (text join, file chips, queued/timestamp footer).
#[derive(Debug, Clone)]
pub struct UserMessageView {
    pub text: String,
    pub files: Vec<(String, bool)>,
    pub queued: bool,
    pub created: i64,
}

pub fn user_message_view(parts: &[Value], created: i64, queued: bool) -> Option<UserMessageView> {
    let texts: Vec<String> = parts
        .iter()
        .filter(|part| {
            part.get("type").and_then(|v| v.as_str()) == Some("text")
                && part.get("synthetic").and_then(|v| v.as_bool()) != Some(true)
        })
        .filter_map(|part| {
            part.get("text")
                .and_then(|v| v.as_str())
                .map(str::to_string)
        })
        .collect();
    if texts.is_empty() {
        return None;
    }
    let files = parts
        .iter()
        .filter(|part| part.get("type").and_then(|v| v.as_str()) == Some("file"))
        .map(|part| {
            (
                part.get("filename")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                part.get("mime").and_then(|v| v.as_str()) == Some("application/x-directory"),
            )
        })
        .collect();
    Some(UserMessageView {
        text: texts.join("\n\n"),
        files,
        queued,
        created,
    })
}

/// Assistant footer line (mirrors the `▣ mode · model · duration` row).
#[derive(Debug, Clone)]
pub struct AssistantFooter {
    pub aborted: bool,
    pub agent_color: String,
    pub mode: String,
    pub model: String,
    pub duration_ms: u64,
    pub interrupted: bool,
}

pub fn assistant_footer(
    message: &Value,
    last: bool,
    finished: bool,
    agent_color: &str,
    model_name: &str,
    user_created: Option<i64>,
) -> Option<AssistantFooter> {
    let error_name = message
        .get("error")
        .and_then(|e| e.get("name"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if !(last || finished || error_name == "MessageAbortedError") {
        return None;
    }
    let duration_ms = match (
        message
            .get("time")
            .and_then(|t| t.get("completed"))
            .and_then(|v| v.as_i64()),
        user_created,
    ) {
        (Some(completed), Some(created)) => completed.saturating_sub(created).max(0) as u64,
        _ => 0,
    };
    Some(AssistantFooter {
        aborted: error_name == "MessageAbortedError",
        agent_color: agent_color.to_string(),
        mode: titlecase(message.get("mode").and_then(|v| v.as_str()).unwrap_or("")),
        model: model_name.to_string(),
        duration_ms,
        interrupted: error_name == "MessageAbortedError",
    })
}

/// Reasoning view (mirrors `ReasoningPart` + header text).
#[derive(Debug, Clone)]
pub struct ReasoningView {
    pub title: Option<String>,
    pub body: String,
    pub done: bool,
    pub duration_ms: u64,
    pub open: bool,
    pub toggleable: bool,
    pub opaque: bool,
}

pub fn reasoning_view(
    text: &str,
    has_metadata: bool,
    end: Option<i64>,
    start: i64,
    hide_mode: bool,
    expanded: bool,
) -> Option<ReasoningView> {
    let content = text.replace("[REDACTED]", "").trim().to_string();
    let opaque = content.is_empty() && has_metadata;
    if content.is_empty() && !opaque {
        return None;
    }
    let done = end.is_some();
    let duration_ms = end.map(|end| (end - start).max(0) as u64).unwrap_or(0);
    let (title, body) = crate::context::thinking::reasoning_summary(&content);
    Some(ReasoningView {
        title,
        body,
        done,
        duration_ms,
        open: !hide_mode || expanded,
        toggleable: hide_mode && !opaque,
        opaque,
    })
}

/// Reasoning header line (mirrors `ReasoningHeader` completed text).
pub fn reasoning_header(
    toggleable: bool,
    open: bool,
    done: bool,
    title: Option<&str>,
    duration_ms: Option<u64>,
    encrypted: bool,
) -> String {
    if !done {
        if let Some(title) = title {
            return format!("Thinking: {title}");
        }
        return "Thinking".to_string();
    }
    if encrypted {
        return match duration_ms {
            Some(duration) => format!("Thought · {}", duration(duration)),
            None => "Thought".to_string(),
        };
    }
    let detail = [title.map(str::to_string), duration_ms.map(duration)]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(" · ");
    let prefix = if toggleable {
        if open {
            "- "
        } else {
            "+ "
        }
    } else {
        ""
    };
    format!(
        "{prefix}Thought{}",
        if detail.is_empty() {
            String::new()
        } else {
            format!(": {detail}")
        }
    )
}

/// Tool view dispatch overview (mirrors `ToolPart` hide rule + mapping).
#[derive(Debug, Clone)]
pub struct ToolViewHead {
    pub display: String,
    pub hidden: bool,
}

pub fn tool_view_head(tool: &str, status: &str, show_details: bool) -> ToolViewHead {
    ToolViewHead {
        display: tool_display(tool).to_string(),
        hidden: !show_details && status == "completed",
    }
}

/// Inline tool error classes (mirrors the denied/failed memos).
pub fn tool_error_class(error: Option<&str>) -> (bool, bool) {
    let Some(error) = error else {
        return (false, false);
    };
    let denied = error.contains("QuestionRejectedError")
        || error.contains("rejected permission")
        || error.contains("specified a rule")
        || error.contains("user dismissed");
    (denied, !denied)
}

/// Shell view descriptor.
#[derive(Debug, Clone)]
pub struct ShellView {
    pub running: bool,
    pub command: String,
    pub output: String,
    pub limited: String,
    pub overflow: bool,
    pub expanded: bool,
    pub workdir_title: Option<String>,
}

pub fn shell_view(
    input: &Value,
    metadata: &Value,
    status: &str,
    width: usize,
    expanded: bool,
    format_path: &dyn Fn(Option<&str>) -> String,
) -> ShellView {
    let command = input
        .get("command")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let output = metadata
        .get("output")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    let collapsed = collapse_tool_output(&output, 10, 10 * width.max(20));
    let workdir = input.get("workdir").and_then(|v| v.as_str()).unwrap_or("");
    let workdir_title = if workdir.is_empty() || workdir == "." {
        None
    } else {
        let formatted = format_path(Some(workdir));
        if formatted == "." {
            None
        } else {
            Some(format!("# Running in {formatted}"))
        }
    };
    ShellView {
        running: status == "running",
        command,
        output: output.clone(),
        limited: if expanded || !collapsed.overflow {
            output
        } else {
            collapsed.output
        },
        overflow: collapsed.overflow,
        expanded,
        workdir_title,
    }
}

/// Task content lines (mirrors the `content` memo).
pub fn task_content(
    description: &str,
    subagent_type: &str,
    background: bool,
    running: bool,
    retry: Option<(u64, String)>,
    current_tool: Option<(String, Option<String>)>,
    tool_count: usize,
    completed: bool,
    duration_ms: u64,
) -> String {
    let mut content = vec![format_subagent_title(
        &titlecase(subagent_type),
        description,
        background,
    )];
    if running {
        if let Some((attempt, message)) = retry {
            content.push(format!(
                "↳ {}",
                format_subagent_retry(attempt, &truncate(&message, 80))
            ));
        } else if tool_count > 0 {
            match current_tool {
                Some((tool, title)) => content.push(format!(
                    "↳ {} {}",
                    titlecase(&tool),
                    title.unwrap_or_default()
                )),
                None => content.push(format!("↳ {}", format_subagent_toolcalls(tool_count))),
            }
        }
    }
    if !running && completed {
        content.push(format!(
            "↳ {}",
            format_completed_subagent_detail(tool_count, &duration(duration_ms as i64))
        ));
    }
    content.join("\n")
}

/// Question tool summary (mirrors the `Question` renderer).
pub fn question_tool_summary(
    questions: &[String],
    answers: Option<&[Vec<String>]>,
) -> (Vec<(String, String)>, String, usize) {
    let rows: Vec<(String, String)> = questions
        .iter()
        .enumerate()
        .map(|(index, question)| {
            let answer = answers
                .and_then(|list| list.get(index))
                .map(|list| list.join(", "))
                .unwrap_or_default();
            let answer = if answer.is_empty() {
                "(no answer)".to_string()
            } else {
                answer
            };
            (question.clone(), answer)
        })
        .collect();
    let count = questions.len();
    (
        rows,
        format!(
            "Asked {count} question{}",
            if count == 1 { "" } else { "s" }
        ),
        count,
    )
}

/// Diagnostic line (mirrors `Diagnostics` rows, 1-based positions).
pub fn diagnostic_line(entry: &DiagnosticEntry) -> String {
    format!(
        "Error [{}:{}] {}",
        entry.line + 1,
        entry.character + 1,
        entry.message
    )
}

/// Session export filename (mirrors `session-${id.slice(0, 8)}.md`).
pub fn export_filename(session_id: &str) -> String {
    format!("session-{}.md", session_id.get(..8).unwrap_or(session_id))
}

/// Transcript builder input (mirrors the copy/export call sites).
pub fn transcript_messages(
    messages: &[Value],
    parts_of: &dyn Fn(&str) -> Vec<Value>,
) -> Vec<crate::util::transcript::MessageWithParts> {
    messages
        .iter()
        .map(|message| crate::util::transcript::MessageWithParts {
            info: serde_json::from_value(message.clone()).unwrap_or_default(),
            parts: parts_of(message.get("id").and_then(|v| v.as_str()).unwrap_or("")),
        })
        .collect()
}

/// Undo target (mirrors the undo run: last user message before revert).
pub fn undo_target(messages: &[Value]) -> Option<Value> {
    messages
        .iter()
        .rev()
        .find(|message| message.get("role").and_then(|v| v.as_str()) == Some("user"))
        .cloned()
}

/// Redo target (mirrors the redo run: first user message after revert).
pub fn redo_target(messages: &[Value], revert_message_id: &str) -> Option<Value> {
    messages
        .iter()
        .find(|message| {
            message.get("role").and_then(|v| v.as_str()) == Some("user")
                && message.get("id").and_then(|v| v.as_str()).unwrap_or("") > revert_message_id
        })
        .cloned()
}

/// Last-user-message jump target (mirrors `messages_last_user`).
pub fn last_user_message_id(
    messages: &[Value],
    parts_of: &dyn Fn(&str) -> Vec<Value>,
) -> Option<String> {
    messages
        .iter()
        .rev()
        .find(|message| {
            if message.get("role").and_then(|v| v.as_str()) != Some("user") {
                return false;
            }
            let id = message.get("id").and_then(|v| v.as_str()).unwrap_or("");
            parts_of(id).iter().any(|part| {
                part.get("type").and_then(|v| v.as_str()) == Some("text")
                    && part.get("synthetic").and_then(|v| v.as_bool()) != Some(true)
                    && part.get("ignored").and_then(|v| v.as_bool()) != Some(true)
            })
        })
        .and_then(|message| message.get("id"))
        .and_then(|v| v.as_str())
        .map(str::to_string)
}

/// Copy-text builders (mirrors the `messages.copy` run).
pub fn last_assistant_text(
    messages: &[Value],
    parts_of: &dyn Fn(&str) -> Vec<Value>,
) -> Result<String, &'static str> {
    let message = messages
        .iter()
        .rev()
        .find(|message| message.get("role").and_then(|v| v.as_str()) == Some("assistant"));
    let Some(message) = message else {
        return Err("No assistant messages found");
    };
    let id = message.get("id").and_then(|v| v.as_str()).unwrap_or("");
    let texts: Vec<String> = parts_of(id)
        .iter()
        .filter(|part| part.get("type").and_then(|v| v.as_str()) == Some("text"))
        .filter_map(|part| {
            part.get("text")
                .and_then(|v| v.as_str())
                .map(str::to_string)
        })
        .collect();
    if texts.is_empty() {
        return Err("No text parts found in last assistant message");
    }
    let text = texts.join("\n").trim().to_string();
    if text.is_empty() {
        return Err("No text content found in last assistant message");
    }
    Ok(text)
}

/// Timeline/dialog option builders shared with the dialogs.
pub fn timeline_select_options(
    messages: &[Value],
    parts_of: &dyn Fn(&str) -> Vec<Value>,
) -> Vec<SelectOption> {
    super::dialog_timeline::timeline_options(messages, parts_of)
}

/// Revert banner descriptor (mirrors the revert match).
#[derive(Debug, Clone)]
pub struct RevertBanner {
    pub reverted_count: usize,
    pub diff_files: Vec<RevertDiffFile>,
}

pub fn revert_banner(messages: &[Value], revert_message_id: &str, diff: &str) -> RevertBanner {
    let reverted = messages
        .iter()
        .skip_while(|message| message.get("id").and_then(|v| v.as_str()) != Some(revert_message_id))
        .filter(|message| message.get("role").and_then(|v| v.as_str()) == Some("user"))
        .count();
    RevertBanner {
        reverted_count: reverted,
        diff_files: get_revert_diff_files(diff),
    }
}

/// Share consent copy verbatim.
pub const SHARE_CONFIRM_TITLE: &str = "Share Session";
pub const SHARE_CONFIRM_MESSAGE: &str = "Are you sure you want to share it?";
pub const SHARE_COPIED_MESSAGE: &str = "Share URL copied to clipboard!";
pub const SHARE_FAILED_MESSAGE: &str = "Failed to copy URL to clipboard";
pub const UNSHARE_DONE_MESSAGE: &str = "Session unshared successfully";
pub const TRANSCRIPT_COPIED_MESSAGE: &str = "Session transcript copied to clipboard!";
pub const TRANSCRIPT_COPY_FAILED_MESSAGE: &str = "Failed to copy session transcript";
pub const EXPORT_FAILED_MESSAGE: &str = "Failed to export session";
pub const SESSION_NOT_FOUND_PREFIX: &str = "Session not found: ";
pub const CREATE_SESSION_FAILED_MESSAGE: &str =
    "Creating a session failed. Open console for more details.";
pub const SEND_PROMPT_FAILED_TITLE: &str = "Failed to send prompt";
pub const COMPACT_NO_PROVIDER_MESSAGE: &str = "Connect a provider to summarize this session";
pub const REDO_CONFIRM_TITLE: &str = "Confirm Redo";
pub const REDO_CONFIRM_MESSAGE: &str = "Are you sure you want to restore the reverted messages?";
pub const RETRY_ERROR_TITLE: &str = "Retry Error";

/// Message visibility under a revert point (mirrors the render Switch).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageVisibility {
    RevertBanner,
    Hidden,
    Shown,
}

pub fn message_visibility(
    message_id: &str,
    revert_message_id: Option<&str>,
    revert_index: i64,
    index: i64,
) -> MessageVisibility {
    match revert_message_id {
        Some(revert_id) if revert_id == message_id => MessageVisibility::RevertBanner,
        Some(_) if revert_index != -1 && index >= revert_index => MessageVisibility::Hidden,
        _ => MessageVisibility::Shown,
    }
}

/// Child navigation helpers (mirrors moveFirstChild/moveChild).
pub fn first_child_id(children: &[Value]) -> Option<String> {
    if children.len() == 1 {
        return None;
    }
    children
        .iter()
        .find(|session| session.get("parentID").is_some())
        .and_then(|session| session.get("id"))
        .and_then(|v| v.as_str())
        .map(str::to_string)
}

pub fn child_id(children: &[Value], current_id: &str, direction: i64) -> Option<String> {
    if children.len() == 1 {
        return None;
    }
    let sessions: Vec<&Value> = children
        .iter()
        .filter(|session| session.get("parentID").is_some())
        .collect();
    let position = sessions
        .iter()
        .position(|session| session.get("id").and_then(|v| v.as_str()) == Some(current_id))?
        as i64
        - direction;
    let len = sessions.len() as i64;
    let next = if position >= len {
        0
    } else if position < 0 {
        len - 1
    } else {
        position
    };
    sessions
        .get(next as usize)
        .and_then(|session| session.get("id"))
        .and_then(|v| v.as_str())
        .map(str::to_string)
}

/// Session init flow descriptor (mirrors the mount effect outcomes).
#[derive(Debug, Clone)]
pub enum SessionInitNext {
    Ready,
    MissingSession { session_id: String },
    Failed { session_id: String, message: String },
}

/// Go-upsell gate (mirrors the `session.status` handler).
#[derive(Debug, Clone)]
pub struct GoUpsellGate {
    pub last_seen_key: String,
    pub dont_show_key: String,
}

pub fn go_upsell_gate(
    provider: Option<&str>,
    reason: Option<&str>,
    has_dialog: bool,
    last_seen: Option<i64>,
    dont_show: bool,
    now_ms: i64,
) -> Option<GoUpsellGate> {
    if has_dialog {
        return None;
    }
    let (last_seen_key, dont_show_key) = go_upsell_keys(provider, reason)?;
    if let Some(seen) = last_seen {
        if now_ms - seen < GO_UPSELL_WINDOW_MS {
            return None;
        }
    }
    if dont_show {
        return None;
    }
    Some(GoUpsellGate {
        last_seen_key,
        dont_show_key,
    })
}

/// Plan switch events (mirrors the `message.part.updated` handler).
pub fn plan_switch_agent(
    tool: &str,
    status: &str,
    part_id: &str,
    last_switch: Option<&str>,
) -> Option<String> {
    if tool != "plan_exit" && tool != "plan_enter" {
        return None;
    }
    if status != "completed" {
        return None;
    }
    if last_switch == Some(part_id) {
        return None;
    }
    Some(if tool == "plan_exit" {
        "build".to_string()
    } else {
        "plan".to_string()
    })
}

/// Thinking toggle title (mirrors the dynamic command title).
pub fn thinking_toggle_title(hide_next: bool) -> &'static str {
    if hide_next {
        "Collapse thinking"
    } else {
        "Expand thinking"
    }
}

/// Session prompt slot descriptor (visibility/disabled wiring).
#[derive(Debug, Clone, Copy)]
pub struct SessionPromptSlot {
    pub visible: bool,
    pub disabled: bool,
}

/// Sidebar mode descriptor (wide → inline, narrow → overlay).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidebarMode {
    Inline,
    Overlay,
    Hidden,
}

pub fn sidebar_mode(has_parent: bool, open: bool, auto_wide: bool) -> SidebarMode {
    if has_parent {
        return SidebarMode::Hidden;
    }
    if open {
        return SidebarMode::Inline;
    }
    if auto_wide {
        return SidebarMode::Inline;
    }
    SidebarMode::Hidden
}

/// Overlay backdrop alpha verbatim (0,0,0,70).
pub const SIDEBAR_OVERLAY_ALPHA: u8 = 70;

/// Session context values shared with message renderers.
#[derive(Debug, Clone)]
pub struct SessionRenderContext {
    pub width: usize,
    pub session_id: String,
    pub conceal: bool,
    pub thinking_hide: bool,
    pub show_timestamps: bool,
    pub show_details: bool,
    pub show_generic_tool_output: bool,
    pub diff_word_wrap: bool,
}

/// Diff view selector shared by Edit/ApplyPatch (mirrors both memos).
pub fn tool_diff_view(diff_style: &str, width: usize) -> &'static str {
    if diff_style == "stacked" {
        return "unified";
    }
    if width > 120 {
        "split"
    } else {
        "unified"
    }
}

/// Rebuilt prompt from undo (mirrors the undo reducer verbatim).
pub fn undo_prompt(parts: &[Value]) -> PromptInfo {
    rebuild_prompt_from_parts(parts)
}

/// Copy prompt builder for `messages.copy` errors (verbatim strings used
/// by the app toasts are exported above).
pub fn copy_text(parts: &[Value]) -> String {
    copy_text_from_parts(parts)
}

/// Quick-switch footer hints (mirrors `quickSwitchFooterHints`).
pub fn quick_switch_hints(
    first: Option<&str>,
    last: Option<&str>,
    slots: usize,
) -> Vec<(String, String)> {
    let (Some(first), Some(last)) = (first, last) else {
        return Vec::new();
    };
    if slots == 0 {
        return Vec::new();
    }
    vec![(
        "switch".to_string(),
        super::super::dialog_session_list::quick_switch_range(first, last),
    )]
}

/// Error copy helpers.
pub fn error_text(error: &Value) -> String {
    error_message_value(error)
}

pub fn today_label(created_ms: i64) -> String {
    today_time_or_date_time(created_ms)
}

pub fn time_label(created_ms: i64) -> String {
    time(created_ms)
}
