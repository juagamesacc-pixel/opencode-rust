//! Rust port of `src/renderer/index.tsx` (opencode v1.18.30).
//!
//! The Solid render tree (`DesktopRoot`, `App`, `Inner`, `LoadingSplash`,
//! `DesktopMemoryRouter`), Sentry setup, `createPlatform`'s `window.api`
//! wiring, and all resource/subscription bootstrapping are PROVISIONAL
//! (Solid/DOM/bridge runtime). Fully ported: the deep-link event name and
//! `emitDeepLinks` queue semantics, the last-active-URL key format and
//! validation, OS detection (which returns `undefined` here, unlike
//! `webview-zoom`'s `"unknown"`), the default storage name, the zoom-action
//! interception in `runDesktopMenuAction`, and the literal constants
//! (notification icon, pasted-image naming, release prefix).
//!
//! Original file: `packages/desktop/src/renderer/index.tsx`

use crate::preload::types::DesktopMenuAction;

pub const DEEP_LINK_EVENT: &str = "opencode:deep-link";
pub const DEFAULT_STORAGE_NAME: &str = "default.dat";
pub const GLOBAL_SETTINGS_STORE: &str = "opencode.global.dat";
pub const LANGUAGE_KEY: &str = "language";
pub const LEGACY_LANGUAGE_KEY: &str = "language.v1";
pub const NOTIFICATION_ICON_URL: &str = "https://opencode.ai/favicon-96x96-v3.png";
pub const PASTED_IMAGE_MIME: &str = "image/png";
pub const PASTED_IMAGE_PREFIX: &str = "pasted-image-";
pub const PASTED_IMAGE_SUFFIX: &str = ".png";
pub const DESKTOP_VERSION: &str = "1.18.30";
pub const SENTRY_RELEASE_PREFIX: &str = "desktop@";

/// Mirrors `window.__OPENCODE__?: { deepLinks?: string[] }` (see
/// `src/renderer/env.d.ts`).
#[derive(Debug, Clone, Default)]
pub struct OpencodeWindowState {
    pub deep_links: Vec<String>,
}

pub struct DeepLinkEvent {
    pub detail_urls: Vec<String>,
}

/// Mirrors `emitDeepLinks`: empty batches are dropped; otherwise pending
/// links accumulate on `window.__OPENCODE__.deepLinks` and a
/// `CustomEvent(DEEP_LINK_EVENT, { detail: { urls } })` is dispatched.
pub fn emit_deep_links(
    state: &mut OpencodeWindowState,
    urls: Vec<String>,
) -> Option<DeepLinkEvent> {
    if urls.is_empty() {
        return None;
    }
    state.deep_links.extend(urls.iter().cloned());
    Some(DeepLinkEvent { detail_urls: urls })
}

/// Mirrors `windowLastActiveUrlKey(windowID)`.
pub fn window_last_active_url_key(window_id: &str) -> String {
    format!("opencode.desktop.window.{}.last-active-url", window_id)
}

/// Mirrors the `getLastActiveUrl` validation over an injected stored value
/// (`localStorage` access itself is PROVISIONAL): only a `/…` value that
/// does not start with `//` is reused, otherwise `"/"`.
pub fn resolve_last_active_url(stored: Option<&str>) -> String {
    match stored {
        Some(value) if value.starts_with('/') && !value.starts_with("//") => value.to_string(),
        _ => "/".to_string(),
    }
}

/// Mirrors the `os` IIFE in `createPlatform` (`undefined` when unmatched).
pub fn detect_platform_os(user_agent: &str) -> Option<&'static str> {
    if user_agent.contains("Mac") {
        return Some("macos");
    }
    if user_agent.contains("Windows") {
        return Some("windows");
    }
    if user_agent.contains("Linux") {
        return Some("linux");
    }
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RendererZoomCommand {
    Reset,
    ZoomIn,
    ZoomOut,
}

/// Mirrors the zoom interception at the top of the platform's
/// `runDesktopMenuAction` (anything else falls through to
/// `window.api.runDesktopMenuAction`, PROVISIONAL).
pub fn renderer_zoom_command(action: DesktopMenuAction) -> Option<RendererZoomCommand> {
    match action {
        DesktopMenuAction::ViewResetZoom => Some(RendererZoomCommand::Reset),
        DesktopMenuAction::ViewZoomIn => Some(RendererZoomCommand::ZoomIn),
        DesktopMenuAction::ViewZoomOut => Some(RendererZoomCommand::ZoomOut),
        _ => None,
    }
}

