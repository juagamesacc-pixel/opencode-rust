// source: packages/tui/src/context/thinking.ts (67 lines, v1.18.30)
// 1:1 port — SolidJS signals become an explicit store over a minimal KV
// seam (`KvAccess`, implemented by the real kv module when it lands).

#![allow(dead_code)]

use serde_json::Value;

/// Mirrors `ThinkingMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThinkingMode {
    Show,
    Hide,
}

impl ThinkingMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ThinkingMode::Show => "show",
            ThinkingMode::Hide => "hide",
        }
    }
}

/// Cycle order verbatim: show → hide → show.
pub const MODES: [ThinkingMode; 2] = [ThinkingMode::Show, ThinkingMode::Hide];

/// Minimal KV surface the thinking store needs (mirrors `kv.get/set`).
pub trait KvAccess {
    fn kv_get(&self, key: &str) -> Option<Value>;
    fn kv_set(&mut self, key: &str, value: Value);
}

/// Mirrors `reasoningSummary` — splits an OpenAI bolded title block
/// (`**Title**\n\n<body>` or a bare trailing `**Title**`) from the body.
/// Implemented without regex (no new crates): title may not contain `*`
/// or a newline; the closer must sit at end-of-input or before a blank line.
pub fn reasoning_summary(text: &str) -> (Option<String>, String) {
    let content = text.trim();
    let Some(rest) = content.strip_prefix("**") else {
        return (None, content.to_string());
    };
    let bytes = rest.as_bytes();
    let mut end: Option<usize> = None;
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'\n' {
            break;
        }
        if b == b'*' {
            if bytes.get(i + 1) == Some(&b'*') {
                end = Some(i);
            }
            break;
        }
        i += 1;
    }
    let close = match end {
        Some(close) => close,
        None => return (None, content.to_string()),
    };
    let title = rest[..close].trim();
    if title.is_empty() {
        return (None, content.to_string());
    }
    let after = &rest[close + 2..];
    let body = if after.is_empty() {
        String::new()
    } else if let Some(stripped) = after
        .strip_prefix("\r\n\r\n")
        .or_else(|| after.strip_prefix("\n\n"))
    {
        stripped.trim_end().to_string()
    } else {
        return (None, content.to_string());
    };
    (Some(title.to_string()), body)
}

/// Mirrors `isThinkingMode`.
pub fn is_thinking_mode(value: &Value) -> bool {
    matches!(value.as_str(), Some("show") | Some("hide"))
}

/// Mirrors `nextThinkingMode`.
pub fn next_thinking_mode(current: ThinkingMode) -> ThinkingMode {
    let idx = MODES.iter().position(|m| *m == current).unwrap_or(0);
    MODES[(idx + 1) % MODES.len()]
}

/// Mirrors `useThinkingMode` — runs the same init/migration sequence
/// against the provided KV (legacy `thinking_visibility` boolean migrated
/// on first run; stray `"minimal"` values collapse to hide).
pub struct ThinkingModeStore {
    mode: ThinkingMode,
}

impl ThinkingModeStore {
    pub fn init(kv: &mut dyn KvAccess) -> Self {
        let had_stored = kv.kv_get("thinking_mode").is_some();
        let legacy = kv.kv_get("thinking_visibility").and_then(|v| v.as_bool());
        let mut stored = kv
            .kv_get("thinking_mode")
            .and_then(|v| v.as_str().map(str::to_string))
            .unwrap_or_else(|| "hide".to_string());
        if !had_stored {
            if legacy == Some(true) {
                stored = "show".to_string();
            } else if legacy == Some(false) {
                stored = "hide".to_string();
            }
            kv.kv_set("thinking_mode", Value::String(stored.clone()));
        }
        if stored == "minimal" {
            stored = "hide".to_string();
            kv.kv_set("thinking_mode", Value::String(stored.clone()));
        }
        let mode = match stored.as_str() {
            "show" => ThinkingMode::Show,
            _ => ThinkingMode::Hide,
        };
        Self { mode }
    }

    pub fn mode(&self) -> ThinkingMode {
        self.mode
    }

    pub fn set(&mut self, kv: &mut dyn KvAccess, next: ThinkingMode) {
        self.mode = next;
        kv.kv_set("thinking_mode", Value::String(next.as_str().to_string()));
    }

    pub fn update(
        &mut self,
        kv: &mut dyn KvAccess,
        next: impl FnOnce(ThinkingMode) -> ThinkingMode,
    ) {
        let mode = next(self.mode);
        self.set(kv, mode);
    }
}
