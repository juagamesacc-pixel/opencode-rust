//! Port of `src/stdlib/date.ts`.

use crate::interpreter_model::{AstNode, DiagnosticKind, InterpreterRuntimeError};
use crate::stdlib_value::{coerce_to_number, coerce_to_string, format_iso8601, DataVal};

/// Date methods, verbatim. Mirrors `dateMethods`.
pub const DATE_METHODS: &[&str] = &[
    "getTime",
    "valueOf",
    "toISOString",
    "toJSON",
    "toString",
    "getFullYear",
    "getMonth",
    "getDate",
    "getDay",
    "getHours",
    "getMinutes",
    "getSeconds",
    "getMilliseconds",
    "getUTCFullYear",
    "getUTCMonth",
    "getUTCDate",
    "getUTCDay",
    "getUTCHours",
    "getUTCMinutes",
    "getUTCSeconds",
    "getUTCMilliseconds",
    "getTimezoneOffset",
];

/// Date statics, verbatim. Mirrors `dateStatics`.
pub const DATE_STATICS: &[&str] = &["now", "parse", "UTC"];

/// Mirrors `dateMethods.has(name)`.
pub fn is_date_method(name: &str) -> bool {
    DATE_METHODS.contains(&name)
}

/// Mirrors `dateStatics.has(name)`.
pub fn is_date_static(name: &str) -> bool {
    DATE_STATICS.contains(&name)
}

/// Clock hook so `Date.now()` stays host-driven without a time dependency in
/// signatures. The interpreter passes the real clock.
pub type Clock = Box<dyn Fn() -> f64 + Send + Sync>;

/// Mirrors `invokeDateStatic(name, args, node)`.
pub fn invoke_date_static(
    name: &str,
    args: &[DataVal],
    node: Option<&AstNode>,
    now_ms: f64,
) -> Result<DataVal, InterpreterRuntimeError> {
    match name {
        "now" => Ok(DataVal::Number(now_ms)),
        "parse" => {
            let text = match args.first() {
                Some(v) => coerce_to_string(v),
                None => coerce_to_string(&DataVal::Undefined),
            };
            Ok(DataVal::Number(parse_iso8601_or_nan(&text)))
        }
        "UTC" => {
            let nums: Vec<f64> = args.iter().map(coerce_to_number).collect();
            Ok(DataVal::Number(date_utc(&nums)))
        }
        _ => Err(InterpreterRuntimeError::new(
            format!("Date.{} is not available in CodeMode.", name),
            node.cloned(),
            DiagnosticKind::ExecutionFailure,
            None,
        )),
    }
}

/// Mirrors `invokeDateMethod(value, name, node)` for a millisecond timestamp.
pub fn invoke_date_method(
    time: f64,
    name: &str,
    node: Option<&AstNode>,
) -> Result<DataVal, InterpreterRuntimeError> {
    match name {
        "getTime" | "valueOf" => Ok(DataVal::Number(time)),
        "toISOString" => {
            if !time.is_finite() {
                return Err(InterpreterRuntimeError::new(
                    "Invalid time value.",
                    node.cloned(),
                    DiagnosticKind::ExecutionFailure,
                    None,
                ));
            }
            Ok(DataVal::Str(format_iso8601(time)))
        }
        "toJSON" => {
            if time.is_finite() {
                Ok(DataVal::Str(format_iso8601(time)))
            } else {
                Ok(DataVal::Null)
            }
        }
        "toString" => {
            if time.is_finite() {
                Ok(DataVal::Str(format_iso8601(time)))
            } else {
                Ok(DataVal::Str("Invalid Date".to_string()))
            }
        }
        "getTimezoneOffset" => Ok(DataVal::Number(0.0)),
        "getFullYear" | "getMonth" | "getDate" | "getDay" | "getHours" | "getMinutes"
        | "getSeconds" | "getMilliseconds" | "getUTCFullYear" | "getUTCMonth" | "getUTCDate"
        | "getUTCDay" | "getUTCHours" | "getUTCMinutes" | "getUTCSeconds"
        | "getUTCMilliseconds" => Ok(DataVal::Number(date_component(time, name))),
        _ => Err(InterpreterRuntimeError::new(
            format!("Date method '{}' is not available in CodeMode.", name),
            node.cloned(),
            DiagnosticKind::ExecutionFailure,
            None,
        )),
    }
}

