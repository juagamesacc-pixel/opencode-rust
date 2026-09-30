//! Rust port of `src/main/windows.ts` (opencode v1.18.30).
//!
//! Fully ported: the protocol/permission/policy constants, the titlebar
//! overlay math (`overlay`), `clampZoom` (NaN-preserving, as in JS),
//! `windowStateFile`/`windowDataFile` naming, `isRendererUrl`,
//! `addRendererHeaders`/`upsertKeyValue`, the `oc://` traversal guard, the
//! dev/prod load-URL selection, the recovery action lists, the
//! load-failure gate (`errorCode === -3`), and the terminal console-message
//! filter. The `PINCH_ZOOM_ENABLED_KEY` store read/write of
//! `getPinchZoomEnabled`/`setPinchZoomEnabled` is real; the per-window
//! `BrowserWindow` fan-out stays PROVISIONAL (`set_pinch_zoom_fanout`).
//! PROVISIONAL: everything else holding a `BrowserWindow`
//! (`createMainWindow`, titlebar/zoom/fullscreen wiring, protocol
//! registration, permission handlers, recovery dialogs, `shell`/`dialog`
//! calls, the window registry instance, and all module-global state).
//! The `oc-2.json` theme background (`oc2Background`) is owned by
//! `@opencode-ai/ui` and has no in-workspace binding.
//!
//! Original file: `packages/desktop/src/main/windows.ts`

use crate::preload::types::{TitlebarMode, TitlebarTheme};
use std::collections::HashMap;

pub const RENDERER_PROTOCOL: &str = "oc";
pub const RENDERER_HOST: &str = "renderer";
pub const CLIPBOARD_WRITE_PERMISSION: &str = "clipboard-sanitized-write";
pub const NOTIFICATION_PERMISSION: &str = "notifications";
pub const DOCUMENT_POLICY_HEADER: &str = "Document-Policy";
pub const JS_CALL_STACKS_DOCUMENT_POLICY: &str = "include-js-call-stacks-in-crash-reports";
pub const TITLEBAR_HEIGHT: u64 = 40;
pub const MAX_ZOOM_LEVEL: f64 = 10.0;
pub const MIN_ZOOM_LEVEL: f64 = 0.2;
pub const ZOOM_STEP: f64 = 0.2;
pub const DEFAULT_WINDOW_WIDTH: u64 = 1280;
pub const DEFAULT_WINDOW_HEIGHT: u64 = 800;
pub const TRAFFIC_LIGHT_X: u64 = 14;
pub const TRAFFIC_LIGHT_Y: u64 = 14;
pub const WINDOW_TITLE: &str = "OpenCode";
pub const RENDERER_ENTRY: &str = "index.html";
pub const PRELOAD_ENTRY: &str = "../preload/index.js";
pub const DEV_RENDERER_URL_ENV: &str = "ELECTRON_RENDERER_URL";
pub const EVENT_PINCH_ZOOM_ENABLED_CHANGED: &str = "pinch-zoom-enabled-changed";
pub const EVENT_ZOOM_FACTOR_CHANGED: &str = "zoom-factor-changed";
pub const EVENT_WINDOW_FULLSCREEN_CHANGED: &str = "window-fullscreen-changed";
pub const EVENT_MENU_COMMAND: &str = "menu-command";
pub const EVENT_DEEP_LINK: &str = "deep-link";
pub const ACCESS_CONTROL_ALLOW_ORIGIN: &str = "Access-Control-Allow-Origin";
pub const ACCESS_CONTROL_ALLOW_HEADERS: &str = "Access-Control-Allow-Headers";
pub const ICON_PNG: &str = "icon.png";
pub const ICON_ICO: &str = "icon.ico";
pub const DOCK_ICON: &str = "dock.png";

pub struct TitlebarOverlay {
    pub color: String,
    pub symbol_color: String,
    pub height: u64,
}

