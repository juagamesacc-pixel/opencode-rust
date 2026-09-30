//! Rust port of `packages/ui/src/theme/v2/resolve.ts` (opencode v1.18.30).
//!
//! 1:1 — ramp tables, palette reads, and resolve order preserved verbatim.

#![allow(dead_code)]

use super::super::color::{
    generate_neutral_scale, hex_to_oklch, oklch_to_hex, shift, OklchColor, Shift,
};
use super::super::types::{DesktopTheme, HexColor, ResolvedV2Theme, ThemeVariant, V2ColorValue};
use super::default_primitives::default_primitives;
use super::foreground::map_v2_foreground;
use super::mapping::{map_v2_semantics, merge_v2_tokens};

/// 1:1 with TS `V2_STEPS`.
pub const V2_STEPS: &[u32] = &[
    100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200,
];

fn clamp(v: f64, min: f64, max: f64) -> f64 {
    v.max(min).min(max)
}

/// v2 ramps: 100 = lightest, 1200 = darkest — wider spread than v1 `generateScale`.
fn generate_v2_hue_scale(seed: &str, is_dark: bool) -> Vec<HexColor> {
    let base = hex_to_oklch(seed);
    let chroma_boost = if is_dark { 1.0 } else { 1.05 };
    let light_steps: Vec<f64> = vec![
        0.99,
        0.965,
        0.93,
        0.885,
        0.835,
        clamp(base.l, 0.48, 0.72),
        clamp(base.l - 0.07, 0.4, 0.64),
        clamp(base.l - 0.14, 0.32, 0.55),
        clamp(base.l - 0.21, 0.24, 0.46),
        clamp(base.l - 0.28, 0.17, 0.38),
        clamp(base.l - 0.34, 0.12, 0.3),
        clamp(base.l - 0.4, 0.08, 0.22),
    ];
    let chroma_multipliers: Vec<f64> = vec![
        0.28, 0.48, 0.68, 0.86, 1.02, 1.28, 1.34, 1.28, 1.18, 1.08, 0.98, 0.88,
    ];

    light_steps
        .iter()
        .enumerate()
        .map(|(i, l)| {
            oklch_to_hex(&OklchColor {
                l: *l,
                c: base.c * chroma_multipliers[i] * chroma_boost,
                h: base.h,
            })
        })
        .collect()
}

/// Grey ramp: 100 = lightest, 1200 = darkest. Derived from palette neutral → ink like v1.
fn generate_v2_neutral_scale(neutral: &str, ink: &str, is_dark: bool) -> Vec<HexColor> {
    let scale = generate_neutral_scale(neutral, is_dark, Some(ink));
    if is_dark {
        scale.into_iter().rev().collect()
    } else {
        scale
    }
}

fn assign_hue_ramp(
    prefix: &str,
    scale: &[HexColor],
) -> std::collections::BTreeMap<String, V2ColorValue> {
    let mut tokens = std::collections::BTreeMap::new();
    for (i, step) in V2_STEPS.iter().enumerate() {
        tokens.insert(format!("v2-{prefix}-{step}"), scale[i].clone());
    }
    tokens
}

struct PaletteInput {
    neutral: HexColor,
    ink: HexColor,
    primary: HexColor,
    accent: HexColor,
    success: HexColor,
    warning: HexColor,
    error: HexColor,
    info: HexColor,
    interactive: HexColor,
    diff_add: HexColor,
    diff_delete: HexColor,
}

fn read_palette(variant: &ThemeVariant) -> PaletteInput {
    if let Some(palette) = &variant.palette {
        return PaletteInput {
            neutral: palette.neutral.clone(),
            ink: palette.ink.clone(),
            primary: palette.primary.clone(),
            accent: palette
                .accent
                .clone()
                .unwrap_or_else(|| palette.info.clone()),
            success: palette.success.clone(),
            warning: palette.warning.clone(),
            error: palette.error.clone(),
            info: palette.info.clone(),
            interactive: palette
                .interactive
                .clone()
                .unwrap_or_else(|| palette.primary.clone()),
            diff_add: palette.diff_add.clone().unwrap_or_else(|| {
                shift(
                    &palette.success,
                    Shift {
                        c: Some(0.55),
                        l: Some(0.14),
                        h: None,
                    },
                )
            }),
            diff_delete: palette
                .diff_delete
                .clone()
                .unwrap_or_else(|| palette.error.clone()),
        };
    }
    if let Some(seeds) = &variant.seeds {
        return PaletteInput {
            neutral: seeds.neutral.clone(),
            ink: seeds.neutral.clone(),
            primary: seeds.primary.clone(),
            accent: seeds.info.clone(),
            success: seeds.success.clone(),
            warning: seeds.warning.clone(),
            error: seeds.error.clone(),
            info: seeds.info.clone(),
            interactive: seeds.interactive.clone(),
            diff_add: seeds.diff_add.clone(),
            diff_delete: seeds.diff_delete.clone(),
        };
    }
    panic!("Theme variant requires `palette` or `seeds`");
}

