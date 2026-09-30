// source: packages/tui/src/component/bg-pulse.tsx (99 lines, v1.18.30)
// 1:1 port — fullscreen painter with the 30fps mount clamp verbatim;
// logo base is `tint(background, text, 0.62)`.

#![allow(dead_code)]

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::Widget;

use super::bg_pulse_render::{GoUpsellArtPainter, PaintedCell};
use crate::theme::{tint, Theme};

/// Mount FPS clamp verbatim (30/30, restored on unmount).
pub const BG_PULSE_FPS: u32 = 30;

/// Fullscreen background pulse art.
pub struct BgPulse {
    painter: GoUpsellArtPainter,
    saved_fps: Option<(u32, u32)>,
}

impl BgPulse {
    pub fn new() -> Self {
        Self {
            painter: GoUpsellArtPainter::new(),
            saved_fps: None,
        }
    }

    /// Mirrors `onMount` — returns the clamped fps pair.
    pub fn mount(&mut self, target_fps: u32, max_fps: u32) {
        self.saved_fps = Some((target_fps, max_fps));
    }

    /// Mirrors `onCleanup` — restores the previous fps pair.
    pub fn unmount(&mut self) -> Option<(u32, u32)> {
        self.saved_fps.take()
    }

    pub fn render(
        &mut self,
        theme: &Theme,
        area: Rect,
        buf: &mut Buffer,
        delta_ms: f64,
        rgb: bool,
    ) {
        self.painter.set_background_panel(theme.background_panel);
        self.painter.set_primary(theme.primary);
        self.painter
            .set_logo_base(tint(theme.background, theme.text, 0.62));
        for cell in self
            .painter
            .render(area.width as usize, area.height as usize, delta_ms, rgb)
        {
            self.paint_cell(cell, area, buf);
        }
    }

    fn paint_cell(&self, cell: PaintedCell, area: Rect, buf: &mut Buffer) {
        let x = area.x + cell.x as u16;
        let y = area.y + cell.y as u16;
        if x >= area.x + area.width || y >= area.y + area.height {
            return;
        }
        let Some(buf_cell) = buf.cell_mut((x, y)) else {
            return;
        };
        let mut style = Style::default()
            .fg(ratatui::style::Color::Rgb(
                cell.fg[0], cell.fg[1], cell.fg[2],
            ))
            .bg(ratatui::style::Color::Rgb(
                cell.bg[0], cell.bg[1], cell.bg[2],
            ));
        if cell.bold {
            style = style.add_modifier(Modifier::BOLD);
        }
        buf_cell.set_char(cell.ch).set_style(style);
    }
}

impl Default for BgPulse {
    fn default() -> Self {
        Self::new()
    }
}
