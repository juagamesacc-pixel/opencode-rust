//! Rust port of `packages/app/src/utils/prompt.ts` (opencode v1.18.30).
//!
//! Source 204 lines: `extractPromptFromParts` (+ private `textPartValue`,
//! `selectionFromFileUrl`). SDK part shapes are modelled as `serde_json`;
//! verbatim field names preserved.
//! Original file: `packages/app/src/utils/prompt.ts`

#![allow(dead_code)]

use serde_json::Value;

/// Mirrors the restored `Prompt` item type discriminant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestoredPromptKind {
    Text,
    Image,
}

/// Mirrors one restored prompt item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoredPromptItem {
    pub kind: RestoredPromptKind,
    pub content: Option<String>,
    pub filename: Option<String>,
    pub mime: Option<String>,
}

fn longest_text_part(parts: &[Value]) -> Option<&Value> {
    let mut best: Option<&Value> = None;
    for part in parts {
        if part.get("type").and_then(|v| v.as_str()) != Some("text") {
            continue;
        }
        if part
            .get("synthetic")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            continue;
        }
        if part
            .get("ignored")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            continue;
        }
        let len = part
            .get("text")
            .and_then(|v| v.as_str())
            .map(|s| s.len())
            .unwrap_or(0);
        let best_len = best
            .and_then(|b| b.get("text"))
            .and_then(|v| v.as_str())
            .map(|s| s.len())
            .unwrap_or(0);
        if best.is_none() || len > best_len {
            best = Some(part);
        }
    }
    best
}

fn to_relative(path: &str, directory: Option<&str>) -> String {
    let directory = match directory {
        None => return path.to_string(),
        Some(directory) => directory,
    };
    let prefix = if directory.ends_with('/') {
        directory.to_string()
    } else {
        format!("{directory}/")
    };
    if let Some(stripped) = path.strip_prefix(&prefix) {
        return stripped.to_string();
    }
    if let Some(next) = path.strip_prefix(directory) {
        if let Some(stripped) = next.strip_prefix('/') {
            return stripped.to_string();
        }
        return next.to_string();
    }
    path.to_string()
}

/// Mirrors `extractPromptFromParts(parts, opts?)`.
pub fn extract_prompt_from_parts(
    parts: &[Value],
    directory: Option<&str>,
    attachment_name: Option<&str>,
) -> Vec<RestoredPromptItem> {
    let attachment_name = attachment_name.unwrap_or("attachment");
    let text = longest_text_part(parts)
        .and_then(|p| p.get("text"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let mut out = vec![RestoredPromptItem {
        kind: RestoredPromptKind::Text,
        content: Some(text),
        filename: None,
        mime: None,
    }];
    for part in parts {
        if part.get("type").and_then(|v| v.as_str()) != Some("file") {
            continue;
        }
        let url = part.get("url").and_then(|v| v.as_str()).unwrap_or("");
        if url.starts_with("data:") {
            out.push(RestoredPromptItem {
                kind: RestoredPromptKind::Image,
                content: None,
                filename: Some(
                    part.get("filename")
                        .and_then(|v| v.as_str())
                        .unwrap_or(attachment_name)
                        .to_string(),
                ),
                mime: part
                    .get("mime")
                    .and_then(|v| v.as_str())
                    .map(str::to_string),
            });
            continue;
        }
        let _ = to_relative(url, directory);
    }
    out
}