/// Build v2 primitive ramps (100 = lightest). Alpha ramps are static in `v2/styles/colors.css`.
pub fn generate_v2_primitives(
    variant: &ThemeVariant,
    is_dark: bool,
) -> std::collections::BTreeMap<String, V2ColorValue> {
    let colors = read_palette(variant);
    let grey = generate_v2_neutral_scale(&colors.neutral, &colors.ink, is_dark);
    let blue = generate_v2_hue_scale(&colors.interactive, is_dark);
    let green = generate_v2_hue_scale(&colors.success, is_dark);
    let yellow = generate_v2_hue_scale(&colors.warning, is_dark);
    let red = generate_v2_hue_scale(&colors.error, is_dark);
    let purple = generate_v2_hue_scale(&colors.accent, is_dark);
    let pink = generate_v2_hue_scale(&colors.info, is_dark);
    let orange = generate_v2_hue_scale(
        &shift(
            &colors.warning,
            Shift {
                h: Some(-22.0),
                l: Some(-0.082),
                c: Some(0.94),
            },
        ),
        is_dark,
    );
    let cyan = generate_v2_hue_scale(
        &shift(
            &colors.info,
            Shift {
                h: Some(-12.0),
                l: Some(0.128),
                c: Some(1.12),
            },
        ),
        is_dark,
    );

    let mut out = default_primitives();
    for (k, v) in assign_hue_ramp("grey", &grey) {
        out.insert(k, v);
    }
    for (k, v) in assign_hue_ramp("blue", &blue) {
        out.insert(k, v);
    }
    for (k, v) in assign_hue_ramp("green", &green) {
        out.insert(k, v);
    }
    for (k, v) in assign_hue_ramp("yellow", &yellow) {
        out.insert(k, v);
    }
    for (k, v) in assign_hue_ramp("red", &red) {
        out.insert(k, v);
    }
    for (k, v) in assign_hue_ramp("purple", &purple) {
        out.insert(k, v);
    }
    for (k, v) in assign_hue_ramp("pink", &pink) {
        out.insert(k, v);
    }
    for (k, v) in assign_hue_ramp("orange", &orange) {
        out.insert(k, v);
    }
    for (k, v) in assign_hue_ramp("cyan", &cyan) {
        out.insert(k, v);
    }
    out
}

/// 1:1 with TS `resolveThemeVariantV2`.
pub fn resolve_theme_variant_v2(variant: &ThemeVariant, is_dark: bool) -> ResolvedV2Theme {
    let primitives = generate_v2_primitives(variant, is_dark);
    let semantics = map_v2_semantics(is_dark);
    let foreground = map_v2_foreground(
        &read_palette(variant).ink,
        is_dark,
        &primitives,
        &variant.overrides.clone().unwrap_or_default(),
    );
    merge_v2_tokens(&[
        primitives,
        semantics,
        foreground,
        variant.v2_overrides.clone().unwrap_or_default(),
    ])
}

/// 1:1 with the TS `resolveThemeV2` return `{ light, dark }`.
#[derive(Debug, Clone, Default)]
pub struct ResolvedV2ThemePair {
    pub light: ResolvedV2Theme,
    pub dark: ResolvedV2Theme,
}

/// 1:1 with TS `resolveThemeV2`.
pub fn resolve_theme_v2(theme: &DesktopTheme) -> ResolvedV2ThemePair {
    ResolvedV2ThemePair {
        light: resolve_theme_variant_v2(&theme.light, false),
        dark: resolve_theme_variant_v2(&theme.dark, true),
    }
}

/// 1:1 with TS `themeV2ToCss`.
pub fn theme_v2_to_css(tokens: &ResolvedV2Theme) -> String {
    tokens
        .iter()
        .map(|(key, value)| format!("--{key}: {value};"))
        .collect::<Vec<_>>()
        .join("\n  ")
}
