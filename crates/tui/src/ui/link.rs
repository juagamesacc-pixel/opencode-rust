// source: packages/tui/src/ui/link.tsx (34 lines, v1.18.30)
// 1:1 port — hyperlink text; click opens the URL via the platform opener
// (`open` crate has no dependency-free equivalent; failures are silent).

#![allow(dead_code)]

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Paragraph, Widget, Wrap};

use crate::theme::Rgba;

pub fn rgba(color: Rgba) -> Color {
    Color::Rgb(
        (color.r * 255.0) as u8,
        (color.g * 255.0) as u8,
        (color.b * 255.0) as u8,
    )
}

/// Mirrors `LinkProps`.
#[derive(Debug, Clone, Default)]
pub struct LinkProps {
    pub href: String,
    pub text: Option<String>,
    pub fg: Option<Rgba>,
    pub bg: Option<Rgba>,
    pub wrap: bool,
}

/// Mirrors the `open(props.href).catch(() => {})` click handler.
pub fn open_url(href: &str) {
    let opener: &[&str] = if cfg!(target_os = "macos") {
        &["open"]
    } else if cfg!(target_os = "windows") {
        &["cmd", "/c", "start"]
    } else {
        &["xdg-open"]
    };
    let _ = std::process::Command::new(opener[0])
        .args(&opener[1..])
        .arg(href)
        .spawn();
}

/// Render the link text (mirrors the `<text>` body; click-through is
/// handled by the app mouse layer calling `open_url`).
pub fn render_link(props: &LinkProps, area: Rect, buf: &mut Buffer) {
    let mut style = Style::default().underlined();
    if let Some(fg) = props.fg {
        style = style.fg(rgba(fg));
    }
    if let Some(bg) = props.bg {
        style = style.bg(rgba(bg));
    }
    let mut paragraph = Paragraph::new(Line::styled(
        props.text.clone().unwrap_or_else(|| props.href.clone()),
        style,
    ));
    if props.wrap {
        paragraph = paragraph.wrap(Wrap { trim: false });
    }
    paragraph.render(area, buf);
}
