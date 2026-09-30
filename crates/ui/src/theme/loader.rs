//! Rust port of `packages/ui/src/theme/loader.ts` (opencode v1.18.30).
//!
//! 1:1 — DOM/style-element operations are `// PROVISIONAL` (no browser runtime);
//! CSS assembly (`buildThemeCss`) and id rules are fully ported.

#![allow(dead_code)]

use super::resolve::{resolve_theme_variant, theme_to_css};
use super::types::{DesktopTheme, ResolvedTheme, ResolvedV2Theme};
use super::v2::resolve::{resolve_theme_variant_v2, theme_v2_to_css};

/// 1:1 with TS `THEME_STYLE_ID` in `loader.ts`.
pub const THEME_STYLE_ID: &str = "opencode-theme";

/// 1:1 with TS `buildThemeCss` — assemble loader CSS for both modes.
/// The default theme (`oc-2`) targets `:root`; others target `html[data-theme="…"]`.
pub fn build_theme_css(
    light: &ResolvedTheme,
    dark: &ResolvedTheme,
    light_v2: &ResolvedV2Theme,
    dark_v2: &ResolvedV2Theme,
    theme_id: &str,
) -> String {
    let is_default_theme = theme_id == "oc-2";
    let light_css = format!("{}\n  {}", theme_to_css(light), theme_v2_to_css(light_v2));
    let dark_css = format!("{}\n  {}", theme_to_css(dark), theme_v2_to_css(dark_v2));

    if is_default_theme {
        return format!(
            "\n:root {{\n  color-scheme: light;\n  --text-mix-blend-mode: multiply;\n\n  {light_css}\n\n  @media (prefers-color-scheme: dark) {{\n    color-scheme: dark;\n    --text-mix-blend-mode: plus-lighter;\n\n    {dark_css}\n  }}\n}}\n"
        );
    }

    format!(
        "\nhtml[data-theme=\"{theme_id}\"] {{\n  color-scheme: light;\n  --text-mix-blend-mode: multiply;\n\n  {light_css}\n\n  @media (prefers-color-scheme: dark) {{\n    color-scheme: dark;\n    --text-mix-blend-mode: plus-lighter;\n\n    {dark_css}\n  }}\n}}\n"
    )
}

/// 1:1 with the token-resolution half of TS `applyTheme` (CSS text only).
pub fn apply_theme_css(theme: &DesktopTheme, theme_id: Option<&str>) -> String {
    let light_tokens = resolve_theme_variant(&theme.light, false);
    let dark_tokens = resolve_theme_variant(&theme.dark, true);
    let light_v2_tokens = resolve_theme_variant_v2(&theme.light, false);
    let dark_v2_tokens = resolve_theme_variant_v2(&theme.dark, true);
    let target_theme_id = theme_id.unwrap_or(&theme.id).to_string();
    build_theme_css(
        &light_tokens,
        &dark_tokens,
        &light_v2_tokens,
        &dark_v2_tokens,
        &target_theme_id,
    )
}

/// 1:1 with the error shape of TS `loadThemeFromUrl` failure.
pub fn load_theme_from_url_error(url: &str, status_text: &str) -> String {
    format!("Failed to load theme from {url}: {status_text}")
}

// PROVISIONAL: DOM operations (`document.getElementById`, `document.createElement`,
// `document.documentElement.setAttribute`, `localStorage`, `fetch`) have no Rust
// runtime here. `ensureLoaderStyleElement`, `applyTheme` (DOM writes),
// `loadThemeFromUrl` (fetch), `getActiveTheme`, `removeTheme`, and
// `setColorScheme` must run in the host web runtime.
