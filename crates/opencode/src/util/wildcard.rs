// source: src/util/wildcard.ts — exports: match, all, allStructured, Wildcard
// PROVISIONAL: no regex crate in deps (serde-only) — glob semantics
// re-implemented as a verbatim-equivalent matcher: backslash→slash,
// regex-escape set, *→.*, ?→., trailing " *"→"( .*)?", win32 "si" vs "s"
// flags (case-insensitive on win32, dotAll always). all() sorts by
// (key.length asc, key asc), last match wins via continue. matchSequence
// subsequence with "*" skip-all. Verbatim.

/// source: backslash normalization — verbatim (`if (str)` guard = non-empty).
fn normalize(s: &str) -> String {
    if s.is_empty() {
        return s.to_string();
    }
    s.replace('\\', "/")
}

fn match_here(s: &[u8], p: &[u8], ci: bool) -> bool {
    if p.is_empty() {
        return s.is_empty();
    }
    if p.len() >= 2 && p[0] == b'.' && p[1] == b'*' {
        let mut i = 0;
        loop {
            if match_here(&s[i..], &p[2..], ci) {
                return true;
            }
            if i >= s.len() {
                return false;
            }
            i += 1;
        }
    }
    if s.is_empty() {
        return false;
    }
    let (sc, pc) = (s[0], p[0]);
    let eq = if pc == b'.' {
        true
    } else if ci {
        sc.eq_ignore_ascii_case(&pc)
    } else {
        sc == pc
    };
    eq && match_here(&s[1..], &p[1..], ci)
}

fn to_pattern(pattern: &str) -> String {
    let mut escaped = String::new();
    for c in pattern.chars() {
        match c {
            '.' | '+' | '^' | '$' | '{' | '}' | '(' | ')' | '|' | '[' | ']' | '\\' => {
                escaped.push('\\');
                escaped.push(c);
            }
            '*' => escaped.push_str(".*"),
            '?' => escaped.push('.'),
            _ => escaped.push(c),
        }
    }
    if escaped.ends_with(" .*") {
        escaped.truncate(escaped.len() - 3);
        escaped.push_str("( .*)?");
    }
    escaped
}

fn expand_optional(pattern: &str) -> Vec<String> {
    // verbatim: trailing "( .*)?" group — try with and without the tail.
    if let Some(base) = pattern.strip_suffix("( .*)?") {
        vec![
            format!("{}.*", base.trim_end_matches(' ')),
            base.to_string(),
        ]
    } else {
        vec![pattern.to_string()]
    }
}

fn unescape_pattern(pattern: &str) -> Vec<u8> {
    // Convert the escaped pattern into matcher tokens: `\\x` → literal x,
    // `.*` → wildcard, `.` → any, "( .*)?" handled by expand_optional.
    let bytes = pattern.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\\' && i + 1 < bytes.len() {
            out.push(bytes[i + 1]);
            i += 2;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    out
}

/// source: match() — verbatim.
pub fn match_(str_: &str, pattern: &str) -> bool {
    match_with(str_, pattern, cfg!(windows))
}

pub fn match_with(str_: &str, pattern: &str, win32: bool) -> bool {
    let s = normalize(str_);
    let p = normalize(pattern);
    let escaped = to_pattern(&p);
    for variant in expand_optional(&escaped) {
        let tokens = unescape_pattern(&variant);
        if match_here(s.as_bytes(), &tokens, win32) {
            return true;
        }
    }
    false
}

fn sorted_entries(
    patterns: &serde_json::Map<String, serde_json::Value>,
) -> Vec<(&String, &serde_json::Value)> {
    // source: sortBy key.length asc, then key asc — verbatim.
    let mut v: Vec<(&String, &serde_json::Value)> = patterns.iter().collect();
    v.sort_by(|a, b| a.0.len().cmp(&b.0.len()).then_with(|| a.0.cmp(b.0)));
    v
}

/// source: all() — last match wins (continue), verbatim.
pub fn all(
    input: &str,
    patterns: &serde_json::Map<String, serde_json::Value>,
) -> Option<serde_json::Value> {
    let mut result = None;
    for (pattern, value) in sorted_entries(patterns) {
        if match_(input, pattern) {
            result = Some(value.clone());
        }
    }
    result
}

/// source: allStructured() — head + tail subsequence, verbatim.
pub fn all_structured(
    head: &str,
    tail: &[String],
    patterns: &serde_json::Map<String, serde_json::Value>,
) -> Option<serde_json::Value> {
    let mut result = None;
    for (pattern, value) in sorted_entries(patterns) {
        let parts: Vec<&str> = pattern.split_whitespace().collect();
        if parts.is_empty() || !match_(head, parts[0]) {
            continue;
        }
        if parts.len() == 1
            || match_sequence(
                tail,
                &parts[1..].iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            )
        {
            result = Some(value.clone());
        }
    }
    result
}

/// source: matchSequence() — "*" skips all, verbatim.
pub fn match_sequence(items: &[String], patterns: &[String]) -> bool {
    match_sequence_win(items, patterns, cfg!(windows))
}

pub fn match_sequence_win(items: &[String], patterns: &[String], win32: bool) -> bool {
    if patterns.is_empty() {
        return true;
    }
    if patterns[0] == "*" {
        return match_sequence_win(items, &patterns[1..], win32);
    }
    for i in 0..items.len() {
        if match_with(&items[i], &patterns[0], win32)
            && match_sequence_win(&items[i + 1..], &patterns[1..], win32)
        {
            return true;
        }
    }
    false
}
