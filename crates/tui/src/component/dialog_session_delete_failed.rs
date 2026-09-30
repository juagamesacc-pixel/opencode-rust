// source: packages/tui/src/component/dialog-session-delete-failed.tsx (99 lines, v1.18.30)
// 1:1 port — delete/restore cards (return/left/up/right/down map,
// verbatim copy); `confirm()` runs the active handler, stays open on
// explicit `false`, calls `onDone` (else clears) on success.

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
pub enum RecoveryChoice {
    Delete,
    Restore,
}

/// Delete-failed dialog state (starts on delete, verbatim).
pub struct DeleteFailedState {
    pub session: String,
    pub workspace: String,
    active: RecoveryChoice,
    /// Set when `confirm()` should run the delete handler.
    pub run_delete: bool,
    /// Set when `confirm()` should run the restore handler.
    pub run_restore: bool,
}

impl DeleteFailedState {
    pub fn new(session: &str, workspace: &str) -> Self {
        Self {
            session: session.to_string(),
            workspace: workspace.to_string(),
            active: RecoveryChoice::Delete,
            run_delete: false,
            run_restore: false,
        }
    }

    /// Mirrors `confirm` — flags the active handler; the caller runs it
    /// and applies the `false`-keeps-open / `onDone`-or-clear rules.
    pub fn confirm(&mut self) {
        match self.active {
            RecoveryChoice::Delete => self.run_delete = true,
            RecoveryChoice::Restore => self.run_restore = true,
        }
    }

    pub fn take_run(&mut self) -> Option<RecoveryChoice> {
        if self.run_delete {
            self.run_delete = false;
            return Some(RecoveryChoice::Delete);
        }
        if self.run_restore {
            self.run_restore = false;
            return Some(RecoveryChoice::Restore);
        }
        None
    }
}

impl DialogContent for DeleteFailedState {
    fn render(&self, theme: &Theme, area: Rect, buf: &mut Buffer) {
        let mut y = area.y;
        Line::styled(
            "Failed to Delete Session",
            Style::default()
                .fg(rgba(theme.text))
                .add_modifier(Modifier::BOLD),
        )
        .render(
            Rect {
                x: area.x + 2,
                y,
                width: area.width.saturating_sub(9),
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
            format!("The session \"{}\" could not be deleted because the workspace \"{}\" is not available.", self.session, self.workspace),
            "Choose how you want to recover this broken workspace session.".to_string(),
        ] {
            Paragraph::new(message).style(Style::default().fg(rgba(theme.text_muted))).wrap(Wrap { trim: false }).render(
                Rect { x: area.x + 2, y, width: area.width.saturating_sub(4), height: 2 },
                buf,
            );
            y += 2;
        }
        for (id, title, description) in [
            (
                RecoveryChoice::Delete,
                "Delete workspace",
                "Delete the workspace and all sessions attached to it.",
            ),
            (
                RecoveryChoice::Restore,
                "Restore to new workspace",
                "Try to restore this session into a new workspace.",
            ),
        ] {
            let active = self.active == id;
            let card = Rect {
                x: area.x + 2,
                y,
                width: area.width.saturating_sub(4),
                height: 3,
            };
            ratatui::widgets::Block::default()
                .style(Style::default().bg(if active {
                    rgba(theme.primary)
                } else {
                    rgba(crate::theme::Rgba::from_ints_alpha(0, 0, 0, 0))
                }))
                .render(card, buf);
            let fg = if active {
                theme.selected_list_item_text
            } else {
                theme.text
            };
            let sub = if active {
                theme.selected_list_item_text
            } else {
                theme.text_muted
            };
            Paragraph::new(Line::styled(
                title,
                Style::default().fg(rgba(fg)).add_modifier(Modifier::BOLD),
            ))
            .render(
                Rect {
                    x: card.x + 1,
                    y: card.y + 1,
                    width: card.width.saturating_sub(2),
                    height: 1,
                },
                buf,
            );
            Paragraph::new(Line::styled(description, Style::default().fg(rgba(sub))))
                .wrap(Wrap { trim: false })
                .render(
                    Rect {
                        x: card.x + 1,
                        y: card.y + 2,
                        width: card.width.saturating_sub(2),
                        height: 1,
                    },
                    buf,
                );
            y += 4;
        }
    }

    fn handle_key(&mut self, key: &KeyEvent, _control: &mut dyn DialogControl) -> bool {
        match key.code {
            KeyCode::Enter => {
                self.confirm();
                true
            }
            KeyCode::Left | KeyCode::Up => {
                self.active = RecoveryChoice::Delete;
                true
            }
            KeyCode::Right | KeyCode::Down => {
                self.active = RecoveryChoice::Restore;
                true
            }
            _ => false,
        }
    }

    fn bindings(&self) -> Vec<DialogBinding> {
        vec![
            DialogBinding {
                key: "return".to_string(),
                desc: "Confirm recovery option".to_string(),
                group: "Dialog".to_string(),
            },
            DialogBinding {
                key: "left".to_string(),
                desc: "Delete broken session".to_string(),
                group: "Dialog".to_string(),
            },
            DialogBinding {
                key: "up".to_string(),
                desc: "Delete broken session".to_string(),
                group: "Dialog".to_string(),
            },
            DialogBinding {
                key: "right".to_string(),
                desc: "Restore broken session".to_string(),
                group: "Dialog".to_string(),
            },
            DialogBinding {
                key: "down".to_string(),
                desc: "Restore broken session".to_string(),
                group: "Dialog".to_string(),
            },
        ]
    }
}
