//! Port of `src/stdlib/object.ts`.

use crate::interpreter_model::{AstNode, DiagnosticKind, InterpreterRuntimeError};
use crate::stdlib_value::{bounded_data, coerce_to_string, DataVal};
use crate::values::{RtValue, SandboxMap, SandboxValue};

/// Object statics, verbatim. Mirrors `objectStatics`.
pub const OBJECT_STATICS: &[&str] = &[
    "keys",
    "values",
    "entries",
    "hasOwn",
    "assign",
    "fromEntries",
];

/// Mirrors `objectStatics.has(name)`.
pub fn is_object_static(name: &str) -> bool {
    OBJECT_STATICS.contains(&name)
}

/// Mirrors `invokeObjectMethod(name, args, node)`.
pub fn invoke_object_method(
    name: &str,
    args: &[DataVal],
    node: Option<&AstNode>,
) -> Result<DataVal, InterpreterRuntimeError> {
    if !is_object_static(name) {
        return Err(InterpreterRuntimeError::new(
            format!("Object.{} is not available in CodeMode.", name),
            node.cloned(),
            DiagnosticKind::ExecutionFailure,
            None,
        ));
    }
    match name {
        "keys" => {
            let input = args.first().cloned().unwrap_or(DataVal::Undefined);
            let value = bounded_data(&input, "Object.keys input")?;
            match value {
                DataVal::Sandbox(_) => Ok(DataVal::Array(vec![])),
                DataVal::Array(items) => Ok(DataVal::Array(
                    (0..items.len())
                        .map(|i| DataVal::Str(i.to_string()))
                        .collect(),
                )),
                DataVal::Object(entries) => Ok(DataVal::Array(
                    entries.into_iter().map(|(k, _)| DataVal::Str(k)).collect(),
                )),
                _ => Err(InterpreterRuntimeError::new(
                    "Object.keys expects a data object or array.",
                    node.cloned(),
                    DiagnosticKind::ExecutionFailure,
                    None,
                )),
            }
        }
        "values" => {
            let obj = require_object(args, name, node)?;
            Ok(DataVal::Array(obj.into_iter().map(|(_, v)| v).collect()))
        }
        "entries" => {
            let obj = require_object(args, name, node)?;
            Ok(DataVal::Array(
                obj.into_iter()
                    .map(|(k, v)| DataVal::Array(vec![DataVal::Str(k), v]))
                    .collect(),
            ))
        }
        "hasOwn" => {
            let obj = require_object(args, name, node)?;
            let key = match args.get(1) {
                Some(v) => coerce_to_string(v),
                None => "undefined".to_string(),
            };
            Ok(DataVal::Bool(obj.iter().any(|(k, _)| k == &key)))
        }
        "assign" => {
            let mut out: Vec<(String, DataVal)> = vec![];
            for source in args {
                if matches!(source, DataVal::Null | DataVal::Undefined) {
                    continue;
                }
                let value = bounded_data(source, "Object.assign input")?;
                if matches!(value, DataVal::Sandbox(_)) {
                    continue;
                }
                match value {
                    DataVal::Object(entries) => {
                        for (key, item) in entries {
                            guarded_set(&mut out, &key, item, node)?;
                        }
                    }
                    _ => {
                        return Err(InterpreterRuntimeError::new(
                            "Object.assign expects data objects.",
                            node.cloned(),
                            DiagnosticKind::ExecutionFailure,
                            None,
                        ))
                    }
                }
            }
            Ok(DataVal::Object(out))
        }
        "fromEntries" => {
            // SandboxMap / URLSearchParams fast paths (verbatim).
            if let Some(DataVal::Sandbox(SandboxValue::Map(m))) = args.first() {
                let entries = map_entries(m, node)?;
                return Ok(DataVal::Object(entries));
            }
            if let Some(DataVal::Sandbox(SandboxValue::UrlSearchParams(p))) = args.first() {
                let mut out = vec![];
                for (key, value) in &p.pairs {
                    guarded_set(&mut out, key, DataVal::Str(value.clone()), node)?;
                }
                return Ok(DataVal::Object(out));
            }
            let input = args.first().cloned().unwrap_or(DataVal::Undefined);
            let pairs = bounded_data(&input, "Object.fromEntries input")?;
            match pairs {
                DataVal::Array(items) => {
                    let mut out = vec![];
                    for pair in items {
                        match pair {
                            DataVal::Array(mut kv) if kv.len() >= 2 => {
                                let v = kv.pop().unwrap();
                                let k = kv.pop().unwrap();
                                guarded_set(&mut out, &coerce_to_string(&k), v, node)?;
                            }
                            _ => {
                                return Err(InterpreterRuntimeError::new(
                                    "Object.fromEntries expects [key, value] pairs.",
                                    node.cloned(),
                                    DiagnosticKind::ExecutionFailure,
                                    None,
                                ))
                            }
                        }
                    }
                    Ok(DataVal::Object(out))
                }
                _ => Err(InterpreterRuntimeError::new(
                    "Object.fromEntries expects an array of [key, value] pairs.",
                    node.cloned(),
                    DiagnosticKind::ExecutionFailure,
                    None,
                )),
            }
        }
        _ => Err(InterpreterRuntimeError::new(
            format!("Object.{} is not available in CodeMode.", name),
            node.cloned(),
            DiagnosticKind::ExecutionFailure,
            None,
        )),
    }
}

