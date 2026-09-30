// source: packages/tui/src/util/transcript.ts (114 lines, v1.18.30)
// 1:1 port — markdown session export; SDK message/part shapes are local
// serde structs (user/assistant merged, parts as data).

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::locale;
use super::model::{self, Provider, ProviderList};

/// Mirrors `TranscriptOptions`.
#[derive(Debug, Clone, Default)]
pub struct TranscriptOptions {
    pub thinking: bool,
    pub tool_details: bool,
    pub assistant_metadata: bool,
    pub providers: Option<Vec<Provider>>,
}

/// Mirrors `SessionInfo`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionInfo {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub time: SessionTime,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionTime {
    #[serde(default)]
    pub created: i64,
    #[serde(default)]
    pub updated: i64,
}

/// Merged `UserMessage | AssistantMessage` fields read by the formatter.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageInfo {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub agent: String,
    #[serde(rename = "providerID", default)]
    pub provider_id: String,
    #[serde(rename = "modelID", default)]
    pub model_id: String,
    #[serde(default)]
    pub time: MessageTime,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageTime {
    #[serde(default)]
    pub created: i64,
    #[serde(default)]
    pub completed: Option<i64>,
}

/// Mirrors `MessageWithParts` (parts stay `Value` — only `type` and the
/// fields below are read).
#[derive(Debug, Clone, Default)]
pub struct MessageWithParts {
    pub info: MessageInfo,
    pub parts: Vec<Value>,
}

fn part_text(part: &Value) -> &str {
    part.get("text").and_then(|t| t.as_str()).unwrap_or("")
}

fn part_state(part: &Value) -> Option<&serde_json::Map<String, Value>> {
    part.get("state")?.as_object()
}

/// Mirrors `formatTranscript`.
pub fn format_transcript(
    session: &SessionInfo,
    messages: &[MessageWithParts],
    options: &TranscriptOptions,
) -> String {
    let providers = options.providers.as_deref().unwrap_or(&[]);
    let mut ordered: Vec<&MessageWithParts> = messages.iter().collect();
    // `toSorted` is stable — ties keep input order.
    ordered.sort_by(|a, b| {
        a.info
            .time
            .created
            .cmp(&b.info.time.created)
            .then_with(|| a.info.id.cmp(&b.info.id))
    });
    let mut transcript = format!("# {}\n\n", session.title);
    transcript += &format!("**Session ID:** {}\n", session.id);
    transcript += &format!(
        "**Created:** {}\n",
        locale::locale_string(session.time.created)
    );
    transcript += &format!(
        "**Updated:** {}\n",
        locale::locale_string(session.time.updated)
    );
    transcript += "---\n\n";
    for msg in ordered {
        transcript += &format_message(
            &msg.info,
            &msg.parts,
            options,
            ProviderList::Slice(providers),
        );
        transcript += "---\n\n";
    }
    transcript
}

/// Mirrors `formatMessage`.
pub fn format_message(
    msg: &MessageInfo,
    parts: &[Value],
    options: &TranscriptOptions,
    providers: ProviderList<'_>,
) -> String {
    let mut result = String::new();
    if msg.role == "user" {
        result += "## User\n\n";
    } else {
        result += &format_assistant_header(msg, options.assistant_metadata, providers);
    }
    for part in parts {
        result += &format_part(part, options);
    }
    result
}

/// Mirrors `formatAssistantHeader`.
pub fn format_assistant_header(
    msg: &MessageInfo,
    include_metadata: bool,
    providers: ProviderList<'_>,
) -> String {
    if !include_metadata {
        return "## Assistant\n\n".to_string();
    }
    let duration = match msg.time.completed {
        Some(completed) if msg.time.created != 0 => {
            format!("{:.1}s", (completed - msg.time.created) as f64 / 1000.0)
        }
        _ => String::new(),
    };
    let model_name = model::name(providers, &msg.provider_id, &msg.model_id);
    format!(
        "## Assistant ({} · {}{})\n\n",
        locale::titlecase(&msg.agent),
        model_name,
        if duration.is_empty() {
            String::new()
        } else {
            format!(" · {duration}")
        }
    )
}

/// Mirrors `formatPart`.
pub fn format_part(part: &Value, options: &TranscriptOptions) -> String {
    let kind = part.get("type").and_then(|t| t.as_str()).unwrap_or("");
    if kind == "text" && part.get("synthetic").and_then(|s| s.as_bool()) != Some(true) {
        return format!("{}\n\n", part_text(part));
    }
    if kind == "reasoning" {
        if options.thinking {
            return format!("_Thinking:_\n\n{}\n\n", part_text(part));
        }
        return String::new();
    }
    if kind == "tool" {
        let tool = part.get("tool").and_then(|t| t.as_str()).unwrap_or("");
        let mut result = format!("**Tool: {tool}**\n");
        if let Some(state) = part_state(part) {
            let input = state.get("input");
            if options.tool_details && input.is_some() {
                let pretty =
                    serde_json::to_string_pretty(input.unwrap_or(&Value::Null)).unwrap_or_default();
                result += &format!("\n**Input:**\n```json\n{pretty}\n```\n");
            }
            let status = state.get("status").and_then(|s| s.as_str()).unwrap_or("");
            if options.tool_details && status == "completed" {
                if let Some(output) = state.get("output").and_then(|o| o.as_str()) {
                    result += &format!("\n**Output:**\n```\n{output}\n```\n");
                }
            }
            if options.tool_details && status == "error" {
                if let Some(error) = state.get("error").and_then(|e| e.as_str()) {
                    result += &format!("\n**Error:**\n```\n{error}\n```\n");
                }
            }
        }
        result += "\n";
        return result;
    }
    String::new()
}
