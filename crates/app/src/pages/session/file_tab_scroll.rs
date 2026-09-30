//! Rust port of `packages/app/src/pages/session/file-tab-scroll.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — same names/behavior/edge-cases/error-strings.
//! Rename: `session/file-tab-scroll.ts` -> `session/file_tab_scroll.rs` (kebab -> snake_case).

pub struct TabScrollInput {
    pub prev_scroll_width: i32,
    pub scroll_width: i32,
    pub client_width: i32,
    pub prev_context_open: bool,
    pub context_open: bool,
}

pub fn next_tab_list_scroll_left(input: TabScrollInput) -> Option<i32> {
    if input.scroll_width <= input.prev_scroll_width {
        return None;
    }
    if !input.prev_context_open && input.context_open {
        return Some(0);
    }
    if input.scroll_width <= input.client_width {
        return None;
    }
    Some(input.scroll_width - input.client_width)
}

// PROVISIONAL: DOM MutationObserver/scrollTo — mirrors createFileTabListSync (browser only)
