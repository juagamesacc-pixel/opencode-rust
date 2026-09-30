//! Rust port of `src/theme/index.ts` (opencode v1.18.30).
//!
//! Theme types, RGBA color, theme resolution with circular-ref detection,
//! the theme store (defaults + plugin + custom + system), ANSI→RGBA mapping,
//! system theme generation, and syntax highlighting rules.
//!
//! Original file: `packages/tui/src/theme/index.ts`

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// RGBA
// ---------------------------------------------------------------------------

/// RGBA color with float components in 0..1 range (mirrors `RGBA` from `@opentui/core`).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Rgba {
    /// Create from 0-255 integer components.
    pub fn from_ints(r: u8, g: u8, b: u8) -> Self {
        Self::from_ints_alpha(r, g, b, 255)
    }

    /// Create from 0-255 integer components with alpha.
    pub fn from_ints_alpha(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    /// Create from a hex color string (`#rgb`, `#rrggbb`, or `#rrggbbaa`).
    pub fn from_hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');
        let expanded;
        let hex = if hex.len() == 3 {
            expanded = hex.chars().flat_map(|c| [c, c]).collect::<String>();
            expanded.as_str()
        } else {
            hex
        };
        let byte = |range: std::ops::Range<usize>, fallback: u8| {
            hex.get(range)
                .and_then(|s| u8::from_str_radix(s, 16).ok())
                .unwrap_or(fallback)
        };
        if hex.len() < 6 {
            return Self::from_ints(0, 0, 0);
        }
        let a = if hex.len() >= 8 { byte(6..8, 255) } else { 255 };
        Self::from_ints_alpha(byte(0..2, 0), byte(2..4, 0), byte(4..6, 0), a)
    }

    /// Create from float values (0..1 range).
    pub fn from_values(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

// ---------------------------------------------------------------------------
// Theme types
// ---------------------------------------------------------------------------

/// A resolved theme with all colors as RGBA.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Theme {
    pub primary: Rgba,
    pub secondary: Rgba,
    pub accent: Rgba,
    pub error: Rgba,
    pub warning: Rgba,
    pub success: Rgba,
    pub info: Rgba,
    pub text: Rgba,
    pub text_muted: Rgba,
    pub selected_list_item_text: Rgba,
    pub background: Rgba,
    pub background_panel: Rgba,
    pub background_element: Rgba,
    pub background_menu: Rgba,
    pub border: Rgba,
    pub border_active: Rgba,
    pub border_subtle: Rgba,
    pub diff_added: Rgba,
    pub diff_removed: Rgba,
    pub diff_context: Rgba,
    pub diff_hunk_header: Rgba,
    pub diff_highlight_added: Rgba,
    pub diff_highlight_removed: Rgba,
    pub diff_added_bg: Rgba,
    pub diff_removed_bg: Rgba,
    pub diff_context_bg: Rgba,
    pub diff_line_number: Rgba,
    pub diff_added_line_number_bg: Rgba,
    pub diff_removed_line_number_bg: Rgba,
    pub markdown_text: Rgba,
    pub markdown_heading: Rgba,
    pub markdown_link: Rgba,
    pub markdown_link_text: Rgba,
    pub markdown_code: Rgba,
    pub markdown_block_quote: Rgba,
    pub markdown_emph: Rgba,
    pub markdown_strong: Rgba,
    pub markdown_horizontal_rule: Rgba,
    pub markdown_list_item: Rgba,
    pub markdown_list_enumeration: Rgba,
    pub markdown_image: Rgba,
    pub markdown_image_text: Rgba,
    pub markdown_code_block: Rgba,
    pub syntax_comment: Rgba,
    pub syntax_keyword: Rgba,
    pub syntax_function: Rgba,
    pub syntax_variable: Rgba,
    pub syntax_string: Rgba,
    pub syntax_number: Rgba,
    pub syntax_type: Rgba,
    pub syntax_operator: Rgba,
    pub syntax_punctuation: Rgba,
    pub thinking_opacity: f64,
    pub has_selected_list_item_text: bool,
}

/// A color value in a theme JSON: hex string, ref name, variant, or ANSI code.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ColorValue {
    Hex(String),
    Ref(String),
    Ansi(u8),
    Variant {
        dark: Box<ColorValue>,
        light: Box<ColorValue>,
    },
}

/// A theme JSON file (mirrors `ThemeJson`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemeJson {
    #[serde(rename = "$schema")]
    pub schema: Option<String>,
    pub defs: Option<HashMap<String, String>>,
    pub theme: HashMap<String, ColorValue>,
}

// ---------------------------------------------------------------------------
// Theme store
// ---------------------------------------------------------------------------

#[derive(Default)]
struct ThemeStore {
    plugin_themes: HashMap<String, ThemeJson>,
    custom_themes: HashMap<String, ThemeJson>,
    system_theme: Option<ThemeJson>,
    subscribers: Vec<ThemeSubscriber>,
}

/// Theme-change subscriber (mirrors `subscribeThemes`).
pub type ThemeSubscriber = Box<dyn Fn(&HashMap<String, ThemeJson>) + Send + Sync>;

static THEME_STORE: OnceLock<Mutex<ThemeStore>> = OnceLock::new();

fn theme_store() -> &'static Mutex<ThemeStore> {
    THEME_STORE.get_or_init(|| Mutex::new(ThemeStore::default()))
}

/// Default theme keys (first 5 from source).
pub const DEFAULT_THEME_KEYS: &[&str] = &["opencode", "catppuccin", "tokyonight", "gruvbox", "ayu"];

