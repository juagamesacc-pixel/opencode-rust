// source: src/config/markdown.ts — exports: FILE_REGEX, SHELL_REGEX,
// files, shell, fallbackSanitization, parse, ConfigMarkdown
// PROVISIONAL pending core config/markdown + v1/config/error: regexes +
// frontmatter-failure message verbatim.

/// source: FILE_REGEX /(?<![\w`])@(\.?[^\s`,.]*(?:\.[^\s`,.]+)*)/g — verbatim.
/// Returns byte ranges of full matches; group-1 ranges alongside.
pub fn file_matches(template: &str) -> Vec<(usize, usize)> {
    // Manual scan mirroring the regex: @ not preceded by [\w`], then
    // \.?[^\s`,.]*(?:\.[^\s`,.]+)* — verbatim semantics.
    let bytes = template.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'@' {
            let prev_ok = if i == 0 {
                true
            } else {
                let p = bytes[i - 1];
                !(p.is_ascii_alphanumeric() || p == b'_' || p == b'`')
            };
            if prev_ok {
                let mut j = i + 1;
                if j < bytes.len() && bytes[j] == b'.' {
                    j += 1;
                }
                let mut k = j;
                while k < bytes.len()
                    && !bytes[k].is_ascii_whitespace()
                    && bytes[k] != b'`'
                    && bytes[k] != b','
                    && bytes[k] != b'.'
                {
                    k += 1;
                }
                // (?:\.[^\s`,.]+)* tail
                loop {
                    if k < bytes.len() && bytes[k] == b'.' {
                        let mut m = k + 1;
                        while m < bytes.len()
                            && !bytes[m].is_ascii_whitespace()
                            && bytes[m] != b'`'
                            && bytes[m] != b','
                            && bytes[m] != b'.'
                        {
                            m += 1;
                        }
                        if m == k + 1 {
                            break;
                        }
                        k = m;
                    } else {
                        break;
                    }
                }
                out.push((i, k));
                i = k.max(i + 1);
                continue;
            }
        }
        i += 1;
    }
    out
}

/// source: SHELL_REGEX /!`([^`]+)`/g — verbatim.
pub fn shell_matches(template: &str) -> Vec<(usize, usize, String)> {
    let bytes = template.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i + 1 < bytes.len() {
        if bytes[i] == b'!' && bytes[i + 1] == b'`' {
            if let Some(end) = template[i + 2..].find('`') {
                let inner = template[i + 2..i + 2 + end].to_string();
                if !inner.is_empty() {
                    out.push((i, i + 2 + end + 1, inner));
                }
                i += 2 + end + 1;
                continue;
            }
        }
        i += 1;
    }
    out
}

/// source: files() — verbatim.
pub fn files(template: &str) -> Vec<(usize, usize)> {
    file_matches(template)
}

/// source: shell() — verbatim.
pub fn shell(template: &str) -> Vec<(usize, usize, String)> {
    shell_matches(template)
}

/// source: `${filePath}: Failed to parse YAML frontmatter: ${msg}` — verbatim.
pub fn frontmatter_message(file_path: &str, err: &str) -> String {
    format!("{}: Failed to parse YAML frontmatter: {}", file_path, err)
}
