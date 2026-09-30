//! Port of packages/app/src/components/prompt-input/history.ts
//! ——— 1:1 exact clone, zero diversion ———
#![allow(clippy::all)]
#![allow(dead_code)]
use serde::{Deserialize, Serialize};

pub const MAX_HISTORY: usize = 100;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SelectedLineRange {
    pub start: usize,
    pub end: usize,
    pub side: Option<String>,
    pub end_side: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PromptHistoryComment {
    pub id: String,
    pub path: String,
    pub selection: SelectedLineRange,
    pub comment: String,
    pub time: i64,
    pub origin: Option<String>,
    pub preview: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PromptPart {
    #[serde(rename = "type")]
    pub part_type: String,
    pub content: Option<String>,
    pub path: Option<String>,
}

pub type Prompt = Vec<PromptPart>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PromptHistoryEntry {
    pub prompt: Prompt,
    pub comments: Vec<PromptHistoryComment>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PromptHistoryStoredEntry {
    Array(Prompt),
    Entry(PromptHistoryEntry),
}

pub fn can_navigate_history_at_cursor(
    direction: &str,
    text: &str,
    cursor: usize,
    in_history: bool,
) -> bool {
    let position = cursor.min(text.len());
    let at_start = position == 0;
    let at_end = position == text.len();
    if in_history {
        return at_start || at_end;
    }
    if direction == "up" {
        return position == 0 && text.is_empty();
    }
    position == text.len()
}

pub fn clone_prompt_parts(prompt: &Prompt) -> Prompt {
    prompt.clone()
}
pub fn clone_prompt_history_comments(
    comments: &[PromptHistoryComment],
) -> Vec<PromptHistoryComment> {
    comments.to_vec()
}

pub fn normalize_prompt_history_entry(entry: &PromptHistoryStoredEntry) -> PromptHistoryEntry {
    match entry {
        PromptHistoryStoredEntry::Array(p) => PromptHistoryEntry {
            prompt: p.clone(),
            comments: vec![],
        },
        PromptHistoryStoredEntry::Entry(e) => e.clone(),
    }
}

pub fn prompt_length(prompt: &Prompt) -> usize {
    prompt
        .iter()
        .filter_map(|p| p.content.as_ref().map(|c| c.len()))
        .sum()
}

pub fn prepend_history_entry(
    entries: Vec<PromptHistoryStoredEntry>,
    prompt: Prompt,
    comments: Vec<PromptHistoryComment>,
    max: usize,
) -> Vec<PromptHistoryStoredEntry> {
    let text: String = prompt
        .iter()
        .filter_map(|p| p.content.clone())
        .collect::<Vec<_>>()
        .join("");
    if text.trim().is_empty()
        && !prompt.iter().any(|p| p.part_type == "image")
        && !comments.iter().any(|c| !c.comment.trim().is_empty())
    {
        return entries;
    }
    let entry = PromptHistoryEntry {
        prompt: prompt.clone(),
        comments: comments.clone(),
    };
    if let Some(first) = entries.first() {
        if is_prompt_equal(first, &PromptHistoryStoredEntry::Entry(entry.clone())) {
            return entries;
        }
    }
    let mut out = vec![PromptHistoryStoredEntry::Entry(entry)];
    out.extend(entries);
    out.truncate(max);
    out
}

fn is_prompt_equal(a: &PromptHistoryStoredEntry, b: &PromptHistoryStoredEntry) -> bool {
    normalize_prompt_history_entry(a) == normalize_prompt_history_entry(b)
}
