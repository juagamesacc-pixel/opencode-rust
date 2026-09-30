//! Rust port of `packages/ui/src/theme/resolve.ts` (opencode v1.18.30).
//!
//! 1:1 — token computation order and formulas preserved verbatim.

#![allow(dead_code)]

use super::color::{
    blend, generate_neutral_scale, generate_scale, hex_to_oklch, hex_to_rgb, shift, with_alpha,
    Shift,
};
use super::types::{ColorValue, DesktopTheme, HexColor, ResolvedTheme, ThemeVariant};

/// 1:1 with TS `resolveThemeVariant`.
pub fn resolve_theme_variant(variant: &ThemeVariant, is_dark: bool) -> ResolvedTheme {
    let colors = get_colors(variant);
    let overrides: std::collections::BTreeMap<String, ColorValue> =
        variant.overrides.clone().unwrap_or_default();

    let neutral = generate_neutral_scale(&colors.neutral, is_dark, colors.ink.as_deref());
    let primary = generate_scale(&colors.primary, is_dark);
    let accent = generate_scale(&colors.accent, is_dark);
    let success = generate_scale(&colors.success, is_dark);
    let warning = generate_scale(&colors.warning, is_dark);
    let error = generate_scale(&colors.error, is_dark);
    let info = generate_scale(&colors.info, is_dark);
    let interactive = generate_scale(&colors.interactive, is_dark);
    let amber = generate_scale(
        &shift(
            &colors.warning,
            if is_dark {
                Shift {
                    h: Some(-16.0),
                    l: Some(-0.058),
                    c: Some(1.14),
                }
            } else {
                Shift {
                    h: Some(-22.0),
                    l: Some(-0.082),
                    c: Some(0.94),
                }
            },
        ),
        is_dark,
    );
    let blue = generate_scale(
        &shift(
            &colors.interactive,
            Shift {
                h: Some(-12.0),
                l: Some(0.128),
                c: Some(1.12),
            },
        ),
        is_dark,
    );
    let diff_add = generate_scale(
        &colors.diff_add.clone().unwrap_or_else(|| {
            shift(
                &colors.success,
                Shift {
                    c: Some(if is_dark { 0.7 } else { 0.55 }),
                    l: Some(if is_dark { -0.18 } else { 0.14 }),
                    h: None,
                },
            )
        }),
        is_dark,
    );
    let diff_delete = generate_scale(
        &colors.diff_delete.clone().unwrap_or_else(|| {
            shift(
                &colors.error,
                Shift {
                    c: Some(if is_dark { 0.82 } else { 0.7 }),
                    l: Some(if is_dark { -0.08 } else { 0.08 }),
                    h: None,
                },
            )
        }),
        is_dark,
    );
    let ink = colors.ink.clone().unwrap_or_else(|| colors.neutral.clone());
    let tint = if colors.compact {
        Some(hex_to_oklch(&ink))
    } else {
        None
    };
    let body: Option<HexColor> = tint.map(|t| {
        shift(
            &ink,
            Shift {
                l: Some(if is_dark {
                    (0.88 - t.l).max(0.0) * 0.4
                } else {
                    -((t.l - 0.18).max(0.0)) * 0.24
                }),
                c: Some(if is_dark { 1.04 } else { 1.02 }),
                h: None,
            },
        )
    });
    let background_override = overrides.get("background-base").cloned();
    let background_hex = background_override.as_deref().and_then(get_hex);
    let overlay = background_override.is_some() && background_hex.is_none();
    let content = |seed: &str, scale: &[HexColor]| -> HexColor {
        let base = hex_to_oklch(seed);
        let value: HexColor = if is_dark {
            if base.l > 0.84 {
                shift(
                    seed,
                    Shift {
                        c: Some(1.18),
                        ..Default::default()
                    },
                )
            } else {
                scale[10].clone()
            }
        } else {
            scale[10].clone()
        };
        shift(
            &value,
            Shift {
                l: Some(if is_dark { 0.034 } else { -0.024 }),
                c: Some(if is_dark { 1.3 } else { 1.18 }),
                h: None,
            },
        )
    };
    let modified = || -> HexColor {
        if !colors.compact {
            return if is_dark {
                "#ffba92".to_string()
            } else {
                "#FF8C00".to_string()
            };
        }
        let warning_hue = hex_to_oklch(&colors.warning).h;
        let delete_hue = hex_to_oklch(colors.diff_delete.as_deref().unwrap_or(&colors.error)).h;
        let delta = (((delete_hue - warning_hue) % 360.0 + 540.0) % 360.0 - 180.0).abs();
        if delta < 48.0 {
            return if is_dark {
                "#ffba92".to_string()
            } else {
                "#FF8C00".to_string()
            };
        }
        content(&colors.warning, &warning)
    };
    struct Surface {
        base: ColorValue,
        weak: ColorValue,
        weaker: ColorValue,
        strong: ColorValue,
        stronger: ColorValue,
    }
    let background: HexColor = background_hex.clone().unwrap_or_else(|| neutral[0].clone());
    let alpha_tone = |color: &str, alpha: f64| -> ColorValue {
        if overlay {
            with_alpha(color, alpha)
        } else {
            blend(color, &background, alpha)
        }
    };
    let border_tone = |light: f64, dark: f64| -> ColorValue {
        alpha_tone(
            &ink,
            if is_dark {
                (dark + 0.024 + if colors.compact { 0.08 } else { 0.0 }).min(1.0)
            } else {
                (light + 0.024).min(1.0)
            },
        )
    };
    let surface =
        |seed: &str, base: f64, weak: f64, weaker: f64, strong: f64, stronger: f64| -> Surface {
            Surface {
                base: alpha_tone(seed, base),
                weak: alpha_tone(seed, weak),
                weaker: alpha_tone(seed, weaker),
                strong: alpha_tone(seed, strong),
                stronger: alpha_tone(seed, stronger),
            }
        };
    let diff_hidden_surface = surface(
        &if is_dark {
            shift(
                &colors.interactive,
                Shift {
                    c: Some(0.55),
                    l: Some(0.0),
                    h: None,
                },
            )
        } else {
            shift(
                &colors.interactive,
                Shift {
                    c: Some(0.45),
                    l: Some(0.08),
                    h: None,
                },
            )
        },
        if is_dark { 0.14 } else { 0.12 },
        0.08,
        if is_dark { 0.18 } else { 0.16 },
        if is_dark { 0.26 } else { 0.24 },
        if is_dark { 0.42 } else { 0.36 },
    );

    let neutral_alpha = generate_neutral_alpha_scale(&neutral, is_dark);
    let brandb = primary[8].clone();
    let brandh = primary[9].clone();
    let interb = interactive[if is_dark { 6 } else { 4 }].clone();
    let interh = interactive[if is_dark { 7 } else { 5 }].clone();
    let interw = interactive[if is_dark { 5 } else { 3 }].clone();
    let succb = success[if is_dark { 6 } else { 4 }].clone();
    let succw = success[if is_dark { 5 } else { 3 }].clone();
    let succs = success[10].clone();
    let warnb = warning[if is_dark { 6 } else { 4 }].clone();
    let warnw = warning[if is_dark { 5 } else { 3 }].clone();
    let warns = warning[10].clone();
    let critb = error[if is_dark { 6 } else { 4 }].clone();
    let critw = error[if is_dark { 5 } else { 3 }].clone();
    let crits = error[10].clone();
    let infob = info[if is_dark { 6 } else { 4 }].clone();
    let infow = info[if is_dark { 5 } else { 3 }].clone();
    let infos = info[10].clone();
    let lum = |hex: &str| -> f64 {
        let (r, g, b) = hex_to_rgb(hex);
        let lift = |v: f64| {
            if v <= 0.04045 {
                v / 12.92
            } else {
                ((v + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * lift(r) + 0.7152 * lift(g) + 0.0722 * lift(b)
    };
    let hit = |a: &str, b: &str| -> f64 {
        let x = lum(a);
        let y = lum(b);
        (x.max(y) + 0.05) / (x.min(y) + 0.05)
    };
    let on = |fill: &str| -> HexColor {
        let light = "#ffffff";
        let dark = "#000000";
        if hit(light, fill) > hit(dark, fill) {
            light.to_string()
        } else {
            dark.to_string()
        }
    };

    let mut tokens: ResolvedTheme = std::collections::BTreeMap::new();
    let mut set = |k: &str, v: ColorValue| {
        tokens.insert(k.to_string(), v);
    };

    set("background-base", neutral[0].clone());
    set("background-weak", neutral[2].clone());
    set("background-strong", neutral[0].clone());
    set(
        "background-stronger",
        if is_dark {
            neutral[1].clone()
        } else {
            "#fcfcfc".to_string()
        },
    );

    set("surface-base", neutral_alpha[1].clone());
    set("base", neutral_alpha[1].clone());
    set("surface-base-hover", neutral_alpha[2].clone());
    set("surface-base-active", neutral_alpha[2].clone());
    set(
        "surface-base-interactive-active",
        with_alpha(&interactive[2], 0.3),
    );
    set("base2", neutral_alpha[1].clone());
    set("base3", neutral_alpha[1].clone());
    set("surface-inset-base", neutral_alpha[1].clone());
    set("surface-inset-base-hover", neutral_alpha[2].clone());
    set(
        "surface-inset-strong",
        if is_dark {
            with_alpha(&neutral[0], 0.5)
        } else {
            with_alpha(&neutral[3], 0.09)
        },
    );
    set(
        "surface-inset-strong-hover",
        tokens["surface-inset-strong"].clone(),
    );
    set("surface-raised-base", neutral_alpha[0].clone());
    set(
        "surface-float-base",
        if is_dark {
            neutral[1].clone()
        } else {
            neutral[11].clone()
        },
    );
    set(
        "surface-float-base-hover",
        if is_dark {
            neutral[2].clone()
        } else {
            neutral[10].clone()
        },
    );
    set("surface-raised-base-hover", neutral_alpha[1].clone());
    set("surface-raised-base-active", neutral_alpha[2].clone());
    set(
        "surface-raised-strong",
        if is_dark {
            neutral_alpha[3].clone()
        } else {
            neutral[0].clone()
        },
    );
    set(
        "surface-raised-strong-hover",
        if is_dark {
            neutral_alpha[5].clone()
        } else {
            "#ffffff".to_string()
        },
    );
    set(
        "surface-raised-stronger",
        if is_dark {
            neutral_alpha[5].clone()
        } else {
            "#ffffff".to_string()
        },
    );
    set(
        "surface-raised-stronger-hover",
        if is_dark {
            neutral_alpha[6].clone()
        } else {
            "#ffffff".to_string()
        },
    );
    set("surface-weak", neutral_alpha[2].clone());
    set("surface-weaker", neutral_alpha[3].clone());
    set(
        "surface-strong",
        if is_dark {
            neutral_alpha[6].clone()
        } else {
            "#ffffff".to_string()
        },
    );
    set(
        "surface-raised-stronger-non-alpha",
        if is_dark {
            neutral[2].clone()
        } else {
            "#ffffff".to_string()
        },
    );

    set("surface-brand-base", brandb.clone());
    set("surface-brand-hover", brandh.clone());

    set("surface-interactive-base", interb.clone());
    set("surface-interactive-hover", interh.clone());
    set("surface-interactive-weak", interw.clone());
    set("surface-interactive-weak-hover", interb.clone());

    set("surface-success-base", succb.clone());
    set("surface-success-weak", succw.clone());
    set("surface-success-strong", succs.clone());
    set("surface-warning-base", warnb.clone());
    set("surface-warning-weak", warnw.clone());
    set("surface-warning-strong", warns.clone());
    set("surface-critical-base", critb.clone());
    set("surface-critical-weak", critw.clone());
    set("surface-critical-strong", crits.clone());
    set("surface-info-base", infob.clone());
    set("surface-info-weak", infow.clone());
    set("surface-info-strong", infos.clone());

    set(
        "surface-diff-unchanged-base",
        if is_dark {
            neutral[0].clone()
        } else {
            "#ffffff00".to_string()
        },
    );
    set(
        "surface-diff-skip-base",
        if is_dark {
            neutral_alpha[0].clone()
        } else {
            neutral[1].clone()
        },
    );
    set("surface-diff-hidden-base", diff_hidden_surface.base);
    set("surface-diff-hidden-weak", diff_hidden_surface.weak);
    set("surface-diff-hidden-weaker", diff_hidden_surface.weaker);
    set("surface-diff-hidden-strong", diff_hidden_surface.strong);
    set("surface-diff-hidden-stronger", diff_hidden_surface.stronger);
    set("surface-diff-add-base", diff_add[2].clone());
    set(
        "surface-diff-add-weak",
        diff_add[if is_dark { 3 } else { 1 }].clone(),
    );
    set(
        "surface-diff-add-weaker",
        diff_add[if is_dark { 2 } else { 0 }].clone(),
    );
    set("surface-diff-add-strong", diff_add[4].clone());
    set(
        "surface-diff-add-stronger",
        diff_add[if is_dark { 10 } else { 8 }].clone(),
    );
    set("surface-diff-delete-base", diff_delete[2].clone());
    set(
        "surface-diff-delete-weak",
        diff_delete[if is_dark { 3 } else { 1 }].clone(),
    );
    set(
        "surface-diff-delete-weaker",
        diff_delete[if is_dark { 2 } else { 0 }].clone(),
    );
    set(
        "surface-diff-delete-strong",
        diff_delete[if is_dark { 4 } else { 5 }].clone(),
    );
    set(
        "surface-diff-delete-stronger",
        diff_delete[if is_dark { 10 } else { 8 }].clone(),
    );

    set(
        "input-base",
        if is_dark {
            neutral[1].clone()
        } else {
            neutral[0].clone()
        },
    );
    set(
        "input-hover",
        if is_dark {
            neutral[2].clone()
        } else {
            neutral[1].clone()
        },
    );
    set(
        "input-active",
        if is_dark {
            interactive[6].clone()
        } else {
            interactive[0].clone()
        },
    );
    set(
        "input-selected",
        if is_dark {
            interactive[7].clone()
        } else {
            interactive[3].clone()
        },
    );
    set(
        "input-focus",
        if is_dark {
            interactive[6].clone()
        } else {
            interactive[0].clone()
        },
    );
    set("input-disabled", neutral[3].clone());

    let body_hex: HexColor = body.clone().unwrap_or_default();
    set(
        "text-base",
        if colors.compact {
            body_hex.clone()
        } else {
            neutral[10].clone()
        },
    );
    set(
        "text-weak",
        if colors.compact {
            shift(
                &body_hex,
                Shift {
                    l: Some(if is_dark { -0.11 } else { 0.11 }),
                    c: Some(0.9),
                    h: None,
                },
            )
        } else {
            neutral[8].clone()
        },
    );
    set(
        "text-weaker",
        if colors.compact {
            shift(
                &body_hex,
                Shift {
                    l: Some(if is_dark { -0.2 } else { 0.21 }),
                    c: Some(if is_dark { 0.78 } else { 0.72 }),
                    h: None,
                },
            )
        } else {
            neutral[7].clone()
        },
    );
    set(
        "text-strong",
        if colors.compact {
            if is_dark {
                blend("#ffffff", &body_hex, 0.9)
            } else {
                shift(
                    &body_hex,
                    Shift {
                        l: Some(-0.07),
                        c: Some(1.04),
                        h: None,
                    },
                )
            }
        } else {
            neutral[11].clone()
        },
    );
    set(
        "text-invert-base",
        if is_dark {
            neutral[10].clone()
        } else {
            neutral[1].clone()
        },
    );
    set(
        "text-invert-weak",
        if is_dark {
            neutral[8].clone()
        } else {
            neutral[2].clone()
        },
    );
    set(
        "text-invert-weaker",
        if is_dark {
            neutral[7].clone()
        } else {
            neutral[3].clone()
        },
    );
    set(
        "text-invert-strong",
        if is_dark {
            neutral[11].clone()
        } else {
            neutral[0].clone()
        },
    );
    set(
        "text-interactive-base",
        interactive[if is_dark { 10 } else { 9 }].clone(),
    );
    set("text-on-brand-base", on(&brandb));
    set("text-on-interactive-base", on(&interb));
    set("text-on-interactive-weak", on(&interb));
    set("text-on-success-base", on(&succb));
    set("text-on-critical-base", on(&critb));
    set("text-on-critical-weak", on(&critb));
    set("text-on-critical-strong", on(&crits));
    set("text-on-warning-base", on(&warnb));
    set("text-on-info-base", on(&infob));
    set("text-diff-add-base", diff_add[10].clone());
    set("text-diff-delete-base", diff_delete[9].clone());
    set("text-diff-delete-strong", diff_delete[11].clone());
    set(
        "text-diff-add-strong",
        diff_add[if is_dark { 7 } else { 11 }].clone(),
    );
    set("text-on-info-weak", on(&infob));
    set("text-on-info-strong", on(&infos));
    set("text-on-warning-weak", on(&warnb));
    set("text-on-warning-strong", on(&warns));
    set("text-on-success-weak", on(&succb));
    set("text-on-success-strong", on(&succs));
    set("text-on-brand-weak", on(&brandb));
    set("text-on-brand-weaker", on(&brandb));
    set("text-on-brand-strong", on(&brandh));

    set("button-primary-base", neutral[11].clone());
    set(
        "button-secondary-base",
        if is_dark {
            neutral[2].clone()
        } else {
            neutral[0].clone()
        },
    );
    set(
        "button-secondary-hover",
        if is_dark {
            neutral[3].clone()
        } else {
            neutral[1].clone()
        },
    );
    set("button-ghost-hover", neutral_alpha[1].clone());
    set("button-ghost-hover2", neutral_alpha[2].clone());

    set(
        "border-base",
        if colors.compact {
            border_tone(0.22, 0.16)
        } else {
            neutral_alpha[6].clone()
        },
    );
    set(
        "border-hover",
        if colors.compact {
            border_tone(0.28, 0.2)
        } else {
            neutral_alpha[7].clone()
        },
    );
    set(
        "border-active",
        if colors.compact {
            border_tone(0.34, 0.24)
        } else {
            neutral_alpha[8].clone()
        },
    );
    set(
        "border-selected",
        with_alpha(&interactive[8], if is_dark { 0.9 } else { 0.99 }),
    );
    set(
        "border-disabled",
        if colors.compact {
            border_tone(0.18, 0.12)
        } else {
            neutral_alpha[7].clone()
        },
    );
    set(
        "border-focus",
        if colors.compact {
            border_tone(0.34, 0.24)
        } else {
            neutral_alpha[8].clone()
        },
    );
    set(
        "border-weak-base",
        if colors.compact {
            border_tone(0.1, 0.08)
        } else {
            neutral_alpha[if is_dark { 5 } else { 4 }].clone()
        },
    );
    set(
        "border-strong-base",
        if colors.compact {
            border_tone(0.34, 0.24)
        } else {
            neutral_alpha[if is_dark { 7 } else { 6 }].clone()
        },
    );
    set(
        "border-strong-hover",
        if colors.compact {
            border_tone(0.4, 0.28)
        } else {
            neutral_alpha[7].clone()
        },
    );
    set(
        "border-strong-active",
        if colors.compact {
            border_tone(0.46, 0.32)
        } else {
            neutral_alpha[if is_dark { 7 } else { 6 }].clone()
        },
    );
    set("border-strong-selected", with_alpha(&interactive[5], 0.6));
    set(
        "border-strong-disabled",
        if colors.compact {
            border_tone(0.14, 0.1)
        } else {
            neutral_alpha[5].clone()
        },
    );
    set(
        "border-strong-focus",
        if colors.compact {
            border_tone(0.46, 0.32)
        } else {
            neutral_alpha[if is_dark { 7 } else { 6 }].clone()
        },
    );
    set(
        "border-weak-hover",
        if colors.compact {
            border_tone(0.16, 0.12)
        } else {
            neutral_alpha[if is_dark { 6 } else { 5 }].clone()
        },
    );
    set(
        "border-weak-active",
        if colors.compact {
            border_tone(0.22, 0.16)
        } else {
            neutral_alpha[if is_dark { 7 } else { 6 }].clone()
        },
    );
    set(
        "border-weak-selected",
        with_alpha(&interactive[4], if is_dark { 0.6 } else { 0.5 }),
    );
    set(
        "border-weak-disabled",
        if colors.compact {
            border_tone(0.08, 0.06)
        } else {
            neutral_alpha[5].clone()
        },
    );
    set(
        "border-weak-focus",
        if colors.compact {
            border_tone(0.22, 0.16)
        } else {
            neutral_alpha[if is_dark { 7 } else { 6 }].clone()
        },
    );
    set(
        "border-weaker-base",
        if colors.compact {
            border_tone(0.06, 0.04)
        } else {
            neutral_alpha[2].clone()
        },
    );

    set("border-interactive-base", interactive[6].clone());
    set("border-interactive-hover", interactive[7].clone());
    set("border-interactive-active", interactive[8].clone());
    set("border-interactive-selected", interactive[8].clone());
    set("border-interactive-disabled", neutral[7].clone());
    set("border-interactive-focus", interactive[8].clone());

    set("border-success-base", success[6].clone());
    set("border-success-hover", success[7].clone());
    set("border-success-selected", success[8].clone());
    set("border-warning-base", warning[6].clone());
    set("border-warning-hover", warning[7].clone());
    set("border-warning-selected", warning[8].clone());
    set("border-critical-base", error[6].clone());
    set("border-critical-hover", error[7].clone());
    set("border-critical-selected", error[8].clone());
    set("border-info-base", info[6].clone());
    set("border-info-hover", info[7].clone());
    set("border-info-selected", info[8].clone());
    set("border-color", "#ffffff".to_string());

    set(
        "icon-base",
        if colors.compact && !is_dark {
            tokens["text-weak"].clone()
        } else {
            neutral[if is_dark { 9 } else { 8 }].clone()
        },
    );
    set(
        "icon-hover",
        if colors.compact && !is_dark {
            tokens["text-base"].clone()
        } else {
            neutral[10].clone()
        },
    );
    set(
        "icon-active",
        if colors.compact && !is_dark {
            tokens["text-strong"].clone()
        } else {
            neutral[11].clone()
        },
    );
    set(
        "icon-selected",
        if colors.compact && !is_dark {
            tokens["text-strong"].clone()
        } else {
            neutral[11].clone()
        },
    );
    set(
        "icon-disabled",
        neutral[if is_dark { 6 } else { 7 }].clone(),
    );
    set(
        "icon-focus",
        if colors.compact && !is_dark {
            tokens["text-strong"].clone()
        } else {
            neutral[11].clone()
        },
    );
    set(
        "icon-invert-base",
        if is_dark {
            neutral[0].clone()
        } else {
            "#ffffff".to_string()
        },
    );
    set(
        "icon-weak-base",
        neutral[if is_dark { 5 } else { 6 }].clone(),
    );
    set(
        "icon-weak-hover",
        neutral[if is_dark { 11 } else { 7 }].clone(),
    );
    set("icon-weak-active", neutral[8].clone());
    set(
        "icon-weak-selected",
        neutral[if is_dark { 8 } else { 9 }].clone(),
    );
    set(
        "icon-weak-disabled",
        neutral[if is_dark { 3 } else { 5 }].clone(),
    );
    set("icon-weak-focus", neutral[8].clone());
    set("icon-strong-base", neutral[11].clone());
    set(
        "icon-strong-hover",
        if is_dark {
            "#f6f3f3".to_string()
        } else {
            "#151313".to_string()
        },
    );
    set(
        "icon-strong-active",
        if is_dark {
            "#fcfcfc".to_string()
        } else {
            "#020202".to_string()
        },
    );
    set(
        "icon-strong-selected",
        if is_dark {
            "#fdfcfc".to_string()
        } else {
            "#020202".to_string()
        },
    );
    set("icon-strong-disabled", neutral[7].clone());
    set(
        "icon-strong-focus",
        if is_dark {
            "#fdfcfc".to_string()
        } else {
            "#020202".to_string()
        },
    );
    set(
        "icon-brand-base",
        if is_dark {
            "#ffffff".to_string()
        } else {
            neutral[11].clone()
        },
    );
    set("icon-interactive-base", interactive[8].clone());
    set(
        "icon-success-base",
        success[if is_dark { 8 } else { 6 }].clone(),
    );
    set("icon-success-hover", success[9].clone());
    set("icon-success-active", success[10].clone());
    set(
        "icon-warning-base",
        amber[if is_dark { 8 } else { 6 }].clone(),
    );
    set("icon-warning-hover", amber[9].clone());
    set("icon-warning-active", amber[10].clone());
    set(
        "icon-critical-base",
        error[if is_dark { 8 } else { 9 }].clone(),
    );
    set("icon-critical-hover", error[9].clone());
    set("icon-critical-active", error[10].clone());
    set("icon-info-base", info[if is_dark { 8 } else { 6 }].clone());
    set("icon-info-hover", info[if is_dark { 9 } else { 7 }].clone());
    set("icon-info-active", info[10].clone());
    set("icon-on-brand-base", on(&brandb));
    set("icon-on-brand-hover", on(&brandh));
    set("icon-on-brand-selected", on(&brandh));
    set("icon-on-interactive-base", on(&interb));

    set("icon-agent-plan-base", info[8].clone());
    set("icon-agent-docs-base", amber[8].clone());
    set("icon-agent-ask-base", blue[8].clone());
    set(
        "icon-agent-build-base",
        interactive[if is_dark { 10 } else { 8 }].clone(),
    );

    set("icon-on-success-base", on(&succb));
    set("icon-on-success-hover", on(&succs));
    set("icon-on-success-selected", on(&succs));
    set("icon-on-warning-base", on(&warnb));
    set("icon-on-warning-hover", on(&warns));
    set("icon-on-warning-selected", on(&warns));
    set("icon-on-critical-base", on(&critb));
    set("icon-on-critical-hover", on(&crits));
    set("icon-on-critical-selected", on(&crits));
    set("icon-on-info-base", on(&infob));
    set("icon-on-info-hover", on(&infos));
    set("icon-on-info-selected", on(&infos));

    set("icon-diff-add-base", diff_add[10].clone());
    set(
        "icon-diff-add-hover",
        diff_add[if is_dark { 9 } else { 11 }].clone(),
    );
    set(
        "icon-diff-add-active",
        diff_add[if is_dark { 10 } else { 11 }].clone(),
    );
    set("icon-diff-delete-base", diff_delete[9].clone());
    set("icon-diff-delete-hover", diff_delete[10].clone());
    set("icon-diff-modified-base", modified());

    if colors.compact {
        set("syntax-comment", "var(--text-weak)".to_string());
        set("syntax-regexp", "var(--text-base)".to_string());
        set("syntax-string", content(&colors.success, &success));
        set("syntax-keyword", content(&colors.accent, &accent));
        set("syntax-primitive", content(&colors.primary, &primary));
        set(
            "syntax-operator",
            if is_dark {
                "var(--text-weak)".to_string()
            } else {
                "var(--text-base)".to_string()
            },
        );
        set("syntax-variable", "var(--text-strong)".to_string());
        set("syntax-property", content(&colors.info, &info));
        set("syntax-type", content(&colors.warning, &warning));
        set("syntax-constant", content(&colors.accent, &accent));
        set(
            "syntax-punctuation",
            if is_dark {
                "var(--text-weak)".to_string()
            } else {
                "var(--text-base)".to_string()
            },
        );
        set("syntax-object", "var(--text-strong)".to_string());
        set("syntax-success", success[10].clone());
        set("syntax-warning", amber[10].clone());
        set("syntax-critical", error[10].clone());
        set("syntax-info", content(&colors.info, &info));
        set("syntax-diff-add", diff_add[10].clone());
        set("syntax-diff-delete", diff_delete[10].clone());
        set("syntax-diff-unknown", "#ff0000".to_string());

        set("markdown-heading", content(&colors.primary, &primary));
        set("markdown-text", tokens["text-base"].clone());
        set("markdown-link", content(&colors.interactive, &interactive));
        set("markdown-link-text", content(&colors.info, &info));
        set("markdown-code", content(&colors.success, &success));
        set("markdown-block-quote", content(&colors.warning, &warning));
        set("markdown-emph", content(&colors.warning, &warning));
        set("markdown-strong", content(&colors.accent, &accent));
        set("markdown-horizontal-rule", tokens["border-base"].clone());
        set(
            "markdown-list-item",
            content(&colors.interactive, &interactive),
        );
        set("markdown-list-enumeration", content(&colors.info, &info));
        set("markdown-image", content(&colors.interactive, &interactive));
        set("markdown-image-text", content(&colors.info, &info));
        set("markdown-code-block", tokens["text-base"].clone());
    }

    if !colors.compact {
        set("syntax-comment", "var(--text-weak)".to_string());
        set("syntax-regexp", "var(--text-base)".to_string());
        set(
            "syntax-string",
            if is_dark {
                "#00ceb9".to_string()
            } else {
                "#006656".to_string()
            },
        );
        set("syntax-keyword", "var(--text-weak)".to_string());
        set(
            "syntax-primitive",
            if is_dark {
                "#ffba92".to_string()
            } else {
                "#fb4804".to_string()
            },
        );
        set(
            "syntax-operator",
            if is_dark {
                "var(--text-weak)".to_string()
            } else {
                "var(--text-base)".to_string()
            },
        );
        set("syntax-variable", "var(--text-strong)".to_string());
        set(
            "syntax-property",
            if is_dark {
                "#ff9ae2".to_string()
            } else {
                "#ed6dc8".to_string()
            },
        );
        set(
            "syntax-type",
            if is_dark {
                "#ecf58c".to_string()
            } else {
                "#596600".to_string()
            },
        );
        set(
            "syntax-constant",
            if is_dark {
                "#93e9f6".to_string()
            } else {
                "#007b80".to_string()
            },
        );
        set(
            "syntax-punctuation",
            if is_dark {
                "var(--text-weak)".to_string()
            } else {
                "var(--text-base)".to_string()
            },
        );
        set("syntax-object", "var(--text-strong)".to_string());
        set("syntax-success", success[10].clone());
        set("syntax-warning", amber[10].clone());
        set("syntax-critical", error[10].clone());
        set(
            "syntax-info",
            if is_dark {
                "#93e9f6".to_string()
            } else {
                "#0092a8".to_string()
            },
        );
        set("syntax-diff-add", diff_add[10].clone());
        set("syntax-diff-delete", diff_delete[10].clone());
        set("syntax-diff-unknown", "#ff0000".to_string());

        set(
            "markdown-heading",
            if is_dark {
                "#9d7cd8".to_string()
            } else {
                "#d68c27".to_string()
            },
        );
        set(
            "markdown-text",
            if is_dark {
                "#eeeeee".to_string()
            } else {
                "#1a1a1a".to_string()
            },
        );
        set(
            "markdown-link",
            if is_dark {
                "#fab283".to_string()
            } else {
                "#3b7dd8".to_string()
            },
        );
        set(
            "markdown-link-text",
            if is_dark {
                "#56b6c2".to_string()
            } else {
                "#318795".to_string()
            },
        );
        set(
            "markdown-code",
            if is_dark {
                "#7fd88f".to_string()
            } else {
                "#3d9a57".to_string()
            },
        );
        set(
            "markdown-block-quote",
            if is_dark {
                "#e5c07b".to_string()
            } else {
                "#b0851f".to_string()
            },
        );
        set(
            "markdown-emph",
            if is_dark {
                "#e5c07b".to_string()
            } else {
                "#b0851f".to_string()
            },
        );
        set(
            "markdown-strong",
            if is_dark {
                "#f5a742".to_string()
            } else {
                "#d68c27".to_string()
            },
        );
        set(
            "markdown-horizontal-rule",
            if is_dark {
                "#808080".to_string()
            } else {
                "#8a8a8a".to_string()
            },
        );
        set(
            "markdown-list-item",
            if is_dark {
                "#fab283".to_string()
            } else {
                "#3b7dd8".to_string()
            },
        );
        set(
            "markdown-list-enumeration",
            if is_dark {
                "#56b6c2".to_string()
            } else {
                "#318795".to_string()
            },
        );
        set(
            "markdown-image",
            if is_dark {
                "#fab283".to_string()
            } else {
                "#3b7dd8".to_string()
            },
        );
        set(
            "markdown-image-text",
            if is_dark {
                "#56b6c2".to_string()
            } else {
                "#318795".to_string()
            },
        );
        set(
            "markdown-code-block",
            if is_dark {
                "#eeeeee".to_string()
            } else {
                "#1a1a1a".to_string()
            },
        );
    }

    set(
        "avatar-background-pink",
        if is_dark {
            "#501b3f".to_string()
        } else {
            "#feeef8".to_string()
        },
    );
    set(
        "avatar-background-mint",
        if is_dark {
            "#033a34".to_string()
        } else {
            "#e1fbf4".to_string()
        },
    );
    set(
        "avatar-background-orange",
        if is_dark {
            "#5f2a06".to_string()
        } else {
            "#fff1e7".to_string()
        },
    );
    set(
        "avatar-background-purple",
        if is_dark {
            "#432155".to_string()
        } else {
            "#f9f1fe".to_string()
        },
    );
    set(
        "avatar-background-cyan",
        if is_dark {
            "#0f3058".to_string()
        } else {
            "#e7f9fb".to_string()
        },
    );
    set(
        "avatar-background-lime",
        if is_dark {
            "#2b3711".to_string()
        } else {
            "#eefadc".to_string()
        },
    );
    set(
        "avatar-text-pink",
        if is_dark {
            "#e34ba9".to_string()
        } else {
            "#cd1d8d".to_string()
        },
    );
    set(
        "avatar-text-mint",
        if is_dark {
            "#95f3d9".to_string()
        } else {
            "#147d6f".to_string()
        },
    );
    set(
        "avatar-text-orange",
        if is_dark {
            "#ff802b".to_string()
        } else {
            "#ed5f00".to_string()
        },
    );
    set(
        "avatar-text-purple",
        if is_dark {
            "#9d5bd2".to_string()
        } else {
            "#8445bc".to_string()
        },
    );
    set(
        "avatar-text-cyan",
        if is_dark {
            "#369eff".to_string()
        } else {
            "#0894b3".to_string()
        },
    );
    set(
        "avatar-text-lime",
        if is_dark {
            "#c4f042".to_string()
        } else {
            "#5d770d".to_string()
        },
    );

    for (key, value) in overrides.iter() {
        tokens.insert(key.clone(), value.clone());
    }

    if colors.compact
        && overrides.contains_key("text-weak")
        && !overrides.contains_key("text-weaker")
    {
        let weak = tokens["text-weak"].clone();
        if weak.starts_with('#') {
            tokens.insert(
                "text-weaker".to_string(),
                shift(
                    &weak,
                    Shift {
                        l: Some(if is_dark { -0.12 } else { 0.12 }),
                        c: Some(0.75),
                        h: None,
                    },
                ),
            );
        } else {
            tokens.insert("text-weaker".to_string(), weak);
        }
    }

    if colors.compact {
        if !overrides.contains_key("markdown-text") {
            let v = tokens["text-base"].clone();
            tokens.insert("markdown-text".to_string(), v);
        }
        if !overrides.contains_key("markdown-code-block") {
            let v = tokens["text-base"].clone();
            tokens.insert("markdown-code-block".to_string(), v);
        }
    }

    if !overrides.contains_key("text-stronger") {
        let v = tokens["text-strong"].clone();
        tokens.insert("text-stronger".to_string(), v);
    }

    tokens
}

struct ThemeColors {
    compact: bool,
    neutral: HexColor,
    ink: Option<HexColor>,
    primary: HexColor,
    accent: HexColor,
    success: HexColor,
    warning: HexColor,
    error: HexColor,
    info: HexColor,
    interactive: HexColor,
    diff_add: Option<HexColor>,
    diff_delete: Option<HexColor>,
}

fn get_colors(variant: &ThemeVariant) -> ThemeColors {
    if variant.palette.is_some() && variant.seeds.is_some() {
        panic!("Theme variant cannot define both `palette` and `seeds`");
    }

    if let Some(palette) = &variant.palette {
        return ThemeColors {
            compact: true,
            neutral: palette.neutral.clone(),
            ink: Some(palette.ink.clone()),
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
            diff_add: palette.diff_add.clone(),
            diff_delete: palette.diff_delete.clone(),
        };
    }

    if let Some(seeds) = &variant.seeds {
        return ThemeColors {
            compact: false,
            neutral: seeds.neutral.clone(),
            ink: None,
            primary: seeds.primary.clone(),
            accent: seeds.info.clone(),
            success: seeds.success.clone(),
            warning: seeds.warning.clone(),
            error: seeds.error.clone(),
            info: seeds.info.clone(),
            interactive: seeds.interactive.clone(),
            diff_add: Some(seeds.diff_add.clone()),
            diff_delete: Some(seeds.diff_delete.clone()),
        };
    }

    panic!("Theme variant requires `palette` or `seeds`");
}

fn generate_neutral_alpha_scale(neutral_scale: &[HexColor], is_dark: bool) -> Vec<HexColor> {
    let alphas: Vec<f64> = if is_dark {
        vec![
            0.038, 0.066, 0.1, 0.142, 0.19, 0.252, 0.334, 0.446, 0.58, 0.718, 0.854, 0.985,
        ]
    } else {
        vec![
            0.03, 0.06, 0.1, 0.145, 0.2, 0.265, 0.35, 0.47, 0.61, 0.74, 0.86, 0.97,
        ]
    };

    alphas
        .iter()
        .map(|alpha| blend(&neutral_scale[11], &neutral_scale[0], *alpha))
        .collect()
}

fn get_hex(value: Option<&str>) -> Option<HexColor> {
    match value {
        Some(v) if v.starts_with('#') => Some(v.to_string()),
        _ => None,
    }
}

/// 1:1 with the TS `resolveTheme` return `{ light, dark }`.
#[derive(Debug, Clone, Default)]
pub struct ResolvedThemePair {
    pub light: ResolvedTheme,
    pub dark: ResolvedTheme,
}

/// 1:1 with TS `resolveTheme`.
pub fn resolve_theme(theme: &DesktopTheme) -> ResolvedThemePair {
    ResolvedThemePair {
        light: resolve_theme_variant(&theme.light, false),
        dark: resolve_theme_variant(&theme.dark, true),
    }
}

/// 1:1 with TS `themeToCss`.
pub fn theme_to_css(tokens: &ResolvedTheme) -> String {
    tokens
        .iter()
        .map(|(key, value)| format!("--{key}: {value};"))
        .collect::<Vec<_>>()
        .join("\n  ")
}
