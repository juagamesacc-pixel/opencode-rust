//! Port of `src/stdlib/math.ts`.

use crate::interpreter_model::{AstNode, DiagnosticKind, InterpreterRuntimeError};
use crate::stdlib_value::DataVal;

/// Math constants, verbatim. Mirrors `mathConstants`.
pub const MATH_CONSTANTS: &[&str] = &[
    "PI", "E", "LN2", "LN10", "LOG2E", "LOG10E", "SQRT2", "SQRT1_2",
];

/// Math methods, verbatim. Mirrors `mathMethods`.
pub const MATH_METHODS: &[&str] = &[
    "max", "min", "abs", "floor", "ceil", "round", "trunc", "sign", "sqrt", "cbrt", "pow", "hypot",
    "log", "log2", "log10", "exp",
];

/// Mirrors `mathMethods.has(name)`.
pub fn is_math_method(name: &str) -> bool {
    MATH_METHODS.contains(&name)
}

/// Value of a Math constant. Mirrors host `Math.*` reads.
pub fn math_constant(name: &str) -> Option<f64> {
    match name {
        "PI" => Some(std::f64::consts::PI),
        "E" => Some(std::f64::consts::E),
        "LN2" => Some(std::f64::consts::LN_2),
        "LN10" => Some(std::f64::consts::LN_10),
        "LOG2E" => Some(std::f64::consts::LOG2_E),
        "LOG10E" => Some(std::f64::consts::LOG10_E),
        "SQRT2" => Some(std::f64::consts::SQRT_2),
        "SQRT1_2" => Some(std::f64::consts::FRAC_1_SQRT_2),
        _ => None,
    }
}

/// Mirrors `invokeMathMethod(name, args, node)`: all args must be numbers
/// (verbatim `Math.<name> expects number arguments.`); dispatch matches JS
/// semantics incl. `max`/`min` with no args (`∓Infinity`) and `pow(a, b)`
/// defaulting to `NaN` for missing args.
pub fn invoke_math_method(
    name: &str,
    args: &[DataVal],
    node: Option<&AstNode>,
) -> Result<DataVal, InterpreterRuntimeError> {
    if !is_math_method(name) {
        return Err(InterpreterRuntimeError::new(
            format!("Math.{} is not available in CodeMode.", name),
            node.cloned(),
            DiagnosticKind::ExecutionFailure,
            None,
        ));
    }
    let mut nums = Vec::with_capacity(args.len());
    for arg in args {
        match arg {
            DataVal::Number(n) => nums.push(*n),
            _ => {
                return Err(InterpreterRuntimeError::new(
                    format!("Math.{} expects number arguments.", name),
                    node.cloned(),
                    DiagnosticKind::ExecutionFailure,
                    None,
                ))
            }
        }
    }
    let a = nums.first().copied().unwrap_or(f64::NAN);
    let b = nums.get(1).copied().unwrap_or(f64::NAN);
    let result = match name {
        "max" => nums.iter().copied().fold(f64::NEG_INFINITY, f64::max),
        "min" => nums.iter().copied().fold(f64::INFINITY, f64::min),
        "abs" => a.abs(),
        "floor" => a.floor(),
        "ceil" => a.ceil(),
        // JS Math.round rounds half up (toward +∞); Rust round() rounds half
        // away from zero — adjust for negative halves.
        "round" => {
            if (a - (a.floor() + 0.5)).abs() < f64::EPSILON && a < 0.0 {
                a.ceil()
            } else {
                (a + 0.5).floor()
            }
        }
        "trunc" => a.trunc(),
        "sign" => {
            if a.is_nan() {
                f64::NAN
            } else if a == 0.0 {
                a
            } else if a > 0.0 {
                1.0
            } else {
                -1.0
            }
        }
        "sqrt" => a.sqrt(),
        "cbrt" => a.cbrt(),
        "pow" => a.powf(b),
        "hypot" => nums.iter().map(|n| n * n).sum::<f64>().sqrt(),
        "log" => a.ln(),
        "log2" => a.log2(),
        "log10" => a.log10(),
        "exp" => a.exp(),
        _ => {
            return Err(InterpreterRuntimeError::new(
                format!("Math.{} is not available in CodeMode.", name),
                node.cloned(),
                DiagnosticKind::ExecutionFailure,
                None,
            ))
        }
    };
    Ok(DataVal::Number(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn math_guards_verbatim() {
        let err = invoke_math_method("random", &[], None).unwrap_err();
        assert_eq!(err.message, "Math.random is not available in CodeMode.");
        let err = invoke_math_method("max", &[DataVal::Str("x".to_string())], None).unwrap_err();
        assert_eq!(err.message, "Math.max expects number arguments.");
    }
}
