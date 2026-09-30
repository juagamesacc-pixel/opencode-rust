//! Port of `src/stdlib/string.ts`.

use crate::interpreter_model::{AstNode, DiagnosticKind, InterpreterRuntimeError};

/// String methods, verbatim. Mirrors `stringMethods`.
pub const STRING_METHODS: &[&str] = &[
    "toLowerCase",
    "toUpperCase",
    "trim",
    "trimStart",
    "trimEnd",
    "trimLeft",
    "trimRight",
    "split",
    "slice",
    "substring",
    "substr",
    "includes",
    "startsWith",
    "endsWith",
    "indexOf",
    "lastIndexOf",
    "replace",
    "replaceAll",
    "repeat",
    "padStart",
    "padEnd",
    "charAt",
    "charCodeAt",
    "codePointAt",
    "at",
    "concat",
    "toString",
    "match",
    "matchAll",
    "search",
    "localeCompare",
    "normalize",
];

/// String statics, verbatim. Mirrors `stringStatics`.
pub const STRING_STATICS: &[&str] = &["fromCharCode", "fromCodePoint"];

/// Mirrors `stringMethods.has(name)`.
pub fn is_string_method(name: &str) -> bool {
    STRING_METHODS.contains(&name)
}

/// Mirrors `stringStatics.has(name)`.
pub fn is_string_static(name: &str) -> bool {
    STRING_STATICS.contains(&name)
}

/// Mirrors `invokeStringStatic(name, args, node)`.
pub fn invoke_string_static(
    name: &str,
    args: &[crate::stdlib_value::DataVal],
    node: Option<&AstNode>,
) -> Result<String, InterpreterRuntimeError> {
    use crate::stdlib_value::DataVal;
    let mut codes = Vec::with_capacity(args.len());
    for arg in args {
        match arg {
            DataVal::Number(n) => codes.push(*n),
            _ => {
                return Err(InterpreterRuntimeError::new(
                    format!("String.{} expects number arguments.", name),
                    node.cloned(),
                    DiagnosticKind::ExecutionFailure,
                    None,
                ))
            }
        }
    }
    match name {
        "fromCharCode" => Ok(codes
            .iter()
            .map(|n| char::from_u32(*n as u32).unwrap_or('\0'))
            .collect::<String>()),
        "fromCodePoint" => {
            let mut out = String::new();
            for n in codes {
                match char::from_u32(n as u32) {
                    Some(c) => out.push(c),
                    None => {
                        // Host-delegated: mirrors the V8 RangeError text.
                        return Err(InterpreterRuntimeError::new(
                            format!(
                                "Invalid code point {}",
                                crate::stdlib_value::js_number_to_string(n)
                            ),
                            node.cloned(),
                            DiagnosticKind::ExecutionFailure,
                            None,
                        )
                        .as_error("RangeError"));
                    }
                }
            }
            Ok(out)
        }
        _ => Err(InterpreterRuntimeError::new(
            format!("String.{} is not available in CodeMode.", name),
            node.cloned(),
            DiagnosticKind::ExecutionFailure,
            None,
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib_value::DataVal;

    #[test]
    fn string_tables_verbatim() {
        assert!(is_string_method("matchAll"));
        assert!(is_string_static("fromCodePoint"));
        assert!(!is_string_method("raw"));
    }

    #[test]
    fn string_static_guards_verbatim() {
        let err = invoke_string_static("raw", &[], None).unwrap_err();
        assert_eq!(err.message, "String.raw is not available in CodeMode.");
        let err = invoke_string_static("fromCharCode", &[DataVal::Str("x".to_string())], None)
            .unwrap_err();
        assert_eq!(err.message, "String.fromCharCode expects number arguments.");
    }
}
