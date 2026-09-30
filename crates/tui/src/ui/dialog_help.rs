// source: packages/tui/src/ui/dialog-help.tsx (40 lines, v1.18.30)
// 1:1 port — return/escape close, palette-shortcut hint line, `ok` button.

#![allow(dead_code)]

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Paragraph, Widget, Wrap};

use super::dialog::{DialogBinding, DialogContent, DialogControl};
use super::link::rgba;
use crate::theme::Theme;

/// Help dialog state (the palette shortcut arrives pre-resolved).
pub struct HelpState {
    pub palette_shortcut: String,
}

impl DialogContent for HelpState {
    fn render(&self, theme: &Theme, area: Rect, buf: &mut Buffer) {
        Line::styled(
            "Help",
            Style::default()
                .fg(rgba(theme.text))
                .add_modifier(Modifier::BOLD),
        )
        .render(
            Rect {
                x: area.x + 2,
                y: area.y,
                width: area.width.saturating_sub(13),
                height: 1,
            },
            buf,
        );
        Line::styled("esc/enter", Style::default().fg(rgba(theme.text_muted))).render(
            Rect {
                x: area.x + area.width.saturating_sub(11),
                y: area.y,
                width: 9,
                height: 1,
            },
            buf,
        );
        Paragraph::new(format!(
            "Press {} to see all available actions and commands in any context.",
            self.palette_shortcut
        ))
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
        let button = Paragraph::new(Line::styled(
            "ok",
            Style::default().fg(rgba(theme.selected_list_item_text)),
        ))
        .block(Block::default().style(Style::default().bg(rgba(theme.primary))));
        button.render(
            Rect {
                x: area.x + area.width.saturating_sub(2 + 8),
                y: area.y + area.height.saturating_sub(2),
                width: 8,
                height: 1,
            },
            buf,
        );
    }

    fn handle_key(&mut self, key: &KeyEvent, control: &mut dyn DialogControl) -> bool {
        if key.code == KeyCode::Enter || key.code == KeyCode::Esc {
            control.clear();
            return true;
        }
        false
    }

    fn bindings(&self) -> Vec<DialogBinding> {
        vec![
            DialogBinding {
                key: "return".to_string(),
                desc: "Close help".to_string(),
                group: "Dialog".to_string(),
            },
            DialogBinding {
                key: "escape".to_string(),
                desc: "Close help".to_string(),
                group: "Dialog".to_string(),
            },
        ]
    }
}
