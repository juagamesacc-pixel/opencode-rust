// source: packages/tui/src/component/dialog-debug.tsx (90 lines, v1.18.30)
// 1:1 port — large dialog with the verbatim entry rows (Version/Date/OS/
// Terminal/Session ID/Model), return-to-copy binding, and the footer copy
// affordance. Version/channel arrive via injection.

#![allow(dead_code)]

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget, Wrap};

use super::super::ui::dialog::{DialogBinding, DialogContent, DialogControl};
use super::super::ui::link::rgba;
use crate::context::clipboard::ClipboardService;
use crate::theme::Theme;
use crate::util::selection::ToastVariant;
use crate::util::system::{describe_os, describe_terminal};

/// One debug row.
#[derive(Debug, Clone)]
pub struct DebugEntry {
    pub label: String,
    pub value: String,
}

/// Build the entry rows (labels verbatim; values injected).
pub fn debug_entries(
    version: &str,
    channel: &str,
    date_iso: &str,
    session_id: Option<&str>,
    model: Option<&str>,
) -> Vec<DebugEntry> {
    vec![
        DebugEntry {
            label: "Version".to_string(),
            value: format!("{version} ({channel})"),
        },
        DebugEntry {
            label: "Date".to_string(),
            value: date_iso.to_string(),
        },
        DebugEntry {
            label: "OS".to_string(),
            value: describe_os(),
        },
        DebugEntry {
            label: "Terminal".to_string(),
            value: describe_terminal(),
        },
        DebugEntry {
            label: "Session ID".to_string(),
            value: session_id.unwrap_or("n/a").to_string(),
        },
        DebugEntry {
            label: "Model".to_string(),
            value: model.unwrap_or("n/a").to_string(),
        },
    ]
}

/// Debug dialog state.
pub struct DebugState {
    pub entries: Vec<DebugEntry>,
    pub copied: bool,
}

impl DebugState {
    /// Mirrors `copy` — clipboard write then the copied flag + toast.
    pub fn copy(
        &mut self,
        clipboard: &ClipboardService,
        toast: &mut dyn FnMut(&str, ToastVariant),
    ) {
        let text = self
            .entries
            .iter()
            .map(|entry| format!("{}: {}", entry.label, entry.value))
            .collect::<Vec<_>>()
            .join("\n");
        clipboard.write(&text);
        self.copied = true;
        toast("Debug info copied to clipboard", ToastVariant::Info);
    }
}

impl DialogContent for DebugState {
    fn render(&self, theme: &Theme, area: Rect, buf: &mut Buffer) {
        let mut y = area.y;
        Line::styled(
            "Debug",
            Style::default()
                .fg(rgba(theme.text))
                .add_modifier(Modifier::BOLD),
        )
        .render(
            Rect {
                x: area.x + 2,
                y,
                width: area.width.saturating_sub(7),
                height: 1,
            },
            buf,
        );
        Line::styled("esc", Style::default().fg(rgba(theme.text_muted))).render(
            Rect {
                x: area.x + area.width.saturating_sub(5),
                y,
                width: 3,
                height: 1,
            },
            buf,
        );
        y += 1;
        for entry in &self.entries {
            let label = format!("{:10}", entry.label);
            Paragraph::new(Line::from(vec![
                Span::styled(label, Style::default().fg(rgba(theme.text_muted))),
                Span::styled(entry.value.clone(), Style::default().fg(rgba(theme.text))),
            ]))
            .wrap(Wrap { trim: false })
            .render(
                Rect {
                    x: area.x + 2,
                    y,
                    width: area.width.saturating_sub(4),
                    height: 2,
                },
                buf,
            );
            y += 2;
        }
        Paragraph::new(Line::styled(
            "Share this when reporting an issue.",
            Style::default().fg(rgba(theme.text_muted)),
        ))
        .render(
            Rect {
                x: area.x + 2,
                y,
                width: area.width.saturating_sub(20),
                height: 1,
            },
            buf,
        );
        let copy_label = if self.copied { "✓ copied" } else { "copy" };
        let copy_fg = if self.copied {
            theme.success
        } else {
            theme.text
        };
        Paragraph::new(Line::from(vec![
            Span::styled(
                copy_label,
                Style::default()
                    .fg(rgba(copy_fg))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
            Span::styled("enter", Style::default().fg(rgba(theme.text_muted))),
        ]))
        .render(
            Rect {
                x: area.x + area.width.saturating_sub(14),
                y,
                width: 12,
                height: 1,
            },
            buf,
        );
    }

    fn handle_key(&mut self, _key: &KeyEvent, _control: &mut dyn DialogControl) -> bool {
        // Return-to-copy is dispatched by the app keymap (see bindings);
        // the caller invokes `copy()` — report unhandled here.
        false
    }

    fn bindings(&self) -> Vec<DialogBinding> {
        vec![DialogBinding {
            key: "return".to_string(),
            desc: "Copy debug info".to_string(),
            group: "Dialog".to_string(),
        }]
    }
}

pub fn debug_return_copies(key: &KeyEvent) -> bool {
    key.code == KeyCode::Enter
}
