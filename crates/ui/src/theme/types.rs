//! Rust port of `packages/ui/src/theme/types.ts` (opencode v1.18.30).
//!
//! 1:1 — type names, field names, and variant shapes preserved verbatim.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/// Hex color string (`#...`), 1:1 with TS `HexColor`.
pub type HexColor = String;

/// Oklch color with lightness 0–1, chroma 0–0.4+, hue 0–360.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct OklchColor {
    pub l: f64,
    pub c: f64,
    pub h: f64,
}

/// 1:1 with TS `ThemeSeedColors`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemeSeedColors {
    pub neutral: HexColor,
    pub primary: HexColor,
    pub success: HexColor,
    pub warning: HexColor,
    pub error: HexColor,
    pub info: HexColor,
    pub interactive: HexColor,
    #[serde(rename = "diffAdd")]
    pub diff_add: HexColor,
    #[serde(rename = "diffDelete")]
    pub diff_delete: HexColor,
}

/// 1:1 with TS `ThemePaletteColors`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemePaletteColors {
    pub neutral: HexColor,
    pub ink: HexColor,
    pub primary: HexColor,
    pub success: HexColor,
    pub warning: HexColor,
    pub error: HexColor,
    pub info: HexColor,
    pub accent: Option<HexColor>,
    pub interactive: Option<HexColor>,
    #[serde(rename = "diffAdd")]
    pub diff_add: Option<HexColor>,
    #[serde(rename = "diffDelete")]
    pub diff_delete: Option<HexColor>,
}

/// 1:1 with TS `ThemeVariant` (seeds xor palette, plus optional overrides).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemeVariant {
    #[serde(default)]
    pub seeds: Option<ThemeSeedColors>,
    #[serde(default)]
    pub palette: Option<ThemePaletteColors>,
    #[serde(default)]
    pub overrides: Option<std::collections::BTreeMap<String, ColorValue>>,
    #[serde(rename = "v2Overrides", default)]
    pub v2_overrides: Option<std::collections::BTreeMap<String, V2ColorValue>>,
}

/// 1:1 with TS `DesktopTheme`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DesktopTheme {
    #[serde(rename = "$schema", default)]
    pub schema: Option<String>,
    pub name: String,
    pub id: String,
    pub light: ThemeVariant,
    pub dark: ThemeVariant,
}

/// 1:1 with TS `TokenCategory`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenCategory {
    #[serde(rename = "background")]
    Background,
    #[serde(rename = "surface")]
    Surface,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "border")]
    Border,
    #[serde(rename = "icon")]
    Icon,
    #[serde(rename = "input")]
    Input,
    #[serde(rename = "button")]
    Button,
    #[serde(rename = "syntax")]
    Syntax,
    #[serde(rename = "markdown")]
    Markdown,
    #[serde(rename = "diff")]
    Diff,
    #[serde(rename = "avatar")]
    Avatar,
}

/// 1:1 with TS `ThemeToken` (a string token name).
pub type ThemeToken = String;

/// 1:1 with TS `ColorValue` (hex or `var(--...)` ref).
pub type ColorValue = String;

/// 1:1 with TS `V2ColorValue` (hex, var ref, or any CSS value).
pub type V2ColorValue = String;

/// 1:1 with TS `ResolvedTheme`.
pub type ResolvedTheme = std::collections::BTreeMap<ThemeToken, ColorValue>;

/// 1:1 with TS `ResolvedV2Theme`.
pub type ResolvedV2Theme = std::collections::BTreeMap<String, V2ColorValue>;

impl Default for OklchColor {
    fn default() -> Self {
        OklchColor {
            l: 0.0,
            c: 0.0,
            h: 0.0,
        }
    }
}
