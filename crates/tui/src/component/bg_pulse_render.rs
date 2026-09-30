// source: packages/tui/src/component/bg-pulse-render.ts (436 lines, v1.18.30)
// 1:1 port — the GO upsell pulse painter math verbatim (ring envelope,
// logo pulse, template kinds, geometry). Output is painted cells instead
// of an OptimizedBuffer; the per-frame cache is elided (a fullscreen
// recompute is microseconds in Rust — same pixels, no frame debt).

#![allow(dead_code)]

use crate::logo::GO;
use crate::theme::Rgba;

const PERIOD: f64 = 4600.0;
const RINGS: f64 = 3.0;
const WIDTH: f64 = 3.8;
const TAIL: f64 = 9.5;
const AMP: f64 = 0.55;
const TAIL_AMP: f64 = 0.16;
const BREATH_AMP: f64 = 0.05;
const BREATH_SPEED: f64 = 0.0008;
/// Phase offset verbatim (ring emits from the GO center at shimmer peak).
const PHASE_OFFSET: f64 = 0.29;
const LOGO_GAP: usize = 1;
const LOGO_TOP_BIAS: i64 = -1;
const LOGO_LEFT_WIDTH: usize = GO.left[0].len();
const LOGO_WIDTH: usize = GO.left[0].len() + LOGO_GAP + GO.right[0].len();
const LOGO_HEIGHT: usize = 4;
const RING_SCALE: f64 = 1.0 / RINGS;
const TAIL_SCALE: f64 = 1.0 / TAIL;
const SPACE: char = ' ';
const TOP_HALF: char = '▀';
const FULL_BLOCK: char = '█';

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LogoCellKind {
    Background,
    Top,
    ShadowTop,
    Solid,
    Char,
}

#[derive(Debug, Clone)]
struct LogoTemplateCell {
    x: usize,
    y: usize,
    kind: LogoCellKind,
    ch: char,
    bold: bool,
    top_dist: f64,
    bottom_dist: f64,
}

fn logo_lines() -> [String; 4] {
    [
        format!("{}{}{}", GO.left[0], " ".repeat(LOGO_GAP), GO.right[0]),
        format!("{}{}{}", GO.left[1], " ".repeat(LOGO_GAP), GO.right[1]),
        format!("{}{}{}", GO.left[2], " ".repeat(LOGO_GAP), GO.right[2]),
        format!("{}{}{}", GO.left[3], " ".repeat(LOGO_GAP), GO.right[3]),
    ]
}

fn logo_template() -> Vec<LogoTemplateCell> {
    let mut cells = Vec::new();
    for (y, line) in logo_lines().iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == ' ' {
                continue;
            }
            let kind = match ch {
                '_' => LogoCellKind::Background,
                '^' => LogoCellKind::Top,
                '~' => LogoCellKind::ShadowTop,
                '█' => LogoCellKind::Solid,
                _ => LogoCellKind::Char,
            };
            cells.push(LogoTemplateCell {
                x,
                y,
                kind,
                ch,
                bold: x > LOGO_LEFT_WIDTH,
                top_dist: ((x as f64 + 0.5 - LOGO_WIDTH as f64 / 2.0).powi(2)
                    + (y as f64 * 2.0 - LOGO_HEIGHT as f64).powi(2))
                .sqrt(),
                bottom_dist: ((x as f64 + 0.5 - LOGO_WIDTH as f64 / 2.0).powi(2)
                    + (y as f64 * 2.0 + 1.0 - LOGO_HEIGHT as f64).powi(2))
                .sqrt(),
            });
        }
    }
    cells
}

/// Mirrors `Rgb`.
pub type Rgb = [u8; 3];

/// One painted terminal cell.
#[derive(Debug, Clone)]
pub struct PaintedCell {
    pub x: usize,
    pub y: usize,
    pub ch: char,
    pub bold: bool,
    pub fg: Rgb,
    pub bg: Rgb,
}

/// Mirrors `toRgb`.
pub fn to_rgb(color: Rgba) -> Rgb {
    [
        (color.r * 255.0).round() as u8,
        (color.g * 255.0).round() as u8,
        (color.b * 255.0).round() as u8,
    ]
}

fn clamp01(n: f64) -> f64 {
    n.clamp(0.0, 1.0)
}

fn mix_channel(base: u8, overlay: u8, alpha: f64) -> u8 {
    (base as f64 + (overlay as f64 - base as f64) * clamp01(alpha)).round() as u8
}

fn logo_tint(base: Rgb, primary: Rgb, primary_mix: f64, peak_mix: f64) -> Rgb {
    let p = clamp01(primary_mix);
    let q = clamp01(peak_mix);
    [
        mix_channel(mix_channel(base[0], primary[0], p), 255, q),
        mix_channel(mix_channel(base[1], primary[1], p), 255, q),
        mix_channel(mix_channel(base[2], primary[2], p), 255, q),
    ]
}

