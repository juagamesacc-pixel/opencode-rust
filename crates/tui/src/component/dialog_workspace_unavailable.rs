// source: packages/tui/src/component/dialog-workspace-unavailable.tsx (69 lines, v1.18.30)
// 1:1 port — cancel/restore toggle (starts on restore), return confirms,
// restore runs `onRestore` and stays open on explicit `false`.

#![allow(dead_code)]

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Paragraph, Widget, Wrap};

use super::super::ui::dialog::{DialogBinding, DialogContent, DialogControl};
use super::super::ui::link::rgba;
use crate::theme::Theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UnavailableChoice {
    Cancel,
    Restore,
}

/// Workspace-unavailable dialog state.
pub struct WorkspaceUnavailableState {
    active: UnavailableChoice,
    pub restore_requested: bool,
    pub closed: bool,
}

impl WorkspaceUnavailableState {
    pub fn new() -> Self {
        Self {
            active: UnavailableChoice::Restore,
            restore_requested: false,
            closed: false,
        }
    }

    /// Mirrors `confirm` — cancel closes; restore flags the caller (which
    /// runs `onRestore` and closes unless it returns false).
    pub fn confirm(&mut self) {
        match self.active {
            UnavailableChoice::Cancel => self.closed = true,
            UnavailableChoice::Restore => self.restore_requested = true,
        }
    }
}

impl Default for WorkspaceUnavailableState {
    fn default() -> Self {
        Self::new()
    }
}

impl DialogContent for WorkspaceUnavailableState {
    fn render(&self, theme: &Theme, area: Rect, buf: &mut Buffer) {
        let mut y = area.y;
        Line::styled(
            "Workspace Unavailable",
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
        for message in [
            "This session is attached to a workspace that is no longer available.",
            "Would you like to restore this session into a new workspace?",
        ] {
            Paragraph::new(message)
                .style(Style::default().fg(rgba(theme.text_muted)))
                .wrap(Wrap { trim: false })
                .render(
                    Rect {
                        x: area.x + 2,
                        y,
                        width: area.width.saturating_sub(4),
                        height: 1,
                    },
                    buf,
                );
            y += 1;
        }
        let mut x = area.x + area.width.saturating_sub(2);
        for choice in [UnavailableChoice::Cancel, UnavailableChoice::Restore] {
            let label = match choice {
                UnavailableChoice::Cancel => "cancel",
                UnavailableChoice::Restore => "restore",
            };
            let width = (label.chars().count() + 4) as u16;
            x = x.saturating_sub(width + 1);
            let active = self.active == choice;
            let style = if active {
                Style::default()
                    .fg(rgba(theme.selected_list_item_text))
                    .bg(rgba(theme.primary))
            } else {
                Style::default().fg(rgba(theme.text_muted))
            };
            Paragraph::new(Line::styled(format!("  {label}  "), style)).render(
                Rect {
                    x,
                    y,
                    width,
                    height: 1,
                },
                buf,
            );
        }
    }

    fn handle_key(&mut self, key: &KeyEvent, _control: &mut dyn DialogControl) -> bool {
        match key.code {
            KeyCode::Enter => {
                self.confirm();
                true
            }
            KeyCode::Left => {
                self.active = UnavailableChoice::Cancel;
                true
            }
            KeyCode::Right => {
                self.active = UnavailableChoice::Restore;
                true
            }
            _ => false,
        }
    }

    fn bindings(&self) -> Vec<DialogBinding> {
        vec![
            DialogBinding {
                key: "return".to_string(),
                desc: "Confirm workspace option".to_string(),
                group: "Dialog".to_string(),
            },
            DialogBinding {
                key: "left".to_string(),
                desc: "Cancel workspace restore".to_string(),
                group: "Dialog".to_string(),
            },
            DialogBinding {
                key: "right".to_string(),
                desc: "Restore workspace".to_string(),
                group: "Dialog".to_string(),
            },
        ]
    }
}
