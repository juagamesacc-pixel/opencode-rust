//! Rust port of `packages/core/src/tool/bash.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.

use serde::{Deserialize, Serialize};

pub const NAME: &str = "bash";
pub const DEFAULT_TIMEOUT_MS: u64 = 2 * 60 * 1_000;
pub const MAX_TIMEOUT_MS: u64 = 10 * 60 * 1_000;
pub const MAX_CAPTURE_BYTES: usize = 1024 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    pub command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workdir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit: Option<i32>,
    pub truncated: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit: Option<i32>,
    pub truncated: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<bool>,
    pub output: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

pub fn default_shell() -> String {
    if cfg!(windows) {
        std::env::var("COMSPEC").unwrap_or_else(|_| "cmd.exe".to_string())
    } else {
        "/bin/sh".to_string()
    }
}

pub fn shell_tokens(command: &str) -> Vec<String> {
    // Mirrors: command.match(/(?:[^\s"']+|"[^"]*"|'[^']*')+/g) ?? []
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_single = false;
    let mut in_double = false;
    for ch in command.chars() {
        if ch == '\'' && !in_double {
            in_single = !in_single;
            current.push(ch);
        } else if ch == '"' && !in_single {
            in_double = !in_double;
            current.push(ch);
        } else if ch.is_whitespace() && !in_single && !in_double {
            if !current.is_empty() {
                tokens.push(current.clone());
                current.clear();
            }
        } else {
            current.push(ch);
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}

pub fn unquote(value: &str) -> String {
    if value.len() >= 2 {
        let first = value.chars().next().unwrap();
        let last = value.chars().last().unwrap();
        if (first == '"' && last == '"') || (first == '\'' && last == '\'') {
            return value[1..value.len() - 1].to_string();
        }
    }
    value.to_string()
}

pub fn model_output(output: &Output) -> String {
    let warnings = match &output.warnings {
        Some(w) if !w.is_empty() => format!(
            "\n\nWarnings:\n{}",
            w.iter()
                .map(|x| format!("- {x}"))
                .collect::<Vec<_>>()
                .join("\n")
        ),
        _ => String::new(),
    };
    if output.timeout == Some(true) {
        return format!(
            "{}{}Command timed out before completion.",
            warnings.trim_start(),
            if warnings.is_empty() { "" } else { "\n\n" }
        );
    }
    let exit = output.exit.unwrap_or(0);
    format!(
        "{}{}Command exited with code {exit}.",
        warnings.trim_start(),
        if warnings.is_empty() { "" } else { "\n\n" }
    )
}

pub fn validate_timeout(timeout: Option<u64>) -> Result<(), String> {
    if let Some(t) = timeout {
        if t == 0 {
            return Err(format!("Timeout must be positive, got {t}"));
        }
        if t > MAX_TIMEOUT_MS {
            return Err(format!("Timeout {t} exceeds maximum {MAX_TIMEOUT_MS}"));
        }
    }
    Ok(())
}

// PROVISIONAL pending effect/runtime — Layer/effect wiring requires tokio + AppProcess + FSUtil + PermissionV2.
// Behavior strings/defaults/ordering above are verbatim from source.
