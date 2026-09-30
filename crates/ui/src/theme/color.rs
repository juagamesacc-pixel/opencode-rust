//! Rust port of `packages/ui/src/theme/color.ts` (opencode v1.18.30).
//!
//! 1:1 — every function, step table, and constant preserved verbatim.

#![allow(dead_code)]

use super::types::{HexColor, OklchColor};

fn clamp(v: f64, min: f64, max: f64) -> f64 {
    v.max(min).min(max)
}

fn hue(v: f64) -> f64 {
    ((v % 360.0) + 360.0) % 360.0
}

/// 1:1 with TS `hexToRgb`.
pub fn hex_to_rgb(hex: &str) -> (f64, f64, f64) {
    let h = hex.trim_start_matches('#');
    let full: String = if h.len() == 3 || h.len() == 4 {
        h.chars().flat_map(|c| [c, c]).collect()
    } else {
        h.to_string()
    };
    let rgb = if full.len() == 8 {
        full[..6].to_string()
    } else {
        full
    };
    let num = u32::from_str_radix(&rgb, 16).unwrap_or(0);
    (
        (((num >> 16) & 255) as f64) / 255.0,
        (((num >> 8) & 255) as f64) / 255.0,
        ((num & 255) as f64) / 255.0,
    )
}

/// 1:1 with TS `rgbToHex`.
pub fn rgb_to_hex(r: f64, g: f64, b: f64) -> HexColor {
    let to_hex = |v: f64| -> String {
        let clamped = clamp(v, 0.0, 1.0);
        format!("{:02x}", (clamped * 255.0).round() as u8)
    };
    format!("#{}{}{}", to_hex(r), to_hex(g), to_hex(b))
}

