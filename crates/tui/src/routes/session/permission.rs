// source: packages/tui/src/routes/session/permission.tsx (719 lines, v1.18.30)
// 1:1 port — permission prompt state machine: per-tool icon/title/body
// builders (edit/read/glob/grep/list/bash/task/webfetch/websearch/
// external_directory/doom_loop/fallback), three stages (permission/
// always/reject), the option prompt (left/h/right/l/return/escape,
// fullscreen toggle, expanded portal, maxHeight 15), and the reply
// payloads verbatim.

#![allow(dead_code)]

use serde_json::Value;

use crate::util::locale::titlecase;
use crate::util::tool_display::web_search_provider_label;

/// Stages verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionStage {
    Permission,
    Always,
    Reject,
}

/// Permission option keys (generic prompt options).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionOption {
    Once,
    Always,
    Reject,
}

/// Reply payload builder (mirrors the three `permission.reply` calls).
pub fn permission_reply_params(
    reply: &str,
    request_id: &str,
    directory: Option<&str>,
    workspace: Option<&str>,
    message: Option<&str>,
) -> Value {
    let mut params = serde_json::json!({ "reply": reply, "requestID": request_id });
    if let Some(directory) = directory {
        params["directory"] = Value::String(directory.to_string());
    }
    if let Some(workspace) = workspace {
        params["workspace"] = Value::String(workspace.to_string());
    }
    if let Some(message) = message {
        params["message"] = Value::String(message.to_string());
    }
    params
}

/// Tool input lookup (mirrors the `input` memo: first non-pending tool
/// state input for the request's call).
pub fn tool_input(parts: &[Value], message_id: &str, call_id: &str) -> Value {
    for part in parts {
        if part.get("type").and_then(|v| v.as_str()) != Some("tool") {
            continue;
        }
        if part.get("id").and_then(|v| v.as_str()) != Some(call_id) {
            continue;
        }
        let _ = message_id;
        if part
            .get("state")
            .and_then(|s| s.get("status"))
            .and_then(|v| v.as_str())
            == Some("pending")
        {
            continue;
        }
        if let Some(input) = part.get("state").and_then(|s| s.get("input")) {
            return input.clone();
        }
    }
    Value::Object(Default::default())
}

/// Body kind per permission (mirrors the `info()` branches).
#[derive(Debug, Clone)]
pub struct PermissionInfo {
    pub icon: String,
    pub title: String,
    pub body: PermissionBody,
}

#[derive(Debug, Clone)]
pub enum PermissionBody {
    Edit {
        filepath: String,
        diff: Option<String>,
    },
    Read {
        file_path: String,
    },
    Glob {
        pattern: String,
    },
    Grep {
        pattern: String,
    },
    List {
        dir: String,
    },
    Bash {
        command: String,
    },
    Task {
        task_type: String,
        desc: String,
    },
    WebFetch {
        url: String,
    },
    WebSearch {
        query: String,
        provider_label: String,
    },
    ExternalDirectory {
        dir: String,
        patterns: Vec<String>,
    },
    DoomLoop,
    Fallback {
        permission: String,
    },
}

/// Build the icon/title/body triple (verbatim strings).
pub fn permission_info(
    permission: &str,
    input: &Value,
    metadata: &Value,
    patterns: &[String],
    format_path: &dyn Fn(Option<&str>) -> String,
) -> PermissionInfo {
    let str_field = |value: &Value, key: &str| {
        value
            .get(key)
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string()
    };
    match permission {
        "edit" => {
            let filepath = metadata
                .get("filepath")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            PermissionInfo {
                icon: "→".to_string(),
                title: format!("Edit {}", format_path(Some(filepath))),
                body: PermissionBody::Edit {
                    filepath: filepath.to_string(),
                    diff: metadata
                        .get("diff")
                        .and_then(|v| v.as_str())
                        .map(str::to_string),
                },
            }
        }
        "read" => {
            let file_path = str_field(input, "filePath");
            PermissionInfo {
                icon: "→".to_string(),
                title: format!("Read {}", format_path(Some(&file_path))),
                body: PermissionBody::Read { file_path },
            }
        }
        "glob" => {
            let pattern = str_field(input, "pattern");
            PermissionInfo {
                icon: "✱".to_string(),
                title: format!("Glob \"{pattern}\""),
                body: PermissionBody::Glob { pattern },
            }
        }
        "grep" => {
            let pattern = str_field(input, "pattern");
            PermissionInfo {
                icon: "✱".to_string(),
                title: format!("Grep \"{pattern}\""),
                body: PermissionBody::Grep { pattern },
            }
        }
        "list" => {
            let dir = str_field(input, "path");
            PermissionInfo {
                icon: "→".to_string(),
                title: format!("List {}", format_path(Some(&dir))),
                body: PermissionBody::List { dir },
            }
        }
        "bash" => {
            let command = str_field(input, "command");
            PermissionInfo {
                icon: "#".to_string(),
                title: "Shell command".to_string(),
                body: PermissionBody::Bash { command },
            }
        }
        "task" => {
            let task_type = str_field(input, "subagent_type");
            let desc = str_field(input, "description");
            PermissionInfo {
                icon: "#".to_string(),
                title: format!("{} Task", titlecase(&task_type)),
                body: PermissionBody::Task { task_type, desc },
            }
        }
        "webfetch" => {
            let url = str_field(input, "url");
            PermissionInfo {
                icon: "%".to_string(),
                title: format!("WebFetch {url}"),
                body: PermissionBody::WebFetch { url },
            }
        }
        "websearch" => {
            let query = str_field(input, "query");
            let label = web_search_provider_label(input.get("provider").unwrap_or(&Value::Null))
                .to_string();
            PermissionInfo {
                icon: "◈".to_string(),
                title: format!("{label} \"{query}\""),
                body: PermissionBody::WebSearch {
                    query,
                    provider_label: label,
                },
            }
        }
        "external_directory" => {
            let parent = metadata
                .get("parentDir")
                .and_then(|v| v.as_str())
                .map(str::to_string);
            let filepath = metadata
                .get("filepath")
                .and_then(|v| v.as_str())
                .map(str::to_string);
            let derived = patterns.first().map(|pattern| {
                if pattern.contains('*') {
                    pattern
                        .rsplit('/')
                        .skip(1)
                        .collect::<Vec<_>>()
                        .into_iter()
                        .rev()
                        .collect::<Vec<_>>()
                        .join("/")
                } else {
                    pattern.clone()
                }
            });
            let raw = parent.or(filepath).or(derived);
            PermissionInfo {
                icon: "←".to_string(),
                title: format!("Access external directory {}", format_path(raw.as_deref())),
                body: PermissionBody::ExternalDirectory {
                    dir: format_path(raw.as_deref()),
                    patterns: patterns.to_vec(),
                },
            }
        }
        "doom_loop" => PermissionInfo {
            icon: "⟳".to_string(),
            title: "Continue after repeated failures".to_string(),
            body: PermissionBody::DoomLoop,
        },
        other => PermissionInfo {
            icon: "⚙".to_string(),
            title: format!("Call tool {other}"),
            body: PermissionBody::Fallback {
                permission: other.to_string(),
            },
        },
    }
}