/// Load a default theme from embedded JSON.
pub fn default_theme(name: &str) -> Option<ThemeJson> {
    let json = match name {
        "aura" => include_str!("assets/aura.json"),
        "ayu" => include_str!("assets/ayu.json"),
        "catppuccin" => include_str!("assets/catppuccin.json"),
        "catppuccin-frappe" => include_str!("assets/catppuccin-frappe.json"),
        "catppuccin-macchiato" => include_str!("assets/catppuccin-macchiato.json"),
        "cobalt2" => include_str!("assets/cobalt2.json"),
        "cursor" => include_str!("assets/cursor.json"),
        "dracula" => include_str!("assets/dracula.json"),
        "everforest" => include_str!("assets/everforest.json"),
        "flexoki" => include_str!("assets/flexoki.json"),
        "github" => include_str!("assets/github.json"),
        "gruvbox" => include_str!("assets/gruvbox.json"),
        "kanagawa" => include_str!("assets/kanagawa.json"),
        "lucent-orng" => include_str!("assets/lucent-orng.json"),
        "material" => include_str!("assets/material.json"),
        "matrix" => include_str!("assets/matrix.json"),
        "mercury" => include_str!("assets/mercury.json"),
        "monokai" => include_str!("assets/monokai.json"),
        "nightowl" => include_str!("assets/nightowl.json"),
        "nord" => include_str!("assets/nord.json"),
        "one-dark" => include_str!("assets/one-dark.json"),
        "osaka-jade" => include_str!("assets/osaka-jade.json"),
        "opencode" => include_str!("assets/opencode.json"),
        "orng" => include_str!("assets/orng.json"),
        "palenight" => include_str!("assets/palenight.json"),
        "rosepine" => include_str!("assets/rosepine.json"),
        "solarized" => include_str!("assets/solarized.json"),
        "synthwave84" => include_str!("assets/synthwave84.json"),
        "tokyonight" => include_str!("assets/tokyonight.json"),
        "vercel" => include_str!("assets/vercel.json"),
        "vesper" => include_str!("assets/vesper.json"),
        "zenburn" => include_str!("assets/zenburn.json"),
        "carbonfox" => include_str!("assets/carbonfox.json"),
        _ => return None,
    };
    serde_json::from_str(json).ok()
}

/// All default themes.
pub fn default_themes() -> HashMap<String, ThemeJson> {
    let mut themes = HashMap::new();
    for (name, _) in [
        ("aura", 0),
        ("ayu", 0),
        ("catppuccin", 0),
        ("catppuccin-frappe", 0),
        ("catppuccin-macchiato", 0),
        ("cobalt2", 0),
        ("cursor", 0),
        ("dracula", 0),
        ("everforest", 0),
        ("flexoki", 0),
        ("github", 0),
        ("gruvbox", 0),
        ("kanagawa", 0),
        ("lucent-orng", 0),
        ("material", 0),
        ("matrix", 0),
        ("mercury", 0),
        ("monokai", 0),
        ("nightowl", 0),
        ("nord", 0),
        ("one-dark", 0),
        ("osaka-jade", 0),
        ("opencode", 0),
        ("orng", 0),
        ("palenight", 0),
        ("rosepine", 0),
        ("solarized", 0),
        ("synthwave84", 0),
        ("tokyonight", 0),
        ("vercel", 0),
        ("vesper", 0),
        ("zenburn", 0),
        ("carbonfox", 0),
    ] {
        if let Some(theme) = default_theme(name) {
            themes.insert(name.to_string(), theme);
        }
    }
    themes
}

/// List all themes (defaults < plugin < custom < system).
pub fn all_themes() -> HashMap<String, ThemeJson> {
    let store = theme_store().lock().unwrap();
    let mut themes = default_themes();
    for (k, v) in &store.plugin_themes {
        themes.insert(k.clone(), v.clone());
    }
    for (k, v) in &store.custom_themes {
        themes.insert(k.clone(), v.clone());
    }
    if let Some(ref system) = store.system_theme {
        themes.insert("system".to_string(), system.clone());
    }
    themes
}

/// Subscribe to theme changes (mirrors `subscribeThemes`).
pub fn subscribe_themes(handler: ThemeSubscriber) {
    theme_store().lock().unwrap().subscribers.push(handler);
}

fn notify_subscribers(snapshot: &HashMap<String, ThemeJson>) {
    let store = theme_store().lock().unwrap();
    for subscriber in &store.subscribers {
        subscriber(snapshot);
    }
}

/// Set custom themes.
pub fn set_custom_themes(themes: HashMap<String, ThemeJson>) {
    theme_store().lock().unwrap().custom_themes = themes;
    notify_subscribers(&all_themes());
}

/// Set the system theme.
pub fn set_system_theme(theme: Option<ThemeJson>) {
    theme_store().lock().unwrap().system_theme = theme;
    notify_subscribers(&all_themes());
}

/// Check if a theme exists.
pub fn has_theme(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    all_themes().contains_key(name)
}

/// Check a raw value is a theme object (mirrors `isTheme`).
pub fn is_theme_value(theme: &serde_json::Value) -> bool {
    match theme.get("theme") {
        Some(t) => t.is_object(),
        None => false,
    }
}