fn date_component(time: f64, name: &str) -> f64 {
    if !time.is_finite() {
        return f64::NAN;
    }
    let secs = (time / 1000.0).floor() as i64;
    let days = secs.div_euclid(86_400);
    // Day-of-week from Unix epoch (Thursday).
    let dow = ((days + 4).rem_euclid(7)) as f64;
    // Civil date via days → y/m/d.
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as f64;
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as f64;
    let year = (if m <= 2.0 { y + 1 } else { y }) as f64;
    let tod = secs.rem_euclid(86_400) as f64;
    match name {
        "getFullYear" | "getUTCFullYear" => year,
        "getMonth" | "getUTCMonth" => m - 1.0,
        "getDate" | "getUTCDate" => d,
        "getDay" | "getUTCDay" => dow,
        "getHours" | "getUTCHours" => (tod / 3600.0).floor(),
        "getMinutes" | "getUTCMinutes" => ((tod % 3600.0) / 60.0).floor(),
        "getSeconds" | "getUTCSeconds" => tod % 60.0,
        "getMilliseconds" | "getUTCMilliseconds" => {
            (time - (time / 1000.0).floor() * 1000.0).round()
        }
        _ => f64::NAN,
    }
}

fn date_utc(nums: &[f64]) -> f64 {
    // Mirrors `Date.UTC(...args.map(coerceToNumber))` incl. defaults.
    let year = nums.first().copied().unwrap_or(f64::NAN);
    if year.is_nan() {
        return f64::NAN;
    }
    let y = if (0.0..100.0).contains(&year) {
        1900.0 + year
    } else {
        year
    };
    let mo = nums.get(1).copied().unwrap_or(0.0);
    let d = nums.get(2).copied().unwrap_or(1.0);
    let h = nums.get(3).copied().unwrap_or(0.0);
    let mi = nums.get(4).copied().unwrap_or(0.0);
    let s = nums.get(5).copied().unwrap_or(0.0);
    let ms = nums.get(6).copied().unwrap_or(0.0);
    days_from_civil(y as i64, mo as i64 + 1, d as i64) as f64 * 86_400_000.0
        + h * 3_600_000.0
        + mi * 60_000.0
        + s * 1000.0
        + ms
}

fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

/// Parses ISO-8601 / RFC-2822-ish date strings; `NaN` when unparseable.
/// Mirrors `Date.parse(coerceToString(args[0]))` for the common subset.
pub fn parse_iso8601_or_nan(text: &str) -> f64 {
    let t = text.trim();
    // Fast path: `YYYY-MM-DDTHH:MM:SS.sssZ`.
    if let Some(ms) = parse_iso_full(t) {
        return ms;
    }
    // Date-only `YYYY-MM-DD`.
    if t.len() == 10 {
        let parts: Vec<&str> = t.split('-').collect();
        if parts.len() == 3 {
            if let (Ok(y), Ok(m), Ok(d)) = (
                parts[0].parse::<i64>(),
                parts[1].parse::<i64>(),
                parts[2].parse::<i64>(),
            ) {
                return days_from_civil(y, m, d) as f64 * 86_400_000.0;
            }
        }
    }
    f64::NAN
}

fn parse_iso_full(t: &str) -> Option<f64> {
    let (date_part, time_part) = t.split_once(['T', ' '])?;
    let dp: Vec<&str> = date_part.split('-').collect();
    if dp.len() != 3 {
        return None;
    }
    let (y, m, d): (i64, i64, i64) = (
        dp[0].parse().ok()?,
        dp[1].parse().ok()?,
        dp[2].parse().ok()?,
    );
    // Strip trailing Z / offset (offset application kept minimal: Z only).
    let time_core = time_part.strip_suffix('Z').unwrap_or(time_part);
    let tp: Vec<&str> = time_core.split(':').collect();
    if tp.len() < 2 {
        return None;
    }
    let h: f64 = tp[0].parse().ok()?;
    let mi: f64 = tp[1].parse().ok()?;
    let (s, ms) = if tp.len() > 2 {
        let sec_parts: Vec<&str> = tp[2].split('.').collect();
        let s: f64 = sec_parts[0].parse().ok()?;
        let ms: f64 = if sec_parts.len() > 1 {
            let frac = format!("0.{}", sec_parts[1]);
            frac.parse::<f64>().ok()? * 1000.0
        } else {
            0.0
        };
        (s, ms)
    } else {
        (0.0, 0.0)
    };
    Some(
        days_from_civil(y, m, d) as f64 * 86_400_000.0
            + h * 3_600_000.0
            + mi * 60_000.0
            + s * 1000.0
            + ms,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unavailable_statics_use_verbatim_message() {
        let err = invoke_date_static("toLocaleString", &[], None, 0.0).unwrap_err();
        assert_eq!(
            err.message,
            "Date.toLocaleString is not available in CodeMode."
        );
    }

    #[test]
    fn invalid_time_value_verbatim() {
        let err = invoke_date_method(f64::NAN, "toISOString", None).unwrap_err();
        assert_eq!(err.message, "Invalid time value.");
    }
}
