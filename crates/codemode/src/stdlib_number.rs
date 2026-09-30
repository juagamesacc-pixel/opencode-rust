//! Port of `src/stdlib/number.ts`.

use crate::interpreter_model::{AstNode, DiagnosticKind, InterpreterRuntimeError};
use crate::stdlib_value::{bounded_data, coerce_to_string, js_parse_float, js_parse_int, DataVal};

/// Number prototype methods, verbatim. Mirrors `numberMethods`.
pub const NUMBER_METHODS: &[&str] = &["toFixed", "toPrecision", "toExponential", "toString"];

/// Number constants, verbatim. Mirrors `numberConstants`.
pub const NUMBER_CONSTANTS: &[&str] = &[
    "MAX_SAFE_INTEGER",
    "MIN_SAFE_INTEGER",
    "MAX_VALUE",
    "MIN_VALUE",
    "EPSILON",
];

/// Number statics, verbatim. Mirrors `numberStatics`.
pub const NUMBER_STATICS: &[&str] = &[
    "isInteger",
    "isFinite",
    "isNaN",
    "isSafeInteger",
    "parseInt",
    "parseFloat",
];

/// Value of a Number constant.
pub fn number_constant(name: &str) -> Option<f64> {
    match name {
        "MAX_SAFE_INTEGER" => Some(9_007_199_254_740_991.0),
        "MIN_SAFE_INTEGER" => Some(-9_007_199_254_740_991.0),
        "MAX_VALUE" => Some(f64::MAX),
        "MIN_VALUE" => Some(f64::MIN_POSITIVE),
        "EPSILON" => Some(f64::EPSILON),
        _ => None,
    }
}

/// Mirrors `invokeNumberMethod(value, name, args, node)`.
pub fn invoke_number_method(
    value: f64,
    name: &str,
    args: &[DataVal],
    node: Option<&AstNode>,
) -> Result<DataVal, InterpreterRuntimeError> {
    let opt_num = |index: usize| -> Result<Option<f64>, InterpreterRuntimeError> {
        match args.get(index) {
            None | Some(DataVal::Undefined) => Ok(None),
            Some(DataVal::Number(n)) => Ok(Some(*n)),
            _ => Err(InterpreterRuntimeError::new(
                format!("Number.{} expects a number argument.", name),
                node.cloned(),
                DiagnosticKind::ExecutionFailure,
                None,
            )),
        }
    };
    let result: String = match name {
        "toFixed" => {
            let digits = opt_num(0)?.unwrap_or(0.0);
            js_to_fixed(value, digits, node)?
        }
        "toExponential" => {
            let digits = opt_num(0)?;
            js_to_exponential(value, digits, node)?
        }
        "toPrecision" => {
            let digits = opt_num(0)?;
            match digits {
                None => js_number_shortest(value),
                Some(d) => js_to_precision(value, d, node)?,
            }
        }
        "toString" => {
            let radix = opt_num(0)?;
            if let Some(r) = radix {
                if !(2.0..=36.0).contains(&r) || r.fract() != 0.0 {
                    // Host-delegated: mirrors the V8 RangeError text.
                    return Err(InterpreterRuntimeError::new(
                        "ToString() radix argument must be between 2 and 36.",
                        node.cloned(),
                        DiagnosticKind::ExecutionFailure,
                        None,
                    )
                    .as_error("RangeError"));
                }
                js_number_to_radix(value, r as u32)
            } else {
                js_number_shortest(value)
            }
        }
        _ => {
            return Err(InterpreterRuntimeError::new(
                format!("Number method '{}' is not available in CodeMode.", name),
                node.cloned(),
                DiagnosticKind::ExecutionFailure,
                None,
            ))
        }
    };
    let checked = bounded_data(
        &DataVal::Str(result.clone()),
        &format!("Number.{} result", name),
    )?;
    Ok(checked)
}

/// Mirrors `invokeNumberStatic(name, args, node)`.
pub fn invoke_number_static(
    name: &str,
    args: &[DataVal],
    node: Option<&AstNode>,
) -> Result<DataVal, InterpreterRuntimeError> {
    let value = args.first().cloned().unwrap_or(DataVal::Undefined);
    Ok(match name {
        "isInteger" => {
            DataVal::Bool(matches!(value, DataVal::Number(n) if n.fract() == 0.0 && n.is_finite()))
        }
        "isFinite" => DataVal::Bool(matches!(value, DataVal::Number(n) if n.is_finite())),
        "isNaN" => DataVal::Bool(matches!(value, DataVal::Number(n) if n.is_nan())),
        "isSafeInteger" => DataVal::Bool(
            matches!(value, DataVal::Number(n) if n.fract() == 0.0 && n.abs() <= 9_007_199_254_740_991.0),
        ),
        "parseInt" => {
            let radix = match args.get(1) {
                None | Some(DataVal::Undefined) => None,
                Some(DataVal::Number(n)) => Some(*n),
                _ => {
                    return Err(InterpreterRuntimeError::new(
                        "Number.parseInt expects a numeric radix.",
                        node.cloned(),
                        DiagnosticKind::ExecutionFailure,
                        None,
                    ))
                }
            };
            DataVal::Number(js_parse_int(&coerce_to_string(&value), radix))
        }
        "parseFloat" => DataVal::Number(js_parse_float(&coerce_to_string(&value))),
        _ => {
            return Err(InterpreterRuntimeError::new(
                format!("Number.{} is not available in CodeMode.", name),
                node.cloned(),
                DiagnosticKind::ExecutionFailure,
                None,
            ))
        }
    })
}

