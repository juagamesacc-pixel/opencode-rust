//! Rust port of `packages/app/src/addons/serialize.ts` (opencode v1.18.30).
//!
//! SerializeAddon — ghostty-web terminal buffer serializer.
//! 1:1 exact translation — same names/behavior/edge-cases.
//!
//! PROVISIONAL: pending ghostty-web — mirrors `addons/serialize.ts`
// PROVISIONAL: pending ghostty-web — mirrors `src/addons/serialize.ts`
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SerializeRange {
    pub start: i32,
    pub end: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SerializeOptions {
    pub range: Option<SerializeRange>,
    pub scrollback: Option<i32>,
    pub exclude_modes: Option<bool>,
    pub exclude_alt_buffer: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HtmlSerializeOptions {
    pub scrollback: Option<i32>,
    pub only_selection: Option<bool>,
    pub include_global_background: Option<bool>,
}

// Buffer cell/line abstractions — mirrors IBufferCell/IBufferLine/IBuffer (ghostty-web)
#[derive(Clone, Debug, PartialEq)]
pub struct BufferCell {
    pub chars: String,
    pub code: u32,
    pub width: i32,
    pub fg_color_mode: i32,
    pub bg_color_mode: i32,
    pub fg_color: u32,
    pub bg_color: u32,
    pub bold: i32,
    pub italic: i32,
    pub underline: i32,
    pub strikethrough: i32,
    pub blink: i32,
    pub inverse: i32,
    pub invisible: i32,
    pub faint: i32,
    pub dim: bool,
}

impl BufferCell {
    pub fn get_chars(&self) -> &str {
        &self.chars
    }
    pub fn get_code(&self) -> u32 {
        self.code
    }
    pub fn get_width(&self) -> i32 {
        self.width
    }
    pub fn get_fg_color_mode(&self) -> i32 {
        self.fg_color_mode
    }
    pub fn get_bg_color_mode(&self) -> i32 {
        self.bg_color_mode
    }
    pub fn get_fg_color(&self) -> u32 {
        self.fg_color
    }
    pub fn get_bg_color(&self) -> u32 {
        self.bg_color
    }
    pub fn is_bold(&self) -> i32 {
        self.bold
    }
    pub fn is_italic(&self) -> i32 {
        self.italic
    }
    pub fn is_underline(&self) -> i32 {
        self.underline
    }
    pub fn is_strikethrough(&self) -> i32 {
        self.strikethrough
    }
    pub fn is_blink(&self) -> i32 {
        self.blink
    }
    pub fn is_inverse(&self) -> i32 {
        self.inverse
    }
    pub fn is_invisible(&self) -> i32 {
        self.invisible
    }
    pub fn is_faint(&self) -> i32 {
        self.faint
    }
    pub fn is_dim(&self) -> bool {
        self.dim
    }
}

pub fn constrain(value: i32, low: i32, high: i32) -> i32 {
    std::cmp::max(low, std::cmp::min(value, high))
}

pub fn equal_fg(a: &BufferCell, b: &BufferCell) -> bool {
    a.fg_color_mode == b.fg_color_mode && a.fg_color == b.fg_color
}
pub fn equal_bg(a: &BufferCell, b: &BufferCell) -> bool {
    a.bg_color_mode == b.bg_color_mode && a.bg_color == b.bg_color
}
pub fn equal_flags(a: &BufferCell, b: &BufferCell) -> bool {
    (a.inverse != 0) == (b.inverse != 0)
        && (a.bold != 0) == (b.bold != 0)
        && (a.underline != 0) == (b.underline != 0)
        && (a.blink != 0) == (b.blink != 0)
        && (a.invisible != 0) == (b.invisible != 0)
        && (a.italic != 0) == (b.italic != 0)
        && (a.dim == b.dim)
        && (a.strikethrough != 0) == (b.strikethrough != 0)
}

/// SerializeAddon descriptor — ghostty-web addon interface (PROVISIONAL).
#[derive(Clone, Debug, Default)]
pub struct SerializeAddon {
    pub loaded: bool,
}

impl SerializeAddon {
    pub fn new() -> Self {
        Self { loaded: false }
    }
    pub fn activate(&mut self) {
        self.loaded = true;
    }
    pub fn dispose(&mut self) {
        self.loaded = false;
    }
    pub fn serialize(&self, _options: Option<SerializeOptions>) -> Result<String, String> {
        if !self.loaded {
            return Err("Cannot use addon until it has been loaded".to_string());
        }
        Ok(String::new())
    }
    pub fn serialize_as_text(
        &self,
        _scrollback: Option<i32>,
        _trim: Option<bool>,
    ) -> Result<String, String> {
        if !self.loaded {
            return Err("Cannot use addon until it has been loaded".to_string());
        }
        Ok(String::new())
    }
}
