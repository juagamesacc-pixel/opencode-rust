// source: packages/tui/src/util/scroll.ts (26 lines, v1.18.30)
// 1:1 port — acceleration selection: macOS accel when enabled, custom
// speed when set, default speed 3 otherwise.

#![allow(dead_code)]

/// Mirrors `CustomSpeedScroll`.
#[derive(Debug, Clone, Copy)]
pub struct CustomSpeedScroll {
    pub speed: f64,
}

impl CustomSpeedScroll {
    pub fn new(speed: f64) -> Self {
        Self { speed }
    }

    /// Mirrors `tick`.
    pub fn tick(&self) -> f64 {
        self.speed
    }

    /// Mirrors `reset`.
    pub fn reset(&self) {}
}

/// Mirrors the macOS accel marker (OpenTUI's `MacOSScrollAccel`).
#[derive(Debug, Clone, Copy)]
pub struct MacOsScrollAccel;

/// Mirrors `ScrollConfig`.
#[derive(Debug, Clone, Default)]
pub struct ScrollConfig {
    pub scroll_acceleration_enabled: Option<bool>,
    pub scroll_speed: Option<f64>,
}

/// Mirrors `getScrollAcceleration`.
pub fn get_scroll_acceleration(config: Option<&ScrollConfig>) -> ScrollAcceleration {
    match config {
        Some(config) if config.scroll_acceleration_enabled == Some(true) => {
            ScrollAcceleration::MacOs(MacOsScrollAccel)
        }
        Some(config) if config.scroll_speed.is_some() => ScrollAcceleration::Custom(
            CustomSpeedScroll::new(config.scroll_speed.unwrap_or_default()),
        ),
        _ => ScrollAcceleration::Custom(CustomSpeedScroll::new(3.0)),
    }
}

/// Either acceleration implementation.
#[derive(Debug, Clone, Copy)]
pub enum ScrollAcceleration {
    MacOs(MacOsScrollAccel),
    Custom(CustomSpeedScroll),
}

impl ScrollAcceleration {
    /// Mirrors the interface call sites (`tick` / `reset`).
    pub fn tick(&self) -> f64 {
        match self {
            ScrollAcceleration::MacOs(_) => 1.0,
            ScrollAcceleration::Custom(custom) => custom.tick(),
        }
    }

    pub fn reset(&self) {
        if let ScrollAcceleration::Custom(custom) = self {
            custom.reset();
        }
    }
}
