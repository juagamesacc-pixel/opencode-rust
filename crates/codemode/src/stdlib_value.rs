//! Port of `src/stdlib/value.ts`.
//!
//! Shared value helpers: error constructor/brand tables, `boundedData`,
//! `coerceToString` / `coerceToNumber`, `invokeCoercion`.
//!
//! ANTI-CYCLE SEAM: the interpreter runtime owns the full `RtValue`
//! representation (see `interpreter_runtime`). The helpers here operate on
//! the owned [`DataVal`] enum plus [`crate::values::SandboxValue`] wrappers;
//! the interpreter maps values across this seam before calling stdlib code.
//! Semantics (incl. verbatim error strings) are preserved.

use crate::interpreter_model::{AstNode, CoercionKind, InterpreterRuntimeError};
use crate::tool_runtime;
use crate::values::SandboxValue;

/// Owned data value for stdlib coercion/boundary helpers.
#[derive(Debug, Clone)]
pub enum DataVal {
    Null,
    Undefined,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<DataVal>),
    Object(Vec<(String, DataVal)>),
    Sandbox(SandboxValue),
}

impl DataVal {
    /// JSON leaf for plain values (sandbox wrappers handled by callers).
    pub fn to_json(&self) -> serde_json::Value {
        match self {
            DataVal::Null | DataVal::Undefined => serde_json::Value::Null,
            DataVal::Bool(b) => serde_json::Value::Bool(*b),
            DataVal::Number(n) => crate::tool_runtime::copy_out_number(*n),
            DataVal::Str(s) => serde_json::Value::String(s.clone()),
            DataVal::Array(items) => {
                serde_json::Value::Array(items.iter().map(|i| i.to_json()).collect())
            }
            DataVal::Object(entries) => serde_json::Value::Object(
                entries
                    .iter()
                    .map(|(k, v)| (k.clone(), v.to_json()))
                    .collect(),
            ),
            DataVal::Sandbox(_) => serde_json::Value::Object(Default::default()),
        }
    }
}

/// Error constructor names. Mirrors `errorConstructors`.
pub fn is_error_constructor(name: &str) -> bool {
    matches!(
        name,
        "Error"
            | "TypeError"
            | "RangeError"
            | "SyntaxError"
            | "ReferenceError"
            | "EvalError"
            | "URIError"
    )
}

/// All error constructor names in source order.
pub const ERROR_CONSTRUCTORS: &[&str] = &[
    "Error",
    "TypeError",
    "RangeError",
    "SyntaxError",
    "ReferenceError",
    "EvalError",
    "URIError",
];

/// Value constructor names. Mirrors `valueConstructors`.
pub const VALUE_CONSTRUCTORS: &[&str] = &["Date", "RegExp", "Map", "Set", "URL", "URLSearchParams"];

/// Compound assignment operators. Mirrors `compoundOperators`.
pub fn is_compound_operator(op: &str) -> bool {
    matches!(
        op,
        "+=" | "-=" | "*=" | "/=" | "%=" | "**=" | "&=" | "|=" | "^=" | "<<=" | ">>=" | ">>>="
    )
}

/// All compound operators in source order.
pub const COMPOUND_OPERATORS: &[&str] = &[
    "+=", "-=", "*=", "/=", "%=", "**=", "&=", "|=", "^=", "<<=", ">>=", ">>>=",
];

/// Branded error value (`{ name, message }` with a brand). The brand is
/// carried alongside the JSON object because Rust JSON values cannot hold
/// symbols. Mirrors `createErrorValue(name, message)`.
#[derive(Debug, Clone)]
pub struct BrandedError {
    pub name: String,
    pub message: String,
    pub value: serde_json::Value,
}

/// Mirrors `createErrorValue(name, message)`.
pub fn create_error_value(name: &str, message: &str) -> BrandedError {
    BrandedError {
        name: name.to_string(),
        message: message.to_string(),
        value: serde_json::json!({"name": name, "message": message}),
    }
}

/// Reads the error brand of a branded value, if any. Mirrors
/// `errorBrandName(value)`.
pub fn error_brand_name(value: &BrandedError) -> Option<&str> {
    Some(value.name.as_str())
}

