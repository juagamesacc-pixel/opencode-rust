// source: packages/tui/src/ui/toast.tsx (102 lines, v1.18.30)
// 1:1 port — explicit toast state with deadline expiry (the `setTimeout`
// becomes `poll()`); layout numbers (top 2, right 2, max width 60,
// paddings 2/2/1/1) and the split border are verbatim.

#![allow(dead_code)]

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph, Widget, Wrap};
use std::time::{Duration, Instant};

use super::border::SPLIT_BORDER_CHARS;
use super::link::rgba;
use crate::theme::Theme;
use crate::util::selection::ToastVariant;

/// Default timeout verbatim (5000ms).
pub const DEFAULT_DURATION_MS: u64 = 5000;

/// Mirrors `ToastOptions`.
#[derive(Debug, Clone)]
pub struct ToastOptions {
    pub title: Option<String>,
    pub message: String,
    pub variant: ToastVariant,
    pub duration_ms: u64,
}

/// Mirrors `ToastInput` (duration optional).
#[derive(Debug, Clone, Default)]
pub struct ToastInput {
    pub title: Option<String>,
    pub message: String,
    pub variant: Option<ToastVariant>,
    pub duration_ms: Option<u64>,
}

/// Mirrors the toast store (`currentToast` + timeout handle).
#[derive(Debug, Default)]
pub struct ToastState {
    current: Option<ToastOptions>,
    deadline: Option<Instant>,
}

impl ToastState {
    /// Mirrors `show` — replaces the current toast and (re)arms the timer.
    pub fn show(&mut self, input: ToastInput) {
        let duration_ms = input.duration_ms.unwrap_or(DEFAULT_DURATION_MS);
        self.current = Some(ToastOptions {
            title: input.title,
            message: input.message,
            variant: input.variant.unwrap_or(ToastVariant::Info),
            duration_ms,
        });
        self.deadline = Some(Instant::now() + Duration::from_millis(duration_ms));
    }

    /// Mirrors the `error` helper.
    pub fn error(&mut self, err: &str) {
        self.show(ToastInput {
            message: if err.is_empty() {
                "An unknown error has occurred".to_string()
            } else {
                err.to_string()
            },
            variant: Some(ToastVariant::Error),
            ..ToastInput::default()
        });
    }

    pub fn current(&self) -> Option<&ToastOptions> {
        self.current.as_ref()
    }

    /// Render-loop expiry (mirrors the timeout clearing the toast).
    pub fn poll(&mut self) {
        if self
            .deadline
            .map(|deadline| Instant::now() >= deadline)
            .unwrap_or(false)
        {
            self.current = None;
            self.deadline = None;
        }
    }

    /// Mirrors the `Toast()` layout — top-right panel with the variant
    /// border color, optional bold title, word-wrapped message.
    pub fn render(&self, theme: &Theme, area: Rect, buf: &mut Buffer) {
        let Some(current) = self.current.as_ref() else {
            return;
        };
        let border_color = match current.variant {
            ToastVariant::Info => theme.info,
            ToastVariant::Success => theme.success,
            ToastVariant::Warning => theme.warning,
            ToastVariant::Error => theme.error,
        };
        let max_width = (60u16).min(area.width.saturating_sub(6));
        let width = max_width + 4;
        let mut lines = Vec::new();
        if let Some(title) = current.title.as_ref() {
            lines.push(Line::styled(
                title.clone(),
                Style::default()
                    .fg(rgba(theme.text))
                    .add_modifier(Modifier::BOLD),
            ));
            lines.push(Line::from(""));
        }
        for chunk in textwrap_lines(&current.message, max_width as usize) {
            lines.push(Line::styled(chunk, Style::default().fg(rgba(theme.text))));
        }
        let height = (lines.len() as u16 + 2).min(area.height.saturating_sub(4));
        let x = area.x + area.width.saturating_sub(width).saturating_sub(2);
        let toast_area = Rect {
            x,
            y: area.y + 2,
            width,
            height,
        };
        let block = Block::default()
            .borders(Borders::LEFT | Borders::RIGHT)
            .border_set(SPLIT_BORDER_CHARS.to_set())
            .border_style(Style::default().fg(rgba(border_color)))
            .style(Style::default().bg(rgba(theme.background_panel)));
        let inner = block.inner(toast_area);
        block.render(toast_area, buf);
        Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .render(inner, buf);
    }
}

fn textwrap_lines(message: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![message.to_string()];
    }
    let mut lines = Vec::new();
    for paragraph in message.split('\n') {
        let mut current = String::new();
        for word in paragraph.split(' ') {
            let candidate = if current.is_empty() {
                word.to_string()
            } else {
                format!("{current} {word}")
            };
            if candidate.chars().count() > width && !current.is_empty() {
                lines.push(std::mem::take(&mut current));
                current = word.to_string();
            } else {
                current = candidate;
            }
        }
        lines.push(current);
    }
    lines
}

/// Toast handle handed to consumers (mirrors `useToast()`).
pub struct ToastHandle<'a> {
    state: &'a mut ToastState,
}

impl<'a> ToastHandle<'a> {
    pub fn new(state: &'a mut ToastState) -> Self {
        Self { state }
    }

    pub fn show(&mut self, message: &str, variant: ToastVariant) {
        self.state.show(ToastInput {
            message: message.to_string(),
            variant: Some(variant),
            ..ToastInput::default()
        });
    }

    pub fn show_full(
        &mut self,
        title: Option<String>,
        message: &str,
        variant: ToastVariant,
        duration_ms: u64,
    ) {
        self.state.show(ToastInput {
            title,
            message: message.to_string(),
            variant: Some(variant),
            duration_ms: Some(duration_ms),
        });
    }

    pub fn error(&mut self, err: &str) {
        self.state.error(err);
    }
}

pub fn variant_name(variant: ToastVariant) -> &'static str {
    match variant {
        ToastVariant::Info => "info",
        ToastVariant::Success => "success",
        ToastVariant::Warning => "warning",
        ToastVariant::Error => "error",
    }
}