/// Mirrors `overlay(theme = {}, zoom = 1)`: transparent color, symbol
/// color from the resolved mode, height scaled by zoom (min 40).
pub fn overlay(mode: Option<TitlebarMode>, tone_dark: bool, zoom: f64) -> TitlebarOverlay {
    let dark = match mode {
        Some(TitlebarMode::Dark) => true,
        Some(TitlebarMode::Light) => false,
        None => tone_dark,
    };
    TitlebarOverlay {
        color: "#00000000".to_string(),
        symbol_color: if dark {
            "white".to_string()
        } else {
            "black".to_string()
        },
        height: (TITLEBAR_HEIGHT as f64).max((TITLEBAR_HEIGHT as f64 * zoom).round()) as u64,
    }
}

/// Mirrors `clampZoom`, preserving JS `NaN` propagation (`Math.min`/`max`
/// yield `NaN` for `NaN` input; Rust's `clamp` would panic instead).
pub fn clamp_zoom(value: f64) -> f64 {
    if value.is_nan() {
        return f64::NAN;
    }
    value.max(MIN_ZOOM_LEVEL).min(MAX_ZOOM_LEVEL)
}

/// Mirrors the `window-state-….json` sanitization.
pub fn sanitize_window_id(id: &str) -> String {
    id.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-' {
                c
            } else {
                '-'
            }
        })
        .collect()
}

pub fn window_state_file(id: &str) -> String {
    format!("window-state-{}.json", sanitize_window_id(id))
}

// Mirrors windowStorage() in packages/app/src/utils/persist.ts, which names
// the per-window renderer store this window persists its tabs into.
pub fn window_data_file(id: &str) -> String {
    format!("opencode.window.{}.dat", sanitize_window_id(id))
}

fn split_url(value: &str) -> Option<(String, String, String)> {
    let colon = value.find(':')?;
    let scheme = value[..colon].to_ascii_lowercase();
    let after = value.get(colon + 1..)?;
    if after.bytes().any(|byte| byte <= 0x20 || byte == 0x7f) {
        return None;
    }
    let (authority, path) = match after.strip_prefix("//") {
        Some(rest) => {
            let end = rest.find(['/', '?', '#']).unwrap_or(rest.len());
            (rest[..end].to_string(), rest[end..].to_string())
        }
        None => (String::new(), after.to_string()),
    };
    Some((scheme, authority, path))
}

fn origin_of(scheme: &str, authority: &str) -> Option<String> {
    if authority.is_empty() {
        return None;
    }
    let host = authority
        .rsplit('@')
        .next()
        .unwrap_or("")
        .to_ascii_lowercase();
    Some(format!("{}://{}", scheme, host))
}

/// Mirrors `isRendererUrl(value?, html = false)`: `oc://renderer` URLs
/// (with the `.html` gate when `html`), or the same origin as
/// `ELECTRON_RENDERER_URL` when set and parseable.
pub fn is_renderer_url(value: Option<&str>, html: bool, dev_url: Option<&str>) -> bool {
    let Some(value) = value else {
        return false;
    };
    let Some((scheme, authority, path)) = split_url(value) else {
        return false;
    };
    if html {
        let path_end = path.find(['?', '#']).unwrap_or(path.len());
        if !path[..path_end].ends_with(".html") {
            return false;
        }
    }
    // Non-special schemes keep host case (`oc://Renderer` ≠ renderer).
    if scheme == RENDERER_PROTOCOL && authority == RENDERER_HOST {
        return true;
    }
    let Some(dev_url) = dev_url else {
        return false;
    };
    let (dev_scheme, dev_authority, _) = match split_url(dev_url) {
        Some(parts) => parts,
        None => return false,
    };
    match (
        origin_of(&scheme, &authority),
        origin_of(&dev_scheme, &dev_authority),
    ) {
        (Some(left), Some(right)) => left == right,
        _ => false,
    }
}

/// Mirrors `upsertKeyValue`: case-insensitive key replace preserving the
/// original key, otherwise insert at the end.
pub fn upsert_header_value(
    headers: &mut HashMap<String, Vec<String>>,
    key: &str,
    value: Vec<String>,
) {
    let lower = key.to_ascii_lowercase();
    for existing in headers.keys().cloned().collect::<Vec<_>>() {
        if existing.to_ascii_lowercase() == lower {
            headers.insert(existing, value);
            return;
        }
    }
    headers.insert(key.to_string(), value);
}

