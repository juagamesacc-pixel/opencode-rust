// source: src/config/variable.ts — exports: substitute, ConfigVariable
// PROVISIONAL pending core v1/config/error InvalidError: {env:VAR} (env-map
// then process.env, missing → ""), {file:path} (//-comment skip, ~ expand,
// resolve vs configDir, missing=error|empty, ENOENT message suffix
// " does not exist", JSON-escaped splice) verbatim. Reader injected (fs on CI).

/// source: missing mode default "error" — verbatim.
pub const MISSING_DEFAULT: &str = "error";

/// source: `bad file reference: "${token}"` — verbatim.
pub fn bad_file_message(token: &str) -> String {
    format!("bad file reference: \"{}\"", token)
}

/// source: `... ${resolvedPath} does not exist` (ENOENT suffix) — verbatim.
pub fn enoent_suffix(resolved: &str) -> String {
    format!(" {} does not exist", resolved)
}

/// source:	env substitution {env:VAR} — map then process env, else "". Verbatim.
pub fn substitute_env(text: &str, env: &std::collections::HashMap<String, String>) -> String {
    let mut out = String::new();
    let mut rest = text;
    while let Some(start) = rest.find("{env:") {
        out.push_str(&rest[..start]);
        let after = &rest[start + 5..];
        match after.find('}') {
            Some(end) => {
                let name = &after[..end];
                let value = env
                    .get(name)
                    .cloned()
                    .or_else(|| std::env::var(name).ok())
                    .unwrap_or_default();
                out.push_str(&value);
                rest = &after[end + 1..];
            }
            None => {
                out.push_str(&rest[start..]);
                rest = "";
            }
        }
    }
    out.push_str(rest);
    out
}

/// source: single {file:…} token resolution — verbatim steps.
pub fn resolve_file_token(
    token: &str,
    config_dir: &str,
    home: &str,
    missing_empty: bool,
    read: &dyn Fn(&str) -> Result<String, Option<String>>,
) -> Result<String, String> {
    let mut path = token
        .strip_prefix("{file:")
        .unwrap_or(token)
        .trim_end_matches('}')
        .to_string();
    if path.starts_with("~/") {
        path = format!("{}/{}", home.trim_end_matches('/'), &path[2..]);
    }
    let resolved = if path.starts_with('/') {
        path
    } else {
        format!("{}/{}", config_dir.trim_end_matches('/'), path)
    };
    match read(&resolved) {
        Ok(content) => Ok(json_escape_inner(content.trim())),
        Err(code) => {
            if missing_empty {
                return Ok(String::new());
            }
            let mut msg = bad_file_message(token);
            if code.as_deref() == Some("ENOENT") {
                msg.push_str(&enoent_suffix(&resolved));
            }
            Err(msg)
        }
    }
}

/// source: JSON.stringify(content).slice(1, -1) — verbatim escape.
pub fn json_escape_inner(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

/// source: substitute() — env pass, then token splice with //-comment guard. Verbatim.
pub fn substitute(
    text: &str,
    config_dir: &str,
    config_source: &str,
    missing_empty: bool,
    env: &std::collections::HashMap<String, String>,
    home: &str,
    read: &dyn Fn(&str) -> Result<String, Option<String>>,
) -> Result<String, String> {
    let text = substitute_env(text, env);
    let tokens = find_file_tokens(&text);
    if tokens.is_empty() {
        return Ok(text);
    }
    let mut out = String::new();
    let mut cursor = 0;
    for (index, len) in tokens {
        out.push_str(&text[cursor..index]);
        let token = &text[index..index + len];
        let line_start = text[..index].rfind('\n').map(|i| i + 1).unwrap_or(0);
        if text[line_start..index].trim_start().starts_with("//") {
            out.push_str(token);
            cursor = index + len;
            continue;
        }
        match resolve_file_token(token, config_dir, home, missing_empty, read) {
            Ok(s) => out.push_str(&s),
            Err(msg) => return Err(format!("{}: {}", config_source, msg)),
        }
        cursor = index + len;
    }
    out.push_str(&text[cursor..]);
    Ok(out)
}

fn find_file_tokens(text: &str) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < text.len() {
        match text[i..].find("{file:") {
            Some(off) => {
                let start = i + off;
                match text[start..].find('}') {
                    Some(end) => {
                        out.push((start, end + 1));
                        i = start + end + 1;
                    }
                    None => break,
                }
            }
            None => break,
        }
    }
    out
}
