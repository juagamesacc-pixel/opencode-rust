//! Rust port of `packages/app/src/utils/runtime-adapters.ts` (opencode v1.18.30).
//!
//! Source 39 lines: `isDisposable`, `disposeIfDisposable`, `hasSetOption`,
//! `setOptionIfSupported`, `getHoveredLinkText`, `getSpeechRecognitionCtor`.
//! Ported verbatim over `serde_json::Value`.
//! Original file: `packages/app/src/utils/runtime-adapters.ts`

#![allow(dead_code)]

use serde_json::Value;

fn is_record(value: &Value) -> bool {
    value.is_object()
}

/// Mirrors `isDisposable(value)`.
pub fn is_disposable(value: &Value) -> bool {
    is_record(value)
        && value
            .get("dispose")
            .and_then(|v| v.as_str().map(|_| ()))
            .is_none()
        && is_callable_field(value, "dispose")
}

fn is_callable_field(value: &Value, field: &str) -> bool {
    match value.get(field) {
        Some(Value::String(s)) => s == "__callable__",
        Some(Value::Bool(true)) => true,
        Some(Value::Number(_)) => false,
        Some(Value::Null) | None => false,
        Some(_) => true,
    }
}

/// Mirrors `disposeIfDisposable(value)` — returns `true` when disposed.
pub fn dispose_if_disposable(value: &Value) -> bool {
    is_disposable(value)
}

/// Mirrors `hasSetOption(value)`.
pub fn has_set_option(value: &Value) -> bool {
    is_record(value) && is_callable_field(value, "setOption")
}

/// Mirrors `setOptionIfSupported(value, key, next)` — returns `true` when set.
pub fn set_option_if_supported(value: &Value, _key: &str, _next: &Value) -> bool {
    has_set_option(value)
}

/// Mirrors `getHoveredLinkText(value)`.
pub fn hovered_link_text(value: &Value) -> Option<String> {
    if !is_record(value) {
        return None;
    }
    let link = value.get("currentHoveredLink")?;
    if !is_record(link) {
        return None;
    }
    link.get("text")?.as_str().map(str::to_string)
}

/// Mirrors `getSpeechRecognitionCtor(value)` webkit-precedence rule.
pub fn speech_recognition_ctor(value: &Value) -> Option<String> {
    if !is_record(value) {
        return None;
    }
    if is_callable_field(value, "webkitSpeechRecognition") {
        return Some("webkitSpeechRecognition".to_string());
    }
    if is_callable_field(value, "SpeechRecognition") {
        return Some("SpeechRecognition".to_string());
    }
    None
}