/// Mirrors `GoUpsellArtPainter` — geometry cached across frames, colors
/// settable (unchanged values skip invalidation, verbatim).
pub struct GoUpsellArtPainter {
    panel_rgb: Rgb,
    primary_rgb: Rgb,
    logo_base_rgb: Rgb,
    elapsed: f64,
    distances: Vec<f32>,
    edge_falloff: Vec<f32>,
    geometry_width: usize,
    geometry_height: usize,
    reach: f64,
    logo_x: usize,
    logo_y: usize,
    logo_rgb: Option<bool>,
}

impl GoUpsellArtPainter {
    pub fn new() -> Self {
        Self {
            panel_rgb: [0, 0, 0],
            primary_rgb: [255, 255, 255],
            logo_base_rgb: [180, 180, 180],
            elapsed: 0.0,
            distances: Vec::new(),
            edge_falloff: Vec::new(),
            geometry_width: 0,
            geometry_height: 0,
            reach: 1.0,
            logo_x: 0,
            logo_y: 0,
            logo_rgb: None,
        }
    }

    pub fn set_background_panel(&mut self, value: Rgba) -> bool {
        let next = to_rgb(value);
        if self.panel_rgb == next {
            return false;
        }
        self.panel_rgb = next;
        true
    }

    pub fn set_logo_base(&mut self, value: Rgba) -> bool {
        let next = to_rgb(value);
        if self.logo_base_rgb == next {
            return false;
        }
        self.logo_base_rgb = next;
        true
    }

    pub fn set_primary(&mut self, value: Rgba) -> bool {
        let next = to_rgb(value);
        if self.primary_rgb == next {
            return false;
        }
        self.primary_rgb = next;
        true
    }

    fn rebuild_geometry(&mut self, width: usize, height: usize) {
        if width == self.geometry_width && height == self.geometry_height {
            return;
        }
        self.geometry_width = width;
        self.geometry_height = height;
        self.logo_x = (width.saturating_sub(LOGO_WIDTH)) / 2;
        // logoY = clamp(round((h - LOGO_H) / 2) + BIAS, 0, h - LOGO_H)
        let centered =
            ((height as i64 - LOGO_HEIGHT as i64) as f64 / 2.0).round() as i64 + LOGO_TOP_BIAS;
        self.logo_y = centered
            .max(0)
            .min(height as i64 - LOGO_HEIGHT as i64)
            .max(0) as usize;
        let center_x = self.logo_x as f64 + LOGO_WIDTH as f64 / 2.0;
        let center_y = self.logo_y as f64 + LOGO_HEIGHT as f64 / 2.0;
        self.reach = (center_x.max(width as f64 - center_x).powi(2)
            + (center_y.max(height as f64 - center_y) * 2.0).powi(2))
        .sqrt()
            + TAIL;
        self.distances = vec![0.0; width * height];
        self.edge_falloff = vec![0.0; width * height];
        for y in 0..height {
            for x in 0..width {
                let index = y * width + x;
                let dist = ((x as f64 + 0.5 - center_x).powi(2)
                    + ((y as f64 + 0.5 - center_y) * 2.0).powi(2))
                .sqrt();
                self.distances[index] = dist as f32;
                self.edge_falloff[index] =
                    (0.0f64).max(1.0 - (dist / (self.reach * 0.85)).powi(2)) as f32;
            }
        }
    }

    /// Mirrors `render` — advances the clock and paints every cell.
    pub fn render(
        &mut self,
        width: usize,
        height: usize,
        delta_ms: f64,
        rgb: bool,
    ) -> Vec<PaintedCell> {
        self.elapsed = (self.elapsed + delta_ms) % PERIOD;
        self.rebuild_geometry(width, height);
        self.logo_rgb = Some(rgb);
        let t = self.elapsed;
        let template = logo_template();
        let logo_reach =
            ((LOGO_WIDTH as f64).powi(2) + (LOGO_HEIGHT as f64 * 2.0).powi(2)).sqrt() + 3.0;
        let mut cells = Vec::with_capacity(width * height + template.len());
        for y in 0..height {
            for x in 0..width {
                let (r, g, b) = self.background_pixel(x, y, t);
                cells.push(PaintedCell {
                    x,
                    y,
                    ch: SPACE,
                    bold: false,
                    fg: [r, g, b],
                    bg: [r, g, b],
                });
            }
        }
        for cell in &template {
            let (x, y) = (self.logo_x + cell.x, self.logo_y + cell.y);
            if x >= width || y >= height {
                continue;
            }
            let (fg, bg, ch) = self.logo_pixel(cell, t, logo_reach, rgb);
            cells.push(PaintedCell {
                x,
                y,
                ch,
                bold: cell.bold,
                fg,
                bg,
            });
        }
        cells
    }