/// Mirrors `addRendererHeaders`: CORS wildcards always, plus the
/// Document-Policy JS-call-stacks header for renderer HTML.
pub fn add_renderer_headers(
    value: &str,
    headers: &mut HashMap<String, Vec<String>>,
    dev_url: Option<&str>,
) {
    upsert_header_value(headers, ACCESS_CONTROL_ALLOW_ORIGIN, vec!["*".to_string()]);
    upsert_header_value(headers, ACCESS_CONTROL_ALLOW_HEADERS, vec!["*".to_string()]);
    if is_renderer_url(Some(value), true, dev_url) {
        upsert_header_value(
            headers,
            DOCUMENT_POLICY_HEADER,
            vec![JS_CALL_STACKS_DOCUMENT_POLICY.to_string()],
        );
    }
}

/// Mirrors `loadWindow`: dev servers load `new URL(html, devUrl)`,
/// packaged builds load `oc://renderer/{html}`.
pub fn load_window_url(dev_url: Option<&str>, html: &str) -> String {
    match dev_url {
        Some(dev_url) => {
            let base = dev_url.trim_end_matches('/');
            if html.contains("://") {
                return html.to_string();
            }
            format!("{}/{}", base, html.trim_start_matches('/'))
        }
        None => format!("{}://{}/{}", RENDERER_PROTOCOL, RENDERER_HOST, html),
    }
}

fn percent_decode_path(input: &str) -> Option<String> {
    let bytes = input.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            if index + 2 >= bytes.len() {
                return None;
            }
            let hex = std::str::from_utf8(&bytes[index + 1..index + 3]).ok()?;
            out.push(u8::from_str_radix(hex, 16).ok()?);
            index += 3;
        } else {
            out.push(bytes[index]);
            index += 1;
        }
    }
    String::from_utf8(out).ok()
}

/// Mirrors the `oc://` traversal guard in `registerRendererProtocol`:
/// the decoded pathname is resolved against the renderer root and
/// rejected when it escapes (`rel` starts with `..`).
pub fn protocol_file_allowed(renderer_root: &str, pathname: &str) -> bool {
    let decoded = match percent_decode_path(pathname) {
        Some(decoded) => decoded,
        // Mirrors `decodeURIComponent` throwing → the `catch` 404.
        None => return false,
    };
    let dotted = format!(".{}", decoded);
    let mut stack: Vec<&str> = renderer_root
        .split('/')
        .filter(|part| !part.is_empty())
        .collect();
    let root_len = stack.len();
    for part in dotted.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                if stack.pop().is_none() || stack.len() < root_len {
                    return false;
                }
            }
            _ => stack.push(part),
        }
    }
    stack.len() >= root_len
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryAction {
    Relaunch,
    ExportLogs,
    KeepWaiting,
    Quit,
}

/// Mirrors the recovery dialog button lists (`wait` selects the waiting
/// variant with keep-waiting, otherwise the quit variant).
pub fn recovery_actions(wait: bool) -> Vec<RecoveryAction> {
    if wait {
        vec![
            RecoveryAction::Relaunch,
            RecoveryAction::ExportLogs,
            RecoveryAction::KeepWaiting,
        ]
    } else {
        vec![
            RecoveryAction::Relaunch,
            RecoveryAction::ExportLogs,
            RecoveryAction::Quit,
        ]
    }
}

/// Mirrors the `failed()` gate: only main-frame failures other than
/// `errorCode === -3` (aborted) surface the dialog.
pub fn should_show_load_failed(is_main_frame: bool, error_code: i32) -> bool {
    is_main_frame && error_code != -3
}

/// Mirrors the `console-message` filter: only terminal-related messages
/// are forwarded to the `pty` log scope.
pub fn is_terminal_console_message(message: &str, source_id: &str) -> bool {
    message.to_ascii_lowercase().contains("terminal")
        || source_id.to_ascii_lowercase().contains("terminal")
}

// PROVISIONAL(packages/desktop/src/main/windows.ts): the Electron
// `BrowserWindow` handle plus every function holding one.
pub struct BrowserWindowHandle {
    _private: (),
}

/// Fan-out seam for `setPinchZoomEnabled`'s `getAllWindows()` loop.
thread_local! {
    static PINCH_ZOOM_FANOUT: std::cell::RefCell<Option<Box<dyn FnMut(bool)>>> =
        const { std::cell::RefCell::new(None) };
}

