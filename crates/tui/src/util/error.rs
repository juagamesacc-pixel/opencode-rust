// source: packages/tui/src/util/error.ts (182 lines, v1.18.30)
// 1:1 port — data-driven branches operate on serde_json::Value; real-error
// branches take an explicit StdError shape (name/message/stack/cause).

#![allow(dead_code)]

use serde_json::Value;
use std::sync::atomic::{AtomicI32, Ordering};

use super::record::as_record;

/// Process exit code requested via a tagged `CliError` (mirrors
/// `process.exitCode = input.exitCode`). `run()` reads it at shutdown.
/// `-1` means unset.
pub static CLI_EXIT_CODE: AtomicI32 = AtomicI32::new(-1);

/// A real error instance (mirrors `instanceof Error` + `cause`).
#[derive(Debug, Clone, Default)]
pub struct StdError {
    pub name: String,
    pub message: String,
    pub stack: Option<String>,
    pub cause: Option<Box<StdError>>,
}

#[derive(Debug, Clone, Default)]
struct ConfigIssue {
    message: String,
    path: Vec<String>,
}

fn tagged(input: &Value, tag: &str) -> bool {
    as_record(input)
        .and_then(|m| m.get("_tag"))
        .and_then(|t| t.as_str())
        == Some(tag)
}

fn named(input: &Value, name: &str) -> bool {
    match as_record(input) {
        Some(map) => {
            map.get("name").and_then(|n| n.as_str()) == Some(name)
                || map.get("_tag").and_then(|t| t.as_str()) == Some(name)
        }
        None => false,
    }
}

fn config_data<'a>(input: &'a Value, tag: &str) -> Option<&'a Value> {
    let map = as_record(input)?;
    if map.get("name").and_then(|n| n.as_str()) == Some(tag) {
        if let Some(data @ Value::Object(_)) = map.get("data") {
            return Some(data);
        }
    }
    if map.get("_tag").and_then(|t| t.as_str()) == Some(tag) {
        return Some(input);
    }
    None
}

fn field(input: &Value, key: &str) -> Option<String> {
    as_record(input)?.get(key)?.as_str().map(str::to_string)
}

fn string_list(value: &Value) -> Vec<String> {
    match value {
        Value::Array(items) => items
            .iter()
            .filter_map(|i| i.as_str().map(str::to_string))
            .collect(),
        _ => Vec::new(),
    }
}

/// Mirrors `cliErrorMessage` for data-driven errors.
pub fn cli_error_message_value(input: &Value) -> Option<String> {
    if let Some(map) = as_record(input) {
        if let Some(Value::Object(_)) = map.get("cause") {
            if let Some(body) = map
                .get("cause")
                .and_then(|c| as_record(c))
                .and_then(|c| c.get("body"))
            {
                if let Some(formatted) = cli_error_message_value(body) {
                    return Some(formatted);
                }
            }
        }
    }

    if tagged(input, "CliError") {
        if let Some(code) = as_record(input)
            .and_then(|m| m.get("exitCode"))
            .and_then(|c| c.as_i64())
        {
            CLI_EXIT_CODE.store(code as i32, Ordering::Relaxed);
        }
        return Some(field(input, "message").unwrap_or_default());
    }
    if tagged(input, "AccountServiceError") || tagged(input, "AccountTransportError") {
        return Some(field(input, "message").unwrap_or_default());
    }

    if let Some(model) = config_data(input, "ProviderModelNotFoundError") {
        let suggestions = string_list(
            as_record(model)
                .and_then(|m| m.get("suggestions"))
                .unwrap_or(&Value::Null),
        );
        let mut lines = vec![format!(
            "Model not found: {}/{}",
            field(model, "providerID").unwrap_or_default(),
            field(model, "modelID").unwrap_or_default()
        )];
        if !suggestions.is_empty() {
            lines.push(format!("Did you mean: {}", suggestions.join(", ")));
        }
        lines.push("Try: `opencode models` to list available models".to_string());
        lines.push("Or check your config (opencode.json) provider/model names".to_string());
        return Some(lines.join("\n"));
    }

    if let Some(provider) = config_data(input, "ProviderInitError") {
        return Some(format!(
            "Failed to initialize provider \"{}\". Check credentials and configuration.",
            field(provider, "providerID").unwrap_or_default()
        ));
    }

    if let Some(json) = config_data(input, "ConfigJsonError") {
        let message = field(json, "message");
        return Some(
            format!(
                "Config file at {} is not valid JSON(C)",
                field(json, "path").unwrap_or_default()
            ) + &message.map(|m| format!(": {m}")).unwrap_or_default(),
        );
    }

    if let Some(directory) = config_data(input, "ConfigDirectoryTypoError") {
        return Some(format!(
            "Directory \"{}\" in {} is not valid. Rename the directory to \"{}\" or remove it. This is a common typo.",
            field(directory, "dir").unwrap_or_default(),
            field(directory, "path").unwrap_or_default(),
            field(directory, "suggestion").unwrap_or_default()
        ));
    }

    if let Some(frontmatter) = config_data(input, "ConfigFrontmatterError") {
        return Some(field(frontmatter, "message").unwrap_or_default());
    }

    if let Some(invalid) = config_data(input, "ConfigInvalidError") {
        let path = field(invalid, "path").unwrap_or_default();
        let message = field(invalid, "message");
        let issues: Vec<ConfigIssue> = match as_record(invalid).and_then(|m| m.get("issues")) {
            Some(Value::Array(items)) => items
                .iter()
                .filter_map(|issue| {
                    let map = as_record(issue)?;
                    let message = map.get("message")?.as_str()?.to_string();
                    let path = match map.get("path") {
                        Some(Value::Array(p)) => p
                            .iter()
                            .map(|i| i.as_str().unwrap_or("").to_string())
                            .collect(),
                        _ => return None,
                    };
                    if !map.get("path").map(|p| p.is_array()).unwrap_or(false) {
                        return None;
                    }
                    Some(ConfigIssue { message, path })
                })
                .collect(),
            _ => Vec::new(),
        };
        let mut lines = vec![
            format!(
                "Configuration is invalid{}",
                if !path.is_empty() && path != "config" {
                    format!(" at {path}")
                } else {
                    String::new()
                }
            ) + &message.map(|m| format!(": {m}")).unwrap_or_default(),
        ];
        lines.extend(
            issues
                .iter()
                .map(|issue| format!("↳ {} {}", issue.message, issue.path.join("."))),
        );
        return Some(lines.join("\n"));
    }

    if tagged(input, "UICancelledError") || named(input, "UICancelledError") {
        return Some(String::new());
    }
    if named(input, "MCPFailed") {
        let name = as_record(input)
            .and_then(|m| m.get("data"))
            .and_then(as_record)
            .and_then(|d| d.get("name"))
            .and_then(|n| n.as_str())
            .unwrap_or_default();
        return Some(format!(
            "MCP server \"{name}\" failed. Note, opencode does not support MCP authentication yet."
        ));
    }
    None
}

