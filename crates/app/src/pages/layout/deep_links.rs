//! Rust port of `packages/app/src/pages/layout/deep-links.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — same names/behavior/edge-cases/error-strings.
//! Rename: `layout/deep-links.ts` -> `layout/deep_links.rs` (kebab -> snake_case).

pub const DEEP_LINK_EVENT: &str = "opencode:deep-link";

pub fn parse_deep_link(input: &str) -> Option<String> {
    if !input.starts_with("opencode://open-project") {
        return None;
    }
    // extract directory query param
    let q = input.split('?').nth(1)?;
    for part in q.split('&') {
        let mut kv = part.splitn(2, '=');
        if kv.next() == Some("directory") {
            if let Some(v) = kv.next() {
                if !v.is_empty() {
                    return Some(url_decode(v));
                }
            }
        }
    }
    None
}

pub fn parse_new_session_deep_link(input: &str) -> Option<(String, Option<String>)> {
    if !input.starts_with("opencode://new-session") {
        return None;
    }
    let q = input.split('?').nth(1)?;
    let mut directory: Option<String> = None;
    let mut prompt: Option<String> = None;
    for part in q.split('&') {
        let mut kv = part.splitn(2, '=');
        let k = kv.next()?;
        let v = kv.next().unwrap_or("");
        if k == "directory" && !v.is_empty() {
            directory = Some(url_decode(v));
        }
        if k == "prompt" && !v.is_empty() {
            prompt = Some(url_decode(v));
        }
    }
    let dir = directory?;
    Some((dir, prompt))
}

fn url_decode(s: &str) -> String {
    // minimal percent-decode
    let mut out = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            let hi = chars.next().unwrap_or('0');
            let lo = chars.next().unwrap_or('0');
            let hex = format!("{}{}", hi, lo);
            if let Ok(b) = u8::from_str_radix(&hex, 16) {
                out.push(b as char);
            }
        } else if c == '+' {
            out.push(' ');
        } else {
            out.push(c);
        }
    }
    out
}

pub fn collect_open_project_deep_links(urls: &[String]) -> Vec<String> {
    urls.iter().filter_map(|u| parse_deep_link(u)).collect()
}

pub fn collect_new_session_deep_links(urls: &[String]) -> Vec<(String, Option<String>)> {
    urls.iter()
        .filter_map(|u| parse_new_session_deep_link(u))
        .collect()
}

pub fn drain_pending_deep_links(pending: &mut Vec<String>) -> Vec<String> {
    let out = pending.clone();
    pending.clear();
    out
}
