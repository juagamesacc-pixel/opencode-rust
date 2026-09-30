//! Port of `test/parity.test.ts` (JS-parity spot checks: forgiving
//! property reads, null/undefined spread, typeof guards, NaN/Infinity flow,
//! error values).

mod common;

use codemode::tool::ToolTree;
use common::*;
use serde_json::json;

fn run_value(code: String) -> serde_json::Value {
    value(run(code, ToolTree::new()))
}

fn run_error(code: String) -> codemode::codemode::Diagnostic {
    error(run(code, ToolTree::new()))
}

#[test]
fn unknown_string_property_is_undefined() {
    let code = prog(vec![ret(Some(dot(lit(json!("hi")), "nope")))]);
    assert_eq!(run_value(code), serde_json::Value::Null);
}

#[test]
fn optional_chain_fallback_on_string() {
    // return ("hi".missing ?? "fallback")
    let code = prog(vec![ret(Some(
        json!({"type": "LogicalExpression", "operator": "??",
        "left": dot(lit(json!("hi")), "missing"), "right": lit(json!("fallback"))}),
    ))]);
    assert_eq!(run_value(code), json!("fallback"));
}

#[test]
fn unknown_number_property_is_undefined() {
    let code = prog(vec![ret(Some(dot(lit(json!(1)), "nope")))]);
    assert_eq!(run_value(code), serde_json::Value::Null);
}

#[test]
fn unknown_array_property_is_undefined() {
    let code = prog(vec![ret(Some(dot(arr(vec![lit(json!(1))]), "nope")))]);
    assert_eq!(run_value(code), serde_json::Value::Null);
}

#[test]
fn spreading_null_is_noop() {
    // return { ...null, a: 1 }
    let code = prog(vec![ret(Some(
        json!({"type": "ObjectExpression", "properties": [
            {"type": "SpreadElement", "argument": lit(json!(null))},
            {"type": "Property", "kind": "init", "computed": false,
                "key": ident("a"), "value": lit(json!(1))},
        ]}),
    ))]);
    assert_eq!(run_value(code), json!({"a": 1}));
}

#[test]
fn spreading_array_into_object_errors() {
    let code = prog(vec![ret(Some(
        json!({"type": "ObjectExpression", "properties": [
            {"type": "SpreadElement", "argument": arr(vec![lit(json!(1)), lit(json!(2))])},
            {"type": "Property", "kind": "init", "computed": false,
                "key": ident("a"), "value": lit(json!(1))},
        ]}),
    ))]);
    let failure = run_error(code);
    assert_eq!(
        failure.message,
        "Object spread requires a data object in CodeMode."
    );
}

#[test]
fn typeof_undeclared_is_undefined() {
    let code = prog(vec![ret(Some(
        json!({"type": "UnaryExpression", "operator": "typeof",
        "argument": ident("nope"), "prefix": true}),
    ))]);
    assert_eq!(run_value(code), json!("undefined"));
}

#[test]
fn undeclared_reference_outside_typeof_throws() {
    let code = prog(vec![ret(Some(ident("nope")))]);
    let failure = run_error(code);
    assert_eq!(failure.message, "Unknown identifier 'nope'.");
}

#[test]
fn nan_guards_run_in_sandbox() {
    // const x = parseInt("nope"); return Number.isNaN(x) ? "nan" : "num"
    let code = prog(vec![
        var_decl(
            "const",
            "x",
            Some(call(ident("parseInt"), vec![lit(json!("nope"))])),
        ),
        ret(Some(json!({"type": "ConditionalExpression",
            "test": call(dot(ident("Number"), "isNaN"), vec![ident("x")]),
            "consequent": lit(json!("nan")), "alternate": lit(json!("num"))}))),
    ]);
    assert_eq!(run_value(code), json!("nan"));
}

#[test]
fn non_finite_normalizes_to_null_at_boundary() {
    assert_eq!(
        run_value(prog(vec![ret(Some(ident("NaN")))])),
        serde_json::Value::Null
    );
    assert_eq!(
        run_value(prog(vec![ret(Some(ident("Infinity")))])),
        serde_json::Value::Null
    );
}

#[test]
fn copy_out_normalizes_non_finite() {
    assert_eq!(
        codemode::tool_runtime::copy_out_number(f64::NAN),
        serde_json::Value::Null
    );
    assert_eq!(
        codemode::tool_runtime::copy_out_number(f64::INFINITY),
        serde_json::Value::Null
    );
    assert_eq!(codemode::tool_runtime::copy_out_number(1.5), json!(1.5));
}

#[test]
fn error_values_carry_name_message_and_instanceof() {
    // const e = new Error("m"); return { name: e.name, isError: e instanceof Error }
    let code = prog(vec![
        var_decl(
            "const",
            "e",
            Some(json!({"type": "NewExpression", "callee": ident("Error"),
                "arguments": [lit(json!("m"))]})),
        ),
        ret(Some(obj(vec![
            ("name".to_string(), dot(ident("e"), "name")),
            (
                "isError".to_string(),
                json!({"type": "BinaryExpression", "operator": "instanceof",
                    "left": ident("e"), "right": ident("Error")}),
            ),
        ]))),
    ]);
    assert_eq!(run_value(code), json!({"name": "Error", "isError": true}));
}
