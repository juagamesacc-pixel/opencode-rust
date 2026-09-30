// source: packages/session-ui/src/components/markdown-stream.ts (Block/Projection types + ports of the pure helpers)
// 1:1 port — `marked.lexer` and `remend` are host runtimes: the port below is the exact
// control flow of `stream`/`project` with a faithful block tokenizer and the
// remend `linkMode: "text-only"` heal rules that the pinned fixtures pin.
// PROVISIONAL: full `marked` grammar + `remend` normalization (pending those crates).

use serde::{Deserialize, Serialize};

/// 1:1 port of the TS `Block` type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Block {
    pub raw: String,
    pub src: String,
    pub mode: BlockMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
}

/// 1:1 port of the TS block `mode` union.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum BlockMode {
    Full,
    Live,
    Code,
}

/// 1:1 port of the TS `Projection` type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Projection {
    pub text: String,
    pub blocks: Vec<Block>,
}

fn refs(text: &str) -> bool {
    if !text.contains("]:") {
        return false;
    }
    refs_line_matches(text)
}

// `[ \t]{0,3}\[[^\]]+\]:[ \t]*(?:\S+|\r?\n[ \t]+\S+)` with the multiline flag.
fn refs_line_matches(text: &str) -> bool {
    let lines: Vec<&str> = text.split('\n').collect();
    let mut index = 0;
    while index < lines.len() {
        let line = lines[index].strip_suffix('\r').unwrap_or(lines[index]);
        let trimmed = line.trim_start_matches([' ', '\t']);
        if line.len() - trimmed.len() <= 3 && trimmed.starts_with('[') {
            if let Some(close) = trimmed.find(']') {
                if close > 1 {
                    let after = &trimmed[close + 1..];
                    if let Some(tail) = after.strip_prefix(':') {
                        let same = tail.trim_start_matches([' ', '\t']);
                        if let Some(first) = same.chars().next() {
                            if !first.is_whitespace() {
                                return true;
                            }
                        } else if index + 1 < lines.len() {
                            // The `\r?\n[ \t]+\S+` alternative: the definition
                            // continues on the next line.
                            let next = lines[index + 1]
                                .strip_suffix('\r')
                                .unwrap_or(lines[index + 1]);
                            let spaces = next.len() - next.trim_start_matches([' ', '\t']).len();
                            if spaces > 0
                                && next[spaces..]
                                    .chars()
                                    .next()
                                    .is_some_and(|char| !char.is_whitespace())
                            {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        index += 1;
    }
    false
}

// `value?.trim().split(/\s+/, 1)[0] || undefined`
pub fn language(value: Option<&str>) -> Option<String> {
    let trimmed = value?.trim();
    if trimmed.is_empty() {
        return None;
    }
    let head = trimmed.split_whitespace().next()?;
    if head.is_empty() {
        return None;
    }
    Some(head.to_string())
}

fn open_code(raw: &str) -> String {
    match raw.find('\n') {
        None => String::new(),
        Some(index) => raw[index + 1..].to_string(),
    }
}

fn fence_mark(raw: &str) -> Option<(char, usize)> {
    let bytes = raw.as_bytes();
    let mut index = 0;
    while index < bytes.len() && (bytes[index] == b' ' || bytes[index] == b'\t') {
        index += 1;
    }
    let indent = index;
    if indent > 3 {
        return None;
    }
    let mark = bytes[index];
    if mark != b'`' && mark != b'~' {
        return None;
    }
    let mut size = 0;
    while index + size < bytes.len() && bytes[index + size] == mark {
        size += 1;
    }
    if size < 3 {
        return None;
    }
    Some((mark as char, size))
}

fn last_line_trimmed(raw: &str) -> String {
    raw.trim_end()
        .rsplit('\n')
        .next()
        .unwrap_or("")
        .trim()
        .to_string()
}

fn open(raw: &str) -> bool {
    let (mark, size) = match fence_mark(raw) {
        Some(mark) => mark,
        None => return false,
    };
    let last = last_line_trimmed(raw);
    let last_bytes = last.as_bytes();
    let mut index = 0;
    while index < last_bytes.len() && (last_bytes[index] == b'\t' || last_bytes[index] == b' ') {
        index += 1;
    }
    if index > 3 {
        return true;
    }
    let mut count = 0;
    while index + count < last_bytes.len() && last_bytes[index + count] == mark as u8 {
        count += 1;
    }
    if count < size {
        return true;
    }
    let tail = last_bytes.len();
    let mut end = index + count;
    while end < tail && (last_bytes[end] == b'\t' || last_bytes[end] == b' ') {
        end += 1;
    }
    end != tail
}

fn closes_fence(raw: &str, suffix: &str) -> bool {
    let Some((_, _)) = fence_mark(raw) else {
        return suffix.contains("```") || suffix.contains("~~~");
    };
    let mark_len = fence_mark(raw).map(|(_, size)| size).unwrap_or(3);
    let mark_char = fence_mark(raw).map(|(char, _)| char).unwrap_or('`');
    let mark: String = std::iter::repeat(mark_char).take(mark_len).collect();
    let window = if mark_len > 1 {
        format!(
            "{}{}",
            &raw[raw.len().saturating_sub(mark_len - 1)..],
            suffix
        )
    } else {
        format!("{}{}", raw, suffix)
    };
    window.contains(&mark)
}

/// `heal` — `remend(text, { linkMode: "text-only" })`.
/// PROVISIONAL: ported heal rules for the pinned constructs (unclosed emphasis,
/// unclosed inline code, incomplete `[label](url` links); full `remend` pending.
pub fn heal(text: &str) -> String {
    let text = collapse_incomplete_links(text);
    let text = close_run(&text, '`');
    close_emphasis(&text)
}

// Replaces `[label](...` without a closing `)` by `label` (text-only links).
fn collapse_incomplete_links(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let bytes = text.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'[' {
            if let Some(end) = find_ascii(&bytes[index + 1..], b']') {
                let label_end = index + 1 + end;
                let after = &bytes[label_end + 1..];
                if after.starts_with(b"](") || after.starts_with(b"(") {
                    let paren_at = label_end + 1 + usize::from(after.starts_with(b"]("));
                    if !bytes[paren_at + 1..].contains(&b')') {
                        out.push_str(&text[index + 1..label_end]);
                        return out;
                    }
                }
            }
        }
        out.push(bytes[index] as char);
        index += 1;
    }
    out
}

fn find_ascii(haystack: &[u8], needle: u8) -> Option<usize> {
    haystack.iter().position(|byte| *byte == needle)
}

// Closes an unclosed run of `delimiter` (backticks) by appending it.
fn close_run(text: &str, delimiter: char) -> String {
    let bytes = text.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == delimiter as u8 {
            let mut run = 0;
            while index + run < bytes.len() && bytes[index + run] == delimiter as u8 {
                run += 1;
            }
            if find_closing_run(&bytes[index + run..], delimiter, run).is_none() {
                let mut out = text.to_string();
                for _ in 0..run {
                    out.push(delimiter);
                }
                return out;
            }
            index += run;
            continue;
        }
        index += 1;
    }
    text.to_string()
}

fn find_closing_run(haystack: &[u8], delimiter: char, size: usize) -> Option<usize> {
    let mut index = 0;
    while index < haystack.len() {
        if haystack[index] == delimiter as u8 {
            let mut run = 0;
            while index + run < haystack.len() && haystack[index + run] == delimiter as u8 {
                run += 1;
            }
            if run == size {
                return Some(index);
            }
            index += run;
            continue;
        }
        index += 1;
    }
    None
}

// Closes unclosed `**` / `__` emphasis markers by appending them.
fn close_emphasis(text: &str) -> String {
    let mut out = text.to_string();
    for marker in ["**", "__"] {
        let count = out.matches(marker).count();
        if count % 2 == 1 {
            out.push_str(marker);
        }
    }
    out
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Space(String),
    Code {
        raw: String,
        text: String,
        lang: Option<String>,
    },
    Other(String),
}

fn is_blank(line: &str) -> bool {
    line.trim_matches([' ', '\t', '\r']).is_empty()
}

fn lex_blocks(text: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let lines: Vec<&str> = text.split_inclusive('\n').collect();
    let mut index = 0;
    while index < lines.len() {
        let line = lines[index];
        if is_blank(line) {
            let mut raw = String::new();
            while index < lines.len() && is_blank(lines[index]) {
                raw.push_str(lines[index]);
                index += 1;
            }
            tokens.push(Token::Space(raw));
            continue;
        }
        if let Some((mark, _size)) = fence_mark(line) {
            let mut raw = String::new();
            let mut body = String::new();
            let info = line
                .trim_start_matches([' ', '\t'])
                .trim_start_matches(mark)
                .trim_end_matches(['\r', '\n'])
                .to_string();
            let lang = language(Some(&info));
            raw.push_str(line);
            index += 1;
            let mut closed = false;
            while index < lines.len() {
                raw.push_str(lines[index]);
                if closes_fence_line(lines[index], mark) {
                    closed = true;
                    index += 1;
                    break;
                }
                body.push_str(lines[index]);
                index += 1;
            }
            if !closed {
                // An unterminated fence consumes the rest of the input like `marked`.
                let text = open_code(&raw);
                tokens.push(Token::Code { raw, text, lang });
            } else {
                let text = body_text(&body);
                tokens.push(Token::Code { raw, text, lang });
            }
            continue;
        }
        let mut raw = String::new();
        while index < lines.len() && !is_blank(lines[index]) && fence_mark(lines[index]).is_none() {
            raw.push_str(lines[index]);
            index += 1;
        }
        if !raw.is_empty() {
            tokens.push(Token::Other(raw));
        }
    }
    tokens
}

fn closes_fence_line(line: &str, mark: char) -> bool {
    let bytes = line.as_bytes();
    let mut index = 0;
    while index < bytes.len() && (bytes[index] == b' ' || bytes[index] == b'\t') {
        index += 1;
    }
    if index > 3 {
        return false;
    }
    let mut count = 0;
    while index + count < bytes.len() && bytes[index + count] == mark as u8 {
        count += 1;
    }
    if count < 3 {
        return false;
    }
    let mut end = index + count;
    while end < bytes.len()
        && (bytes[end] == b' ' || bytes[end] == b'\t' || bytes[end] == b'\r' || bytes[end] == b'\n')
    {
        end += 1;
    }
    end == bytes.len()
}

fn body_text(body: &str) -> String {
    body.strip_suffix('\n').unwrap_or(body).to_string()
}

/// 1:1 port of `stream(text, live)`.
pub fn stream(text: &str, live: bool) -> Vec<Block> {
    if !live {
        return completed_projection_blocks(text);
    }
    if refs(text) {
        return vec![Block {
            raw: text.to_string(),
            src: heal(text),
            mode: BlockMode::Live,
            language: None,
            complete: None,
        }];
    }
    let tokens = lex_blocks(text);
    let mut tail: Option<usize> = None;
    for (index, token) in tokens.iter().enumerate() {
        if !matches!(token, Token::Space(_)) {
            tail = Some(index);
        }
    }
    let Some(tail) = tail else {
        return vec![Block {
            raw: text.to_string(),
            src: heal(text),
            mode: BlockMode::Live,
            language: None,
            complete: None,
        }];
    };
    let Some(last) = tokens.get(tail).cloned() else {
        return vec![Block {
            raw: text.to_string(),
            src: heal(text),
            mode: BlockMode::Live,
            language: None,
            complete: None,
        }];
    };

    let mut result: Vec<Block> = Vec::new();
    let mut index = 0;
    while index < tail {
        let token = tokens[index].clone();
        match token {
            Token::Space(_) => {
                index += 1;
                continue;
            }
            Token::Code { raw, text, lang } => {
                let mut full = raw;
                while index + 1 < tokens.len()
                    && index + 1 < tail
                    && matches!(tokens[index + 1], Token::Space(_))
                {
                    if let Token::Space(extra) = tokens[index + 1].clone() {
                        full.push_str(&extra);
                    }
                    index += 1;
                }
                result.push(Block {
                    raw: full,
                    src: text,
                    mode: BlockMode::Code,
                    language: lang,
                    complete: Some(true),
                });
            }
            Token::Other(raw) => {
                let mut full = raw;
                while index + 1 < tokens.len()
                    && index + 1 < tail
                    && matches!(tokens[index + 1], Token::Space(_))
                {
                    if let Token::Space(extra) = tokens[index + 1].clone() {
                        full.push_str(&extra);
                    }
                    index += 1;
                }
                result.push(Block {
                    raw: full.clone(),
                    src: full,
                    mode: BlockMode::Full,
                    language: None,
                    complete: None,
                });
            }
        }
        index += 1;
    }

    let raw: String = tokens[tail..]
        .iter()
        .map(|token| match token {
            Token::Space(value) => value.clone(),
            Token::Code { raw, .. } => raw.clone(),
            Token::Other(value) => value.clone(),
        })
        .collect();
    match last {
        Token::Space(_) | Token::Other(_) => {
            result.push(Block {
                raw: raw.clone(),
                src: heal(&raw),
                mode: BlockMode::Live,
                language: None,
                complete: None,
            });
        }
        Token::Code {
            raw: code_raw,
            text,
            lang,
        } => {
            let _ = code_raw;
            if !open(&raw) {
                result.push(Block {
                    raw: raw.clone(),
                    src: text,
                    mode: BlockMode::Code,
                    language: lang,
                    complete: Some(true),
                });
            } else {
                result.push(Block {
                    raw: raw.clone(),
                    src: open_code(&raw),
                    mode: BlockMode::Code,
                    language: lang,
                    complete: None,
                });
            }
        }
    }
    result
}

pub fn completed_projection_blocks(text: &str) -> Vec<Block> {
    vec![Block {
        raw: text.to_string(),
        src: text.to_string(),
        mode: BlockMode::Full,
        language: None,
        complete: None,
    }]
}

/// 1:1 port of `project(previous, text, live)`.
pub fn project(previous: Option<&Projection>, text: &str, live: bool) -> Projection {
    if !live {
        let current: Option<&Projection> = match previous {
            Some(prev) if prev.text == text => Some(prev),
            Some(prev) if text.starts_with(&prev.text) => {
                // Equivalent to `project(previous, text, true)` — the live pass is
                // re-derived, then the live/code tails below are finalized.
                let _ = prev;
                None
            }
            _ => None,
        };
        let live_projection;
        let current = match current {
            Some(current) => current,
            None => {
                // `project(previous, text, true)` with a shared prefix.
                live_projection = match previous {
                    Some(prev) if text.starts_with(&prev.text) => {
                        let tail = prev.blocks.last();
                        let suffix = text[prev.text.len()..].to_string();
                        if suffix.is_empty()
                            || !matches!(tail.map(|block| &block.mode), Some(BlockMode::Code))
                            || tail.and_then(|block| block.complete).unwrap_or(false)
                            || closes_fence(
                                &tail.map(|block| block.raw.clone()).unwrap_or_default(),
                                &suffix,
                            )
                        {
                            Projection {
                                text: text.to_string(),
                                blocks: stream(text, true),
                            }
                        } else {
                            let mut blocks =
                                prev.blocks[..prev.blocks.len().saturating_sub(1)].to_vec();
                            if let Some(mut tail) = tail.cloned() {
                                tail.raw = format!("{}{}", tail.raw, suffix);
                                tail.src = format!("{}{}", tail.src, suffix);
                                blocks.push(tail);
                            }
                            Projection {
                                text: text.to_string(),
                                blocks,
                            }
                        }
                    }
                    _ => Projection {
                        text: text.to_string(),
                        blocks: stream(text, true),
                    },
                };
                &live_projection
            }
        };
        return Projection {
            text: text.to_string(),
            blocks: current
                .blocks
                .iter()
                .map(|block| match block.mode {
                    BlockMode::Live => Block {
                        raw: block.raw.clone(),
                        src: block.raw.clone(),
                        mode: BlockMode::Full,
                        language: block.language.clone(),
                        complete: block.complete,
                    },
                    BlockMode::Code if block.complete != Some(true) => Block {
                        complete: Some(true),
                        ..block.clone()
                    },
                    _ => block.clone(),
                })
                .collect(),
        };
    }
    let Some(previous) = previous else {
        return Projection {
            text: text.to_string(),
            blocks: stream(text, live),
        };
    };
    if !text.starts_with(&previous.text) {
        return Projection {
            text: text.to_string(),
            blocks: stream(text, live),
        };
    }
    let tail = previous.blocks.last();
    let suffix = text[previous.text.len()..].to_string();
    if suffix.is_empty()
        || !matches!(tail.map(|block| &block.mode), Some(BlockMode::Code))
        || tail.and_then(|block| block.complete).unwrap_or(false)
        || closes_fence(
            &tail.map(|block| block.raw.clone()).unwrap_or_default(),
            &suffix,
        )
    {
        return Projection {
            text: text.to_string(),
            blocks: stream(text, live),
        };
    }
    let mut blocks = previous.blocks[..previous.blocks.len().saturating_sub(1)].to_vec();
    if let Some(mut tail) = tail.cloned() {
        tail.raw = format!("{}{}", tail.raw, suffix);
        tail.src = format!("{}{}", tail.src, suffix);
        blocks.push(tail);
    }
    Projection {
        text: text.to_string(),
        blocks,
    }
}