fn linear_to_srgb(c: f64) -> f64 {
    if c <= 0.0031308 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

fn srgb_to_linear(c: f64) -> f64 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// 1:1 with TS `rgbToOklch`.
pub fn rgb_to_oklch(r: f64, g: f64, b: f64) -> OklchColor {
    let lr = srgb_to_linear(r);
    let lg = srgb_to_linear(g);
    let lb = srgb_to_linear(b);

    let l_ = 0.4122214708 * lr + 0.5363325363 * lg + 0.0514459929 * lb;
    let m_ = 0.2119034982 * lr + 0.6806995451 * lg + 0.1073969566 * lb;
    let s_ = 0.0883024619 * lr + 0.2817188376 * lg + 0.6299787005 * lb;

    let l = l_.cbrt();
    let m = m_.cbrt();
    let s = s_.cbrt();

    let l_out = 0.2104542553 * l + 0.793617785 * m - 0.0040720468 * s;
    let a = 1.9779984951 * l - 2.428592205 * m + 0.4505937099 * s;
    let b_ok = 0.0259040371 * l + 0.7827717662 * m - 0.808675766 * s;

    let c = (a * a + b_ok * b_ok).sqrt();
    let mut h = b_ok.atan2(a) * (180.0 / std::f64::consts::PI);
    if h < 0.0 {
        h += 360.0;
    }

    OklchColor { l: l_out, c, h }
}

/// 1:1 with TS `oklchToRgb`.
pub fn oklch_to_rgb(oklch: &OklchColor) -> (f64, f64, f64) {
    let l0 = oklch.l;
    let c = oklch.c;
    let h = oklch.h;

    let a = c * (h * std::f64::consts::PI / 180.0).cos();
    let b = c * (h * std::f64::consts::PI / 180.0).sin();

    let l = l0 + 0.3963377774 * a + 0.2158037573 * b;
    let m = l0 - 0.1055613458 * a - 0.0638541728 * b;
    let s = l0 - 0.0894841775 * a - 1.291485548 * b;

    let l3 = l * l * l;
    let m3 = m * m * m;
    let s3 = s * s * s;

    let lr = 4.0767416621 * l3 - 3.3077115913 * m3 + 0.2309699292 * s3;
    let lg = -1.2684380046 * l3 + 2.6097574011 * m3 - 0.3413193965 * s3;
    let lb = -0.0041960863 * l3 - 0.7034186147 * m3 + 1.707614701 * s3;

    (linear_to_srgb(lr), linear_to_srgb(lg), linear_to_srgb(lb))
}

/// 1:1 with TS `hexToOklch`.
pub fn hex_to_oklch(hex: &str) -> OklchColor {
    let (r, g, b) = hex_to_rgb(hex);
    rgb_to_oklch(r, g, b)
}

/// 1:1 with TS `fitOklch`.
pub fn fit_oklch(oklch: &OklchColor) -> OklchColor {
    let base = OklchColor {
        l: clamp(oklch.l, 0.0, 1.0),
        c: oklch.c.max(0.0),
        h: hue(oklch.h),
    };

    let (r, g, b) = oklch_to_rgb(&base);
    if r >= 0.0 && r <= 1.0 && g >= 0.0 && g <= 1.0 && b >= 0.0 && b <= 1.0 {
        return base;
    }

    let mut c = base.c;
    for _ in 0..24 {
        c *= 0.9;
        let next = OklchColor { c, ..base };
        let (r, g, b) = oklch_to_rgb(&next);
        if r >= 0.0 && r <= 1.0 && g >= 0.0 && g <= 1.0 && b >= 0.0 && b <= 1.0 {
            return next;
        }
    }

    OklchColor { c: 0.0, ..base }
}

/// 1:1 with TS `oklchToHex`.
pub fn oklch_to_hex(oklch: &OklchColor) -> HexColor {
    let fitted = fit_oklch(oklch);
    let (r, g, b) = oklch_to_rgb(&fitted);
    rgb_to_hex(r, g, b)
}

/// 1:1 with TS `generateScale`.
pub fn generate_scale(seed: &str, is_dark: bool) -> Vec<HexColor> {
    let base = hex_to_oklch(seed);

    let light_steps: Vec<f64> = if is_dark {
        vec![
            0.118,
            0.138,
            0.167,
            0.202,
            0.246,
            0.304,
            0.378,
            0.468,
            clamp(base.l * 0.825, 0.53, 0.705),
            clamp(base.l * 0.89, 0.61, 0.79),
            clamp(base.l + 0.033, 0.868, 0.943),
            0.984,
        ]
    } else {
        vec![
            0.993,
            0.983,
            0.962,
            0.936,
            0.906,
            0.866,
            0.811,
            0.74,
            base.l,
            (base.l - 0.036).max(0.0),
            0.49,
            0.27,
        ]
    };

    let chroma_multipliers: Vec<f64> = if is_dark {
        vec![
            0.52, 0.68, 0.86, 1.02, 1.14, 1.24, 1.36, 1.48, 1.56, 1.64, 1.62, 1.15,
        ]
    } else {
        vec![
            0.12, 0.24, 0.46, 0.68, 0.84, 0.98, 1.08, 1.16, 1.22, 1.26, 1.18, 0.98,
        ]
    };

    (0..12)
        .map(|i| {
            oklch_to_hex(&OklchColor {
                l: light_steps[i],
                c: base.c * chroma_multipliers[i],
                h: base.h,
            })
        })
        .collect()
}

/// 1:1 with TS `generateNeutralScale`.
pub fn generate_neutral_scale(seed: &str, is_dark: bool, ink: Option<&str>) -> Vec<HexColor> {
    if let Some(ink) = ink {
        let base = hex_to_oklch(seed);
        let lift = |tone: f64| -> HexColor {
            oklch_to_hex(&OklchColor {
                l: base.l + (1.0 - base.l) * tone,
                c: base.c * (1.0 - tone).max(0.0),
                h: base.h,
            })
        };
        let sink = |tone: f64| -> HexColor {
            oklch_to_hex(&OklchColor {
                l: base.l * (1.0 - tone),
                c: base.c * (1.0 - tone * if is_dark { 0.12 } else { 0.3 }).max(0.0),
                h: base.h,
            })
        };
        let bg = if is_dark {
            sink(clamp(
                0.19 + (base.l - 0.12).max(0.0) * 0.33 + base.c * 1.95,
                0.17,
                0.27,
            ))
        } else if base.l < 0.82 {
            lift(0.86)
        } else {
            lift(clamp(
                0.1 + base.c * 3.2 + (0.95 - base.l).max(0.0) * 0.35,
                0.1,
                0.28,
            ))
        };
        let steps: Vec<f64> = if is_dark {
            vec![
                0.0, 0.018, 0.039, 0.064, 0.097, 0.143, 0.212, 0.31, 0.46, 0.649, 0.845, 0.984,
            ]
        } else {
            vec![
                0.0, 0.022, 0.042, 0.068, 0.102, 0.146, 0.208, 0.296, 0.432, 0.61, 0.81, 0.965,
            ]
        };
        return steps
            .iter()
            .map(|step| mix_colors(&bg, ink, *step))
            .collect();
    }

    let base = hex_to_oklch(seed);
    let neutral_chroma = base.c.min(if is_dark { 0.068 } else { 0.04 });

    let light_steps: Vec<f64> = if is_dark {
        vec![
            0.138,
            0.156,
            0.178,
            0.202,
            0.232,
            0.272,
            0.326,
            0.404,
            clamp(base.l * 0.83, 0.43, 0.55),
            0.596,
            0.719,
            0.956,
        ]
    } else {
        vec![
            0.991, 0.979, 0.964, 0.946, 0.931, 0.913, 0.891, 0.83, base.l, 0.617, 0.542, 0.205,
        ]
    };

    (0..12)
        .map(|i| {
            oklch_to_hex(&OklchColor {
                l: light_steps[i],
                c: neutral_chroma,
                h: base.h,
            })
        })
        .collect()
}

/// 1:1 with TS `generateAlphaScale`.
pub fn generate_alpha_scale(scale: &[HexColor], is_dark: bool) -> Vec<HexColor> {
    let alphas: Vec<f64> = if is_dark {
        vec![
            0.02, 0.04, 0.08, 0.12, 0.16, 0.2, 0.26, 0.36, 0.44, 0.52, 0.76, 0.96,
        ]
    } else {
        vec![
            0.01, 0.03, 0.06, 0.09, 0.12, 0.15, 0.2, 0.28, 0.48, 0.56, 0.64, 0.88,
        ]
    };

    scale
        .iter()
        .enumerate()
        .map(|(i, hex)| {
            let (r, g, b) = hex_to_rgb(hex);
            let a = alphas[i];
            let bg = if is_dark { 0.0 } else { 1.0 };
            rgb_to_hex(
                r * a + bg * (1.0 - a),
                g * a + bg * (1.0 - a),
                b * a + bg * (1.0 - a),
            )
        })
        .collect()
}

/// 1:1 with TS `mixColors`.
pub fn mix_colors(color1: &str, color2: &str, amount: f64) -> HexColor {
    let c1 = hex_to_oklch(color1);
    let c2 = hex_to_oklch(color2);
    let delta = (((c2.h - c1.h) % 360.0 + 540.0) % 360.0) - 180.0;

    oklch_to_hex(&OklchColor {
        l: c1.l + (c2.l - c1.l) * amount,
        c: c1.c + (c2.c - c1.c) * amount,
        h: c1.h + delta * amount,
    })
}

/// Shift amounts for `shift` (1:1 with TS `{ l?; c?; h? }`).
#[derive(Debug, Clone, Copy, Default)]
pub struct Shift {
    pub l: Option<f64>,
    pub c: Option<f64>,
    pub h: Option<f64>,
}

/// 1:1 with TS `shift`.
pub fn shift(color: &str, value: Shift) -> HexColor {
    let base = hex_to_oklch(color);
    oklch_to_hex(&OklchColor {
        l: base.l + value.l.unwrap_or(0.0),
        c: base.c * value.c.unwrap_or(1.0),
        h: base.h + value.h.unwrap_or(0.0),
    })
}

/// 1:1 with TS `contrastRatio` (note: exported in source file body though not re-exported from index).
pub fn contrast_ratio(foreground: &str, background: &str) -> f64 {
    let linear = |c: f64| {
        if c <= 0.03928 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };
    let luminance = |hex: &str| {
        let (r, g, b) = hex_to_rgb(hex);
        0.2126 * linear(r) + 0.587 * linear(g) + 0.0722 * linear(b)
    };
    let fg = luminance(foreground);
    let bg = luminance(background);
    let lighter = fg.max(bg);
    let darker = fg.min(bg);
    (lighter + 0.05) / (darker + 0.05)
}

/// 1:1 with TS `blend`.
pub fn blend(color: &str, background: &str, alpha: f64) -> HexColor {
    let (fr, fg, fb) = hex_to_rgb(color);
    let (br, bg, bb) = hex_to_rgb(background);
    rgb_to_hex(
        fr * alpha + br * (1.0 - alpha),
        fg * alpha + bg * (1.0 - alpha),
        fb * alpha + bb * (1.0 - alpha),
    )
}

/// 1:1 with TS `lighten`.
pub fn lighten(color: &str, amount: f64) -> HexColor {
    let oklch = hex_to_oklch(color);
    oklch_to_hex(&OklchColor {
        l: clamp(oklch.l + amount, 0.0, 1.0),
        ..oklch
    })
}

/// 1:1 with TS `darken`.
pub fn darken(color: &str, amount: f64) -> HexColor {
    let oklch = hex_to_oklch(color);
    oklch_to_hex(&OklchColor {
        l: clamp(oklch.l - amount, 0.0, 1.0),
        ..oklch
    })
}

/// 1:1 with TS `withAlpha`.
pub fn with_alpha(color: &str, alpha: f64) -> String {
    let (r, g, b) = hex_to_rgb(color);
    format!(
        "rgba({}, {}, {}, {})",
        (r * 255.0).round() as u8,
        (g * 255.0).round() as u8,
        (b * 255.0).round() as u8,
        alpha
    )
}