fn require_object(
    args: &[DataVal],
    name: &str,
    node: Option<&AstNode>,
) -> Result<Vec<(String, DataVal)>, InterpreterRuntimeError> {
    let input = args.first().cloned().unwrap_or(DataVal::Undefined);
    let value = bounded_data(&input, &format!("Object.{} input", name))?;
    if matches!(value, DataVal::Sandbox(_)) {
        return Ok(vec![]);
    }
    match value {
        DataVal::Object(entries) => Ok(entries),
        _ => Err(InterpreterRuntimeError::new(
            format!("Object.{} expects a data object.", name),
            node.cloned(),
            DiagnosticKind::ExecutionFailure,
            None,
        )),
    }
}

fn guarded_set(
    out: &mut Vec<(String, DataVal)>,
    key: &str,
    item: DataVal,
    node: Option<&AstNode>,
) -> Result<(), InterpreterRuntimeError> {
    if crate::tool_runtime::is_blocked_member(key) {
        return Err(InterpreterRuntimeError::new(
            format!("Property '{}' is not available in CodeMode.", key),
            node.cloned(),
            DiagnosticKind::ExecutionFailure,
            None,
        ));
    }
    if let Some(slot) = out.iter_mut().find(|(k, _)| k == key) {
        slot.1 = item;
    } else {
        out.push((key.to_string(), item));
    }
    Ok(())
}

fn map_entries(
    map: &SandboxMap,
    node: Option<&AstNode>,
) -> Result<Vec<(String, DataVal)>, InterpreterRuntimeError> {
    let mut out = vec![];
    for (key, item) in &map.entries {
        let key_str = rt_key_to_string(key);
        guarded_set(&mut out, &key_str, rt_to_data_value(item), node)?;
    }
    Ok(out)
}

/// Coerces a live map key to its `Object.fromEntries` property name.
fn rt_key_to_string(key: &RtValue) -> String {
    match key {
        RtValue::Str(s) => s.clone(),
        RtValue::Number(n) => crate::stdlib_value::js_number_to_string(*n),
        RtValue::Bool(b) => b.to_string(),
        RtValue::Null => "null".to_string(),
        RtValue::Undefined => "undefined".to_string(),
        _ => "unknown".to_string(),
    }
}

/// Converts a live runtime value to a [`DataVal`] for stdlib checkpoints.
fn rt_to_data_value(value: &RtValue) -> DataVal {
    match value {
        RtValue::Undefined => DataVal::Undefined,
        RtValue::Null => DataVal::Null,
        RtValue::Bool(b) => DataVal::Bool(*b),
        RtValue::Number(n) => DataVal::Number(*n),
        RtValue::Str(s) => DataVal::Str(s.clone()),
        RtValue::Array(arr) => DataVal::Array(arr.items.iter().map(rt_to_data_value).collect()),
        RtValue::Object(obj) => DataVal::Object(
            obj.entries
                .iter()
                .map(|(k, v)| (k.clone(), rt_to_data_value(v)))
                .collect(),
        ),
        RtValue::Sandbox(s) => DataVal::Sandbox(s.clone()),
        _ => DataVal::Undefined,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_guards_verbatim() {
        let err = invoke_object_method("getPrototypeOf", &[], None).unwrap_err();
        assert_eq!(
            err.message,
            "Object.getPrototypeOf is not available in CodeMode."
        );
        let err = invoke_object_method("keys", &[DataVal::Number(1.0)], None).unwrap_err();
        assert_eq!(err.message, "Object.keys expects a data object or array.");
    }
}