fn js_number_shortest(n: f64) -> String {
    crate::stdlib_value::js_number_to_string(n)
}

fn js_to_fixed(
    value: f64,
    digits: f64,
    node: Option<&AstNode>,
) -> Result<String, InterpreterRuntimeError> {
    if !(0.0..=100.0).contains(&digits) || digits.fract() != 0.0 {
        // Host-delegated: mirrors the V8 RangeError text the TS port
        // surfaces via host `Number.prototype.toFixed`.
        return Err(InterpreterRuntimeError::new(
            "toFixed() digits argument must be between 0 and 100.",
            node.cloned(),
            DiagnosticKind::ExecutionFailure,
            None,
        )
        .as_error("RangeError"));
    }
    if !value.is_finite() {
        return Ok(js_number_shortest(value));
    }
    Ok(format!("{:.1$}", value, digits as usize))
}

fn js_to_exponential(
    value: f64,
    digits: Option<f64>,
    node: Option<&AstNode>,
) -> Result<String, InterpreterRuntimeError> {
    if !value.is_finite() {
        return Ok(js_number_shortest(value));
    }
    match digits {
        None => Ok(format!("{:e}", value)),
        Some(d) => {
            // Host-delegated: mirrors the V8 RangeError text.
            if !(0.0..=100.0).contains(&d) || d.fract() != 0.0 {
                return Err(InterpreterRuntimeError::new(
                    "toExponential() argument must be between 0 and 100.",
                    node.cloned(),
                    DiagnosticKind::ExecutionFailure,
                    None,
                )
                .as_error("RangeError"));
            }
            Ok(format!("{:.1$e}", value, d as usize))
        }
    }
}

fn js_to_precision(
    value: f64,
    digits: f64,
    node: Option<&AstNode>,
) -> Result<String, InterpreterRuntimeError> {
    if !(1.0..=100.0).contains(&digits) || digits.fract() != 0.0 {
        // Host-delegated: mirrors the V8 RangeError text.
        return Err(InterpreterRuntimeError::new(
            "toPrecision() precision argument must be between 1 and 100.",
            node.cloned(),
            DiagnosticKind::ExecutionFailure,
            None,
        )
        .as_error("RangeError"));
    }
    if !value.is_finite() {
        return Ok(js_number_shortest(value));
    }
    // Significant-digit rounding, JS-style.
    let p = digits as i32;
    if value == 0.0 {
        return Ok(format!("0.{}e+0", "0".repeat((p - 1) as usize)));
    }
    let magnitude = value.abs().log10().floor() as i32;
    let scale = 10f64.powi(p - 1 - magnitude);
    let rounded = (value * scale).round() / scale;
    Ok(js_number_shortest(rounded))
}

fn js_number_to_radix(value: f64, radix: u32) -> String {
    if !value.is_finite() {
        return js_number_shortest(value);
    }
    const DIGITS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";
    let neg = value < 0.0;
    let mut n = value.abs().trunc() as u64;
    // Fractional part kept JS-shortest (full radix-fraction expansion is out
    // of scope for orchestration code; integer conversion is exact).
    let mut out = vec![];
    if n == 0 {
        out.push('0');
    } else {
        while n > 0 {
            out.push(DIGITS[(n % radix as u64) as usize] as char);
            n /= radix as u64;
        }
        out.reverse();
    }
    let mut s: String = out.into_iter().collect();
    let frac = value.abs().fract();
    if frac > 0.0 {
        s.push('.');
        let mut f = frac;
        for _ in 0..12 {
            f *= radix as f64;
            let d = f.trunc() as usize;
            s.push(DIGITS[d.min(35)] as char);
            f -= d as f64;
            if f == 0.0 {
                break;
            }
        }
    }
    if neg {
        format!("-{}", s)
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_guards_verbatim() {
        let err = invoke_number_method(1.0, "toLocaleString", &[], None).unwrap_err();
        assert_eq!(
            err.message,
            "Number method 'toLocaleString' is not available in CodeMode."
        );
        let err =
            invoke_number_method(1.0, "toString", &[DataVal::Number(37.0)], None).unwrap_err();
        // Host-delegated V8 spelling.
        assert_eq!(
            err.message,
            "ToString() radix argument must be between 2 and 36."
        );
        assert_eq!(err.error_name, "RangeError");
    }
}
