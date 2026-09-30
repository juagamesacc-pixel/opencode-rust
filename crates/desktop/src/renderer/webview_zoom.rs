//! Rust port of `src/renderer/webview-zoom.ts` (opencode v1.18.30).
//!
//! Fully ported: the Tauri-copyright header (verbatim), OS detection, the
//! zoom/pinch constants, `clamp`, `normalizeWheelDelta`, the wheel-pinch
//! gesture state machine (`updateWheelPinch` decision math), the keydown
//! dispatch, and the zoom-in/out/reset arithmetic. PROVISIONAL: the Solid
//! signal, the `window.api` round-trips (`setZoomFactor` promise race,
//! `onZoomFactorChanged`, pinch-enabled fetch/subscription), the
//! `setTimeout` gesture-end timer, and the DOM `wheel`/`keydown` listeners.
//!
//! Original file: `packages/desktop/src/renderer/webview-zoom.ts`

// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

const MAX_ZOOM_LEVEL: f64 = 10.0;
const MIN_ZOOM_LEVEL: f64 = 0.2;
const WHEEL_PINCH_THRESHOLD: f64 = 20.0;
const WHEEL_PINCH_STEP: f64 = 0.2;
const WHEEL_PINCH_END_DELAY_MS: u64 = 160;
const ZOOM_STEP: f64 = 0.2;

/// Mirrors the `OS_NAME` IIFE over `navigator.userAgent`.
pub fn os_name_from_user_agent(user_agent: &str) -> &'static str {
    if user_agent.contains("Mac") {
        return "macos";
    }
    if user_agent.contains("Windows") {
        return "windows";
    }
    if user_agent.contains("Linux") {
        return "linux";
    }
    "unknown"
}