/// Intra-sandbox data checkpoint. Mirrors `boundedData(value, label)` =
/// `copyIn(value, label, true)`: sandbox wrappers pass through untouched as
/// leaves; plain data is depth/blocked-member validated.
pub fn bounded_data(value: &DataVal, label: &str) -> Result<DataVal, InterpreterRuntimeError> {
    // Materialize to JSON for the contract walk, then map back. Sandbox
    // wrappers are leaves (never walked into), matching `preserveSandboxValues`.
    match value {
        DataVal::Sandbox(_) => Ok(value.clone()),
        _ => {
            let json = value.to_json();
            tool_runtime::copy_in(&json, label, true)
                .map(|_| value.clone())
                .map_err(|e| {
                    InterpreterRuntimeError::new(
                        e.message,
                        None,
                        crate::interpreter_model::DiagnosticKind::InvalidDataValue,
                        None,
                    )
                })
        }
    }
}

/// JS `String(v)` coercion incl. sandbox wrappers. Mirrors `coerceToString`.
pub fn coerce_to_string(value: &DataVal) -> String {
    match value {
        DataVal::Null => "null".to_string(),
        DataVal::Undefined => "undefined".to_string(),
        DataVal::Bool(b) => b.to_string(),
        DataVal::Number(n) => js_number_to_string(*n),
        DataVal::Str(s) => s.clone(),
        DataVal::Array(items) => items
            .iter()
            .map(|item| match item {
                DataVal::Null | DataVal::Undefined => String::new(),
                other => coerce_to_string(other),
            })
            .collect::<Vec<_>>()
            .join(","),
        DataVal::Object(_) => "[object Object]".to_string(),
        DataVal::Sandbox(s) => match s {
            SandboxValue::Date(d) => {
                if d.time.is_finite() {
                    format_iso8601(d.time)
                } else {
                    "Invalid Date".to_string()
                }
            }
            SandboxValue::RegExp(r) => format!("/{}/{}", r.pattern, r.flags),
            SandboxValue::Map(_) => "[object Map]".to_string(),
            SandboxValue::Set(_) => "[object Set]".to_string(),
            SandboxValue::Url(u) => u.href.clone(),
            SandboxValue::UrlSearchParams(p) => url_search_params_to_string(&p.pairs),
            SandboxValue::Promise(_) => "[object Promise]".to_string(),
        },
    }
}

/// JS `Number(v)` coercion incl. sandbox wrappers. Mirrors `coerceToNumber`.
pub fn coerce_to_number(value: &DataVal) -> f64 {
    match value {
        DataVal::Sandbox(s) => match s {
            SandboxValue::Date(d) => d.time,
            _ => f64::NAN,
        },
        DataVal::Null => 0.0,
        DataVal::Undefined => f64::NAN,
        DataVal::Bool(true) => 1.0,
        DataVal::Bool(false) => 0.0,
        DataVal::Number(n) => *n,
        DataVal::Str(s) => js_string_to_number(s),
        DataVal::Array(_) => f64::NAN,
        DataVal::Object(_) => f64::NAN,
    }
}

/// JS number→string (matches `String(n)` for the common cases; handles
/// `NaN`, `±Infinity`, `-0`, integers, and float round-trip).
pub fn js_number_to_string(n: f64) -> String {
    if n.is_nan() {
        return "NaN".to_string();
    }
    if n.is_infinite() {
        return if n > 0.0 {
            "Infinity".to_string()
        } else {
            "-Infinity".to_string()
        };
    }
    if n == 0.0 {
        return "0".to_string();
    }
    // Rust `{}` prints integers without `.0` and uses shortest round-trip
    // for most floats, matching JS for the values tests exercise.
    let s = format!("{}", n);
    // JS prints large/small magnitudes in exponential form; Rust never does.
    // Normalize e.g. 1e21 → "1e+21".
    if !s.contains('.') && !s.contains('e') && !s.contains("inf") && n.abs() >= 1e21 {
        return format!(
            "{:.0}e+{}",
            n / 10f64.powf(n.abs().log10().floor()),
            n.abs().log10().floor() as i64
        );
    }
    s
}

