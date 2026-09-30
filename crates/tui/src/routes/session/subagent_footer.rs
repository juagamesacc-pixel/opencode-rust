// source: packages/tui/src/routes/session/subagent-footer.tsx (132 lines, v1.18.30)
// 1:1 port — `@agent subagent` label parsing, sibling index/total, token
// usage line, and the Parent/Prev/Next command dispatches.

#![allow(dead_code)]

use serde_json::Value;

use crate::component::prompt::format_money;
use crate::util::locale::{number, titlecase};

/// Parse the subagent label from the session title (`@name subagent`).
pub fn subagent_label(title: &str) -> String {
    match title
        .split('@')
        .nth(1)
        .and_then(|rest| rest.split_whitespace().next())
    {
        Some(name) if title.contains(" subagent") => titlecase(name),
        _ => "Subagent".to_string(),
    }
}

/// Sibling position among same-parent sessions (created order).
pub fn sibling_position(
    sessions: &[Value],
    session_id: &str,
    parent_id: Option<&str>,
) -> (usize, usize) {
    let Some(parent_id) = parent_id else {
        return (0, 0);
    };
    let mut siblings: Vec<&Value> = sessions
        .iter()
        .filter(|s| s.get("parentID").and_then(|v| v.as_str()) == Some(parent_id))
        .collect();
    siblings.sort_by_key(|s| {
        s.get("time")
            .and_then(|t| t.get("created"))
            .and_then(|v| v.as_i64())
            .unwrap_or(0)
    });
    let total = siblings.len();
    let index = siblings
        .iter()
        .position(|s| s.get("id").and_then(|v| v.as_str()) == Some(session_id))
        .map(|index| index + 1)
        .unwrap_or(0);
    (index, total)
}

/// Token usage line (mirrors the `usage` memo + prompt usage builder).
pub fn subagent_usage(
    messages: &[Value],
    sessions: &[Value],
    providers: &[Value],
    session_id: &str,
) -> Option<(String, Option<String>)> {
    let last = messages.iter().rev().find(|item| {
        item.get("role").and_then(|v| v.as_str()) == Some("assistant")
            && item
                .get("tokens")
                .and_then(|t| t.get("output"))
                .and_then(|v| v.as_i64())
                .unwrap_or(0)
                > 0
    })?;
    let tokens = ["input", "output", "reasoning"]
        .into_iter()
        .map(|key| {
            last.get("tokens")
                .and_then(|t| t.get(key))
                .and_then(|v| v.as_i64())
                .unwrap_or(0)
        })
        .sum::<i64>()
        + last
            .get("tokens")
            .and_then(|t| t.get("cache"))
            .and_then(|c| c.get("read"))
            .and_then(|v| v.as_i64())
            .unwrap_or(0)
        + last
            .get("tokens")
            .and_then(|t| t.get("cache"))
            .and_then(|c| c.get("write"))
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
    if tokens <= 0 {
        return None;
    }
    let provider_id = last
        .get("providerID")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let model_id = last.get("modelID").and_then(|v| v.as_str()).unwrap_or("");
    let context_pct = providers
        .iter()
        .find(|p| p.get("id").and_then(|v| v.as_str()) == Some(provider_id))
        .and_then(|p| p.get("models"))
        .and_then(|m| m.get(model_id))
        .and_then(|m| m.get("limit"))
        .and_then(|l| l.get("context"))
        .and_then(|v| v.as_i64())
        .map(|limit| format!("{}%", (tokens as f64 / limit as f64 * 100.0).round() as i64));
    let context = match context_pct {
        Some(pct) => format!("{} ({pct})", number(tokens as f64)),
        None => number(tokens as f64),
    };
    let cost = sessions
        .iter()
        .find(|s| s.get("id").and_then(|v| v.as_str()) == Some(session_id))
        .and_then(|s| s.get("cost"))
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    Some((
        context,
        if cost > 0.0 {
            Some(format_money(cost))
        } else {
            None
        },
    ))
}

/// Footer nav commands verbatim.
pub const SUBAGENT_COMMAND_PARENT: &str = "session.parent";
pub const SUBAGENT_COMMAND_PREV: &str = "session.child.previous";
pub const SUBAGENT_COMMAND_NEXT: &str = "session.child.next";
