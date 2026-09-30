// source: packages/tui/src/component/spinner.tsx (26 lines, v1.18.30)
// 1:1 port — braille frames verbatim (10 frames, 80ms); the disabled path
// renders `⋯`; color defaults to theme textMuted.

#![allow(dead_code)]

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget};

use super::register_spinner::register_opencode_spinner;
use crate::theme::{Rgba, Theme};

/// Mirrors `SPINNER_FRAMES`.
pub const SPINNER_FRAMES: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

/// Frame interval verbatim (80ms).
pub const SPINNER_INTERVAL_MS: u64 = 80;

/// Disabled fallback glyph verbatim.
pub const SPINNER_DISABLED: &str = "⋯";

/// Spinner state (frame advanced by the render loop every 80ms).
#[derive(Debug, Clone)]
pub struct Spinner {
    frame: usize,
    pub animations_enabled: bool,
}

impl Spinner {
    pub fn new(animations_enabled: bool) -> Self {
        register_opencode_spinner();
        Self {
            frame: 0,
            animations_enabled,
        }
    }

    pub fn tick(&mut self) {
        self.frame = (self.frame + 1) % SPINNER_FRAMES.len();
    }

    pub fn frame(&self) -> &str {
        SPINNER_FRAMES[self.frame % SPINNER_FRAMES.len()]
    }

    /// Mirrors the `Spinner` render (spinner + gap + children, or `⋯`).
    pub fn render(
        &self,
        theme: &Theme,
        color: Option<Rgba>,
        text: Option<&str>,
        area: Rect,
        buf: &mut Buffer,
    ) {
        let color = color.unwrap_or(theme.text_muted);
        let style = Style::default().fg(crate::ui::link::rgba(color));
        if !self.animations_enabled {
            let line = match text {
                Some(text) => format!("{SPINNER_DISABLED} {text}"),
                None => SPINNER_DISABLED.to_string(),
            };
            Paragraph::new(Line::styled(line, style)).render(area, buf);
            return;
        }
        let mut spans = vec![Span::styled(self.frame(), style)];
        if let Some(text) = text {
            spans.push(Span::raw(" "));
            spans.push(Span::styled(text.to_string(), style));
        }
        Paragraph::new(Line::from(spans)).render(area, buf);
    }
}
