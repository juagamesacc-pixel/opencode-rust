// source: packages/tui/src/component/dialog-retry-action.tsx (160 lines, v1.18.30)
// 1:1 port — dismiss/action toggle (left/right/tab), return dispatch,
// `don't show again` vs label buttons, GO treatment flag, `show()`
// promise (`boolean`, close → `false`).

#![allow(dead_code)]

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Paragraph, Widget, Wrap};
use tokio::sync::oneshot;

use super::super::ui::link::{open_url, rgba};
use crate::theme::{Rgba, Theme};
use std::sync::{Arc, Mutex};

use super::super::ui::dialog::{DialogBinding, DialogContent, DialogControl, DialogStack};

/// GO upsell URL verbatim.
pub const GO_URL: &str = "https://opencode.ai/go";
const PAD_X: u16 = 3;
/// Foreground alpha verbatim.
pub const FOREGROUND_ALPHA: u8 = 186;

/// Mirrors `DialogRetryActionProps`.
#[derive(Debug, Clone)]
pub struct RetryProps {
    pub title: String,
    pub message: String,
    pub label: String,
    pub link: Option<String>,
}

type SharedDone = Arc<Mutex<Option<oneshot::Sender<bool>>>>;

fn resolve_shared(shared: &SharedDone, result: bool) {
    if let Ok(mut guard) = shared.lock() {
        if let Some(tx) = guard.take() {
            let _ = tx.send(result);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RetryChoice {
    Dismiss,
    Action,
}

/// Retry dialog state (starts on action, verbatim).
pub struct RetryState {
    props: RetryProps,
    selected: RetryChoice,
    done: SharedDone,
}

impl RetryState {
    fn panel_overlay(&self, theme: &Theme) -> Option<Rgba> {
        if !self.show_go_treatment() {
            return None;
        }
        let panel = theme.background_panel;
        Some(Rgba::from_ints_alpha(
            (panel.r * 255.0) as u8,
            (panel.g * 255.0) as u8,
            (panel.b * 255.0) as u8,
            FOREGROUND_ALPHA,
        ))
    }

    fn show_go_treatment(&self) -> bool {
        self.props.link.as_deref() == Some(GO_URL)
    }

    fn run_action(&mut self, control: &mut dyn DialogControl) {
        if let Some(link) = self.props.link.clone() {
            open_url(&link);
        }
        resolve_shared(&self.done, false);
        control.clear();
    }

    fn dismiss(&mut self, control: &mut dyn DialogControl) {
        resolve_shared(&self.done, true);
        control.clear();
    }

    fn toggle(&mut self) {
        self.selected = match self.selected {
            RetryChoice::Action => RetryChoice::Dismiss,
            RetryChoice::Dismiss => RetryChoice::Action,
        };
    }
}

impl DialogContent for RetryState {
    fn render(&self, theme: &Theme, area: Rect, buf: &mut Buffer) {
        let fg = crate::theme::selected_foreground(theme, None);
        let bg = self.panel_overlay(theme);
        let mut style = Style::default().fg(rgba(theme.text));
        if let Some(bg) = bg {
            style = style.bg(rgba(bg));
        }
        let mut y = area.y;
        Line::styled(self.props.title.clone(), style.add_modifier(Modifier::BOLD)).render(
            Rect {
                x: area.x + PAD_X,
                y,
                width: area.width.saturating_sub(PAD_X * 2 + 5),
                height: 1,
            },
            buf,
        );
        Line::styled("esc", Style::default().fg(rgba(theme.text_muted))).render(
            Rect {
                x: area.x + area.width.saturating_sub(PAD_X + 3),
                y,
                width: 3,
                height: 1,
            },
            buf,
        );
        y += 1;
        Paragraph::new(Line::styled(
            self.props.message.clone(),
            Style::default().fg(rgba(theme.text_muted)),
        ))
        .wrap(Wrap { trim: false })
        .render(
            Rect {
                x: area.x + PAD_X,
                y,
                width: area.width.saturating_sub(PAD_X * 2),
                height: 2,
            },
            buf,
        );
        y += 2;
        if let Some(link) = self.props.link.as_ref() {
            Paragraph::new(Line::styled(
                link.clone(),
                Style::default().fg(rgba(theme.primary)),
            ))
            .render(
                Rect {
                    x: area.x + PAD_X,
                    y,
                    width: area.width.saturating_sub(PAD_X * 2),
                    height: 1,
                },
                buf,
            );
            y += 1;
        }
        // Buttons row (space-between): dismiss left, action right.
        let dismiss_active = self.selected == RetryChoice::Dismiss;
        let action_active = self.selected == RetryChoice::Action;
        let dismiss_label = "don't show again";
        let action_label = self.props.label.clone();
        let dismiss_width = (dismiss_label.chars().count() + 4) as u16;
        let action_width = (action_label.chars().count() + 4) as u16;
        render_button(
            dismiss_label,
            dismiss_active,
            fg,
            theme,
            Rect {
                x: area.x + PAD_X,
                y,
                width: dismiss_width,
                height: 1,
            },
            buf,
        );
        render_button(
            &action_label,
            action_active,
            fg,
            theme,
            Rect {
                x: area.x + area.width.saturating_sub(PAD_X + action_width),
                y,
                width: action_width,
                height: 1,
            },
            buf,
        );
    }

    fn handle_key(&mut self, key: &KeyEvent, control: &mut dyn DialogControl) -> bool {
        match key.code {
            KeyCode::Left | KeyCode::Right | KeyCode::Tab => {
                self.toggle();
                true
            }
            KeyCode::Enter => {
                match self.selected {
                    RetryChoice::Action => self.run_action(control),
                    RetryChoice::Dismiss => self.dismiss(control),
                }
                true
            }
            _ => false,
        }
    }

    fn bindings(&self) -> Vec<DialogBinding> {
        vec![
            DialogBinding {
                key: "left".to_string(),
                desc: "Previous retry option".to_string(),
                group: "Dialog".to_string(),
            },
            DialogBinding {
                key: "right".to_string(),
                desc: "Next retry option".to_string(),
                group: "Dialog".to_string(),
            },
            DialogBinding {
                key: "tab".to_string(),
                desc: "Next retry option".to_string(),
                group: "Dialog".to_string(),
            },
            DialogBinding {
                key: "return".to_string(),
                desc: "Confirm retry option".to_string(),
                group: "Dialog".to_string(),
            },
        ]
    }
}

fn render_button(label: &str, active: bool, fg: Rgba, theme: &Theme, area: Rect, buf: &mut Buffer) {
    let style = if active {
        Style::default()
            .fg(rgba(fg))
            .bg(rgba(theme.primary))
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(rgba(theme.text_muted))
    };
    Block::default()
        .style(Style::default().bg(if active {
            rgba(theme.primary)
        } else {
            rgba(Rgba::from_ints_alpha(0, 0, 0, 0))
        }))
        .render(area, buf);
    Paragraph::new(Line::styled(format!("  {label}  "), style)).render(area, buf);
}

/// Mirrors `DialogRetryAction.show`.
pub fn show_retry(stack: &mut DialogStack, props: RetryProps) -> oneshot::Receiver<bool> {
    let (tx, rx) = oneshot::channel();
    let shared: SharedDone = Arc::new(Mutex::new(Some(tx)));
    let on_close = shared.clone();
    stack.replace(
        Box::new(RetryState {
            props,
            selected: RetryChoice::Action,
            done: shared,
        }),
        Some(Box::new(move || resolve_shared(&on_close, false))),
    );
    rx
}