/// Add a plugin theme (mirrors `addTheme`).
pub fn add_theme(name: &str, theme: &serde_json::Value) -> bool {
    if name.is_empty() {
        return false;
    }
    if !is_theme_value(theme) {
        return false;
    }
    if has_theme(name) {
        return false;
    }
    let parsed: ThemeJson = match serde_json::from_value(theme.clone()) {
        Ok(t) => t,
        Err(_) => return false,
    };
    theme_store()
        .lock()
        .unwrap()
        .plugin_themes
        .insert(name.to_string(), parsed);
    notify_subscribers(&all_themes());
    true
}

/// Upsert a theme (custom takes precedence over plugin).
pub fn upsert_theme(name: &str, theme: &serde_json::Value) -> bool {
    if name.is_empty() {
        return false;
    }
    if !is_theme_value(theme) {
        return false;
    }
    let parsed: ThemeJson = match serde_json::from_value(theme.clone()) {
        Ok(t) => t,
        Err(_) => return false,
    };
    let mut store = theme_store().lock().unwrap();
    if store.custom_themes.contains_key(name) {
        store.custom_themes.insert(name.to_string(), parsed);
    } else {
        store.plugin_themes.insert(name.to_string(), parsed);
    }
    drop(store);
    notify_subscribers(&all_themes());
    true
}

// ---------------------------------------------------------------------------
// Color resolution
// ---------------------------------------------------------------------------

/// ANSI color code to RGBA.
pub fn ansi_to_rgba(code: u8) -> Rgba {
    if code < 16 {
        let colors = [
            "#000000", "#800000", "#008000", "#808000", "#000080", "#800080", "#008080", "#c0c0c0",
            "#808080", "#ff0000", "#00ff00", "#ffff00", "#0000ff", "#ff00ff", "#00ffff", "#ffffff",
        ];
        Rgba::from_hex(colors.get(code as usize).unwrap_or(&"#000000"))
    } else if code < 232 {
        let index = code - 16;
        let b = index % 6;
        let g = (index / 6) % 6;
        let r = index / 36;
        let val = |x: u8| -> u8 {
            if x == 0 {
                0
            } else {
                x * 40 + 55
            }
        };
        Rgba::from_ints(val(r), val(g), val(b))
    } else {
        // Grayscale Ramp (232-255; u8 maxes at 255, so this is the remainder).
        let gray = (code - 232) * 10 + 8;
        Rgba::from_ints(gray, gray, gray)
    }
}

/// Tint a base color with an overlay at a given alpha.
pub fn tint(base: Rgba, overlay: Rgba, alpha: f64) -> Rgba {
    let r = base.r + (overlay.r - base.r) * alpha as f32;
    let g = base.g + (overlay.g - base.g) * alpha as f32;
    let b = base.b + (overlay.b - base.b) * alpha as f32;
    Rgba::from_values(r, g, b, 1.0)
}

/// Terminal mode from background color.
pub fn terminal_mode(default_background: Option<&str>) -> Option<&'static str> {
    let bg = default_background?;
    let rgba = Rgba::from_hex(bg);
    let luminance = 0.299 * rgba.r + 0.587 * rgba.g + 0.114 * rgba.b;
    if luminance > 0.5 {
        Some("light")
    } else {
        Some("dark")
    }
}

/// Selected foreground color.
pub fn selected_foreground(theme: &Theme, bg: Option<Rgba>) -> Rgba {
    if theme.has_selected_list_item_text {
        return theme.selected_list_item_text;
    }
    if theme.background.a == 0.0 {
        let target = bg.unwrap_or(theme.primary);
        let luminance = 0.299 * target.r + 0.587 * target.g + 0.114 * target.b;
        if luminance > 0.5 {
            return Rgba::from_ints(0, 0, 0);
        } else {
            return Rgba::from_ints(255, 255, 255);
        }
    }
    theme.background
}

