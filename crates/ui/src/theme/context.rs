//! Rust port of `packages/ui/src/theme/context.tsx` (opencode v1.18.30).
//!
//! 1:1 — storage keys, style id, theme display names, `normalize`, CSS text
//! assembly, and the theme/color-scheme/preview state machine are fully ported
//! as a runtime-free state machine. Solid reactivity + DOM writes are
//! `// PROVISIONAL` (no browser runtime).

#![allow(dead_code)]

use super::resolve::{resolve_theme_variant, theme_to_css};
use super::types::DesktopTheme;
use super::v2::resolve::{resolve_theme_variant_v2, theme_v2_to_css};

/// 1:1 with TS `ColorScheme`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorScheme {
    Light,
    Dark,
    #[default]
    System,
}

impl ColorScheme {
    /// 1:1 parse of the stored `opencode-color-scheme` value (fall back to `system`).
    pub fn parse(raw: Option<&str>) -> ColorScheme {
        match raw {
            Some("light") => ColorScheme::Light,
            Some("dark") => ColorScheme::Dark,
            _ => ColorScheme::System,
        }
    }

    /// 1:1 storage spelling.
    pub fn as_str(&self) -> &'static str {
        match self {
            ColorScheme::Light => "light",
            ColorScheme::Dark => "dark",
            ColorScheme::System => "system",
        }
    }
}

/// Display mode (1:1 with the `"light" | "dark"` mode used across `context.tsx`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorMode {
    Light,
    Dark,
}

impl ColorMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ColorMode::Light => "light",
            ColorMode::Dark => "dark",
        }
    }

    pub fn is_dark(&self) -> bool {
        matches!(self, ColorMode::Dark)
    }
}

/// 1:1 with TS `STORAGE_KEYS`.
pub struct StorageKeys;
impl StorageKeys {
    pub const THEME_ID: &'static str = "opencode-theme-id";
    pub const COLOR_SCHEME: &'static str = "opencode-color-scheme";
    pub const THEME_CSS_LIGHT: &'static str = "opencode-theme-css-light";
    pub const THEME_CSS_DARK: &'static str = "opencode-theme-css-dark";
}

/// 1:1 with TS `THEME_STYLE_ID` in `context.tsx`.
pub const THEME_STYLE_ID: &str = "oc-theme";

/// 1:1 with TS `names` (source order = sorted theme ids).
pub const THEME_NAMES: &[(&str, &str)] = &[
    ("oc-2", "OC-2"),
    ("amoled", "AMOLED"),
    ("aura", "Aura"),
    ("ayu", "Ayu"),
    ("carbonfox", "Carbonfox"),
    ("catppuccin", "Catppuccin"),
    ("catppuccin-frappe", "Catppuccin Frappe"),
    ("catppuccin-macchiato", "Catppuccin Macchiato"),
    ("cobalt2", "Cobalt2"),
    ("cursor", "Cursor"),
    ("dracula", "Dracula"),
    ("everforest", "Everforest"),
    ("flexoki", "Flexoki"),
    ("github", "GitHub"),
    ("gruvbox", "Gruvbox"),
    ("kanagawa", "Kanagawa"),
    ("lucent-orng", "Lucent Orng"),
    ("material", "Material"),
    ("matrix", "Matrix"),
    ("mercury", "Mercury"),
    ("monokai", "Monokai"),
    ("nightowl", "Night Owl"),
    ("nord", "Nord"),
    ("one-dark", "One Dark"),
    ("onedarkpro", "One Dark Pro"),
    ("opencode", "OpenCode"),
    ("orng", "Orng"),
    ("osaka-jade", "Osaka Jade"),
    ("palenight", "Palenight"),
    ("rosepine", "Rose Pine"),
    ("shadesofpurple", "Shades of Purple"),
    ("solarized", "Solarized"),
    ("synthwave84", "Synthwave '84"),
    ("tokyonight", "Tokyonight"),
    ("vercel", "Vercel"),
    ("vesper", "Vesper"),
    ("zenburn", "Zenburn"),
];

/// 1:1 with TS `normalize` (`oc-1` migrates to `oc-2`).
pub fn normalize(id: Option<&str>) -> Option<String> {
    match id {
        Some("oc-1") => Some("oc-2".to_string()),
        Some(other) => Some(other.to_string()),
        None => None,
    }
}

/// Display name for a theme id (1:1 with the `name` accessor: store name, then `names`, then id).
pub fn theme_display_name<'a>(store_name: Option<&'a str>, id: &'a str) -> &'a str {
    if let Some(name) = store_name {
        return name;
    }
    for (key, name) in THEME_NAMES {
        if *key == id {
            return name;
        }
    }
    id
}