pub fn set_relaunch_handler(_handler: Box<dyn Fn()>) {
    unimplemented!("module-global Electron state binding")
}

pub fn set_app_quitting(_quitting: bool) {
    unimplemented!("module-global window registry binding")
}

pub fn set_background_color(_color: &str) {
    unimplemented!("Electron BrowserWindow binding")
}

pub fn get_background_color() -> Option<String> {
    unimplemented!("module-global Electron state binding")
}

pub fn set_titlebar(_win: &BrowserWindowHandle, _theme: &TitlebarTheme) {
    unimplemented!("Electron BrowserWindow + nativeTheme binding")
}

pub fn update_titlebar(_win: &mut dyn crate::main::desktop_menu_actions::MenuWindow) {
    unimplemented!("Electron BrowserWindow binding")
}

/// `export function setPinchZoomEnabled(enabled)` — the store write is real;
/// the per-window `WeakMap` mirror + `webContents.send` fan-out is PROVISIONAL
/// through [`set_pinch_zoom_fanout`].
pub fn set_pinch_zoom_enabled(enabled: bool) {
    // `getStore().set(PINCH_ZOOM_ENABLED_KEY, enabled)`
    if let Ok(store) = super::store::get_store(None) {
        store.borrow_mut().set(
            super::store_keys::PINCH_ZOOM_ENABLED_KEY,
            serde_json::Value::Bool(enabled),
        );
    }
    // PROVISIONAL: `getAllWindows().forEach((win) => { pinchZoomEnabled.set(win,
    // enabled); win.webContents.send("pinch-zoom-enabled-changed", enabled);
    // if (!enabled && zoomFactor !== 1) setZoomFactor(1); updateZoom(win) })`.
    let mut owned = PINCH_ZOOM_FANOUT.borrow_mut().take();
    if let Some(fanout) = owned.as_mut() {
        fanout(enabled);
    }
    *PINCH_ZOOM_FANOUT.borrow_mut() = owned;
}

/// `getStore().get(PINCH_ZOOM_ENABLED_KEY) === true`
pub fn get_pinch_zoom_enabled() -> bool {
    // PROVISIONAL: `getStore()` needs the Electron app binding; a missing
    // store reads falsy, matching `undefined === true → false`.
    match super::store::get_store(None) {
        Ok(store) => pinch_zoom_enabled_from(
            store
                .borrow()
                .get(super::store_keys::PINCH_ZOOM_ENABLED_KEY)
                .as_ref(),
        ),
        Err(_) => false,
    }
}

/// `value === true` for the stored `PINCH_ZOOM_ENABLED_KEY` value.
pub fn pinch_zoom_enabled_from(value: Option<&serde_json::Value>) -> bool {
    value == Some(&serde_json::Value::Bool(true))
}

/// Installs the optional per-window fan-out seam for
/// [`set_pinch_zoom_enabled`] (the source's `getAllWindows()` loop).
pub fn set_pinch_zoom_fanout(fanout: impl FnMut(bool) + 'static) {
    *PINCH_ZOOM_FANOUT.borrow_mut() = Some(Box::new(fanout));
}

pub fn get_window_id(_win: &BrowserWindowHandle) -> Option<String> {
    unimplemented!("module-global window id map binding")
}

pub fn get_last_focused_window() {
    unimplemented!("Electron BrowserWindow binding")
}

pub fn restore_main_windows() {
    unimplemented!("window registry + BrowserWindow binding")
}

pub fn set_dock_icon() {
    unimplemented!("Electron nativeImage + dock binding")
}

pub fn create_main_window() -> BrowserWindowHandle {
    unimplemented!("Electron BrowserWindow + electron-window-state binding")
}

pub fn open_external_url(_value: &str) {
    unimplemented!("Electron shell binding")
}

pub fn open_local_file_url(_value: &str) {
    unimplemented!("Electron shell binding")
}

pub fn register_renderer_protocol() {
    unimplemented!("Electron protocol + net binding")
}

pub fn send_menu_command(_id: &str) {
    unimplemented!("Electron webContents.send binding")
}

pub fn send_deep_links(_urls: &[String]) {
    unimplemented!("Electron webContents.send binding")
}

