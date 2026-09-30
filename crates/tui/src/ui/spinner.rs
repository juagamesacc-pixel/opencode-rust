// source: packages/tui/src/ui/spinner.ts (368 lines, v1.18.30)
// 1:1 port — Knight Rider scanner math verbatim (state machine, color
// index, trail derivation, frame generation). Rendering consumes frames.

#![allow(dead_code)]

use crate::theme::Rgba;

/// Mirrors the scan direction union.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanDirection {
    Forward,
    Backward,
    Bidirectional,
}

/// Mirrors `ScannerState`.
#[derive(Debug, Clone, Copy)]
pub struct ScannerState {
    pub active_position: usize,
    pub is_holding: bool,
    pub hold_progress: usize,
    pub hold_total: usize,
    pub movement_progress: usize,
    pub movement_total: usize,
    pub is_moving_forward: bool,
}

/// Mirrors `getScannerState`.
pub fn get_scanner_state(
    frame_index: usize,
    total_chars: usize,
    direction: ScanDirection,
    hold_start: usize,
    hold_end: usize,
) -> ScannerState {
    match direction {
        ScanDirection::Bidirectional => {
            let forward_frames = total_chars;
            let backward_frames = total_chars.saturating_sub(1);
            if frame_index < forward_frames {
                ScannerState {
                    active_position: frame_mod(frame_index, total_chars),
                    is_holding: false,
                    hold_progress: 0,
                    hold_total: 0,
                    movement_progress: frame_index,
                    movement_total: forward_frames,
                    is_moving_forward: true,
                }
            } else if frame_index < forward_frames + hold_end {
                ScannerState {
                    active_position: total_chars.saturating_sub(1),
                    is_holding: true,
                    hold_progress: frame_index - forward_frames,
                    hold_total: hold_end,
                    movement_progress: 0,
                    movement_total: 0,
                    is_moving_forward: true,
                }
            } else if frame_index < forward_frames + hold_end + backward_frames {
                let backward_index = frame_index - forward_frames - hold_end;
                ScannerState {
                    active_position: total_chars.saturating_sub(2).saturating_sub(backward_index),
                    is_holding: false,
                    hold_progress: 0,
                    hold_total: 0,
                    movement_progress: backward_index,
                    movement_total: backward_frames,
                    is_moving_forward: false,
                }
            } else {
                ScannerState {
                    active_position: 0,
                    is_holding: true,
                    hold_progress: frame_index - forward_frames - hold_end - backward_frames,
                    hold_total: hold_start,
                    movement_progress: 0,
                    movement_total: 0,
                    is_moving_forward: false,
                }
            }
        }
        ScanDirection::Backward => ScannerState {
            active_position: total_chars
                .saturating_sub(1)
                .saturating_sub(frame_index % total_chars.max(1)),
            is_holding: false,
            hold_progress: 0,
            hold_total: 0,
            movement_progress: frame_index % total_chars.max(1),
            movement_total: total_chars,
            is_moving_forward: false,
        },
        ScanDirection::Forward => ScannerState {
            active_position: frame_index % total_chars.max(1),
            is_holding: false,
            hold_progress: 0,
            hold_total: 0,
            movement_progress: frame_index % total_chars.max(1),
            movement_total: total_chars,
            is_moving_forward: true,
        },
    }
}

fn frame_mod(frame_index: usize, total_chars: usize) -> usize {
    frame_index % total_chars.max(1)
}

/// Trail configuration (mirrors the `Pick<AdvancedGradientOptions>` args).
#[derive(Debug, Clone, Copy)]
pub struct TrailOptions {
    pub direction: ScanDirection,
    pub hold_start: usize,
    pub hold_end: usize,
    pub trail_length: usize,
}

/// Mirrors `calculateColorIndex` — gradient-trail color slot per char
/// (`-1` renders the default color).
#[allow(clippy::too_many_arguments)]
pub fn calculate_color_index(
    frame_index: usize,
    char_index: usize,
    total_chars: usize,
    options: &TrailOptions,
    state: Option<ScannerState>,
) -> i64 {
    let state = state.unwrap_or_else(|| {
        get_scanner_state(
            frame_index,
            total_chars,
            options.direction,
            options.hold_start,
            options.hold_end,
        )
    });
    let directional_distance = if state.is_moving_forward {
        state.active_position as i64 - char_index as i64
    } else {
        char_index as i64 - state.active_position as i64
    };
    if state.is_holding {
        return directional_distance + state.hold_progress as i64;
    }
    if directional_distance > 0 && (directional_distance as usize) < options.trail_length {
        return directional_distance;
    }
    if directional_distance == 0 {
        return 0;
    }
    -1
}

/// Mirrors `deriveTrailColors` (alpha falloff + i==1 bloom, verbatim).
pub fn derive_trail_colors(bright: Rgba, steps: usize) -> Vec<Rgba> {
    let mut colors = Vec::with_capacity(steps);
    for i in 0..steps {
        let (alpha, brightness) = if i == 0 {
            (1.0, 1.0)
        } else if i == 1 {
            (0.9, 1.15)
        } else {
            (0.65f32.powi((i - 1) as i32), 1.0)
        };
        colors.push(Rgba {
            r: (bright.r * brightness).min(1.0),
            g: (bright.g * brightness).min(1.0),
            b: (bright.b * brightness).min(1.0),
            a: alpha,
        });
    }
    colors
}