fn clamp(value: f64) -> f64 {
    value.clamp(MIN_ZOOM_LEVEL, MAX_ZOOM_LEVEL)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WheelDeltaMode {
    Pixel,
    Line,
    Page,
}

/// Mirrors `normalizeWheelDelta` (`DOM_DELTA_LINE` × 16,
/// `DOM_DELTA_PAGE` × `window.innerHeight`).
pub fn normalize_wheel_delta(delta_y: f64, mode: WheelDeltaMode, inner_height: f64) -> f64 {
    match mode {
        WheelDeltaMode::Line => delta_y * 16.0,
        WheelDeltaMode::Page => delta_y * inner_height,
        WheelDeltaMode::Pixel => delta_y,
    }
}

#[derive(Debug, Clone, Default)]
struct WheelPinch {
    active: bool,
    start_zoom: f64,
    total_delta: f64,
}

/// Mirrors the module state (`webviewZoom` signal value, `requestedZoom`,
/// `pinchZoomEnabled`, `wheelPinch`).
#[derive(Debug, Clone)]
pub struct WebviewZoomState {
    pub webview_zoom: f64,
    requested_zoom: f64,
    pinch_zoom_enabled: bool,
    wheel_pinch: Option<WheelPinch>,
}

impl WebviewZoomState {
    pub fn new() -> Self {
        Self {
            webview_zoom: 1.0,
            requested_zoom: 1.0,
            pinch_zoom_enabled: false,
            wheel_pinch: None,
        }
    }

    /// Mirrors the `onZoomFactorChanged` handler.
    pub fn on_zoom_factor_changed(&mut self, factor: f64) {
        self.requested_zoom = clamp(factor);
        self.webview_zoom = self.requested_zoom;
    }

    /// Mirrors the `getPinchZoomEnabled().then(…)` /
    /// `onPinchZoomEnabledChanged` assignments (minus the timer reset,
    /// which is `reset_wheel_pinch` below).
    pub fn set_pinch_zoom_enabled_local(&mut self, enabled: bool) {
        self.pinch_zoom_enabled = enabled;
        self.reset_wheel_pinch();
    }

    pub fn pinch_zoom_enabled(&self) -> bool {
        self.pinch_zoom_enabled
    }

    /// Mirrors `resetZoom` / `zoomIn` / `zoomOut` request values (the
    /// `window.api.setZoomFactor` round-trip is PROVISIONAL).
    pub fn reset_zoom_request(&self) -> f64 {
        1.0
    }

    pub fn zoom_in_request(&self) -> f64 {
        clamp(self.requested_zoom + ZOOM_STEP)
    }

    pub fn zoom_out_request(&self) -> f64 {
        clamp(self.requested_zoom - ZOOM_STEP)
    }

    /// Mirrors `applyZoom` bookkeeping (`requestedZoom = next`); returns
    /// the value sent to `setZoomFactor`.
    pub fn apply_zoom(&mut self, next: f64) -> f64 {
        self.requested_zoom = next;
        next
    }

    /// Mirrors the `setZoomFactor().then()` settlement: applies only when
    /// no newer request superseded it.
    pub fn settle_zoom(&mut self, next: f64) {
        if self.requested_zoom != next {
            return;
        }
        self.webview_zoom = next;
    }

    /// Mirrors the `.catch()` rollback (`requestedZoom = webviewZoom()`).
    pub fn rollback_zoom(&mut self, next: f64) {
        if self.requested_zoom != next {
            return;
        }
        self.requested_zoom = self.webview_zoom;
    }

    fn reset_wheel_pinch(&mut self) {
        // Mirrors `resetWheelPinch` minus the `clearTimeout` call, whose
        // timer handle lives in the DOM runtime (PROVISIONAL).
        self.wheel_pinch = None;
    }

    /// Mirrors `updateWheelPinch`: feeds one normalized wheel delta and
    /// returns the zoom to apply, if the gesture produced one. The
    /// `WHEEL_PINCH_END_DELAY` re-arm timer is PROVISIONAL.
    pub fn update_wheel_pinch(&mut self, delta: f64) -> Option<f64> {
        let pinch = self.wheel_pinch.get_or_insert(WheelPinch {
            active: false,
            start_zoom: self.requested_zoom,
            total_delta: 0.0,
        });
        pinch.total_delta += delta;

        if !pinch.active && pinch.total_delta.abs() < WHEEL_PINCH_THRESHOLD {
            return None;
        }
        if !pinch.active {
            pinch.active = true;
            pinch.start_zoom = self.requested_zoom;
            pinch.total_delta = 0.0;
            return None;
        }

        pinch.active = true;
        Some(clamp(
            pinch.start_zoom - (pinch.total_delta / WHEEL_PINCH_THRESHOLD) * WHEEL_PINCH_STEP,
        ))
    }
}

impl Default for WebviewZoomState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZoomKeyCommand {
    ZoomOut,
    ZoomIn,
    Reset,
}

/// Mirrors the `keydown` listener dispatch (modifier gate + `-`/`=`/`+`/`0`).
/// `os` is the `OS_NAME` value (`"macos"` uses `metaKey`, others `ctrlKey`).
pub fn zoom_key_command(
    os: &str,
    meta_key: bool,
    ctrl_key: bool,
    key: &str,
) -> Option<ZoomKeyCommand> {
    if !(if os == "macos" { meta_key } else { ctrl_key }) {
        return None;
    }
    match key {
        "-" => Some(ZoomKeyCommand::ZoomOut),
        "=" | "+" => Some(ZoomKeyCommand::ZoomIn),
        "0" => Some(ZoomKeyCommand::Reset),
        _ => None,
    }
}

// PROVISIONAL(packages/desktop/src/renderer/webview-zoom.ts): the `wheel`
// listener (`ctrlKey` gate + `preventDefault`, `{ passive: false }`), the
// `keydown` listener registration, the `WHEEL_PINCH_END_DELAY` timer, the
// Solid `webviewZoom` signal, and the `window.api` round-trips
// (`setZoomFactor` / `onZoomFactorChanged` / `getPinchZoomEnabled` /
// `onPinchZoomEnabledChanged` / `setPinchZoomEnabled`). The exported
// surface is `webviewZoom`, `resetZoom`, `setPinchZoomEnabled`, `zoomIn`,
// `zoomOut`; see `WebviewZoomState` for the portable logic.
#[allow(dead_code)]
const WHEEL_PINCH_END_DELAY_MS_ALIAS: u64 = WHEEL_PINCH_END_DELAY_MS;

#[cfg(test)]
mod tests {
    // No `src/renderer/webview-zoom.test.ts` exists in the source; the
    // cases below pin the ported gesture/keyboard math.
    use super::*;

    #[test]
    fn os_detection_matches_user_agent_tokens() {
        assert_eq!(os_name_from_user_agent("Mozilla Macintosh"), "macos");
        assert_eq!(os_name_from_user_agent("Windows NT"), "windows");
        assert_eq!(os_name_from_user_agent("Linux x86_64"), "linux");
        assert_eq!(os_name_from_user_agent("FreeBSD"), "unknown");
    }

    #[test]
    fn zoom_requests_clamp_to_min_max() {
        let mut state = WebviewZoomState::new();
        state.apply_zoom(100.0);
        assert_eq!(state.zoom_in_request(), 10.0);
        state.apply_zoom(-5.0);
        assert_eq!(state.zoom_out_request(), 0.2);
    }

    #[test]
    fn wheel_pinch_arms_then_applies_relative_to_start() {
        let mut state = WebviewZoomState::new();
        assert_eq!(state.update_wheel_pinch(10.0), None);
        // Crossing the threshold arms the gesture without applying.
        assert_eq!(state.update_wheel_pinch(15.0), None);
        // Further movement applies relative to the armed start zoom.
        assert_eq!(state.update_wheel_pinch(-20.0), Some(1.2));
    }

    #[test]
    fn zoom_key_dispatch_gates_on_platform_modifier() {
        assert_eq!(
            zoom_key_command("macos", true, false, "-"),
            Some(ZoomKeyCommand::ZoomOut)
        );
        assert_eq!(zoom_key_command("macos", false, true, "-"), None);
        assert_eq!(
            zoom_key_command("windows", false, true, "="),
            Some(ZoomKeyCommand::ZoomIn)
        );
        assert_eq!(
            zoom_key_command("windows", false, true, "+"),
            Some(ZoomKeyCommand::ZoomIn)
        );
        assert_eq!(
            zoom_key_command("linux", false, true, "0"),
            Some(ZoomKeyCommand::Reset)
        );
        assert_eq!(zoom_key_command("linux", false, true, "1"), None);
    }
}
