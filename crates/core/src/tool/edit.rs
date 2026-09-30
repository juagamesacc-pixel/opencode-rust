//! Rust port of `packages/core/src/tool/edit.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.

use serde::{Deserialize, Serialize};

pub const NAME: &str = "edit";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    pub path: String,
    #[serde(rename = "oldString")]
    pub old_string: String,
    #[serde(rename = "newString")]
    pub new_string: String,
    #[serde(rename = "replaceAll", skip_serializing_if = "Option::is_none")]
    pub replace_all: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    pub files: Vec<FileDiffInfo>,
    pub replacements: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDiffInfo {
    pub file: String,
    pub patch: String,
    pub status: String,
    pub additions: usize,
    pub deletions: usize,
}

pub fn normalize_line_endings(text: &str) -> String {
    text.replace("\r\n", "\n")
}

pub fn detect_line_ending(text: &str) -> &'static str {
    if text.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    }
}

pub fn convert_to_line_ending(text: &str, ending: &str) -> String {
    if ending == "\n" {
        normalize_line_endings(text)
    } else {
        normalize_line_endings(text).replace('\n', "\r\n")
    }
}

pub fn split_bom(text: &str) -> (bool, String) {
    match text.strip_prefix('\u{FEFF}') {
        Some(rest) => (true, rest.to_string()),
        None => (false, text.to_string()),
    }
}

pub fn join_bom(text: String, bom: bool) -> String {
    if bom {
        format!("\u{FEFF}{text}")
    } else {
        text
    }
}

pub fn decode_utf8(content: &[u8]) -> (bool, String) {
    let bom = content.len() >= 3 && content[0] == 0xef && content[1] == 0xbb && content[2] == 0xbf;
    let slice = if bom { &content[3..] } else { content };
    let text = String::from_utf8_lossy(slice).to_string();
    (bom, text)
}

pub fn count_occurrences(content: &str, search: &str) -> usize {
    if search.is_empty() {
        return content.len() + 1;
    }
    let mut count = 0;
    let mut offset = 0;
    while let Some(pos) = content[offset..].find(search) {
        count += 1;
        offset += pos + search.len();
        if offset > content.len() {
            break;
        }
    }
    count
}

pub fn preview_lines(value: &str, prefix: char) -> Vec<String> {
    let normalized = normalize_line_endings(value);
    let lines: Vec<&str> = normalized.split('\n').collect();
    let mut shown: Vec<String> = lines
        .iter()
        .take(6)
        .map(|line| {
            let truncated = if line.len() > 240 {
                format!("{}...", &line[..240])
            } else {
                line.to_string()
            };
            format!("{prefix}{truncated}")
        })
        .collect();
    if lines.len() > shown.len() {
        shown.push(format!("{prefix}..."));
    }
    shown
}

pub fn to_model_output(output: &Output, old_string: &str, new_string: &str) -> String {
    let file = output.files.first().map(|f| f.file.as_str()).unwrap_or("");
    let mut lines = vec![
        format!("Edited file successfully: {file}"),
        format!("Replacements: {}", output.replacements),
        "```diff".to_string(),
    ];
    lines.extend(preview_lines(old_string, '-'));
    lines.extend(preview_lines(new_string, '+'));
    lines.push("```".to_string());
    lines.join("\n")
}

pub fn validate_input(input: &Input) -> Result<(), String> {
    if input.old_string == input.new_string {
        return Err("No changes to apply: oldString and newString are identical.".to_string());
    }
    if input.old_string.is_empty() {
        return Err(
            "oldString must not be empty. Use write to create or overwrite a file.".to_string(),
        );
    }
    Ok(())
}

// PROVISIONAL pending effect/runtime — Layer wiring requires LocationMutation + FileMutation + FSUtil + PermissionV2 + tokio.
