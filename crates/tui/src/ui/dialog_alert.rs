// source: packages/tui/src/ui/dialog-alert.tsx (66 lines, v1.18.30)
// 1:1 port — layout (paddings, right-aligned `ok` button, `esc` hint) and
// the return-key binding are verbatim; `show()` resolves a future.

#![allow(dead_code)]

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Paragraph, Widget, Wrap};
use tokio::sync::oneshot;

use super::dialog::{DialogBinding, DialogContent, DialogControl, DialogStack};
use super::link::rgba;
use crate::theme::Theme;
use std::sync::{Arc, Mutex};

/// Mirrors `DialogAlertProps`.
#[derive(Debug, Clone)]
pub struct AlertProps {
    pub title: String,
    pub message: String,
}

/// Alert dialog state.
pub struct AlertState {
    props: AlertProps,
    done: SharedDone,
}

type SharedDone = Arc<Mutex<Option<oneshot::Sender<()>>>>;

fn resolve_shared(shared: &SharedDone) {
    if let Ok(mut guard) = shared.lock() {
        if let Some(tx) = guard.take() {
            let _ = tx.send(());
        }
    }
}

impl AlertState {
    fn confirm(&mut self, control: &mut dyn DialogControl) {
        resolve_shared(&self.done);
        control.clear();
    }
}

impl DialogContent for AlertState {
    fn render(&self, theme: &Theme, area: Rect, buf: &mut Buffer) {
        let title = Line::styled(
            self.props.title.clone(),
            Style::default()
                .fg(rgba(theme.text))
                .add_modifier(Modifier::BOLD),
        );
        let hint = Line::styled("esc", Style::default().fg(rgba(theme.text_muted)));
        let title_width = area.width.saturating_sub(4);
        let hint_width = 3u16;
        title.render(
            Rect {
                x: area.x + 2,
                y: area.y,
                width: title_width.saturating_sub(hint_width + 1),
                height: 1,
            },
            buf,
        );
        hint.render(
            Rect {
                x: area.x + area.width.saturating_sub(2 + hint_width),
                y: area.y,
                width: hint_width,
                height: 1,
            },
            buf,
        );
        let message = Paragraph::new(self.props.message.clone())
            .style(Style::default().fg(rgba(theme.text_muted)))
            .wrap(Wrap { trim: false });
        message.render(
            Rect {
                x: area.x + 2,
                y: area.y + 1,
                width: area.width.saturating_sub(4),
                height: area.height.saturating_sub(3),
            },
            buf,
        );
        let button = Paragraph::new(Line::styled(
            "ok",
            Style::default().fg(rgba(theme.selected_list_item_text)),
        ))
        .block(Block::default().style(Style::default().bg(rgba(theme.primary))));
        let button_width = 3 + 3 + 2;
        button.render(
            Rect {
                x: area.x + area.width.saturating_sub(2 + button_width),
                y: area.y + area.height.saturating_sub(2),
                width: button_width,
                height: 1,
            },
            buf,
        );
    }

    fn handle_key(&mut self, key: &KeyEvent, control: &mut dyn DialogControl) -> bool {
        if key.code == KeyCode::Enter {
            self.confirm(control);
            return true;
        }
        false
    }

    fn bindings(&self) -> Vec<DialogBinding> {
        vec![DialogBinding {
            key: "return".to_string(),
            desc: "Confirm alert".to_string(),
            group: "Dialog".to_string(),
        }]
    }
}

/// Mirrors `DialogAlert.show` — replaces the stack, resolves on confirm
/// or on dialog close (both resolve, matching the two `resolve()` calls).
pub fn show_alert(stack: &mut DialogStack, title: &str, message: &str) -> oneshot::Receiver<()> {
    let (tx, rx) = oneshot::channel();
    let shared: SharedDone = Arc::new(Mutex::new(Some(tx)));
    let on_close = shared.clone();
    stack.replace(
        Box::new(AlertState {
            props: AlertProps {
                title: title.to_string(),
                message: message.to_string(),
            },
            done: shared,
        }),
        Some(Box::new(move || resolve_shared(&on_close))),
    );
    rx
}
