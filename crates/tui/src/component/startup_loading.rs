// source: packages/tui/src/component/startup-loading.tsx (63 lines, v1.18.30)
// 1:1 port — 500ms show delay + 3000ms minimum hold verbatim; bottom
// bar with spinner + `Loading plugins…` / `Finishing startup…`.

#![allow(dead_code)]

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, Paragraph, Widget};
use std::time::{Duration, Instant};

use super::super::ui::link::rgba;
use super::spinner::Spinner;
use crate::theme::Theme;

/// Show delay verbatim (500ms).
pub const STARTUP_SHOW_DELAY_MS: u64 = 500;
/// Minimum visible hold verbatim (3000ms).
pub const STARTUP_HOLD_MS: u64 = 3000;

/// Startup loading bar state.
#[derive(Debug)]
pub struct StartupLoading {
    show: bool,
    wait_deadline: Option<Instant>,
    hold_deadline: Option<Instant>,
    stamp: Option<Instant>,
}

impl StartupLoading {
    pub fn new() -> Self {
        Self {
            show: false,
            wait_deadline: None,
            hold_deadline: None,
            stamp: None,
        }
    }

    /// Render-loop driver (mirrors the ready effect + cleanup).
    pub fn update(&mut self, ready: bool) {
        let now = Instant::now();
        if ready {
            self.wait_deadline = None;
            if !self.show {
                return;
            }
            if self.hold_deadline.is_some() {
                return;
            }
            let left = self
                .stamp
                .map(|stamp| {
                    STARTUP_HOLD_MS.saturating_sub(now.duration_since(stamp).as_millis() as u64)
                })
                .unwrap_or(0);
            if left == 0 {
                self.show = false;
                return;
            }
            self.hold_deadline = Some(now + Duration::from_millis(left));
            return;
        }
        if self.hold_deadline.is_some() {
            self.hold_deadline = None;
        }
        if self.show {
            return;
        }
        if self.wait_deadline.is_some() {
            return;
        }
        self.wait_deadline = Some(now + Duration::from_millis(STARTUP_SHOW_DELAY_MS));
    }

    /// Deadline driver (call each frame after `update`).
    pub fn poll(&mut self) {
        let now = Instant::now();
        if self.wait_deadline.map(|d| now >= d).unwrap_or(false) {
            self.wait_deadline = None;
            self.stamp = Some(now);
            self.show = true;
        }
        if self.hold_deadline.map(|d| now >= d).unwrap_or(false) {
            self.hold_deadline = None;
            self.show = false;
        }
    }

    pub fn visible(&self) -> bool {
        self.show
    }

    /// Bottom bar render (mirrors the absolute bottom-1 panel).
    pub fn render(
        &self,
        theme: &Theme,
        spinner: &Spinner,
        ready: bool,
        area: Rect,
        buf: &mut Buffer,
    ) {
        if !self.show {
            return;
        }
        let text = if ready {
            "Finishing startup…"
        } else {
            "Loading plugins…"
        };
        let width = (text.chars().count() + 6) as u16;
        let bar = Rect {
            x: area.x + area.width.saturating_sub(width) / 2,
            y: area.y + area.height.saturating_sub(2),
            width: width.min(area.width),
            height: 1,
        };
        Block::default()
            .style(Style::default().bg(rgba(theme.background_panel)))
            .render(bar, buf);
        let inner = Rect {
            x: bar.x + 1,
            y: bar.y,
            width: bar.width.saturating_sub(2),
            height: 1,
        };
        spinner.render(theme, Some(theme.text_muted), Some(text), inner, buf);
        let _ = Line::default();
    }
}

impl Default for StartupLoading {
    fn default() -> Self {
        Self::new()
    }
}
