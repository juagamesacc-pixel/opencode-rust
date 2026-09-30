//! Rust port of `packages/app/src/utils/server-errors.ts` (opencode v1.18.30).
//!
//! Source 109 lines: `formatServerError`, `sessionNotFoundError`,
//! `isLocalSessionNotFoundError`, `isSessionNotFoundError`,
//! `parseReadableConfigInvalidError` (+ provider-model formatting).
//! Verbatim error strings/templates preserved.
//! Original file: `packages/app/src/utils/server-errors.ts`

#![allow(dead_code)]

use serde_json::Value;

/// Mirrors the `Session not found: {sessionID}` message contract (verbatim).
pub fn session_not_found_message(session_id: &str) -> String {
    format!("Session not found: {session_id}")
}

/// Mirrors `sessionNotFoundError(sessionID)`.
pub fn session_not_found_error(session_id: &str) -> String {
    session_not_found_message(session_id)
}

/// Mirrors `isLocalSessionNotFoundError(error, sessionID)`.
pub fn is_local_session_not_found_error(message: &str, session_id: &str) -> bool {
    message == session_not_found_message(session_id)
}

/// Mirrors `isSessionNotFoundError(error, sessionID)` over unwrapped bodies.
pub fn is_session_not_found_error(body: &Value, session_id: &str) -> bool {
    body.get("_tag").and_then(|v| v.as_str()) == Some("SessionNotFoundError")
        && body.get("sessionID").and_then(|v| v.as_str()) == Some(session_id)
}

fn lookup(key: &str, vars: Option<&[(&str, &str)]>) -> Option<String> {
    let _ = (key, vars);
    None
}

/// Mirrors `parseReadableConfigInvalidError(error, translator?)`.
/// Default (untranslated) templates are verbatim:
/// `Config file at {path} is invalid` / `Config file at {path} is invalid: {message}`.
#[allow(clippy::type_complexity)] // 1:1 preserves exact signatures
pub fn parse_readable_config_invalid_error(
    path: Option<&str>,
    message: Option<&str>,
    issues: &[(Vec<String>, String)],
    translate: Option<&dyn Fn(&str, Option<&[(&str, &str)]>) -> String>,
) -> String {
    let file = match path {
        Some(path) if path != "config" => path,
        _ => "config",
    };
    let detail = message.unwrap_or("").trim().to_string();
    let mut lines: Vec<String> = vec![];
    for (issue_path, issue_message) in issues {
        let msg = issue_message.trim();
        if msg.is_empty() {
            continue;
        }
        if issue_path.is_empty() {
            lines.push(msg.to_string());
        } else {
            lines.push(format!("{}: {msg}", issue_path.join(".")));
        }
    }
    let msg = if !lines.is_empty() {
        lines.join("\n")
    } else {
        detail
    };
    if msg.is_empty() {
        match translate {
            Some(t) => {
                let out = t("error.chain.configInvalid", Some(&[("path", file)]));
                if out.is_empty() || out == "error.chain.configInvalid" {
                    return format!("Config file at {file} is invalid");
                }
                out
            }
            None => format!("Config file at {file} is invalid"),
        }
    } else {
        match translate {
            Some(t) => {
                let out = t(
                    "error.chain.configInvalidWithMessage",
                    Some(&[("path", file), ("message", &msg)]),
                );
                if out.is_empty() || out == "error.chain.configInvalidWithMessage" {
                    return format!("Config file at {file} is invalid: {msg}");
                }
                out
            }
            None => format!("Config file at {file} is invalid: {msg}"),
        }
    }
}

/// Mirrors the provider-model templates (verbatim defaults):
/// `Model not found: {provider}/{model}`, `Did you mean: {suggestions}`,
/// `Check your config (opencode.json) provider/model names`, `Unknown error`.
#[allow(clippy::type_complexity)] // 1:1 preserves exact signatures
pub fn format_provider_model_not_found(
    provider: &str,
    model: &str,
    suggestions: &[String],
    translate: Option<&dyn Fn(&str, Option<&[(&str, &str)]>) -> String>,
) -> String {
    let provider = provider.trim();
    let model = model.trim();
    let body = match translate {
        Some(t) => {
            let out = t(
                "error.chain.modelNotFound",
                Some(&[("provider", provider), ("model", model)]),
            );
            if out.is_empty() || out == "error.chain.modelNotFound" {
                format!("Model not found: {provider}/{model}")
            } else {
                out
            }
        }
        None => format!("Model not found: {provider}/{model}"),
    };
    let tail = match translate {
        Some(t) => {
            let out = t("error.chain.checkConfig", None);
            if out.is_empty() || out == "error.chain.checkConfig" {
                "Check your config (opencode.json) provider/model names".to_string()
            } else {
                out
            }
        }
        None => "Check your config (opencode.json) provider/model names".to_string(),
    };
    let list: Vec<String> = suggestions
        .iter()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .collect();
    if list.is_empty() {
        return format!("{body}\n{tail}");
    }
    let joined = list.iter().take(5).cloned().collect::<Vec<_>>().join(", ");
    let middle = match translate {
        Some(t) => {
            let out = t("error.chain.didYouMean", Some(&[("suggestions", &joined)]));
            if out.is_empty() || out == "error.chain.didYouMean" {
                format!("Did you mean: {joined}")
            } else {
                out
            }
        }
        None => format!("Did you mean: {joined}"),
    };
    format!("{body}\n{middle}\n{tail}")
}

/// Mirrors `formatServerError(error, translate?, fallback?)`.
pub fn format_server_error(
    message: Option<&str>,
    string_error: Option<&str>,
    fallback: Option<&str>,
) -> String {
    if let Some(message) = message {
        if !message.is_empty() {
            return message.to_string();
        }
    }
    if let Some(string_error) = string_error {
        if !string_error.is_empty() {
            return string_error.to_string();
        }
    }
    if let Some(fallback) = fallback {
        return fallback.to_string();
    }
    let _ = lookup("error.chain.unknown", None);
    "Unknown error".to_string()
}