/// Resolve a theme JSON to a full Theme.
pub fn resolve_theme(theme: &ThemeJson, mode: &str) -> Result<Theme, String> {
    let defs = theme.defs.clone().unwrap_or_default();
    let theme_map = &theme.theme;

    fn resolve_name(
        name: &str,
        defs: &HashMap<String, String>,
        theme_map: &HashMap<String, ColorValue>,
        mode: &str,
        chain: &mut Vec<String>,
    ) -> Result<Rgba, String> {
        if chain.iter().any(|c| c == name) {
            chain.push(name.to_string());
            return Err(format!("Circular color reference: {}", chain.join(" -> ")));
        }
        let next = defs
            .get(name)
            .map(|v| {
                if v.starts_with('#') || v == "transparent" || v == "none" {
                    ColorValue::Hex(v.clone())
                } else {
                    ColorValue::Ref(v.clone())
                }
            })
            .or_else(|| theme_map.get(name).cloned());
        match next {
            None => Err(format!(
                "Color reference \"{}\" not found in defs or theme",
                name
            )),
            Some(value) => {
                chain.push(name.to_string());
                let result = resolve_color(&value, defs, theme_map, mode, chain);
                chain.pop();
                result
            }
        }
    }

    fn resolve_color(
        c: &ColorValue,
        defs: &HashMap<String, String>,
        theme_map: &HashMap<String, ColorValue>,
        mode: &str,
        chain: &mut Vec<String>,
    ) -> Result<Rgba, String> {
        match c {
            // JSON strings deserialize as `Hex`; only `#`-prefixed strings
            // are hex colors, the rest are ref names (mirrors
            // `c.startsWith("#")` in source).
            ColorValue::Hex(hex) => {
                if hex == "transparent" || hex == "none" {
                    Ok(Rgba::from_ints_alpha(0, 0, 0, 0))
                } else if hex.starts_with('#') {
                    Ok(Rgba::from_hex(hex))
                } else {
                    resolve_name(hex, defs, theme_map, mode, chain)
                }
            }
            ColorValue::Ref(name) => resolve_name(name, defs, theme_map, mode, chain),
            ColorValue::Ansi(code) => Ok(ansi_to_rgba(*code)),
            ColorValue::Variant { dark, light } => {
                let inner = if mode == "dark" { dark } else { light };
                resolve_color(inner, defs, theme_map, mode, chain)
            }
        }
    }

    let mut chain = Vec::new();
    let mut get = |key: &str| -> Result<Rgba, String> {
        let val = theme_map
            .get(key)
            .ok_or_else(|| format!("Missing theme color: {}", key))?;
        resolve_color(val, &defs, theme_map, mode, &mut chain)
    };

    let has_selected = theme_map.contains_key("selectedListItemText");
    let selected_list_item_text = if has_selected {
        get("selectedListItemText")?
    } else {
        get("background")?
    };
    let background_menu = if theme_map.contains_key("backgroundMenu") {
        get("backgroundMenu")?
    } else {
        get("backgroundElement")?
    };
    let thinking_opacity = 0.6;

    Ok(Theme {
        primary: get("primary")?,
        secondary: get("secondary")?,
        accent: get("accent")?,
        error: get("error")?,
        warning: get("warning")?,
        success: get("success")?,
        info: get("info")?,
        text: get("text")?,
        text_muted: get("textMuted")?,
        selected_list_item_text,
        background: get("background")?,
        background_panel: get("backgroundPanel")?,
        background_element: get("backgroundElement")?,
        background_menu,
        border: get("border")?,
        border_active: get("borderActive")?,
        border_subtle: get("borderSubtle")?,
        diff_added: get("diffAdded")?,
        diff_removed: get("diffRemoved")?,
        diff_context: get("diffContext")?,
        diff_hunk_header: get("diffHunkHeader")?,
        diff_highlight_added: get("diffHighlightAdded")?,
        diff_highlight_removed: get("diffHighlightRemoved")?,
        diff_added_bg: get("diffAddedBg")?,
        diff_removed_bg: get("diffRemovedBg")?,
        diff_context_bg: get("diffContextBg")?,
        diff_line_number: get("diffLineNumber")?,
        diff_added_line_number_bg: get("diffAddedLineNumberBg")?,
        diff_removed_line_number_bg: get("diffRemovedLineNumberBg")?,
        markdown_text: get("markdownText")?,
        markdown_heading: get("markdownHeading")?,
        markdown_link: get("markdownLink")?,
        markdown_link_text: get("markdownLinkText")?,
        markdown_code: get("markdownCode")?,
        markdown_block_quote: get("markdownBlockQuote")?,
        markdown_emph: get("markdownEmph")?,
        markdown_strong: get("markdownStrong")?,
        markdown_horizontal_rule: get("markdownHorizontalRule")?,
        markdown_list_item: get("markdownListItem")?,
        markdown_list_enumeration: get("markdownListEnumeration")?,
        markdown_image: get("markdownImage")?,
        markdown_image_text: get("markdownImageText")?,
        markdown_code_block: get("markdownCodeBlock")?,
        syntax_comment: get("syntaxComment")?,
        syntax_keyword: get("syntaxKeyword")?,
        syntax_function: get("syntaxFunction")?,
        syntax_variable: get("syntaxVariable")?,
        syntax_string: get("syntaxString")?,
        syntax_number: get("syntaxNumber")?,
        syntax_type: get("syntaxType")?,
        syntax_operator: get("syntaxOperator")?,
        syntax_punctuation: get("syntaxPunctuation")?,
        thinking_opacity,
        has_selected_list_item_text: has_selected,
    })
}

// ---------------------------------------------------------------------------
// System theme generation
// ---------------------------------------------------------------------------

/// Terminal colors input for system theme generation.
#[derive(Debug, Clone, Default)]
pub struct TerminalColors {
    pub palette: Vec<Option<String>>,
    pub default_foreground: Option<String>,
    pub default_background: Option<String>,
}

fn generate_gray_scale(bg: Rgba, is_dark: bool) -> HashMap<u8, Rgba> {
    let mut grays = HashMap::new();
    let bg_r = bg.r * 255.0;
    let bg_g = bg.g * 255.0;
    let bg_b = bg.b * 255.0;
    let luminance = 0.299 * bg_r + 0.587 * bg_g + 0.114 * bg_b;

    for i in 1..=12u8 {
        let factor = f32::from(i) / 12.0;
        let (new_r, new_g, new_b) = if is_dark {
            if luminance < 10.0 {
                let v = (factor * 0.4 * 255.0) as u8;
                (v, v, v)
            } else {
                let new_lum = luminance + (255.0 - luminance) * factor * 0.4;
                let ratio = new_lum / luminance;
                (
                    (bg_r * ratio).min(255.0) as u8,
                    (bg_g * ratio).min(255.0) as u8,
                    (bg_b * ratio).min(255.0) as u8,
                )
            }
        } else {
            if luminance > 245.0 {
                let v = (255.0 - factor * 0.4 * 255.0) as u8;
                (v, v, v)
            } else {
                let new_lum = luminance * (1.0 - factor * 0.4);
                let ratio = new_lum / luminance;
                (
                    (bg_r * ratio).max(0.0) as u8,
                    (bg_g * ratio).max(0.0) as u8,
                    (bg_b * ratio).max(0.0) as u8,
                )
            }
        };
        grays.insert(i, Rgba::from_ints(new_r, new_g, new_b));
    }
    grays
}

