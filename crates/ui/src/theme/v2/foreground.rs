//! Rust port of `packages/ui/src/theme/v2/foreground.ts` (opencode v1.18.30).
//!
//! 1:1 — grey-step tables, contrast picks, and token formulas preserved verbatim.

#![allow(dead_code)]

use super::super::color::{blend, contrast_ratio, hex_to_oklch, shift, Shift};
use super::super::types::{ColorValue, HexColor, V2ColorValue};
use super::mapping::map_v2_semantics;

/// 1:1 with TS `GREY_STEPS`.
pub const GREY_STEPS: &[u32] = &[
    100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200,
];

/// 1:1 with TS `greyRef`.
pub fn grey_ref(step: u32) -> V2ColorValue {
    format!("var(--v2-grey-{step})")
}

fn grey_hex(
    primitives: &std::collections::BTreeMap<String, V2ColorValue>,
    step: u32,
) -> Option<HexColor> {
    primitives.get(&format!("v2-grey-{step}")).and_then(|hex| {
        if hex.starts_with('#') {
            Some(hex.clone())
        } else {
            None
        }
    })
}

fn resolve_grey_ref(
    value: &str,
    primitives: &std::collections::BTreeMap<String, V2ColorValue>,
) -> HexColor {
    let step: &str = value
        .strip_prefix("var(--v2-grey-")
        .and_then(|s| s.strip_suffix(')'))
        .unwrap_or_else(|| panic!("Expected grey primitive ref, got {value}"));
    grey_hex(
        primitives,
        step.parse::<u32>()
            .unwrap_or_else(|_| panic!("Expected grey primitive ref, got {value}")),
    )
    .unwrap_or_else(|| panic!("Missing grey primitive v2-grey-{step}"))
}

fn pick_grey(
    primitives: &std::collections::BTreeMap<String, V2ColorValue>,
    background: &str,
    min_contrast: f64,
    target: u32,
) -> u32 {
    let matches: Vec<u32> = GREY_STEPS
        .iter()
        .copied()
        .filter(|step| {
            grey_hex(primitives, *step)
                .map(|hex| contrast_ratio(&hex, background) >= min_contrast)
                .unwrap_or(false)
        })
        .collect();
    if matches.is_empty() {
        return target;
    }
    matches.into_iter().fold(u32::MAX, |best, step| {
        if best == u32::MAX {
            return step;
        }
        let d_step = (step as i64 - target as i64).abs();
        let d_best = (best as i64 - target as i64).abs();
        if d_step < d_best {
            step
        } else {
            best
        }
    })
}

/// 1:1 with TS `mapV2Foreground`.
pub fn map_v2_foreground(
    ink: &str,
    is_dark: bool,
    primitives: &std::collections::BTreeMap<String, V2ColorValue>,
    overrides: &std::collections::BTreeMap<String, ColorValue>,
) -> std::collections::BTreeMap<String, V2ColorValue> {
    let tint = hex_to_oklch(ink);
    let body = shift(
        ink,
        Shift {
            l: Some(if is_dark {
                (0.88 - tint.l).max(0.0) * 0.4
            } else {
                -((tint.l - 0.18).max(0.0)) * 0.24
            }),
            c: Some(if is_dark { 1.04 } else { 1.02 }),
            h: None,
        },
    );

    let semantics = map_v2_semantics(is_dark);
    let bg_base = resolve_grey_ref(&semantics["v2-background-bg-base"], primitives);
    let bg_contrast = resolve_grey_ref(&semantics["v2-background-bg-contrast"], primitives);
    let bg_inverse = resolve_grey_ref(&semantics["v2-background-bg-inverse"], primitives);
    let inverse_target: u32 = if hex_to_oklch(&bg_inverse).l > 0.55 {
        1100
    } else if grey_hex(primitives, 50).is_some() {
        50
    } else {
        100
    };

    let mut out = std::collections::BTreeMap::new();
    out.insert(
        "v2-text-text-base".to_string(),
        if is_dark {
            blend("#ffffff", &body, 0.9)
        } else {
            shift(
                &body,
                Shift {
                    l: Some(-0.07),
                    c: Some(1.04),
                    h: None,
                },
            )
        },
    );
    out.insert(
        "v2-text-text-muted".to_string(),
        overrides.get("text-weak").cloned().unwrap_or_else(|| {
            shift(
                &body,
                Shift {
                    l: Some(if is_dark { -0.11 } else { 0.11 }),
                    c: Some(0.9),
                    h: None,
                },
            )
        }),
    );
    out.insert(
        "v2-text-text-faint".to_string(),
        shift(
            &body,
            Shift {
                l: Some(if is_dark { -0.2 } else { 0.21 }),
                c: Some(if is_dark { 0.78 } else { 0.72 }),
                h: None,
            },
        ),
    );
    out.insert(
        "v2-icon-icon-base".to_string(),
        grey_ref(pick_grey(
            primitives,
            &bg_base,
            7.0,
            if is_dark { 400 } else { 800 },
        )),
    );
    out.insert(
        "v2-icon-icon-muted".to_string(),
        grey_ref(pick_grey(primitives, &bg_base, 3.0, 600)),
    );
    out.insert(
        "v2-icon-icon-inverse".to_string(),
        grey_ref(pick_grey(primitives, &bg_inverse, 7.0, inverse_target)),
    );
    out.insert(
        "v2-icon-icon-contrast".to_string(),
        grey_ref(pick_grey(primitives, &bg_contrast, 7.0, 100)),
    );
    out.insert(
        "v2-icon-icon-accent".to_string(),
        if is_dark {
            "var(--v2-blue-400)".to_string()
        } else {
            "var(--v2-blue-600)".to_string()
        },
    );
    out.insert(
        "v2-icon-icon-accent-hover".to_string(),
        if is_dark {
            "var(--v2-blue-300)".to_string()
        } else {
            "var(--v2-blue-700)".to_string()
        },
    );
    out
}