/// JS `Number(string)`tring trimming + decimal/hex/binary/octal/Infinity handling.
pub fn js_string_to_number(s: &str) -> f64 {
    let t = s.trim();
    if t.is_empty() {
        return 0.0;
    }
    if t.eq_ignore_ascii_case("infinity") {
        return f64::INFINITY;
    }
    if t.eq_ignore_ascii_case("-infinity") {
        return f64::NEG_INFINITY;
    }
    if t.eq_ignore_ascii_case("+infinity") {
        return f64::INFINITY;
    }
    // Hex / binary / octal literals.
    let (neg, rest) = match t.strip_prefix(['-', '+']) {
        Some(r) => (t.starts_with('-'), r),
        None => (false, t),
    };
    let magnitude = if let Some(hex) = rest.strip_prefix("0x").or_else(|| rest.strip_prefix("0X")) {
        u64::from_str_radix(hex, 16)
            .map(|v| v as f64)
            .unwrap_or(f64::NAN)
    } else if let Some(bin) = rest.strip_prefix("0b").or_else(|| rest.strip_prefix("0B")) {
        u64::from_str_radix(bin, 2)
            .map(|v| v as f64)
            .unwrap_or(f64::NAN)
    } else if let Some(oct) = rest.strip_prefix("0o").or_else(|| rest.strip_prefix("0O")) {
        u64::from_str_radix(oct, 8)
            .map(|v| v as f64)
            .unwrap_or(f64::NAN)
    } else {
        rest.parse::<f64>().unwrap_or(f64::NAN)
    };
    if neg {
        -magnitude
    } else {
        magnitude
    }
}

/// Minimal ISO-8601 formatter for millisecond timestamps (UTC).
pub fn format_iso8601(millis: f64) -> String {
    let secs = (millis / 1000.0).floor() as i64;
    let ms = (millis - secs as f64 * 1000.0).round() as i64;
    let (y, mo, d, h, mi, s) = civil_from_unix_secs(secs);
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
        y, mo, d, h, mi, s, ms
    )
}

fn civil_from_unix_secs(secs: i64) -> (i64, u32, u32, u32, u32, u32) {
    // Howard Hinnant's civil_from_days algorithm.
    let days = secs.div_euclid(86_400);
    let tod = secs.rem_euclid(86_400);
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (
        if m <= 2 { y + 1 } else { y },
        m,
        d,
        (tod / 3600) as u32,
        ((tod % 3600) / 60) as u32,
        (tod % 60) as u32,
    )
}

fn url_search_params_to_string(pairs: &[(String, String)]) -> String {
    pairs
        .iter()
        .map(|(k, v)| format!("{}={}", percent_encode_query(k), percent_encode_query(v)))
        .collect::<Vec<_>>()
        .join("&")
}

/// `application/x-www-form-urlencoded` percent-encoding for query pairs.
pub fn percent_encode_query(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }
    out
}

/// Coercion-function dispatch. Mirrors `invokeCoercion(ref, args, node)`.
pub fn invoke_coercion(
    name: CoercionKind,
    args: &[DataVal],
    node: Option<&AstNode>,
) -> Result<DataVal, InterpreterRuntimeError> {
    let raw = args.first().cloned().unwrap_or(DataVal::Undefined);
    if matches!(raw, DataVal::Sandbox(_)) {
        return Ok(match name {
            CoercionKind::Boolean => DataVal::Bool(true),
            CoercionKind::Number => DataVal::Number(coerce_to_number(&raw)),
            CoercionKind::String => DataVal::Str(coerce_to_string(&raw)),
            CoercionKind::ParseInt => DataVal::Number(js_parse_int(&coerce_to_string(&raw), None)),
            CoercionKind::ParseFloat => DataVal::Number(js_parse_float(&coerce_to_string(&raw))),
        });
    }
    let value = bounded_data(&raw, &format!("{} input", name.as_str()))?;
    Ok(match name {
        CoercionKind::Number => DataVal::Number(coerce_to_number(&value)),
        CoercionKind::Boolean => DataVal::Bool(is_truthy(&value)),
        CoercionKind::ParseInt => {
            let radix = args.get(1);
            if let Some(r) = radix {
                if !matches!(r, DataVal::Undefined) && !matches!(r, DataVal::Number(_)) {
                    return Err(InterpreterRuntimeError::new(
                        "parseInt expects a numeric radix.",
                        node.cloned(),
                        crate::interpreter_model::DiagnosticKind::ExecutionFailure,
                        None,
                    ));
                }
            }
            let radix_num = match radix {
                Some(DataVal::Number(n)) => Some(*n),
                _ => None,
            };
            DataVal::Number(js_parse_int(&coerce_to_string(&value), radix_num))
        }
        CoercionKind::ParseFloat => DataVal::Number(js_parse_float(&coerce_to_string(&value))),
        CoercionKind::String => DataVal::Str(coerce_to_string(&value)),
    })
}

