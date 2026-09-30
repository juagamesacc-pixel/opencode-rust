// source: packages/tui/src/component/workspace-label.tsx (19 lines, v1.18.30)
// 1:1 port — `● name (type)` spans; connected→success, error→error,
// otherwise textMuted; name always text.

#![allow(dead_code)]

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget};

use super::super::ui::link::rgba;
use crate::context::project::WorkspaceStatus;
use crate::theme::Theme;

/// Workspace label render.
pub fn render_workspace_label(
    theme: &Theme,
    label_type: &str,
    name: &str,
    status: Option<WorkspaceStatus>,
    icon: bool,
    area: Rect,
    buf: &mut Buffer,
) {
    let color = match status {
        Some(WorkspaceStatus::Connected) => theme.success,
        Some(WorkspaceStatus::Error) => theme.error,
        _ => theme.text_muted,
    };
    let mut spans = Vec::new();
    if icon {
        spans.push(Span::styled("● ", Style::default().fg(rgba(color))));
    }
    spans.push(Span::styled(
        name.to_string(),
        Style::default().fg(rgba(theme.text)),
    ));
    spans.push(Span::styled(
        format!(" ({label_type})"),
        Style::default().fg(rgba(theme.text_muted)),
    ));
    Paragraph::new(Line::from(spans)).render(area, buf);
}
