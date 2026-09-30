// source: packages/tui/src/prompt/display.ts (48 lines, v1.18.30)
// 1:1 port — display-width slicing over chars (grapheme segmentation has
// no dependency-free equivalent; CJK/wide chars use width 2 like
// `Bun.stringWidth`, combining marks width 0, `\n` width 1 — same offsets
// for all BMP text).

#![allow(dead_code)]

/// Character display width (mirrors `Bun.stringWidth` per segment).
pub fn char_width(ch: char) -> usize {
    if ch == '\n' {
        return 1;
    }
    if ch == '\0' {
        return 0;
    }
    // Combining marks / variation selectors / zero-width joiners.
    if ('\u{0300}'..='\u{036F}').contains(&ch)
        || ('\u{1AB0}'..='\u{1AFF}').contains(&ch)
        || ('\u{1DC0}'..='\u{1DFF}').contains(&ch)
        || ('\u{20D0}'..='\u{20FF}').contains(&ch)
        || ('\u{FE00}'..='\u{FE0F}').contains(&ch)
        || ch == '\u{200D}'
        || ch == '\u{FEFF}'
    {
        return 0;
    }
    let code = ch as u32;
    if ((0x1100..=0x115F).contains(&code))
        || code == 0x2329
        || code == 0x232A
        || ((0x2E80..=0x303E).contains(&code))
        || ((0x3041..=0x33FF).contains(&code))
        || ((0x3400..=0x4DBF).contains(&code))
        || ((0x4E00..=0xA4CF).contains(&code))
        || ((0xA960..=0xA97F).contains(&code))
        || ((0xAC00..=0xD7FF).contains(&code))
        || ((0xF900..=0xFAFF).contains(&code))
        || ((0xFE30..=0xFE4F).contains(&code))
        || ((0xFF00..=0xFF60).contains(&code))
        || ((0xFFE0..=0xFFE6).contains(&code))
        || ((0x20000..=0x3FFFD).contains(&code))
    {
        return 2;
    }
    1
}

/// Mirrors `promptOffsetWidth`.
pub fn prompt_offset_width(value: &str) -> usize {
    value.chars().map(char_width).sum()
}

fn offset_index(value: &str, offset: usize) -> usize {
    if offset == 0 {
        return 0;
    }
    let mut width = 0;
    for (index, ch) in value.char_indices() {
        let next = width + char_width(ch);
        if next > offset {
            return index;
        }
        width = next;
    }
    value.len()
}

/// Mirrors `displaySlice` (byte-index conversion keeps `\n` at width 1).
pub fn display_slice(value: &str, start: usize, end: usize) -> &str {
    let from = offset_index(value, start).min(value.len());
    let to = offset_index(value, end).min(value.len());
    if from >= to {
        return "";
    }
    &value[from..to]
}

/// Mirrors `displayCharAt`.
pub fn display_char_at(value: &str, offset: usize) -> Option<char> {
    let mut width = 0;
    for ch in value.chars() {
        let next = width + char_width(ch);
        if offset == width || offset < next {
            return Some(ch);
        }
        width = next;
    }
    None
}

/// Mirrors `mentionTriggerIndex` — display offset of a trigger `@` whose
/// query has no whitespace (or `None`).
pub fn mention_trigger_index(value: &str, offset: usize) -> Option<usize> {
    let end = offset.min(prompt_offset_width(value));
    let text = display_slice(value, 0, end);
    let index = text.rfind('@')?;
    let before = if index == 0 {
        None
    } else {
        text[..index].chars().last()
    };
    let query = &text[index..];
    if (before.is_none() || before.is_some_and(|ch| ch.is_whitespace()))
        && !query.chars().any(|ch| ch.is_whitespace())
    {
        // Byte index → display offset of the trigger start.
        let prefix_width: usize = text[..index].chars().map(char_width).sum();
        return Some(prefix_width);
    }
    None
}