/// JS truthiness.
pub fn is_truthy(value: &DataVal) -> bool {
    match value {
        DataVal::Null | DataVal::Undefined => false,
        DataVal::Bool(b) => *b,
        DataVal::Number(n) => *n != 0.0 && !n.is_nan(),
        DataVal::Str(s) => !s.is_empty(),
        DataVal::Array(_) | DataVal::Object(_) | DataVal::Sandbox(_) => true,
    }
}

/// JS `parseInt(string, radix?)`.
pub fn js_parse_int(s: &str, radix: Option<f64>) -> f64 {
    let radix = radix.unwrap_or(0.0) as i32;
    let t = s.trim_start();
    let (neg, rest) = match t.strip_prefix('-') {
        Some(r) => (true, r),
        None => (false, t.strip_prefix('+').unwrap_or(t)),
    };
    let (radix, rest) = if radix == 0 {
        if rest.starts_with("0x") || rest.starts_with("0X") {
            (16, &rest[2..])
        } else {
            (10, rest)
        }
    } else if radix == 16 && (rest.starts_with("0x") || rest.starts_with("0X")) {
        (16, &rest[2..])
    } else {
        (radix, rest)
    };
    if !(2..=36).contains(&radix) {
        return f64::NAN;
    }
    let mut value: f64 = 0.0;
    let mut consumed = false;
    for c in rest.chars() {
        let digit = c.to_digit(radix as u32);
        match digit {
            Some(d) => {
                value = value * radix as f64 + d as f64;
                consumed = true;
            }
            None => break,
        }
    }
    if !consumed {
        return f64::NAN;
    }
    if neg {
        -value
    } else {
        value
    }
}

/// JS `parseFloat(string)`.
pub fn js_parse_float(s: &str) -> f64 {
    let t = s.trim_start();
    // Longest valid float prefix: sign, digits, dot, exponent.
    let mut end = 0usize;
    let bytes = t.as_bytes();
    let mut i = 0;
    if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
        i += 1;
    }
    if t[i..].starts_with("Infinity") {
        let v = f64::INFINITY;
        return if t.starts_with('-') { -v } else { v };
    }
    let mut seen_digit = false;
    let mut seen_dot = false;
    while i < bytes.len() {
        let c = bytes[i] as char;
        if c.is_ascii_digit() {
            seen_digit = true;
            i += 1;
            end = i;
        } else if c == '.' && !seen_dot {
            seen_dot = true;
            i += 1;
        } else if (c == 'e' || c == 'E') && seen_digit {
            let mut j = i + 1;
            if j < bytes.len() && (bytes[j] == b'+' || bytes[j] == b'-') {
                j += 1;
            }
            let k = j;
            while j < bytes.len() && (bytes[j] as char).is_ascii_digit() {
                j += 1;
            }
            if j > k {
                i = j;
                end = i;
            }
            break;
        } else {
            break;
        }
    }
    if !seen_digit {
        return f64::NAN;
    }
    // A trailing dot without fraction digits still parses ("12." → 12).
    t[..end].parse::<f64>().unwrap_or(f64::NAN)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coerce_to_string_matches_js() {
        assert_eq!(coerce_to_string(&DataVal::Null), "null");
        assert_eq!(coerce_to_string(&DataVal::Undefined), "undefined");
        assert_eq!(coerce_to_string(&DataVal::Bool(true)), "true");
        assert_eq!(
            coerce_to_string(&DataVal::Array(vec![DataVal::Number(1.0), DataVal::Null])),
            "1,"
        );
    }

    #[test]
    fn compound_operators_verbatim() {
        assert!(is_compound_operator("+="));
        assert!(is_compound_operator(">>>="));
        assert!(!is_compound_operator("="));
        assert_eq!(COMPOUND_OPERATORS.len(), 12);
    }
}
