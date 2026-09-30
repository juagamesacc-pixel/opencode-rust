//! Rust port of `packages/app/src/utils/aim.ts` (opencode v1.18.30).
//!
//! Source 138 lines: `createAim` hover-intent state machine (locs/timer/
//! pending/over/last + `cancel`/`reset`/`move`/`wait`/`activate`/`request`/
//! `enter`/`leave`). DOM access (`getBoundingClientRect`, `setTimeout`) is
//! PROVISIONAL; the slope/window geometry math is ported verbatim.
//! Original file: `packages/app/src/utils/aim.ts`

#![allow(dead_code)]

/// Mirrors `Point`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// Mirrors the `createAim` options (`enabled`/`active`/`el` closures become
/// explicit inputs; DOM element rect is passed explicitly).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AimConfig {
    pub delay: u64,
    pub max: usize,
    pub tolerance: f64,
    pub edge: f64,
}

impl Default for AimConfig {
    fn default() -> Self {
        Self {
            delay: 250,
            max: 4,
            tolerance: 80.0,
            edge: 18.0,
        }
    }
}

/// Mirrors the `createAim` internal state.
#[derive(Debug, Clone, Default)]
pub struct AimState {
    pub locs: Vec<Point>,
    pub pending: Option<String>,
    pub over: Option<String>,
    pub last: Option<Point>,
    pub timer_armed: bool,
}

impl AimState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Mirrors `cancel()`.
    pub fn cancel(&mut self) {
        self.timer_armed = false;
        self.pending = None;
    }

    /// Mirrors `reset()`.
    pub fn reset(&mut self) {
        self.cancel();
        self.over = None;
        self.last = None;
        self.locs.clear();
    }

    /// Mirrors `move(event)` rect containment + ring-buffer push.
    /// Returns `true` when the point was recorded.
    pub fn push_point(&mut self, point: Point, rect: (f64, f64, f64, f64), max: usize) -> bool {
        let (left, right, top, bottom) = rect;
        if point.x < left || point.x > right || point.y < top || point.y > bottom {
            return false;
        }
        self.locs.push(point);
        if self.locs.len() > max {
            self.locs.remove(0);
        }
        true
    }

    /// Mirrors `wait()` — returns the activation delay ms (0 = activate now).
    pub fn wait_ms(
        &mut self,
        active: Option<&str>,
        rect: Option<(f64, f64, f64, f64)>,
        config: &AimConfig,
    ) -> u64 {
        if active.is_none() {
            return 0;
        }
        let (left, right, top, bottom) = match rect {
            None => return 0,
            Some(rect) => rect,
        };
        if self.locs.len() < 2 {
            return 0;
        }
        let loc = match self.locs.last().copied() {
            None => return 0,
            Some(loc) => loc,
        };
        let prev = self.locs.first().copied().unwrap_or(loc);
        if prev.x < left || prev.x > right || prev.y < top || prev.y > bottom {
            return 0;
        }
        if self.last == Some(loc) {
            return 0;
        }
        if right - loc.x <= config.edge {
            self.last = Some(loc);
            return config.delay;
        }
        let upper = Point {
            x: right,
            y: top - config.tolerance,
        };
        let lower = Point {
            x: right,
            y: bottom + config.tolerance,
        };
        let slope = |a: Point, b: Point| (b.y - a.y) / (b.x - a.x);
        let decreasing = slope(loc, upper);
        let increasing = slope(loc, lower);
        let prev_decreasing = slope(prev, upper);
        let prev_increasing = slope(prev, lower);
        if decreasing < prev_decreasing && increasing > prev_increasing {
            self.last = Some(loc);
            return config.delay;
        }
        self.last = None;
        0
    }
}

// PROVISIONAL: pending solid-js/dom timers (`window.setTimeout`, `MouseEvent`) — mirrors `packages/app/src/utils/aim.ts`.
/// Mirrors the `createAim` return shape (`move`/`enter`/`leave`/`activate`/
/// `request`/`cancel`/`reset`) as an explicit handle.
#[derive(Debug, Default)]
pub struct Aim {
    pub state: AimState,
    pub config: AimConfig,
}

impl Aim {
    pub fn new(config: AimConfig) -> Self {
        Self {
            state: AimState::new(),
            config,
        }
    }

    pub fn update(&mut self) -> &mut AimState {
        &mut self.state
    }

    pub fn transition_reset(&mut self) {
        self.state.reset();
    }
}