    fn background_pixel(&self, x: usize, y: usize, t: f64) -> (u8, u8, u8) {
        let index = y * self.geometry_width + x;
        let dist = self.distances[index] as f64;
        let falloff = self.edge_falloff[index] as f64;
        let breath = (0.5 + 0.5 * (t * BREATH_SPEED).sin()) * BREATH_AMP;
        let phases = [0.0, 1.0 / RINGS, 2.0 / RINGS]
            .map(|off| ((t / PERIOD + off - PHASE_OFFSET) % 1.0 + 1.0) % 1.0);
        let mut level = 0.0;
        for phase in phases {
            let envelope = (phase * std::f64::consts::PI).sin();
            let eased = envelope * envelope * (3.0 - 2.0 * envelope);
            let head = phase * self.reach;
            let delta = dist - head;
            let crest = if delta.abs() < WIDTH {
                0.5 + 0.5 * ((delta / WIDTH) * std::f64::consts::PI).cos()
            } else {
                0.0
            };
            let tail = if delta < 0.0 && delta > -TAIL {
                (1.0 + delta * TAIL_SCALE).powf(2.3)
            } else {
                0.0
            };
            level += (crest * AMP + tail * TAIL_AMP) * eased;
        }
        let strength = ((level * RING_SCALE + breath) * falloff).min(1.0) * 0.7;
        (
            (self.panel_rgb[0] as f64
                + (self.primary_rgb[0] as f64 - self.panel_rgb[0] as f64) * strength)
                .round() as u8,
            (self.panel_rgb[1] as f64
                + (self.primary_rgb[1] as f64 - self.panel_rgb[1] as f64) * strength)
                .round() as u8,
            (self.panel_rgb[2] as f64
                + (self.primary_rgb[2] as f64 - self.panel_rgb[2] as f64) * strength)
                .round() as u8,
        )
    }

    fn logo_pulse(
        &self,
        dist: f64,
        head0: f64,
        eased0: f64,
        head1: f64,
        eased1: f64,
    ) -> (f64, f64) {
        let mut peak = 0.04;
        let mut primary = 0.0;
        for (head, eased) in [(head0, eased0), (head1, eased1)] {
            let delta = dist - head;
            let core = (-((delta.abs() / 1.2).powf(1.8))).exp();
            let soft = (-((delta.abs() / 7.0).powf(1.6))).exp();
            let tail = if delta < 0.0 && delta > -7.0 {
                (1.0 + delta / 7.0).powf(2.6)
            } else {
                0.0
            };
            peak += core * 0.65 * eased;
            primary += (soft * 0.16 + tail * 0.22) * eased;
        }
        (peak.min(1.0), primary.min(1.0))
    }

    fn logo_pixel(
        &self,
        cell: &LogoTemplateCell,
        t: f64,
        logo_reach: f64,
        rgb: bool,
    ) -> (Rgb, Rgb, char) {
        let shadow = [
            mix_channel(self.panel_rgb[0], self.logo_base_rgb[0], 0.25),
            mix_channel(self.panel_rgb[1], self.logo_base_rgb[1], 0.25),
            mix_channel(self.panel_rgb[2], self.logo_base_rgb[2], 0.25),
        ];
        let phase0 = (t / PERIOD) % 1.0;
        let phase1 = (t / PERIOD + 0.5) % 1.0;
        let eased = |phase: f64| {
            let envelope = (phase * std::f64::consts::PI).sin();
            envelope * envelope * (3.0 - 2.0 * envelope)
        };
        let (eased0, eased1) = (eased(phase0), eased(phase1));
        let (head0, head1) = (phase0 * logo_reach, phase1 * logo_reach);
        let (top_peak, top_primary) = self.logo_pulse(cell.top_dist, head0, eased0, head1, eased1);
        let (bottom_peak, bottom_primary) =
            self.logo_pulse(cell.bottom_dist, head0, eased0, head1, eased1);
        let ch = match cell.kind {
            LogoCellKind::Background => SPACE,
            LogoCellKind::Top | LogoCellKind::ShadowTop => TOP_HALF,
            LogoCellKind::Solid => {
                if rgb {
                    TOP_HALF
                } else {
                    FULL_BLOCK
                }
            }
            LogoCellKind::Char => cell.ch,
        };
        match cell.kind {
            LogoCellKind::Background => {
                let bg = logo_tint(
                    shadow,
                    self.primary_rgb,
                    0.0,
                    top_peak.max(bottom_peak) * 0.18,
                );
                (bg, bg, ch)
            }
            LogoCellKind::Top => (
                logo_tint(self.logo_base_rgb, self.primary_rgb, top_primary, top_peak),
                logo_tint(shadow, self.primary_rgb, 0.0, bottom_peak * 0.18),
                ch,
            ),
            LogoCellKind::ShadowTop => {
                let fg = logo_tint(shadow, self.primary_rgb, 0.0, top_peak * 0.18);
                (fg, shadow, ch)
            }
            LogoCellKind::Solid if rgb => (
                logo_tint(self.logo_base_rgb, self.primary_rgb, top_primary, top_peak),
                logo_tint(
                    self.logo_base_rgb,
                    self.primary_rgb,
                    bottom_primary,
                    bottom_peak,
                ),
                ch,
            ),
            _ => {
                let tinted = logo_tint(
                    self.logo_base_rgb,
                    self.primary_rgb,
                    (top_primary + bottom_primary) / 2.0,
                    (top_peak + bottom_peak) / 2.0,
                );
                (tinted, self.panel_rgb, ch)
            }
        }
    }
}

impl Default for GoUpsellArtPainter {
    fn default() -> Self {
        Self::new()
    }
}