fn generate_muted_text_color(bg: Rgba, is_dark: bool) -> Rgba {
    let bg_r = bg.r * 255.0;
    let bg_g = bg.g * 255.0;
    let bg_b = bg.b * 255.0;
    let bg_lum = 0.299 * bg_r + 0.587 * bg_g + 0.114 * bg_b;

    let gray = if is_dark {
        if bg_lum < 10.0 {
            180
        } else {
            ((160.0 + bg_lum * 0.3) as u8).min(200)
        }
    } else {
        if bg_lum > 245.0 {
            75
        } else {
            ((100.0 - (255.0 - bg_lum) * 0.2) as u8).max(60)
        }
    };
    Rgba::from_ints(gray, gray, gray)
}

/// Generate a system theme from terminal colors.
pub fn generate_system(colors: &TerminalColors, mode: &str) -> ThemeJson {
    let bg = Rgba::from_hex(
        colors
            .default_background
            .as_deref()
            .or_else(|| colors.palette.first().and_then(|p| p.as_deref()))
            .unwrap_or("#000000"),
    );
    let fg = Rgba::from_hex(
        colors
            .default_foreground
            .as_deref()
            .or_else(|| colors.palette.get(7).and_then(|p| p.as_deref()))
            .unwrap_or("#ffffff"),
    );
    let transparent = Rgba::from_values(bg.r, bg.g, bg.b, 0.0);
    let is_dark = mode == "dark";

    let col = |i: usize| -> Rgba {
        colors
            .palette
            .get(i)
            .and_then(|p| p.as_deref())
            .map(Rgba::from_hex)
            .unwrap_or_else(|| ansi_to_rgba(i as u8))
    };

    let grays = generate_gray_scale(bg, is_dark);
    let text_muted = generate_muted_text_color(bg, is_dark);

    let ansi_colors: HashMap<&str, Rgba> = HashMap::from([
        ("black", col(0)),
        ("red", col(1)),
        ("green", col(2)),
        ("yellow", col(3)),
        ("blue", col(4)),
        ("magenta", col(5)),
        ("cyan", col(6)),
        ("white", col(7)),
        ("redBright", col(9)),
        ("greenBright", col(10)),
    ]);

    let diff_alpha = if is_dark { 0.22 } else { 0.14 };
    let diff_added_bg = tint(bg, ansi_colors["green"], diff_alpha);
    let diff_removed_bg = tint(bg, ansi_colors["red"], diff_alpha);
    let diff_added_line_number_bg = tint(grays[&2], ansi_colors["green"], diff_alpha);
    let diff_removed_line_number_bg = tint(grays[&2], ansi_colors["red"], diff_alpha);

    // Computed colors land in `defs` as hex so the generated theme
    // resolves through the same `resolve_theme` path as file themes.
    let hex = |color: Rgba| {
        let byte = |v: f32| (v * 255.0).round().clamp(0.0, 255.0) as u8;
        format!(
            "#{:02x}{:02x}{:02x}{:02x}",
            byte(color.r),
            byte(color.g),
            byte(color.b),
            byte(color.a)
        )
    };
    let black = ansi_colors["black"];
    let white = ansi_colors["white"];
    let red_bright = ansi_colors["redBright"];
    let green_bright = ansi_colors["greenBright"];
    let red = ansi_colors["red"];
    let green = ansi_colors["green"];
    let yellow = ansi_colors["yellow"];
    let blue = ansi_colors["blue"];
    let magenta = ansi_colors["magenta"];
    let cyan = ansi_colors["cyan"];
    let grays2 = grays[&2];
    let grays3 = grays[&3];
    let grays6 = grays[&6];
    let grays7 = grays[&7];
    let grays8 = grays[&8];
    let _ = (black, white);
    let mut defs = HashMap::new();
    defs.insert("sysBg".to_string(), hex(bg));
    defs.insert("sysFg".to_string(), hex(fg));
    defs.insert("sysTransparent".to_string(), hex(transparent));
    defs.insert("sysTextMuted".to_string(), hex(text_muted));
    defs.insert("sysBlack".to_string(), hex(black));
    defs.insert("sysRed".to_string(), hex(red));
    defs.insert("sysGreen".to_string(), hex(green));
    defs.insert("sysYellow".to_string(), hex(yellow));
    defs.insert("sysBlue".to_string(), hex(blue));
    defs.insert("sysMagenta".to_string(), hex(magenta));
    defs.insert("sysCyan".to_string(), hex(cyan));
    defs.insert("sysWhite".to_string(), hex(white));
    defs.insert("sysRedBright".to_string(), hex(red_bright));
    defs.insert("sysGreenBright".to_string(), hex(green_bright));
    defs.insert("sysGrays2".to_string(), hex(grays2));
    defs.insert("sysGrays3".to_string(), hex(grays3));
    defs.insert("sysGrays6".to_string(), hex(grays6));
    defs.insert("sysGrays7".to_string(), hex(grays7));
    defs.insert("sysGrays8".to_string(), hex(grays8));
    defs.insert("sysDiffAddedBg".to_string(), hex(diff_added_bg));
    defs.insert("sysDiffRemovedBg".to_string(), hex(diff_removed_bg));
    defs.insert(
        "sysDiffAddedLnBg".to_string(),
        hex(diff_added_line_number_bg),
    );
    defs.insert(
        "sysDiffRemovedLnBg".to_string(),
        hex(diff_removed_line_number_bg),
    );
    let pairs = [
        ("primary", "sysCyan"),
        ("secondary", "sysMagenta"),
        ("accent", "sysCyan"),
        ("error", "sysRed"),
        ("warning", "sysYellow"),
        ("success", "sysGreen"),
        ("info", "sysCyan"),
        ("text", "sysFg"),
        ("textMuted", "sysTextMuted"),
        ("selectedListItemText", "sysBg"),
        ("background", "sysTransparent"),
        ("backgroundPanel", "sysGrays2"),
        ("backgroundElement", "sysGrays3"),
        ("backgroundMenu", "sysGrays3"),
        ("borderSubtle", "sysGrays6"),
        ("border", "sysGrays7"),
        ("borderActive", "sysGrays8"),
        ("diffAdded", "sysGreen"),
        ("diffRemoved", "sysRed"),
        ("diffContext", "sysGrays7"),
        ("diffHunkHeader", "sysGrays7"),
        ("diffHighlightAdded", "sysGreenBright"),
        ("diffHighlightRemoved", "sysRedBright"),
        ("diffAddedBg", "sysDiffAddedBg"),
        ("diffRemovedBg", "sysDiffRemovedBg"),
        ("diffContextBg", "sysGrays2"),
        ("diffLineNumber", "sysTextMuted"),
        ("diffAddedLineNumberBg", "sysDiffAddedLnBg"),
        ("diffRemovedLineNumberBg", "sysDiffRemovedLnBg"),
        ("markdownText", "sysFg"),
        ("markdownHeading", "sysFg"),
        ("markdownLink", "sysBlue"),
        ("markdownLinkText", "sysCyan"),
        ("markdownCode", "sysGreen"),
        ("markdownBlockQuote", "sysYellow"),
        ("markdownEmph", "sysYellow"),
        ("markdownStrong", "sysFg"),
        ("markdownHorizontalRule", "sysGrays7"),
        ("markdownListItem", "sysBlue"),
        ("markdownListEnumeration", "sysCyan"),
        ("markdownImage", "sysBlue"),
        ("markdownImageText", "sysCyan"),
        ("markdownCodeBlock", "sysFg"),
        ("syntaxComment", "sysTextMuted"),
        ("syntaxKeyword", "sysMagenta"),
        ("syntaxFunction", "sysBlue"),
        ("syntaxVariable", "sysFg"),
        ("syntaxString", "sysGreen"),
        ("syntaxNumber", "sysYellow"),
        ("syntaxType", "sysCyan"),
        ("syntaxOperator", "sysCyan"),
        ("syntaxPunctuation", "sysFg"),
    ];
    let mut theme = HashMap::new();
    for (key, def) in pairs {
        theme.insert(key.to_string(), ColorValue::Ref(def.to_string()));
    }
    ThemeJson {
        schema: None,
        defs: Some(defs),
        theme,
    }
}

