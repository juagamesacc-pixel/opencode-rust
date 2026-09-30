//! Port of packages/app/src/components/prompt-input/paste.ts
//! ——— 1:1 exact clone, zero diversion ———
#![allow(clippy::all)]
#![allow(dead_code)]

const LARGE_PASTE_CHARS: usize = 8000;
const LARGE_PASTE_BREAKS: usize = 120;

fn large_paste(text: &str) -> bool {
    if text.len() >= LARGE_PASTE_CHARS {
        return true;
    }
    let mut breaks = 0;
    for c in text.chars() {
        if c == '\n' {
            breaks += 1;
            if breaks >= LARGE_PASTE_BREAKS {
                return true;
            }
        }
    }
    false
}

pub fn normalize_paste(text: &str) -> String {
    if !text.contains('\r') {
        return text.to_string();
    }
    text.replace("\r\n", "\n").replace('\r', "\n")
}

pub fn paste_mode(text: &str) -> &'static str {
    if large_paste(text) {
        return "manual";
    }
    if text.contains('\n') || text.contains('\r') {
        return "manual";
    }
    "native"
}
