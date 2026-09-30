// source: packages/tui/src/ui/dialog-confirm.tsx (108 lines, v1.18.30)
// 1:1 port — cancel/confirm toggle (left/right), return dispatch, label
// override on cancel (`titlecase(label ?? key)`), `show()` result verbatim
// (`true`/`false`/`undefined` → `Some(true)`/`Some(false)`/`None`).

#![allow(dead_code)]

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Paragraph, Widget, Wrap};
use tokio::sync::oneshot;

use super::dialog::{DialogBinding, DialogContent, DialogControl, DialogStack};
use super::link::rgba;
use crate::theme::Theme;
use crate::util::locale::titlecase;
use std::sync::{Arc, Mutex};

/// Mirrors `DialogConfirmProps`.
#[derive(Debug, Clone)]
pub struct ConfirmProps {
    pub title: String,
    pub message: String,
    pub label: Option<String>,
}

/// Mirrors `DialogConfirmResult`.
pub type ConfirmResult = Option<bool>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice {
    Confirm,
    Cancel,
}

/// Confirm dialog state (`active` starts on confirm, verbatim).
pub struct ConfirmState {
    props: ConfirmProps,
    active: Choice,
    done: SharedDone,
}

type SharedDone = Arc<Mutex<Option<oneshot::Sender<ConfirmResult>>>>;

fn resolve_shared(shared: &SharedDone, result: ConfirmResult) {
    if let Ok(mut guard) = shared.lock() {
        if let Some(tx) = guard.take() {
            let _ = tx.send(result);
        }
    }
}

impl ConfirmState {
    fn resolve(&mut self, control: &mut dyn DialogControl, result: ConfirmResult) {
        resolve_shared(&self.done, result);
        control.clear();
    }

    fn toggle(&mut self) {
        self.active = match self.active {
            Choice::Confirm => Choice::Cancel,
            Choice::Cancel => Choice::Confirm,
        };
    }

    fn label(&self, choice: Choice) -> String {
        match choice {
            Choice::Confirm => titlecase("confirm"),
            Choice::Cancel => titlecase(self.props.label.as_deref().unwrap_or("cancel")),
        }
    }
}

impl DialogContent for ConfirmState {
    fn render(&self, theme: &Theme, area: Rect, buf: &mut Buffer) {
        let title = Line::styled(
            self.props.title.clone(),
            Style::default()
                .fg(rgba(theme.text))
                .add_modifier(Modifier::BOLD),
        );
        title.render(
            Rect {
                x: area.x + 2,
                y: area.y,
                width: area.width.saturating_sub(9),
                height: 1,
            },
            buf,
        );
        Line::styled("esc", Style::default().fg(rgba(theme.text_muted))).render(
            Rect {
                x: area.x + area.width.saturating_sub(5),
                y: area.y,
                width: 3,
                height: 1,
            },
            buf,
        );
        Paragraph::new(self.props.message.clone())
            .style(Style::default().fg(rgba(theme.text_muted)))
            .wrap(Wrap { trim: false })
            .render(
                Rect {
                    x: area.x + 2,
                    y: area.y + 1,
                    width: area.width.saturating_sub(4),
                    height: area.height.saturating_sub(3),
                },
                buf,
            );
        let mut x = area.x + area.width.saturating_sub(2);
        for choice in [Choice::Cancel, Choice::Confirm] {
            let label = self.label(choice);
            let width = (label.chars().count() + 2) as u16;
            x = x.saturating_sub(width);
            let active = self.active == choice;
            let style = if active {
                Style::default()
                    .fg(rgba(theme.selected_list_item_text))
                    .bg(rgba(theme.primary))
            } else {
                Style::default().fg(rgba(theme.text_muted))
            };
            Paragraph::new(Line::styled(format!(" {label} "), style)).render(
                Rect {
                    x,
                    y: area.y + area.height.saturating_sub(2),
                    width,
                    height: 1,
                },
                buf,
            );
        }
    }

    fn handle_key(&mut self, key: &KeyEvent, control: &mut dyn DialogControl) -> bool {
        match key.code {
            KeyCode::Enter => {
                match self.active {
                    Choice::Confirm => self.resolve(control, Some(true)),
                    Choice::Cancel => self.resolve(control, Some(false)),
                }
                true
            }
            KeyCode::Left | KeyCode::Right => {
                self.toggle();
                true
            }
            _ => false,
        }
    }

    fn bindings(&self) -> Vec<DialogBinding> {
        vec![
            DialogBinding {
                key: "return".to_string(),
                desc: "Confirm dialog selection".to_string(),
                group: "Dialog".to_string(),
            },
            DialogBinding {
                key: "left".to_string(),
                desc: "Previous dialog option".to_string(),
                group: "Dialog".to_string(),
            },
            DialogBinding {
                key: "right".to_string(),
                desc: "Next dialog option".to_string(),
                group: "Dialog".to_string(),
            },
        ]
    }
}

/// Mirrors `DialogConfirm.show` (`true`/`false`/`None` for close).
pub fn show_confirm(
    stack: &mut DialogStack,
    title: &str,
    message: &str,
    label: Option<String>,
) -> oneshot::Receiver<ConfirmResult> {
    let (tx, rx) = oneshot::channel();
    let shared: SharedDone = Arc::new(Mutex::new(Some(tx)));
    let on_close = shared.clone();
    stack.replace(
        Box::new(ConfirmState {
            props: ConfirmProps {
                title: title.to_string(),
                message: message.to_string(),
                label,
            },
            active: Choice::Confirm,
            done: shared,
        }),
        Some(Box::new(move || resolve_shared(&on_close, None))),
    );
    rx
}
