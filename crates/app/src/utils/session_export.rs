//! Rust port of `packages/app/src/utils/session-export.ts` (opencode v1.18.30).
//!
//! Source 61 lines: `SessionExportData`, `fetchSessionExport`,
//! `sessionExportFilename`, `downloadSessionExport`. Verbatim error strings
//! `Session not found: {id}` / `Failed to load messages for session: {id}`
//! preserved; DOM download is PROVISIONAL.
//! Original file: `packages/app/src/utils/session-export.ts`

#![allow(dead_code)]

use serde_json::Value;

/// Mirrors the `Session not found: {sessionID}` error (verbatim).
pub fn session_export_missing_session(session_id: &str) -> String {
    format!("Session not found: {session_id}")
}

/// Mirrors the `Failed to load messages for session: {sessionID}` error (verbatim).
pub fn session_export_missing_messages(session_id: &str) -> String {
    format!("Failed to load messages for session: {session_id}")
}

/// Mirrors `fetchSessionExport` response selection.
pub fn select_session_export(
    session: Option<Value>,
    messages: Option<Value>,
    session_id: &str,
) -> Result<Value, String> {
    let session = session.ok_or_else(|| session_export_missing_session(session_id))?;
    let messages = messages.ok_or_else(|| session_export_missing_messages(session_id))?;
    Ok(serde_json::json!({ "info": session, "messages": messages }))
}

/// Mirrors `sessionExportFilename(session)` sanitization.
pub fn session_export_filename(id: &str, title: Option<&str>, slug: Option<&str>) -> String {
    let name = title.or(slug).unwrap_or(id);
    let lower = name.to_lowercase();
    let mut clean = String::with_capacity(lower.len());
    for ch in lower.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
            clean.push(ch);
        } else {
            clean.push('-');
        }
    }
    let trimmed = clean.trim_matches('-').to_string();
    if trimmed.is_empty() {
        format!("{id}.json")
    } else {
        format!("{trimmed}.json")
    }
}

// PROVISIONAL: pending DOM download — mirrors `packages/app/src/utils/session-export.ts`.
/// Mirrors `downloadSessionExport(filename, data)` descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionExportDownload {
    pub filename: String,
    pub mime: String,
}

impl SessionExportDownload {
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            mime: "application/json".to_string(),
        }
    }
}