/// 1:1 with TS `applyThemeCss` CSS text (the `:root` payload written to the style element).
pub fn apply_theme_css_text(theme: &DesktopTheme, mode: ColorMode) -> String {
    let is_dark = mode.is_dark();
    let variant = if is_dark { &theme.dark } else { &theme.light };
    let tokens = resolve_theme_variant(variant, is_dark);
    let css = theme_to_css(&tokens);
    let v2 = theme_v2_to_css(&resolve_theme_variant_v2(variant, is_dark));

    format!(
        ":root {{\n  color-scheme: {};\n  --text-mix-blend-mode: {};\n  {css}\n  {v2}\n}}",
        mode.as_str(),
        if is_dark { "plus-lighter" } else { "multiply" },
    )
}

/// 1:1 with TS `cacheThemeVariants` — cached CSS text per mode.
pub fn cache_theme_variants_css(theme: &DesktopTheme) -> [(ColorMode, String); 2] {
    [ColorMode::Light, ColorMode::Dark].map(|mode| {
        let is_dark = mode.is_dark();
        let variant = if is_dark { &theme.dark } else { &theme.light };
        let tokens = resolve_theme_variant(variant, is_dark);
        let css = theme_to_css(&tokens);
        let v2 = theme_v2_to_css(&resolve_theme_variant_v2(variant, is_dark));
        (mode, format!("{css}\n  {v2}"))
    })
}

/// 1:1 with the `Theme` store shape from `createSimpleContext({ name: "Theme", init })`.
#[derive(Debug, Clone)]
pub struct ThemeState {
    pub theme_id: String,
    pub color_scheme: ColorScheme,
    pub mode: ColorMode,
    pub preview_theme_id: Option<String>,
    pub preview_scheme: Option<ColorScheme>,
}

impl Default for ThemeState {
    fn default() -> Self {
        ThemeState {
            theme_id: "oc-2".to_string(),
            color_scheme: ColorScheme::System,
            mode: ColorMode::Light,
            preview_theme_id: None,
            preview_scheme: None,
        }
    }
}

impl ThemeState {
    /// 1:1 with the store init: stored theme id → default → `oc-2`; stored scheme → `system`.
    pub fn init(
        stored_theme_id: Option<&str>,
        default_theme: Option<&str>,
        stored_scheme: Option<&str>,
        system: ColorMode,
    ) -> ThemeState {
        let theme_id =
            normalize(stored_theme_id.or(default_theme)).unwrap_or_else(|| "oc-2".to_string());
        let color_scheme = ColorScheme::parse(stored_scheme);
        let mode = match color_scheme {
            ColorScheme::System => system,
            ColorScheme::Light => ColorMode::Light,
            ColorScheme::Dark => ColorMode::Dark,
        };
        ThemeState {
            theme_id,
            color_scheme,
            mode,
            preview_theme_id: None,
            preview_scheme: None,
        }
    }

    /// 1:1 with `setTheme` validation (returns the normalized id or `None` when unknown).
    pub fn set_theme(&mut self, id: &str, known: &dyn Fn(&str) -> bool) -> Option<String> {
        let next = normalize(Some(id))?;
        if next != "oc-2" && !known(&next) {
            return None;
        }
        self.theme_id = next.clone();
        Some(next)
    }

    /// 1:1 with `setColorScheme`.
    pub fn set_color_scheme(&mut self, scheme: ColorScheme, system: ColorMode) {
        self.color_scheme = scheme;
        self.mode = match scheme {
            ColorScheme::System => system,
            ColorScheme::Light => ColorMode::Light,
            ColorScheme::Dark => ColorMode::Dark,
        };
    }

    /// 1:1 with `commitPreview`.
    pub fn commit_preview(&mut self, known: &dyn Fn(&str) -> bool, system: ColorMode) {
        if let Some(id) = self.preview_theme_id.clone() {
            self.set_theme(&id, known);
        }
        if let Some(scheme) = self.preview_scheme {
            self.set_color_scheme(scheme, system);
        }
        self.preview_theme_id = None;
        self.preview_scheme = None;
    }

    /// 1:1 with `cancelPreview` (state reset; the re-apply runs in the host).
    pub fn cancel_preview(&mut self) {
        self.preview_theme_id = None;
        self.preview_scheme = None;
    }

    /// 1:1 with `previewColorScheme` mode math.
    pub fn preview_mode(&self, scheme: ColorScheme, system: ColorMode) -> ColorMode {
        match scheme {
            ColorScheme::System => system,
            ColorScheme::Light => ColorMode::Light,
            ColorScheme::Dark => ColorMode::Dark,
        }
    }
}

// PROVISIONAL: Solid store/effects (`createStore`, `createEffect`, `onMount`),
// `import.meta.glob("./themes/*.json")` lazy loading, `localStorage` persistence,
// `matchMedia` listeners, and DOM writes (`oc-theme` style element,
// `document.documentElement.dataset`, `theme-color` meta) need the browser/Solid
// runtime. Port the host wiring there; keep the state math above verbatim.
