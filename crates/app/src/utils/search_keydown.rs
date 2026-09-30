//! Rust port of `packages/app/src/utils/search-keydown.ts` (opencode v1.18.30).
//!
//! Source 116 lines: `handleDocumentSearchKeydown` (+ private
//! `searchKeyAction`/`moveSelection`/boundary/update helpers). DOM focus and
//! selection ranges are modelled as explicit input/output structs.
//! Original file: `packages/app/src/utils/search-keydown.ts`

#![allow(dead_code)]

/// Mirrors the `editableSelector` guard (verbatim).
pub const EDITABLE_SELECTOR: &str =
    "input, textarea, select, [contenteditable=''], [contenteditable='true']";

/// Mirrors the `searchKeyAction` result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchKeyAction {
    SelectAll,
    Insert { value: String },
    DeleteBackward,
    DeleteForward,
    Move { delta: i32 },
    Home,
    End,
}

/// Mirrors the key-event input (`ctrlKey`/`metaKey`/`altKey`/`key`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyEvent {
    pub key: String,
    pub ctrl: bool,
    pub meta: bool,
    pub alt: bool,
}

/// Mirrors `searchKeyAction(event)`.
pub fn search_key_action(event: &KeyEvent) -> Option<SearchKeyAction> {
    if (event.ctrl || event.meta) && !event.alt && event.key.to_lowercase() == "a" {
        return Some(SearchKeyAction::SelectAll);
    }
    if event.ctrl || event.meta || event.alt {
        return None;
    }
    if event.key.len() == 1 {
        return Some(SearchKeyAction::Insert {
            value: event.key.clone(),
        });
    }
    match event.key.as_str() {
        "Backspace" => Some(SearchKeyAction::DeleteBackward),
        "Delete" => Some(SearchKeyAction::DeleteForward),
        "ArrowLeft" => Some(SearchKeyAction::Move { delta: -1 }),
        "ArrowRight" => Some(SearchKeyAction::Move { delta: 1 }),
        "Home" => Some(SearchKeyAction::Home),
        "End" => Some(SearchKeyAction::End),
        _ => None,
    }
}

/// Mirrors the document-search input state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchInput {
    pub value: String,
    pub start: usize,
    pub end: usize,
    pub has_input: bool,
    pub default_prevented: bool,
    pub is_composing: bool,
    pub target_is_input: bool,
    pub target_is_editable: bool,
}

/// Mirrors `handleDocumentSearchKeydown` — returns the updated input state,
/// or `None` when the event is ignored.
pub fn handle_document_search_keydown(
    state: &SearchInput,
    event: &KeyEvent,
    extend: bool,
) -> Option<SearchInput> {
    if !state.has_input || state.default_prevented || state.is_composing {
        return None;
    }
    if state.target_is_input || state.target_is_editable {
        return None;
    }
    let action = search_key_action(event)?;
    let mut next = state.clone();
    match action {
        SearchKeyAction::SelectAll => {
            next.start = 0;
            next.end = next.value.len();
        }
        SearchKeyAction::Insert { value } => {
            next.value = format!(
                "{}{}{}",
                &state.value[..state.start],
                value,
                &state.value[state.end..]
            );
            let caret = state.start + value.len();
            next.start = caret;
            next.end = caret;
        }
        SearchKeyAction::DeleteBackward => {
            if state.start != state.end {
                next.value = format!(
                    "{}{}",
                    &state.value[..state.start],
                    &state.value[state.end..]
                );
                next.end = state.start;
            } else if state.start == 0 {
                // no-op, still handled
            } else {
                next.value = format!(
                    "{}{}",
                    &state.value[..state.start - 1],
                    &state.value[state.end..]
                );
                next.start -= 1;
                next.end = next.start;
            }
        }
        SearchKeyAction::DeleteForward => {
            if state.start != state.end {
                next.value = format!(
                    "{}{}",
                    &state.value[..state.start],
                    &state.value[state.end..]
                );
                next.end = state.start;
            } else if state.end == state.value.len() {
                // no-op, still handled
            } else {
                next.value = format!(
                    "{}{}",
                    &state.value[..state.start],
                    &state.value[state.end + 1..]
                );
            }
        }
        SearchKeyAction::Move { delta } => {
            if !extend && state.start != state.end {
                let caret = if delta < 0 { state.start } else { state.end };
                next.start = caret;
                next.end = caret;
            } else {
                let base = if extend { state.end } else { state.start };
                let caret =
                    (base as i64 + delta as i64).clamp(0, state.value.len() as i64) as usize;
                if extend {
                    next.end = caret;
                } else {
                    next.start = caret;
                    next.end = caret;
                }
            }
        }
        SearchKeyAction::Home => {
            if extend {
                next.end = next.start;
                next.start = 0;
            } else {
                next.start = 0;
                next.end = 0;
            }
        }
        SearchKeyAction::End => {
            let len = state.value.len();
            if extend {
                next.end = len;
            } else {
                next.start = len;
                next.end = len;
            }
        }
    }
    Some(next)
}