/// Mirrors `cliErrorMessage` for real errors (follows the `cause.body` chain).
pub fn cli_error_message_std(error: &StdError) -> Option<String> {
    let cause = error.cause.as_ref()?;
    cause_body_message(cause)
}

fn cause_body_message(cause: &StdError) -> Option<String> {
    if cause.message.is_empty() {
        None
    } else {
        Some(cause.message.clone())
    }
}

/// Mirrors `errorFormat` for real errors.
pub fn error_format_std(error: &StdError) -> String {
    format!("{}: {}", error.name, error.message)
}

/// Mirrors `errorFormat` for data errors / primitives.
pub fn error_format_value(input: &Value) -> String {
    match input {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.clone(),
        Value::Array(_) | Value::Object(_) => {
            let json = serde_json::to_string_pretty(input)
                .unwrap_or_else(|_| "Unexpected error (unserializable)".to_string());
            if json == "{}" {
                return "Error (no message)".to_string();
            }
            json
        }
    }
}

/// Mirrors `errorMessage` for real errors.
pub fn error_message_std(error: &StdError) -> String {
    if !error.message.is_empty() {
        return error.message.clone();
    }
    if !error.name.is_empty() {
        return error.name.clone();
    }
    "unknown error".to_string()
}

/// Mirrors `errorMessage` for data errors / primitives.
pub fn error_message_value(input: &Value) -> String {
    if let Some(map) = as_record(input) {
        if let Some(message) = map
            .get("message")
            .and_then(|m| m.as_str())
            .filter(|m| !m.is_empty())
        {
            return message.to_string();
        }
        if let Some(data) = map.get("data").and_then(as_record) {
            if let Some(message) = data
                .get("message")
                .and_then(|m| m.as_str())
                .filter(|m| !m.is_empty())
            {
                return message.to_string();
            }
        }
    }
    if let Value::String(s) = input {
        if !s.is_empty() && s != "[object Object]" {
            return s.clone();
        }
    }
    let formatted = error_format_value(input);
    if !formatted.is_empty() {
        return formatted;
    }
    "unknown error".to_string()
}

/// Mirrors `errorData` for real errors.
pub fn error_data_std(error: &StdError) -> Value {
    serde_json::json!({
        "type": error.name,
        "message": error_message_std(error),
        "stack": error.stack,
        "cause": error.cause.as_ref().map(|c| error_format_std(c)),
        "formatted": error_format_std(error),
    })
}

/// Mirrors `errorData` for data errors / primitives.
pub fn error_data_value(input: &Value) -> Value {
    if !is_record_value(input) {
        return serde_json::json!({
            "type": json_type_name(input),
            "message": error_message_value(input),
            "formatted": error_format_value(input),
        });
    }
    let map = as_record(input).cloned().unwrap_or_default();
    let mut data = serde_json::Map::new();
    for (key, value) in &map {
        if value.is_null() {
            continue;
        }
        data.insert(
            key.clone(),
            match value {
                Value::String(_) | Value::Number(_) | Value::Bool(_) => value.clone(),
                _ => Value::String(value.to_string()),
            },
        );
    }
    if !data
        .get("message")
        .and_then(|m| m.as_str())
        .map(|m| !m.is_empty())
        .unwrap_or(false)
    {
        data.insert(
            "message".to_string(),
            Value::String(error_message_value(input)),
        );
    }
    if !data
        .get("type")
        .and_then(|t| t.as_str())
        .map(|t| !t.is_empty())
        .unwrap_or(false)
    {
        data.insert("type".to_string(), Value::String("Error".to_string()));
    }
    data.insert(
        "formatted".to_string(),
        Value::String(error_format_value(input)),
    );
    Value::Object(data)
}

fn is_record_value(input: &Value) -> bool {
    matches!(input, Value::Object(_))
}

fn json_type_name(input: &Value) -> &'static str {
    match input {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}