#[cfg(test)]
mod tests {
    // No `src/main/windows.test.ts` exists in the source; the cases below
    // pin the ported pure helpers to the source's inline behavior.
    use super::*;

    #[test]
    fn overlay_scales_height_and_symbol_color() {
        let overlay = overlay(Some(TitlebarMode::Dark), false, 1.0);
        assert_eq!(overlay.color, "#00000000");
        assert_eq!(overlay.symbol_color, "white");
        assert_eq!(overlay.height, 40);
        assert_eq!(overlay(None, false, 1.5).height, 60);
        assert_eq!(overlay(None, true, 0.5).symbol_color, "white");
        assert_eq!(overlay(None, false, 0.0).height, 40);
    }

    #[test]
    fn window_file_names_sanitize_ids() {
        assert_eq!(window_state_file("abc"), "window-state-abc.json");
        assert_eq!(window_state_file("a/b:c"), "window-state-a-b-c.json");
        assert_eq!(window_data_file("abc"), "opencode.window.abc.dat");
    }

    #[test]
    fn renderer_url_policy_matches_protocol_or_dev_origin() {
        assert!(is_renderer_url(
            Some("oc://renderer/index.html"),
            false,
            None
        ));
        assert!(is_renderer_url(
            Some("oc://renderer/index.html"),
            true,
            None
        ));
        assert!(!is_renderer_url(Some("oc://renderer/app.js"), true, None));
        assert!(!is_renderer_url(Some("https://example.com/"), false, None));
        assert!(is_renderer_url(
            Some("http://localhost:3000/x"),
            false,
            Some("http://localhost:3000/")
        ));
        assert!(!is_renderer_url(None, false, None));
    }

    #[test]
    fn header_upsert_replaces_case_insensitively() {
        let mut headers = HashMap::from([(
            "access-control-allow-origin".to_string(),
            vec!["old".to_string()],
        )]);
        upsert_header_value(
            &mut headers,
            "Access-Control-Allow-Origin",
            vec!["*".to_string()],
        );
        assert_eq!(headers.len(), 1);
        assert_eq!(
            headers.get("access-control-allow-origin"),
            Some(&vec!["*".to_string()])
        );
    }

    #[test]
    fn protocol_guard_rejects_traversal() {
        assert!(protocol_file_allowed("/app/renderer", "/index.html"));
        assert!(!protocol_file_allowed("/app/renderer", "/../secret.dat"));
        assert!(!protocol_file_allowed(
            "/app/renderer",
            "/%2e%2e/secret.dat"
        ));
        assert!(!protocol_file_allowed("/app/renderer", "/%zz"));
    }

    #[test]
    fn recovery_gates_match_source() {
        assert_eq!(
            recovery_actions(true),
            vec![
                RecoveryAction::Relaunch,
                RecoveryAction::ExportLogs,
                RecoveryAction::KeepWaiting,
            ]
        );
        assert!(should_show_load_failed(true, -100));
        assert!(!should_show_load_failed(true, -3));
        assert!(!should_show_load_failed(false, -100));
        assert!(is_terminal_console_message("Terminal output", "x"));
        assert!(!is_terminal_console_message("hello", "world"));
    }

    #[test]
    fn pinch_zoom_enabled_strictly_compares_true() {
        // `getStore().get(PINCH_ZOOM_ENABLED_KEY) === true`
        assert!(pinch_zoom_enabled_from(Some(&serde_json::Value::Bool(
            true
        ))));
        assert!(!pinch_zoom_enabled_from(Some(&serde_json::Value::Bool(
            false
        ))));
        assert!(!pinch_zoom_enabled_from(Some(&serde_json::Value::String(
            "true".to_string()
        ))));
        assert!(!pinch_zoom_enabled_from(None));
    }

    #[test]
    fn pinch_zoom_set_fans_out_to_all_windows() {
        // `getAllWindows().forEach(...)` — the seam receives the value once.
        let mut seen = None;
        set_pinch_zoom_fanout(|enabled| seen = Some(enabled));
        set_pinch_zoom_enabled(true);
        assert_eq!(seen, Some(true));
    }

    #[test]
    fn pinch_zoom_get_reads_falsy_without_store() {
        // PROVISIONAL store binding → `undefined === true` is false.
        assert!(!get_pinch_zoom_enabled());
    }
}
