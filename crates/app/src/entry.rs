//! Rust port of `packages/app/src/entry.tsx` (opencode v1.18.30).
//!
//! Source 180 lines: web entrypoint wiring `PlatformProvider` +
//! `AppBaseProviders` + `AppInterface`, locale detection, localStorage
//! default-server helpers, notification/external/restart adapters, Sentry
//! init, auth-token bootstrap. DOM/SolidJS render is PROVISIONAL; pure
//! helpers and verbatim literals are fully ported.
//! Original file: `packages/app/src/entry.tsx`

#![allow(dead_code)]

/// Mirrors `DEFAULT_SERVER_URL_KEY`.
pub const DEFAULT_SERVER_URL_KEY: &str = "opencode.settings.dat:defaultServerUrl";

/// Mirrors the `error.dev.rootNotFound` i18n key used for the missing-root error.
pub const ROOT_NOT_FOUND_KEY: &str = "error.dev.rootNotFound";

/// Mirrors the notification icon URL.
pub const NOTIFICATION_ICON_URL: &str = "https://opencode.ai/favicon-96x96-v3.png";

/// Mirrors the web `Platform` identity (`platform: "web"`).
pub const PLATFORM_ID: &str = "web";

/// Mirrors `getLocale()` — `zh` prefix maps to `"zh"`, everything else `"en"`.
pub fn detect_locale(languages: &[String]) -> &'static str {
    for language in languages {
        if language.to_lowercase().starts_with("zh") {
            return "zh";
        }
    }
    "en"
}

/// Mirrors `getRootNotFoundError()` locale selection (zh falls back to en).
pub fn root_not_found_key(locale: &str) -> &'static str {
    let _ = locale;
    ROOT_NOT_FOUND_KEY
}

/// Mirrors `getCurrentUrl()` host selection.
/// `is_opencode_host`: `location.hostname.includes("opencode.ai")`.
/// `dev`: `import.meta.env.DEV`; host/port mirror `VITE_OPENCODE_SERVER_*` defaults.
pub fn current_url(
    is_opencode_host: bool,
    dev: bool,
    host: Option<&str>,
    port: Option<&str>,
    origin: &str,
) -> String {
    if is_opencode_host {
        return "http://localhost:4096".to_string();
    }
    if dev {
        let h = host.unwrap_or("localhost");
        let p = port.unwrap_or("4096");
        return format!("http://{h}:{p}");
    }
    origin.to_string()
}

/// Mirrors `clearAuthToken()` query cleanup — returns the cleaned query string.
pub fn strip_auth_token_param(query: &str) -> String {
    let mut params: Vec<(&str, &str)> = query
        .split('&')
        .filter_map(|pair| {
            if pair.is_empty() {
                return None;
            }
            let mut split = pair.splitn(2, '=');
            Some((split.next().unwrap_or(""), split.next().unwrap_or("")))
        })
        .filter(|(k, _)| *k != "auth_token")
        .collect();
    let _ = &mut params;
    params
        .iter()
        .map(|(k, v)| {
            if v.is_empty() {
                k.to_string()
            } else {
                format!("{k}={v}")
            }
        })
        .collect::<Vec<_>>()
        .join("&")
}

/// Mirrors `openExternal` protocol allowlist (`http:`, `https:`, `mailto:`).
pub fn is_external_url_allowed(protocol: &str) -> bool {
    matches!(protocol, "http:" | "https:" | "mailto:")
}

// PROVISIONAL: pending solid-js/tauri/vite runtime — mirrors `packages/app/src/entry.tsx` render wiring.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntryBootstrap {
    pub locale: String,
    pub default_url: String,
    pub has_auth: bool,
    pub disable_health_check: bool,
}

impl EntryBootstrap {
    pub fn new(locale: &str, default_url: &str, has_auth: bool) -> Self {
        Self {
            locale: locale.to_string(),
            default_url: default_url.to_string(),
            has_auth,
            disable_health_check: true,
        }
    }
}