/// Mirrors `deriveInactiveColor`.
pub fn derive_inactive_color(bright: Rgba, factor: f32) -> Rgba {
    Rgba {
        r: bright.r,
        g: bright.g,
        b: bright.b,
        a: factor,
    }
}

/// Mirrors `KnightRiderStyle`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KnightRiderStyle {
    Blocks,
    Diamonds,
}

/// Mirrors `KnightRiderOptions` (defaults verbatim).
#[derive(Debug, Clone)]
pub struct KnightRiderOptions {
    pub width: usize,
    pub style: KnightRiderStyle,
    pub hold_start: usize,
    pub hold_end: usize,
    pub colors: Vec<Rgba>,
    pub default_color: Rgba,
    pub enable_fading: bool,
    pub min_alpha: f32,
}

impl Default for KnightRiderOptions {
    fn default() -> Self {
        Self {
            width: 8,
            style: KnightRiderStyle::Diamonds,
            hold_start: 30,
            hold_end: 9,
            colors: [
                "#ff0000", "#ff5555", "#dd0000", "#aa0000", "#770000", "#440000",
            ]
            .iter()
            .map(|hex| Rgba::from_hex(hex))
            .collect(),
            default_color: Rgba::from_hex("#330000"),
            enable_fading: true,
            min_alpha: 0.0,
        }
    }
}

/// Mirrors `createFrames` — one string per animation frame.
pub fn create_frames(options: &KnightRiderOptions) -> Vec<String> {
    let width = options.width;
    let trail = TrailOptions {
        direction: ScanDirection::Bidirectional,
        hold_start: options.hold_start,
        hold_end: options.hold_end,
        trail_length: options.colors.len(),
    };
    // Forward (width) + hold end + backward (width-1) + hold start.
    let total_frames = width + options.hold_end + width.saturating_sub(1) + options.hold_start;
    (0..total_frames)
        .map(|frame_index| {
            (0..width)
                .map(|char_index| {
                    let index = calculate_color_index(frame_index, char_index, width, &trail, None);
                    match options.style {
                        KnightRiderStyle::Diamonds => {
                            const SHAPES: [&str; 4] = ["⬥", "◆", "⬩", "⬪"];
                            if index >= 0 && (index as usize) < options.colors.len() {
                                SHAPES[(index as usize).min(SHAPES.len() - 1)]
                            } else {
                                "·"
                            }
                        }
                        KnightRiderStyle::Blocks => {
                            if index >= 0 && (index as usize) < options.colors.len() {
                                "■"
                            } else {
                                "⬝"
                            }
                        }
                    }
                })
                .collect::<Vec<_>>()
                .join("")
        })
        .collect()
}

/// Mirrors the `ColorGenerator` closure — per-frame color of one char,
/// with the frame-state cache and hold/movement fading verbatim.
pub struct KnightRiderColors {
    options: KnightRiderOptions,
    cached_frame: Option<usize>,
    cached_state: Option<ScannerState>,
    base_inactive_alpha: f32,
}

impl KnightRiderColors {
    pub fn new(options: KnightRiderOptions) -> Self {
        let base_inactive_alpha = options.default_color.a;
        Self {
            options,
            cached_frame: None,
            cached_state: None,
            base_inactive_alpha,
        }
    }

    pub fn color_at(&mut self, frame_index: usize, char_index: usize, total_chars: usize) -> Rgba {
        if self.cached_frame != Some(frame_index) {
            self.cached_frame = Some(frame_index);
            self.cached_state = Some(get_scanner_state(
                frame_index,
                total_chars,
                ScanDirection::Bidirectional,
                self.options.hold_start,
                self.options.hold_end,
            ));
        }
        let state = self.cached_state.unwrap();
        let trail = TrailOptions {
            direction: ScanDirection::Bidirectional,
            hold_start: self.options.hold_start,
            hold_end: self.options.hold_end,
            trail_length: self.options.colors.len(),
        };
        let index =
            calculate_color_index(frame_index, char_index, total_chars, &trail, Some(state));
        let mut fade = 1.0f32;
        if self.options.enable_fading {
            if state.is_holding && state.hold_total > 0 {
                let progress = (state.hold_progress as f32 / state.hold_total as f32).min(1.0);
                fade = self
                    .options
                    .min_alpha
                    .max(1.0 - progress * (1.0 - self.options.min_alpha));
            } else if !state.is_holding && state.movement_total > 0 {
                let progress = (state.movement_progress as f32
                    / (state.movement_total.saturating_sub(1).max(1) as f32))
                    .min(1.0);
                fade = self.options.min_alpha + progress * (1.0 - self.options.min_alpha);
            }
        }
        let mut inactive = self.options.default_color;
        inactive.a = self.base_inactive_alpha * fade;
        if index == -1 {
            return inactive;
        }
        self.options
            .colors
            .get(index as usize)
            .copied()
            .unwrap_or(inactive)
    }
}
