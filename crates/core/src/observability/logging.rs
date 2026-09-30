//! Rust port of `packages/core/src/observability/logging.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.

use std::path::PathBuf;

use crate::observability::shared::run_id;

pub fn formatter_id(id: Option<&str>) -> String {
    id.unwrap_or_else(|| run_id()).to_string()
}

pub fn file_logger_path(file: Option<&str>, id: Option<&str>) -> (PathBuf, String) {
    let path = file
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/tmp/opencode.log"));
    let run = formatter_id(id);
    (path, run)
}

pub fn minimum_log_level() -> String {
    match std::env::var("OPENCODE_LOG_LEVEL")
        .ok()
        .map(|v| v.to_uppercase())
        .as_deref()
    {
        Some("DEBUG") => "Debug".to_string(),
        Some("INFO") => "Info".to_string(),
        Some("WARN") => "Warn".to_string(),
        Some("ERROR") => "Error".to_string(),
        _ => "Info".to_string(),
    }
}

pub fn loggers() -> Vec<String> {
    if std::env::var("OPENCODE_PRINT_LOGS").ok().as_deref() == Some("1") {
        vec!["file".to_string(), "stderr".to_string()]
    } else {
        vec!["file".to_string()]
    }
}

pub const LOG_FILE_DEFAULT: &str = "opencode.log";
