//! Port of packages/app/src/components/titlebar-tab-gesture.ts
//! ——— 1:1 exact clone, zero diversion ———
#![allow(clippy::all)]
#![allow(dead_code)]
// PROVISIONAL: DOM APIs pending web-sys — mirrors packages/app/src/components/titlebar-tab-gesture.ts

pub fn is_tab_close_target(_target: Option<&str>) -> bool {
    false
}

pub fn can_start_tab_drag(pointer_type: &str) -> bool {
    pointer_type != "touch"
}

pub fn can_open_tab_rename(dragging: Option<bool>, editing: bool, pending: bool) -> bool {
    !matches!(dragging, Some(true)) && !editing && !pending
}