/// Mirrors the `openPath` Windows branch decision: only on Windows is
/// `resolveAppPath` consulted before `openPath`.
pub fn resolves_app_before_open(os: Option<&str>) -> bool {
    os == Some("windows")
}

/// Mirrors the pasted-image filename (`pasted-image-${Date.now()}.png`).
pub fn pasted_image_name(now_ms: u64) -> String {
    format!("{}{}{}", PASTED_IMAGE_PREFIX, now_ms, PASTED_IMAGE_SUFFIX)
}

/// Mirrors the locale-extraction regex in `loadLocale`:
/// `/"locale"\s*:\s*"([^"]+)"/` over the stored language payload.
pub fn extract_stored_locale(raw: &str) -> Option<String> {
    let key = raw.find("\"locale\"")?;
    let after_key = &raw[key + "\"locale\"".len()..];
    let colon = after_key.find(':')?;
    let mut value = after_key[colon + 1..].trim_start();
    // Mirrors `\s*` (ASCII whitespace subset sufficient for JSON output).
    while value.starts_with([' ', '\t', '\n', '\r']) {
        value = &value[1..];
    }
    let quoted = value.strip_prefix('"')?;
    let end = quoted.find('"')?;
    Some(quoted[..end].to_string())
}

// PROVISIONAL(packages/desktop/src/renderer/index.tsx): the Solid render
// tree (`render(…)` over `DesktopRoot`/`App`/`Inner`/`LoadingSplash`/
// `DesktopMemoryRouter`), `createPlatform`'s `window.api` wiring
// (storage/draftStore/updater/pickers/clipboard/notify/fetch/servers),
// Sentry init (`VITE_SENTRY_DSN`, `desktop@${pkg.version}` release,
// `Breadcrumbs`/`GlobalHandlers`/`BrowserApiErrors` filtering with the
// `OPENCODE_CHANNEL === "prod"` gate), `listenForDeepLinks`, the
// `onMenuCommand` → `menuTrigger` chain, and the dev root check
// (`t("desktop.error.dev.rootNotFound")`). Preserved literal constants
// are the `pub const` items above.

#[cfg(test)]
mod tests {
    // No `src/renderer/index.test.ts` exists in the source; the cases
    // below pin the ported pure helpers to the source's inline behavior.
    use super::*;

    #[test]
    fn emit_deep_links_drops_empty_batches_and_queues() {
        let mut state = OpencodeWindowState::default();
        assert!(emit_deep_links(&mut state, Vec::new()).is_none());
        let event = emit_deep_links(&mut state, vec!["opencode://a".to_string()]).expect("event");
        assert_eq!(event.detail_urls, vec!["opencode://a".to_string()]);
        assert_eq!(state.deep_links, vec!["opencode://a".to_string()]);
    }

    #[test]
    fn last_active_url_validation_rejects_non_paths() {
        assert_eq!(resolve_last_active_url(Some("/session/1")), "/session/1");
        assert_eq!(resolve_last_active_url(Some("//evil")), "/");
        assert_eq!(resolve_last_active_url(Some("https://x")), "/");
        assert_eq!(resolve_last_active_url(None), "/");
        assert_eq!(
            window_last_active_url_key("abc"),
            "opencode.desktop.window.abc.last-active-url"
        );
    }

    #[test]
    fn zoom_actions_are_intercepted_before_ipc() {
        assert_eq!(
            renderer_zoom_command(DesktopMenuAction::ViewResetZoom),
            Some(RendererZoomCommand::Reset)
        );
        assert_eq!(renderer_zoom_command(DesktopMenuAction::EditCopy), None);
    }

    #[test]
    fn stored_locale_extraction_reads_the_locale_field() {
        assert_eq!(
            extract_stored_locale(r#"{"locale" : "de"}"#),
            Some("de".to_string())
        );
        assert_eq!(extract_stored_locale("{}"), None);
    }
}
