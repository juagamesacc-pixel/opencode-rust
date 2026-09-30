//! Rust port of `packages/app/src/utils/comment-note.ts` (opencode v1.18.30).
//!
//! Source 88 lines: `PromptComment`, `createCommentMetadata`,
//! `readCommentMetadata`, `formatCommentNote`, `parseCommentNote`.
//! Verbatim strings/regex preserved.
//! Original file: `packages/app/src/utils/comment-note.ts`

#![allow(dead_code)]

use serde_json::{json, Value};

/// Mirrors `FileSelection` (`@/context/file`).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct FileSelection {
    #[serde(rename = "startLine")]
    pub start_line: i64,
    #[serde(rename = "startChar")]
    pub start_char: i64,
    #[serde(rename = "endLine")]
    pub end_line: i64,
    #[serde(rename = "endChar")]
    pub end_char: i64,
}

/// Mirrors `PromptComment`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PromptComment {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection: Option<FileSelection>,
    pub comment: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}

fn parse_selection(value: &Value) -> Option<FileSelection> {
    let start_line = value.get("startLine")?.as_i64()?;
    let start_char = value.get("startChar")?.as_i64()?;
    let end_line = value.get("endLine")?.as_i64()?;
    let end_char = value.get("endChar")?.as_i64()?;
    Some(FileSelection {
        start_line,
        start_char,
        end_line,
        end_char,
    })
}

/// Mirrors `createCommentMetadata(input)`.
pub fn create_comment_metadata(input: &PromptComment) -> Value {
    json!({
        "opencodeComment": {
            "path": input.path,
            "selection": input.selection,
            "comment": input.comment,
            "preview": input.preview,
            "origin": input.origin,
        }
    })
}

/// Mirrors `readCommentMetadata(value)`.
pub fn read_comment_metadata(value: &Value) -> Option<PromptComment> {
    let meta = value.get("opencodeComment")?;
    if !meta.is_object() {
        return None;
    }
    let path = meta.get("path")?.as_str()?.to_string();
    let comment = meta.get("comment")?.as_str()?.to_string();
    let preview = meta
        .get("preview")
        .and_then(|v| v.as_str())
        .map(str::to_string);
    let origin = match meta.get("origin").and_then(|v| v.as_str()) {
        Some("review") => Some("review".to_string()),
        Some("file") => Some("file".to_string()),
        _ => None,
    };
    let selection = meta.get("selection").and_then(parse_selection);
    Some(PromptComment {
        path,
        selection,
        comment,
        preview,
        origin,
    })
}

/// Mirrors `formatCommentNote(input)`.
pub fn format_comment_note(path: &str, selection: Option<&FileSelection>, comment: &str) -> String {
    let range = match selection {
        None => "this file".to_string(),
        Some(sel) => {
            let start = sel.start_line.min(sel.end_line);
            let end = sel.start_line.max(sel.end_line);
            if start == end {
                format!("line {start}")
            } else {
                format!("lines {start} through {end}")
            }
        }
    };
    format!("The user made the following comment regarding {range} of {path}: {comment}")
}

/// Mirrors `parseCommentNote(text)`.
#[allow(clippy::question_mark)]
pub fn parse_comment_note(text: &str) -> Option<PromptComment> {
    let prefix = "The user made the following comment regarding ";
    let rest = text.strip_prefix(prefix)?;
    let (range_part, tail) = rest.split_once(" of ")?;
    let (path, comment) = tail.split_once(": ")?;
    if path.is_empty() || comment.is_empty() {
        return None;
    }
    let selection = if range_part == "this file" {
        None
    } else if let Some(line) = range_part.strip_prefix("line ") {
        let n: i64 = line.parse().ok()?;
        Some(FileSelection {
            start_line: n,
            start_char: 0,
            end_line: n,
            end_char: 0,
        })
    } else if let Some(lines) = range_part.strip_prefix("lines ") {
        let (a, b) = lines.split_once(" through ")?;
        let start: i64 = a.parse().ok()?;
        let end: i64 = b.parse().ok()?;
        Some(FileSelection {
            start_line: start,
            start_char: 0,
            end_line: end,
            end_char: 0,
        })
    } else {
        return None;
    };
    Some(PromptComment {
        path: path.to_string(),
        selection,
        comment: comment.to_string(),
        preview: None,
        origin: None,
    })
}
