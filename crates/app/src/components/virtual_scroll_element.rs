//! Port of packages/app/src/components/virtual-scroll-element.ts
//! ——— 1:1 exact clone, zero diversion ———
#![allow(clippy::all)]
#![allow(dead_code)]
// PROVISIONAL: DOM HTMLElement pending web-sys — mirrors packages/app/src/components/virtual-scroll-element.ts

#[derive(Debug, Clone, PartialEq)]
pub struct ElementStub {
    pub is_connected: bool,
}

pub fn virtual_scroll_element(root: Option<&ElementStub>) -> Option<String> {
    let r = root?;
    if !r.is_connected {
        return None;
    }
    Some(".scroll-view__viewport".to_string())
}
