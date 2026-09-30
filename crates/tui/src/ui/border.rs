// source: packages/tui/src/ui/border.ts (21 lines, v1.18.30)
// 1:1 port — border char tables verbatim, plus the ratatui conversion.

#![allow(dead_code)]

use ratatui::symbols::border;

/// Mirrors `EmptyBorder` — every edge blank except the horizontal spacer.
#[derive(Debug, Clone)]
pub struct BorderChars {
    pub top_left: &'static str,
    pub top_right: &'static str,
    pub bottom_left: &'static str,
    pub bottom_right: &'static str,
    pub vertical: &'static str,
    pub horizontal: &'static str,
    pub bottom_t: &'static str,
    pub top_t: &'static str,
    pub cross: &'static str,
    pub left_t: &'static str,
    pub right_t: &'static str,
}

pub const EMPTY_BORDER: BorderChars = BorderChars {
    top_left: "",
    bottom_left: "",
    vertical: "",
    top_right: "",
    bottom_right: "",
    horizontal: " ",
    bottom_t: "",
    top_t: "",
    cross: "",
    left_t: "",
    right_t: "",
};

/// Mirrors `SplitBorder.customBorderChars` (empty + `┃` verticals).
pub const SPLIT_BORDER_CHARS: BorderChars = BorderChars {
    vertical: "┃",
    ..EMPTY_BORDER
};

impl BorderChars {
    /// Ratatui border set (multi-byte verticals render as-is).
    pub fn to_set(&self) -> border::Set {
        border::Set {
            top_left: self.top_left,
            top_right: self.top_right,
            bottom_left: self.bottom_left,
            bottom_right: self.bottom_right,
            vertical_left: self.vertical,
            vertical_right: self.vertical,
            horizontal_top: self.horizontal,
            horizontal_bottom: self.horizontal,
        }
    }
}
