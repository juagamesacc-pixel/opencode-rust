// source: packages/tui/src/component/todo-item.tsx (32 lines, v1.18.30)
// 1:1 port — `[✓]`/`[•]`/`[ ]` markers and warning/muted colors verbatim.

#![allow(dead_code)]

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget, Wrap};

use super::super::ui::link::rgba;
use crate::theme::Theme;

/// Mirrors `TodoItemProps`.
#[derive(Debug, Clone)]
pub struct TodoItemProps {
    pub status: String,
    pub content: String,
}

/// Todo row render.
pub fn render_todo_item(props: &TodoItemProps, theme: &Theme, area: Rect, buf: &mut Buffer) {
    let color = if props.status == "in_progress" {
        theme.warning
    } else {
        theme.text_muted
    };
    let marker = if props.status == "completed" {
        "✓"
    } else if props.status == "in_progress" {
        "•"
    } else {
        " "
    };
    let line = Line::from(vec![
        Span::styled(format!("[{marker}] "), Style::default().fg(rgba(color))),
        Span::styled(props.content.clone(), Style::default().fg(rgba(color))),
    ]);
    Paragraph::new(line)
        .wrap(Wrap { trim: false })
        .render(area, buf);
}
