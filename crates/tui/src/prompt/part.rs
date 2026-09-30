// source: packages/tui/src/prompt/part.ts (29 lines, v1.18.30)
// 1:1 port — prompt part ID stripping and pasted-text expansion verbatim.

#![allow(dead_code)]

use serde_json::Value;

use super::display::display_slice;

/// Mirrors `stripPromptPartIDs` — drops `id`/`messageID`/`sessionID`.
pub fn strip_prompt_part_ids(part: &Value) -> Value {
    match part {
        Value::Object(map) => {
            let mut rest = map.clone();
            rest.remove("id");
            rest.remove("messageID");
            rest.remove("sessionID");
            Value::Object(rest)
        }
        other => other.clone(),
    }
}

fn is_pasted_text_part(part: &Value) -> Option<(&str, &str)> {
    let map = part.as_object()?;
    if map.get("type")?.as_str()? != "text" {
        return None;
    }
    let text = map.get("text")?.as_str()?;
    let value = map.get("source")?.get("text")?.get("value")?.as_str()?;
    Some((value, text))
}

/// Mirrors `expandPastedTextPlaceholders`.
pub fn expand_pasted_text_placeholders(text: &str, parts: &[Value]) -> String {
    let mut result = text.to_string();
    for part in parts {
        if let Some((placeholder, replacement)) = is_pasted_text_part(part) {
            result = result.replacen(placeholder, replacement, 1);
        }
    }
    result
}

/// Mirrors `expandTrackedPastedText` — ranges applied back-to-front.
pub fn expand_tracked_pasted_text(text: &str, ranges: &[(usize, usize, String)]) -> String {
    let mut ordered: Vec<(usize, usize, &String)> = ranges
        .iter()
        .map(|(start, end, text)| (*start, *end, text))
        .collect();
    ordered.sort_by_key(|(start, _, _)| std::cmp::Reverse(*start));
    let mut result = text.to_string();
    for (start, end, replacement) in ordered {
        result = format!(
            "{}{}{}",
            display_slice(&result, 0, start),
            replacement,
            display_slice(&result, end, usize::MAX)
        );
    }
    result
}
