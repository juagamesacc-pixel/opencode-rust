// source: packages/tui/src/component/logo.tsx (61 lines, v1.18.30)
// 1:1 port — per-char mapping verbatim (`_` bg-space, `^` fg-on-shadow ▀,
// `~` shadow ▀, `,` shadow ▄); left half textMuted flat, right half text
// bold; rows joined with a 1-cell gap.

#![allow(dead_code)]

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::text::Span;
use ratatui::widgets::{Paragraph, Widget};

use super::super::ui::link::rgba;
use crate::logo::LOGO;
use crate::theme::{tint, Rgba, Theme};

fn render_line(line: &str, fg: Rgba, shadow: Rgba, bold: bool) -> Vec<Span<'static>> {
    let mut style = Style::default();
    if bold {
        style = style.add_modifier(Modifier::BOLD);
    }
    line.chars()
        .map(|ch| match ch {
            '_' => Span::styled(" ", style.fg(rgba(fg)).bg(rgba(shadow))),
            '^' => Span::styled("▀", style.fg(rgba(fg)).bg(rgba(shadow))),
            '~' => Span::styled("▀", style.fg(rgba(shadow))),
            ',' => Span::styled("▄", style.fg(rgba(shadow))),
            _ => Span::styled(ch.to_string(), style.fg(rgba(fg))),
        })
        .collect()
}

/// Logo render (4 rows; left textMuted, right text bold).
pub fn render_logo(theme: &Theme, area: Rect, buf: &mut Buffer) {
    let mut lines = Vec::new();
    for index in 0..LOGO.left.len() {
        let left_fg = theme.text_muted;
        let right_fg = theme.text;
        let mut spans = render_line(
            LOGO.left[index],
            left_fg,
            tint(theme.background, left_fg, 0.25),
            false,
        );
        spans.push(Span::raw(" "));
        spans.extend(render_line(
            LOGO.right[index],
            right_fg,
            tint(theme.background, right_fg, 0.25),
            true,
        ));
        lines.push(Line::from(spans));
    }
    Paragraph::new(lines).render(area, buf);
}