/// Diff view selector (mirrors the `view` memo: stacked → unified,
// otherwise split past 120 columns).
pub fn diff_view(diff_style: &str, term_width: u16) -> &'static str {
    if diff_style == "stacked" {
        return "unified";
    }
    if term_width > 120 {
        "split"
    } else {
        "unified"
    }
}

/// Always-stage body lines (single `*` vs pattern list, verbatim).
pub fn always_body(permission: &str, always: &[String]) -> Vec<String> {
    if always.len() == 1 && always.first().map(|s| s.as_str()) == Some("*") {
        return vec![format!(
            "This will allow {permission} until OpenCode is restarted."
        )];
    }
    let mut lines =
        vec!["This will allow the following patterns until OpenCode is restarted".to_string()];
    lines.extend(always.iter().map(|pattern| format!("- {pattern}")));
    lines
}

/// Generic option prompt state (mirrors the inner `Prompt`).
pub struct OptionPrompt {
    pub options: Vec<(String, String)>,
    pub selected: usize,
    pub expanded: bool,
}

impl OptionPrompt {
    pub fn new(options: Vec<(String, String)>) -> Self {
        Self {
            options,
            selected: 0,
            expanded: false,
        }
    }

    pub fn move_selection(&mut self, direction: i64) {
        if self.options.is_empty() {
            return;
        }
        let len = self.options.len() as i64;
        let index = self
            .options
            .iter()
            .position(|(key, _)| false)
            .unwrap_or(self.selected);
        let _ = index;
        let current = self.selected as i64;
        self.selected = (current + direction).rem_euclid(len) as usize;
    }

    pub fn selected_key(&self) -> Option<&str> {
        self.options.get(self.selected).map(|(key, _)| key.as_str())
    }

    pub fn toggle_expanded(&mut self, fullscreen: bool) {
        if !fullscreen {
            return;
        }
        self.expanded = !self.expanded;
    }

    /// Expanded portal geometry verbatim (absolute overlay).
    pub fn expanded_geometry(term_height: u16) -> (i64, u16, u16, u16) {
        (term_height as i64 * -1 + 1, 1, 2, 2)
    }
}

/// Permission prompt state (stage + option prompt).
pub struct PermissionPromptState {
    pub stage: PermissionStage,
    pub options: OptionPrompt,
}

impl PermissionPromptState {
    pub fn permission() -> Self {
        Self {
            stage: PermissionStage::Permission,
            options: OptionPrompt::new(vec![
                ("once".to_string(), "Allow once".to_string()),
                ("always".to_string(), "Allow always".to_string()),
                ("reject".to_string(), "Reject".to_string()),
            ]),
        }
    }

    pub fn always() -> Self {
        Self {
            stage: PermissionStage::Always,
            options: OptionPrompt::new(vec![
                ("confirm".to_string(), "Confirm".to_string()),
                ("cancel".to_string(), "Cancel".to_string()),
            ]),
        }
    }

    /// Stage transition on option select (mirrors the permission-stage
    /// `onSelect`: always → always-stage; reject → reject-stage when the
    /// session has a parent, else the reply is sent by the caller).
    pub fn select(&mut self, key: &str, session_has_parent: bool) -> PermissionSelectNext {
        match key {
            "always" => {
                self.stage = PermissionStage::Always;
                self.options = Self::always().options;
                PermissionSelectNext::StageChanged
            }
            "reject" => {
                if session_has_parent {
                    self.stage = PermissionStage::Reject;
                    PermissionSelectNext::StageChanged
                } else {
                    PermissionSelectNext::Reply {
                        reply: "reject".to_string(),
                        message: None,
                    }
                }
            }
            _ => PermissionSelectNext::Reply {
                reply: "once".to_string(),
                message: None,
            },
        }
    }
}

pub enum PermissionSelectNext {
    StageChanged,
    Reply {
        reply: String,
        message: Option<String>,
    },
}

/// Reject message (empty → `None`, mirrors `message || undefined`).
pub fn reject_message(message: &str) -> Option<String> {
    let trimmed = message.to_string();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

/// Narrow layout breakpoint verbatim (80 columns).
pub const PERMISSION_NARROW_WIDTH: u16 = 80;
/// Inline max height verbatim.
pub const PERMISSION_MAX_HEIGHT: u16 = 15;
