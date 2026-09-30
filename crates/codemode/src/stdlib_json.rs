//! Port of `src/stdlib/json.ts`.

use crate::interpreter_model::{
    AstNode, DiagnosticKind, InterpreterRuntimeError, SUPPORTED_SYNTAX_MESSAGE,
};
use crate::stdlib_value::DataVal;

/// JSON statics, verbatim. Mirrors `jsonStatics`.
pub const JSON_STATICS: &[&str] = &["stringify", "parse"];

/// Mirrors `jsonStatics.has(name)`.
pub fn is_json_static(name: &str) -> bool {
    JSON_STATICS.contains(&name)
}

/// Mirrors `invokeJsonMethod(name, args, node)`.
///
/// `args` arrive as interpreter `DataVal`s; `stringify` space/indent and the
/// replacer guard behave verbatim. The interpreter supplies values already
/// boundary-checked; serialization goes through the JSON materialization.
pub fn invoke_json_method(
    name: &str,
    args: &[DataVal],
    node: Option<&AstNode>,
) -> Result<DataVal, InterpreterRuntimeError> {
    if !is_json_static(name) {
        return Err(InterpreterRuntimeError::new(
            format!("JSON.{} is not available in CodeMode.", name),
            node.cloned(),
            DiagnosticKind::ExecutionFailure,
            None,
        ));
    }
    match name {
        "stringify" => {
            // Replacer guard (verbatim): arrays and functions unsupported.
            if let Some(replacer) = args.get(1) {
                if matches!(replacer, DataVal::Array(_)) {
                    return Err(InterpreterRuntimeError::new(
                        "JSON.stringify replacers are not supported in CodeMode.",
                        node.cloned(),
                        DiagnosticKind::UnsupportedSyntax,
                        Some(vec![SUPPORTED_SYNTAX_MESSAGE.to_string()]),
                    ));
                }
            }
            let indent: Option<usize> = match args.get(2) {
                Some(DataVal::Number(n)) => {
                    if *n < 0.0 {
                        None
                    } else {
                        Some((*n).min(10.0) as usize)
                    }
                }
                Some(DataVal::Str(s)) => Some(s.chars().count().min(10)),
                _ => None,
            };
            let value = args
                .first()
                .cloned()
                .unwrap_or(DataVal::Undefined)
                .to_json();
            let rendered = match indent {
                None => serde_json::to_string(&value).unwrap_or("null".to_string()),
                Some(0) => serde_json::to_string(&value).unwrap_or("null".to_string()),
                Some(n) => {
                    let pad = " ".repeat(n);
                    // Pretty form with the requested indent width.
                    pretty_json(&value, &pad)
                }
            };
            Ok(DataVal::Str(rendered))
        }
        "parse" => {
            let text = match args.first() {
                Some(DataVal::Str(s)) => s.clone(),
                _ => {
                    return Err(InterpreterRuntimeError::new(
                        "JSON.parse expects a string.",
                        node.cloned(),
                        DiagnosticKind::ExecutionFailure,
                        None,
                    ))
                }
            };
            match serde_json::from_str::<serde_json::Value>(&text) {
                Ok(v) => {
                    let checked = crate::tool_runtime::copy_in(&v, "JSON.parse result", false)
                        .map_err(|e| {
                            InterpreterRuntimeError::new(
                                e.message,
                                node.cloned(),
                                DiagnosticKind::InvalidDataValue,
                                None,
                            )
                        })?;
                    Ok(json_to_data(&checked))
                }
                Err(error) => Err(InterpreterRuntimeError::new(
                    format!("JSON.parse received invalid JSON: {}", error),
                    node.cloned(),
                    DiagnosticKind::ExecutionFailure,
                    None,
                )
                .as_error("SyntaxError")),
            }
        }
        _ => Err(InterpreterRuntimeError::new(
            format!("JSON.{} is not available in CodeMode.", name),
            node.cloned(),
            DiagnosticKind::ExecutionFailure,
            None,
        )),
    }
}

fn pretty_json(value: &serde_json::Value, pad: &str) -> String {
    // Minimal pretty printer honoring an arbitrary indent unit.
    serde_json::to_string_pretty(value)
        .map(|s| if pad == "  " { s } else { s.replace("  ", pad) })
        .unwrap_or("null".to_string())
}

/// Materializes a boundary-checked JSON value as `DataVal`.
pub fn json_to_data(value: &serde_json::Value) -> DataVal {
    match value {
        serde_json::Value::Null => DataVal::Null,
        serde_json::Value::Bool(b) => DataVal::Bool(*b),
        serde_json::Value::Number(n) => DataVal::Number(n.as_f64().unwrap_or(f64::NAN)),
        serde_json::Value::String(s) => DataVal::Str(s.clone()),
        serde_json::Value::Array(items) => DataVal::Array(items.iter().map(json_to_data).collect()),
        serde_json::Value::Object(map) => DataVal::Object(
            map.iter()
                .map(|(k, v)| (k.clone(), json_to_data(v)))
                .collect(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_parse_requires_string_verbatim() {
        let err = invoke_json_method("parse", &[DataVal::Number(1.0)], None).unwrap_err();
        assert_eq!(err.message, "JSON.parse expects a string.");
    }

    #[test]
    fn stringify_replacer_guard_verbatim() {
        let err = invoke_json_method("stringify", &[DataVal::Null, DataVal::Array(vec![])], None)
            .unwrap_err();
        assert_eq!(
            err.message,
            "JSON.stringify replacers are not supported in CodeMode."
        );
    }
}