// ---------------------------------------------------------------------------
// Syntax rules
// ---------------------------------------------------------------------------

/// A syntax highlighting rule.
#[derive(Debug, Clone, PartialEq)]
pub struct SyntaxRule {
    pub scope: Vec<String>,
    pub foreground: Option<Rgba>,
    pub background: Option<Rgba>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

/// Get syntax rules for a theme.
pub fn get_syntax_rules(theme: &Theme) -> Vec<SyntaxRule> {
    let rule = |scope: &[&str], fg: Rgba| SyntaxRule {
        scope: scope.iter().map(|s| s.to_string()).collect(),
        foreground: Some(fg),
        background: None,
        bold: false,
        italic: false,
        underline: false,
    };
    let rule_bg = |scope: &[&str], fg: Rgba, bg: Rgba| SyntaxRule {
        scope: scope.iter().map(|s| s.to_string()).collect(),
        foreground: Some(fg),
        background: Some(bg),
        bold: false,
        italic: false,
        underline: false,
    };
    let rule_bold = |scope: &[&str], fg: Rgba| SyntaxRule {
        scope: scope.iter().map(|s| s.to_string()).collect(),
        foreground: Some(fg),
        background: None,
        bold: true,
        italic: false,
        underline: false,
    };
    let rule_italic = |scope: &[&str], fg: Rgba| SyntaxRule {
        scope: scope.iter().map(|s| s.to_string()).collect(),
        foreground: Some(fg),
        background: None,
        bold: false,
        italic: true,
        underline: false,
    };
    let rule_bold_italic = |scope: &[&str], fg: Rgba| SyntaxRule {
        scope: scope.iter().map(|s| s.to_string()).collect(),
        foreground: Some(fg),
        background: None,
        bold: true,
        italic: true,
        underline: false,
    };
    let rule_underline = |scope: &[&str], fg: Rgba| SyntaxRule {
        scope: scope.iter().map(|s| s.to_string()).collect(),
        foreground: Some(fg),
        background: None,
        bold: false,
        italic: false,
        underline: true,
    };

    vec![
        rule(&["default"], theme.text),
        rule(&["prompt"], theme.accent),
        rule_bold(&["extmark.file"], theme.warning),
        rule_bold(&["extmark.agent"], theme.secondary),
        {
            let mut r = rule(
                &["extmark.paste"],
                selected_foreground(theme, Some(theme.warning)),
            );
            r.background = Some(theme.warning);
            r.bold = true;
            r
        },
        rule_italic(&["comment"], theme.syntax_comment),
        rule_italic(&["comment.documentation"], theme.syntax_comment),
        rule(&["string", "symbol"], theme.syntax_string),
        rule(&["number", "boolean"], theme.syntax_number),
        rule(&["character.special"], theme.syntax_string),
        rule_italic(
            &[
                "keyword.return",
                "keyword.conditional",
                "keyword.repeat",
                "keyword.coroutine",
            ],
            theme.syntax_keyword,
        ),
        rule_bold_italic(&["keyword.type"], theme.syntax_type),
        rule(
            &["keyword.function", "function.method"],
            theme.syntax_function,
        ),
        rule_italic(&["keyword"], theme.syntax_keyword),
        rule(&["keyword.import"], theme.syntax_keyword),
        rule(
            &["operator", "keyword.operator", "punctuation.delimiter"],
            theme.syntax_operator,
        ),
        rule(&["keyword.conditional.ternary"], theme.syntax_operator),
        rule(
            &[
                "variable",
                "variable.parameter",
                "function.method.call",
                "function.call",
            ],
            theme.syntax_variable,
        ),
        rule(
            &["variable.member", "function", "constructor"],
            theme.syntax_function,
        ),
        rule(&["type", "module"], theme.syntax_type),
        rule(&["constant"], theme.syntax_number),
        rule(&["property"], theme.syntax_variable),
        rule(&["class"], theme.syntax_type),
        rule(&["parameter"], theme.syntax_variable),
        rule(
            &["punctuation", "punctuation.bracket"],
            theme.syntax_punctuation,
        ),
        rule(
            &[
                "variable.builtin",
                "type.builtin",
                "function.builtin",
                "module.builtin",
                "constant.builtin",
            ],
            theme.error,
        ),
        rule(&["variable.super"], theme.error),
        rule(&["string.escape", "string.regexp"], theme.syntax_keyword),
        rule_italic(&["keyword.directive"], theme.syntax_keyword),
        rule(&["punctuation.special"], theme.syntax_operator),
        rule_italic(&["keyword.modifier"], theme.syntax_keyword),
        rule_italic(&["keyword.exception"], theme.syntax_keyword),
        rule_bold(&["markup.heading"], theme.markdown_heading),
        {
            let mut r = rule_bold(&["markup.heading.1"], theme.markdown_heading);
            r.underline = true;
            r
        },
        rule_bold(&["markup.heading.2"], theme.markdown_heading),
        rule_bold(&["markup.heading.3"], theme.markdown_heading),
        rule_bold(&["markup.heading.4"], theme.markdown_heading),
        rule_bold(&["markup.heading.5"], theme.markdown_heading),
        rule_bold(&["markup.heading.6"], theme.markdown_heading),
        rule_bold(&["markup.bold", "markup.strong"], theme.markdown_strong),
        rule_italic(&["markup.italic"], theme.markdown_emph),
        rule(&["markup.list"], theme.markdown_list_item),
        rule_italic(&["markup.quote"], theme.markdown_block_quote),
        rule(&["markup.raw", "markup.raw.block"], theme.markdown_code),
        rule_bg(
            &["markup.raw.inline"],
            theme.markdown_code,
            theme.background,
        ),
        rule_underline(&["markup.link"], theme.markdown_link),
        rule_underline(&["markup.link.label"], theme.markdown_link_text),
        rule_underline(&["markup.link.url"], theme.markdown_link),
        rule(&["label"], theme.markdown_link_text),
        rule(&["spell", "nospell"], theme.text),
        rule(&["conceal"], theme.text_muted),
        rule_underline(
            &["string.special", "string.special.url"],
            theme.markdown_link,
        ),
        rule(&["character"], theme.syntax_string),
        rule(&["float"], theme.syntax_number),
        rule_bold_italic(&["comment.error"], theme.error),
        rule_bold_italic(&["comment.warning"], theme.warning),
        rule_bold_italic(&["comment.todo", "comment.note"], theme.info),
        rule(&["namespace"], theme.syntax_type),
        rule(&["field"], theme.syntax_variable),
        rule_bold(&["type.definition"], theme.syntax_type),
        rule(&["keyword.export"], theme.syntax_keyword),
        rule(&["attribute", "annotation"], theme.warning),
        rule(&["tag"], theme.error),
        rule(&["tag.attribute"], theme.syntax_keyword),
        rule(&["tag.delimiter"], theme.syntax_operator),
        rule(&["markup.strikethrough"], theme.text_muted),
        rule_underline(&["markup.underline"], theme.text),
        rule(&["markup.list.checked"], theme.success),
        rule(&["markup.list.unchecked"], theme.text_muted),
        rule_bg(&["diff.plus"], theme.diff_added, theme.diff_added_bg),
        rule_bg(&["diff.minus"], theme.diff_removed, theme.diff_removed_bg),
        rule_bg(&["diff.delta"], theme.diff_context, theme.diff_context_bg),
        rule_bold(&["error"], theme.error),
        rule_bold(&["warning"], theme.warning),
        rule(&["info"], theme.info),
        rule(&["debug"], theme.text_muted),
    ]
}

/// Generate syntax rules with thinking opacity applied.
pub fn generate_subtle_syntax(theme: &Theme) -> Vec<SyntaxRule> {
    get_syntax_rules(theme)
        .into_iter()
        .map(|mut rule| {
            if let Some(ref mut fg) = rule.foreground {
                let r = (fg.r * 255.0).round() as u8;
                let g = (fg.g * 255.0).round() as u8;
                let b = (fg.b * 255.0).round() as u8;
                let a = (theme.thinking_opacity * 255.0).round() as u8;
                *fg = Rgba::from_ints_alpha(r, g, b, a);
            }
            rule
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgba_from_hex() {
        let c = Rgba::from_hex("#ff0000");
        assert!((c.r - 1.0).abs() < 0.01);
        assert!(c.g.abs() < 0.01);
        assert!(c.b.abs() < 0.01);
    }

    #[test]
    fn rgba_from_ints() {
        let c = Rgba::from_ints(255, 0, 0);
        assert!((c.r - 1.0).abs() < 0.01);
        assert!(c.g.abs() < 0.01);
    }

    #[test]
    fn ansi_to_rgba_basic() {
        let black = ansi_to_rgba(0);
        assert!(black.r.abs() < 0.01);
        let white = ansi_to_rgba(15);
        assert!((white.r - 1.0).abs() < 0.01);
    }

    #[test]
    fn ansi_to_rgba_cube() {
        let c = ansi_to_rgba(16);
        assert!(c.r.abs() < 0.01);
        assert!(c.g.abs() < 0.01);
        assert!(c.b.abs() < 0.01);
    }

    #[test]
    fn ansi_to_rgba_grayscale() {
        let c = ansi_to_rgba(232);
        assert!((c.r - 8.0 / 255.0).abs() < 0.01);
    }

    #[test]
    fn tint_blends() {
        let base = Rgba::from_ints(0, 0, 0);
        let overlay = Rgba::from_ints(255, 255, 255);
        let result = tint(base, overlay, 0.5);
        assert!((result.r - 0.5).abs() < 0.01);
    }

    #[test]
    fn terminal_mode_light() {
        assert_eq!(terminal_mode(Some("#fbf1c7")), Some("light"));
    }

    #[test]
    fn terminal_mode_dark() {
        assert_eq!(terminal_mode(Some("#1a1b26")), Some("dark"));
    }

    #[test]
    fn terminal_mode_none() {
        assert_eq!(terminal_mode(None), None);
    }

    #[test]
    fn default_themes_count() {
        let themes = default_themes();
        assert_eq!(themes.len(), 33);
    }

    #[test]
    fn has_theme_builtin() {
        assert!(has_theme("opencode"));
        assert!(!has_theme("nonexistent"));
    }

    #[test]
    fn add_theme_works() {
        let theme = default_theme("opencode").unwrap();
        let value = serde_json::to_value(&theme).unwrap();
        assert!(add_theme("test-theme-1", &value));
        assert!(has_theme("test-theme-1"));
        assert!(!add_theme("test-theme-1", &value));
    }

    #[test]
    fn add_theme_rejects_value_without_theme_object() {
        let bad = serde_json::json!({ "defs": { "a": "#ffffff" } });
        assert!(!add_theme("test-theme-bad", &bad));
        assert!(!has_theme("test-theme-bad"));
    }

    #[test]
    fn resolve_theme_opencode() {
        let json = default_theme("opencode").unwrap();
        let theme = resolve_theme(&json, "dark").unwrap();
        assert!(theme.primary.r > 0.0);
        assert!(theme.thinking_opacity > 0.0);
    }

    #[test]
    fn resolve_theme_circular_ref() {
        let mut json = default_theme("opencode").unwrap();
        let mut defs = json.defs.clone().unwrap_or_default();
        defs.insert("one".to_string(), "two".to_string());
        defs.insert("two".to_string(), "one".to_string());
        json.defs = Some(defs);
        json.theme
            .insert("primary".to_string(), ColorValue::Ref("one".to_string()));
        let result = resolve_theme(&json, "dark");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Circular color reference"));
    }

    #[test]
    fn selected_foreground_with_explicit() {
        let json = default_theme("opencode").unwrap();
        let mut theme = resolve_theme(&json, "dark").unwrap();
        theme.has_selected_list_item_text = true;
        theme.selected_list_item_text = Rgba::from_ints(1, 2, 3);
        let fg = selected_foreground(&theme, None);
        assert!((fg.r - 1.0 / 255.0).abs() < 0.01);
    }

    #[test]
    fn selected_foreground_transparent_bg() {
        let json = default_theme("opencode").unwrap();
        let mut theme = resolve_theme(&json, "dark").unwrap();
        theme.has_selected_list_item_text = false;
        theme.background = Rgba::from_ints_alpha(0, 0, 0, 0);
        theme.primary = Rgba::from_ints(0, 0, 0);
        let fg = selected_foreground(&theme, None);
        assert!((fg.r - 1.0).abs() < 0.01);
    }

    #[test]
    fn generate_system_dark() {
        let colors = TerminalColors {
            default_background: Some("#1a1b26".to_string()),
            default_foreground: Some("#c0caf5".to_string()),
            palette: vec![None; 16],
        };
        let json = generate_system(&colors, "dark");
        assert!(json.theme.contains_key("primary"));
        // The generated theme must resolve through the same path as file themes.
        let theme = resolve_theme(&json, "dark").unwrap();
        assert!(theme.primary.b > 0.5);
        assert_eq!(theme.background.a, 0.0);
    }

    #[test]
    fn generate_subtle_syntax_applies_opacity() {
        let json = default_theme("opencode").unwrap();
        let theme = resolve_theme(&json, "dark").unwrap();
        let rules = generate_subtle_syntax(&theme);
        assert!(!rules.is_empty());
        let rule = &rules[0];
        assert!(rule.foreground.is_some());
    }

    #[test]
    fn get_syntax_rules_count() {
        let json = default_theme("opencode").unwrap();
        let theme = resolve_theme(&json, "dark").unwrap();
        let rules = get_syntax_rules(&theme);
        assert!(rules.len() >= 70);
    }
}
