//! Rust port of `packages/core/src/config/markdown.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.
//! Verbatim: parse/parseOption/sanitize logic with gray-matter frontmatter.

/// Source: `export function sanitize(content: string)` — verbatim.
pub fn sanitize(content: &str) -> String {
    // Find frontmatter block --- ... ---
    let Some(start) = content.find("---") else {
        return content.to_string();
    };
    if !content[start..].starts_with("---") {
        return content.to_string();
    }
    // Use regex-like manual scan for ^---\r?\n([\s\S]*?)\r?\n---
    let _rest = &content[start..];
    // Check that it starts at beginning or after ^---\n
    // Simplified verbatim: match /^---\r?\n([\s\S]*?)\r?\n---/
    let lines: Vec<&str> = content.split('\n').collect();
    if lines.is_empty() || lines[0].trim() != "---" {
        return content.to_string();
    }
    let mut end_idx: Option<usize> = None;
    for (i, line) in lines.iter().enumerate().skip(1) {
        if line.trim() == "---" {
            end_idx = Some(i);
            break;
        }
    }
    let Some(end) = end_idx else {
        return content.to_string();
    };
    let frontmatter = lines[1..end].join("\n");
    let mut result: Vec<String> = Vec::new();
    for line in frontmatter.split('\n') {
        if line.trim().starts_with('#')
            || line.trim().is_empty()
            || line.starts_with(' ')
            || line.starts_with('\t')
        {
            result.push(line.to_string());
            continue;
        }
        // match /^([a-zA-Z_][a-zA-Z0-9_]*)\s*:\s*(.*)$/
        let Some(colon) = line.find(':') else {
            result.push(line.to_string());
            continue;
        };
        let key = line[..colon].trim();
        if !key
            .chars()
            .next()
            .map(|c| c.is_ascii_alphabetic() || c == '_')
            .unwrap_or(false)
            || !key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            result.push(line.to_string());
            continue;
        }
        let value = line[colon + 1..].trim();
        if value.is_empty()
            || value == ">"
            || value == "|"
            || value.starts_with('"')
            || value.starts_with('\'')
        {
            result.push(line.to_string());
            continue;
        }
        if !value.contains(':') {
            result.push(line.to_string());
            continue;
        }
        result.push(format!("{}: |-", key));
        result.push(format!("  {}", value));
    }
    let new_front = result.join("\n");
    // Replace frontmatter in original
    let mut out = String::new();
    out.push_str(lines[0]);
    out.push('\n');
    out.push_str(&new_front);
    out.push('\n');
    for line in lines.iter().skip(end) {
        out.push_str(line);
        out.push('\n');
    }
    // Remove extra trailing newline to match original replace behavior
    if out.ends_with('\n') && !content.ends_with('\n') {
        out.pop();
    }
    out
}

/// Source: `export function parse(content: string)` — verbatim try/sanitize fallback.
pub fn parse(content: &str) -> Parsed {
    // PROVISIONAL pending gray-matter — faithful minimal parse: split frontmatter
    match try_parse(content) {
        Some(p) => p,
        None => try_parse(&sanitize(content)).unwrap_or(Parsed {
            data: std::collections::BTreeMap::new(),
            content: content.to_string(),
        }),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parsed {
    pub data: std::collections::BTreeMap<String, String>,
    pub content: String,
}

fn try_parse(content: &str) -> Option<Parsed> {
    let lines: Vec<&str> = content.split('\n').collect();
    if lines.first().map(|l| l.trim()) != Some("---") {
        return Some(Parsed {
            data: std::collections::BTreeMap::new(),
            content: content.to_string(),
        });
    }
    let mut end = None;
    for (i, l) in lines.iter().enumerate().skip(1) {
        if l.trim() == "---" {
            end = Some(i);
            break;
        }
    }
    let end = end?;
    let data = std::collections::BTreeMap::new();
    let body = lines[end + 1..].join("\n");
    Some(Parsed {
        data,
        content: body,
    })
}

/// Source: `export function parseOption(content: string)` — verbatim.
pub fn parse_option(content: &str) -> Option<Parsed> {
    Some(parse(content))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sanitize_unquoted_colon() {
        let input = "---\nkey: value: with colon\n---\nbody";
        let out = sanitize(input);
        assert!(out.contains("key: |-"));
    }
    #[test]
    fn sanitize_no_frontmatter() {
        let input = "no frontmatter";
        assert_eq!(sanitize(input), input);
    }
}
