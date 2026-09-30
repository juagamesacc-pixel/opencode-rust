// source: src/util/wildcard.ts — exports: Wildcard (match), match
//
// TS translation (verbatim):
//   normalized = input.replaceAll("\\", "/")
//   escaped = pattern.replaceAll("\\","/")
//     .replace(/[.+^${}()|[\]\\]/g, "\\$&")   // regex-escape the set
//     .replace(/\*/g, ".*").replace(/\?/g, ".")
//   if escaped endsWith " .*": escaped = escaped.slice(0,-3) + "( .*)?"
//   return new RegExp("^" + escaped + "$", win32 ? "si" : "s").test(normalized)
//
// No regex crate is available (std-only build); the escaped string is parsed
// into literal / any-char / any-run tokens plus the terminal optional group
// `( .*)?` (the only group the translation ever produces — its `(`, `)`, `?`
// are unescaped, whereas pattern `(`,`)` are backslash-escaped), then matched
// recursively with dotAll (`s`) and win32 case-insensitivity (`i`) exactly as
// the RegExp would. PORTING-equivalent implementation, flagged PROVISIONAL
// pending a regex dependency.

/// Mirrors `process.platform === "win32"`.
pub const IS_WIN32: bool = cfg!(windows);

// RENAME record (Rust keyword): TS export `match` → `match_pattern` here.
// Behavior identical; `Wildcard.match` from TS maps to `Wildcard::match_pattern`.

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Literal(char),
    Any,                 // `.` — any single char incl. newline (dotAll)
    AnyStar,             // `.*` — any run incl. empty (dotAll)
    OptionalSpaceAnyRun, // `( .*)?` — " " + any-run, or nothing (terminal suffix)
}

/// Escapes a single regex metachar from the source set `[.+^${}()|[\]\\]`.
fn is_escaped_char(c: char) -> bool {
    matches!(
        c,
        '.' | '+' | '^' | '$' | '{' | '}' | '(' | ')' | '|' | '[' | ']' | '\\'
    )
}

/// Builds the escaped pattern string exactly as the TS does.
fn escaped_pattern(pattern: &str) -> String {
    let mut escaped = String::new();
    for c in pattern.replace('\\', "/").chars() {
        match c {
            '*' => escaped.push_str(".*"),
            '?' => escaped.push('.'),
            c if is_escaped_char(c) => {
                escaped.push('\\');
                escaped.push(c);
            }
            c => escaped.push(c),
        }
    }
    if escaped.ends_with(" .*") {
        let trimmed = escaped.trim_end_matches(" .*");
        escaped = format!("{trimmed}( .*)?");
    }
    escaped
}

/// Parses the post-replace `escaped` string into match tokens. A terminal
/// unescaped `( .*)?` (only ever produced by the suffix rewrite) becomes
/// `OptionalSpaceAnyRun`; everything else is literal/`.`/`.*`.
fn tokenize(escaped: &str) -> Vec<Token> {
    let chars: Vec<char> = escaped.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        // Terminal optional group: "( .*)?"
        if chars[i] == '('
            && i + 5 < chars.len()
            && chars[i + 1] == ' '
            && chars[i + 2] == '.'
            && chars[i + 3] == '*'
            && chars[i + 4] == ')'
            && chars[i + 5] == '?'
        {
            tokens.push(Token::OptionalSpaceAnyRun);
            i += 6;
            continue;
        }
        match chars[i] {
            '\\' => {
                if i + 1 < chars.len() {
                    tokens.push(Token::Literal(chars[i + 1]));
                    i += 2;
                } else {
                    tokens.push(Token::Literal('\\'));
                    i += 1;
                }
            }
            '.' => {
                if i + 1 < chars.len() && chars[i + 1] == '*' {
                    tokens.push(Token::AnyStar);
                    i += 2;
                } else {
                    tokens.push(Token::Any);
                    i += 1;
                }
            }
            c => {
                tokens.push(Token::Literal(c));
                i += 1;
            }
        }
    }
    tokens
}

fn char_eq_case(a: char, b: char, ci: bool) -> bool {
    if ci {
        a.to_ascii_lowercase() == b.to_ascii_lowercase()
    } else {
        a == b
    }
}

/// Backtracking full-match over tokens (dotAll: `.`/`.*` cross newlines).
fn match_tokens(input: &[char], tokens: &[Token], ci: bool) -> bool {
    // `.*` followed by `tokens` — greedy-backtracking any run (dotAll).
    fn rec_star(input: &[char], tokens: &[Token], ci: bool) -> bool {
        if rec(input, tokens, ci) {
            return true;
        }
        !input.is_empty() && rec_star(&input[1..], tokens, ci)
    }

    fn rec(input: &[char], tokens: &[Token], ci: bool) -> bool {
        if tokens.is_empty() {
            return input.is_empty();
        }
        match &tokens[0] {
            Token::Literal(c) => {
                !input.is_empty()
                    && char_eq_case(input[0], *c, ci)
                    && rec(&input[1..], &tokens[1..], ci)
            }
            Token::Any => !input.is_empty() && rec(&input[1..], &tokens[1..], ci),
            Token::AnyStar => rec_star(input, &tokens[1..], ci),
            Token::OptionalSpaceAnyRun => {
                // Absent, or " " then `.*` then the rest.
                if rec(input, &tokens[1..], ci) {
                    return true;
                }
                !input.is_empty() && input[0] == ' ' && rec_star(&input[1..], &tokens[1..], ci)
            }
        }
    }

    rec(input, tokens, ci)
}

/// source: `Wildcard.match(input, pattern)` (TS name `match`; Rust keyword).
pub fn match_pattern(input: &str, pattern: &str) -> bool {
    let normalized: String = input.replace('\\', "/");
    let escaped = escaped_pattern(pattern);
    let tokens = tokenize(&escaped);
    let chars: Vec<char> = normalized.chars().collect();
    match_tokens(&chars, &tokens, IS_WIN32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert!(match_pattern("hello world", "hello world"));
        assert!(match_pattern("hello world", "hello *"));
        assert!(match_pattern("hello", "hello *"));
        assert!(match_pattern("foo/bar/baz", "foo/*/baz"));
        assert!(match_pattern("abc", "a?c"));
        assert!(!match_pattern("abc", "a?d"));
        assert!(!match_pattern("hello", "world"));
    }

    #[test]
    fn literal_regex_chars() {
        // Pattern chars in the escape set must be literal.
        assert!(match_pattern("a+b", "a+b"));
        assert!(!match_pattern("aab", "a+b"));
        assert!(match_pattern("x.y", "x.y"));
        assert!(match_pattern("(x)", "(x)"));
    }

    #[test]
    fn trailing_star_space_group() {
        // Pattern ending in " *" becomes the optional "( .*)?" group.
        assert!(match_pattern("file", "file *"));
        assert!(match_pattern("file x", "file *"));
        assert!(match_pattern("file ", "file *"));
        assert!(!match_pattern("files", "file *"));
    }

    #[test]
    fn windows_separator_normalized() {
        assert!(match_pattern("a\\b\\c", "a/b/c"));
    }
}
