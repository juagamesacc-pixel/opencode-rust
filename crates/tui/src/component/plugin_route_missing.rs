// source: packages/tui/src/component/plugin-route-missing.tsx (14 lines, v1.18.30)
// 1:1 port — centered warning + `go home` button.

#![allow(dead_code)]

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, Paragraph, Widget};

use super::super::ui::link::rgba;
use crate::theme::Theme;

/// Missing-route notice state (`go_home` flips on button click).
#[derive(Debug, Clone)]
pub struct PluginRouteMissing {
    pub id: String,
    pub go_home: bool,
}

impl PluginRouteMissing {
    /// Mirrors the `go home` mouse-up handler.
    pub fn click_home(&mut self) {
        self.go_home = true;
    }

    pub fn render(&self, theme: &Theme, area: Rect, buf: &mut Buffer) {
        let warning = Paragraph::new(Line::styled(
            format!("Unknown plugin route: {}", self.id),
            Style::default().fg(rgba(theme.warning)),
        ));
        let button = Paragraph::new(Line::styled(
            "go home",
            Style::default().fg(rgba(theme.text)),
        ))
        .block(Block::default().style(Style::default().bg(rgba(theme.background_element))));
        let cx = area.x + area.width / 2;
        let cy = area.y + area.height / 2;
        warning.render(
            Rect {
                x: cx.saturating_sub(20),
                y: cy.saturating_sub(1),
                width: 40.min(area.width),
                height: 1,
            },
            buf,
        );
        button.render(
            Rect {
                x: cx.saturating_sub(6),
                y: cy + 1,
                width: 12.min(area.width),
                height: 1,
            },
            buf,
        );
    }
}
