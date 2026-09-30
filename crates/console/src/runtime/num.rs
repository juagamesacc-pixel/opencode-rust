//! Host-runtime shims for the ECMAScript numeric/string coercions used by
//! `packages/console`.
//!
//! 1:1 port helper — NOT a source file. `Number(...)`, `parseInt(...)`,
//! `Math.round`, `Number.prototype.toFixed` and `String(value)` are host
//! runtime behavior, reproduced here without reinterpretation.

/// `Math.round(x)` — JS rounds halves toward positive infinity.
pub fn round(x: f64) -> f64 {
    (x + 0.5).floor()
}

/// `Math.round(x)` for the integer-valued money/counter fields the source
/// stores in `bigint` columns.
pub fn round_int(x: f64) -> i64 {
    (x + 0.5).floor() as i64
}

/// `Math.floor(x)` for integer fields.
pub fn floor_int(x: f64) -> i64 {
    x.floor() as i64
}

/// `Math.ceil(x)`.
pub fn ceil(x: f64) -> f64 {
    x.ceil()
}

/// `Math.ceil(a / b)` for positive operands.
pub fn ceil_div(a: f64, b: f64) -> f64 {
    (a / b).ceil()
}

/// `Number.parseInt(value)` — leading integer prefix, `NaN` when absent.
pub fn parse_int(value: &str) -> f64 {
    let trimmed = value.trim();
    let bytes = trimmed.as_bytes();
    let mut end = 0;
    if end < bytes.len() && (bytes[end] == b'+' || bytes[end] == b'-') {
        end += 1;
    }
    let digits_start = end;
    while end < bytes.len() && bytes[end].is_ascii_digit() {
        end += 1;
    }
    if end == digits_start {
        return f64::NAN;
    }
    trimmed[..end].parse().unwrap_or(f64::NAN)
}

/// `parseFloat(value)` — leading float prefix, `NaN` when absent.
pub fn parse_float(value: &str) -> f64 {
    let trimmed = value.trim();
    let bytes = trimmed.as_bytes();
    let mut end = 0;
    if end < bytes.len() && (bytes[end] == b'+' || bytes[end] == b'-') {
        end += 1;
    }
    let mut seen_dot = false;
    let mut seen_exp = false;
    let digits_start = end;
    while end < bytes.len() {
        let byte = bytes[end];
        if byte.is_ascii_digit() {
            end += 1;
        } else if byte == b'.' && !seen_dot && !seen_exp {
            seen_dot = true;
            end += 1;
        } else if matches!(byte, b'e' | b'E') && !seen_exp && end > digits_start {
            seen_exp = true;
            end += 1;
            if end < bytes.len() && (bytes[end] == b'+' || bytes[end] == b'-') {
                end += 1;
            }
        } else {
            break;
        }
    }
    if end == digits_start {
        return f64::NAN;
    }
    trimmed[..end].parse().unwrap_or(f64::NAN)
}

/// `Number(value)` for the string/JSON scalars the source coerces.
pub fn number_from_str(value: &str) -> f64 {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return 0.0;
    }
    parse_float(trimmed)
}

/// `Number.isFinite(x)`.
pub fn is_finite(x: f64) -> bool {
    x.is_finite()
}

/// `Number.prototype.toFixed(digits)` — half-up rounding, always fixed width.
pub fn to_fixed(x: f64, digits: usize) -> String {
    if !x.is_finite() {
        return if x.is_nan() {
            "NaN".to_string()
        } else if x > 0.0 {
            "Infinity".to_string()
        } else {
            "-Infinity".to_string()
        };
    }
    let negative = x < 0.0 || (x == 0.0 && x.is_sign_negative());
    let magnitude = x.abs();
    let factor = 10f64.powi(digits as i32);
    let scaled = (magnitude * factor + 0.5).floor() / factor;
    let mut text = format!("{:.*}", digits, scaled);
    if text.starts_with("-0") && scaled == 0.0 {
        text = text[1..].to_string();
    }
    if negative && scaled != 0.0 {
        format!("-{}", text)
    } else {
        text
    }
}

/// `String(number)` — integral values render without a fractional part, matching
/// `${value}` interpolation in the ported templates.
pub fn number_to_string(x: f64) -> String {
    if x.is_nan() {
        return "NaN".to_string();
    }
    if x.is_infinite() {
        return if x > 0.0 { "Infinity" } else { "-Infinity" }.to_string();
    }
    if x == x.trunc() && x.abs() < 1e21 {
        return format!("{}", x as i64);
    }
    let mut text = format!("{}", x);
    if text.contains('e') {
        text = format!("{:?}", x);
    }
    text
}

/// `String(value)` for a boolean.
pub fn boolean_to_string(value: bool) -> String {
    if value { "true" } else { "false" }.to_string()
}

/// `String(value)` for an optional string, i.e. `String(x)` on a template slot.
pub fn option_to_string(value: &Option<String>) -> String {
    match value {
        Some(inner) => inner.clone(),
        None => "undefined".to_string(),
    }
}
