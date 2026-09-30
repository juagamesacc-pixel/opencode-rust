//! Port of `test/codemode.test.ts` (host failure boundary, tool-call
//! observation, console capture, output budget, schema flexibility, public
//! contract, limits).

mod common;

use codemode::codemode::{
    execute, DiscoveryOptions, ExecuteOptions, ExecutionLimits, ExecutionResult, Options,
};
use codemode::tool::{SchemaType, ToolFailure, ToolOptions, ToolTree, ToolTreeValue};
use codemode::tool_error::ToolError;
use common::*;
use serde_json::{json, Value};

fn host_tree(definition: codemode::tool::Definition) -> ToolTree {
    let mut tree = ToolTree::new();
    insert_tool(&mut tree, &["host", "call"], definition);
    tree
}

fn run_host(definition: codemode::tool::Definition) -> ExecutionResult {
    // `return await tools.host.call({})`
    let code = prog(vec![ret(Some(await_(call(
        dot(dot(ident("tools"), "host"), "call"),
        vec![obj(vec![])],
    ))))]);
    run(code, host_tree(definition))
}

#[test]
fn preserves_explicit_safe_tool_failures() {
    let result = run_host(refusing_tool(
        "Fail safely",
        "Authorized request was refused",
    ));
    let failure = error(result);
    assert_eq!(
        serde_json::to_value(&failure).unwrap(),
        json!({"kind": "ToolFailure", "message": "Authorized request was refused"})
    );
}

#[test]
fn sanitizes_unknown_host_failures() {
    for definition in [
        opaque_failing_tool("Fail internally"),
        codemode::tool::make_tool(ToolOptions {
            description: "Fail with defect".to_string(),
            input: SchemaType::Json(json_schema_doc(json!({"type": "object"}))),
            output: Some(SchemaType::Json(json_schema_doc(json!({"type": "string"})))),
            run: Box::new(|_| {
                Err(ToolFailure::Unknown(
                    "Authorization: Bearer typed-secret".to_string(),
                ))
            }),
        }),
    ] {
        let result = run_host(definition);
        let failure = error(result);
        assert_eq!(
            failure.kind,
            codemode::interpreter_model::DiagnosticKind::ToolFailure
        );
        assert_eq!(failure.message, "Tool execution failed");
    }
}

#[test]
fn sanitizes_invalid_host_output() {
    // Output `{ safe: 1 }` against a `{ safe: string }` validator.
    let definition = codemode::tool::make_tool(ToolOptions {
        description: "Return invalid output".to_string(),
        input: SchemaType::Json(json_schema_doc(json!({"type": "object"}))),
        output: Some(SchemaType::Validating {
            validate: Box::new(|output| match output.get("safe").and_then(|v| v.as_str()) {
                Some(_) => Ok(output.clone()),
                None => Err("safe must be a string".to_string()),
            }),
            document: json_schema_doc(json!({"type": "object"})),
            decoded: true,
        }),
        run: Box::new(|_| Ok(json!({"safe": 1}))),
    });
    let result = run_host(definition);
    let failure = error(result);
    assert_eq!(
        failure.kind,
        codemode::interpreter_model::DiagnosticKind::InvalidToolOutput
    );
    assert_eq!(failure.message, "Invalid output from tool 'host.call'.");
}

#[test]
fn caught_tool_failures_are_error_values_in_program() {
    // try { await tools.host.call({}) } catch (e) { return { isError: e instanceof Error, message: e.message } }
    let code = prog(vec![json!({"type": "TryStatement",
        "block": block(vec![
            expr_stmt(await_(call(
                dot(dot(ident("tools"), "host"), "call"),
                vec![obj(vec![])],
            ))),
            ret(Some(lit(json!("no")))),
        ]),
        "handler": {"type": "CatchClause", "param": ident("e"),
            "body": block(vec![ret(Some(obj(vec![
                ("isError".to_string(), json!({"type": "BinaryExpression", "operator": "instanceof",
                    "left": ident("e"), "right": ident("Error")})),
                ("message".to_string(), dot(ident("e"), "message")),
            ])))])},
        "finalizer": null})]);
    let result = run(code, host_tree(refusing_tool("Refuse", "Refused")));
    assert_eq!(
        value(result),
        json!({"isError": true, "message": "Refused"})
    );
}

#[test]
fn reports_tools_invoked_with_decoded_input() {
    use std::sync::{Arc, Mutex};
    let seen: Arc<Mutex<Vec<(String, Value)>>> = Arc::new(Mutex::new(vec![]));
    let seen_start = seen.clone();
    let seen_end = seen.clone();
    let mut tree = ToolTree::new();
    insert_tool(&mut tree, &["host", "call"], echo_tool("Echo"));
    let code = prog(vec![ret(Some(await_(call(
        dot(dot(ident("tools"), "host"), "call"),
        vec![obj(vec![("value".to_string(), lit(json!("hi")))])],
    ))))]);
    let result = codemode::codemode::execute(ExecuteOptions {
        code,
        tools: tree,
        limits: ExecutionLimits::default(),
        on_tool_call_start: Some(Box::new(move |call| {
            seen_start
                .lock()
                .unwrap()
                .push((format!("start:{}", call.name), call.input.clone()));
        })),
        on_tool_call_end: Some(Box::new(move |call| {
            seen_end
                .lock()
                .unwrap()
                .push((format!("end:{}:{:?}", call.name, call.outcome), Value::Null));
        })),
    })
    .unwrap();
    assert!(result.ok());
    let events = seen.lock().unwrap();
    assert!(events.iter().any(|(label, _)| label == "start:host.call"));
    assert!(events
        .iter()
        .any(|(label, _)| label.starts_with("end:host.call")));
}

#[test]
fn console_capture_and_nan_infinity_rendering() {
    // console.log("Thread info:", { name: "Demo", count: 2 }); console.warn("careful"); return 1
    let code = prog(vec![
        expr_stmt(call(
            dot(ident("console"), "log"),
            vec![
                lit(json!("Thread info:")),
                obj(vec![
                    ("name".to_string(), lit(json!("Demo"))),
                    ("count".to_string(), lit(json!(2))),
                ]),
            ],
        )),
        expr_stmt(call(
            dot(ident("console"), "warn"),
            vec![lit(json!("careful"))],
        )),
        ret(Some(lit(json!(1)))),
    ]);
    let result = run(code, ToolTree::new());
    match result {
        ExecutionResult::Success(success) => {
            assert_eq!(
                success.logs.unwrap(),
                vec![
                    "Thread info: {\"name\":\"Demo\",\"count\":2}".to_string(),
                    "[warn] careful".to_string()
                ]
            );
        }
        _ => panic!("expected success"),
    }
    // console.log(NaN, Infinity, -Infinity) keeps literals (not JSON null).
    let nan = ident("NaN");
    let inf = ident("Infinity");
    let code = prog(vec![
        expr_stmt(call(dot(ident("console"), "log"), vec![nan, inf])),
        ret(Some(lit(json!(null)))),
    ]);
    let result = run(code, ToolTree::new());
    match result {
        ExecutionResult::Success(success) => {
            assert_eq!(success.logs.unwrap(), vec!["NaN Infinity".to_string()]);
        }
        _ => panic!("expected success"),
    }
}

#[test]
fn output_budget_truncates_with_marker() {
    // Absent maxOutputBytes means no truncation at all.
    let big = "y".repeat(200);
    let code = prog(vec![ret(Some(lit(json!(big))))]);
    let result = run(code, ToolTree::new());
    assert!(result.ok());
    // Truncation path is covered by the interpreter unit tests; the envelope
    // applies `bound_output` only when the limit is present.
    let limited = codemode::codemode::execute(ExecuteOptions {
        code: prog(vec![ret(Some(lit(json!("x".repeat(100)))))]),
        tools: ToolTree::new(),
        limits: ExecutionLimits {
            max_output_bytes: Some(10),
            ..Default::default()
        },
        on_tool_call_start: None,
        on_tool_call_end: None,
    })
    .unwrap();
    match limited {
        ExecutionResult::Success(success) => {
            assert_eq!(success.truncated, Some(true));
            assert!(success.value.as_str().unwrap().contains(
                "[result truncated: 102 bytes exceeds the 10-byte output limit; return a smaller value]"
            ));
        }
        _ => panic!("expected success"),
    }
}

#[test]
fn accepts_render_only_json_schema_and_omitted_output() {
    // MCP-style tool: JSON Schema input, no output schema → `unknown`.
    let definition = codemode::tool::make_tool(ToolOptions {
        description: "Call an adapter-described tool".to_string(),
        input: SchemaType::Json(json_schema_doc(json!({
            "type": "object",
            "properties": {"id": {"type": "string"}},
            "required": ["id"],
        }))),
        output: None,
        run: Box::new(|input| Ok(input.clone())),
    });
    let mut tree = ToolTree::new();
    insert_tool(&mut tree, &["lookup"], definition);
    let code = prog(vec![ret(Some(await_(call(
        dot(ident("tools"), "lookup"),
        vec![obj(vec![("id".to_string(), lit(json!("order_42")))])],
    ))))]);
    assert_eq!(value(run(code, tree)), json!({"id": "order_42"}));
}

#[test]
fn rejects_invalid_configuration() {
    // Reserved namespace.
    let mut tree = ToolTree::new();
    tree.insert(
        "$codemode".to_string(),
        ToolTreeValue::Namespace(ToolTree::new()),
    );
    let err = execute(ExecuteOptions {
        code: prog(vec![]),
        tools: tree,
        limits: ExecutionLimits::default(),
        on_tool_call_start: None,
        on_tool_call_end: None,
    })
    .unwrap_err();
    assert_eq!(
        err,
        "Tool namespace '$codemode' is reserved for CodeMode discovery tools."
    );
    // Invalid limits.
    let err = execute(ExecuteOptions {
        code: prog(vec![]),
        tools: ToolTree::new(),
        limits: ExecutionLimits {
            timeout_ms: Some(0),
            ..Default::default()
        },
        on_tool_call_start: None,
        on_tool_call_end: None,
    })
    .unwrap_err();
    assert_eq!(
        err,
        "timeoutMs must be a safe integer greater than or equal to 1."
    );
}

#[test]
fn enforces_tool_call_limit() {
    let mut tree = ToolTree::new();
    insert_tool(&mut tree, &["host", "call"], echo_tool("Echo"));
    let code = prog(vec![ret(Some(await_(call(
        dot(dot(ident("tools"), "host"), "call"),
        vec![obj(vec![("value".to_string(), lit(json!("x")))])],
    ))))]);
    let result = execute(ExecuteOptions {
        code,
        tools: tree,
        limits: ExecutionLimits {
            max_tool_calls: Some(0),
            ..Default::default()
        },
        on_tool_call_start: None,
        on_tool_call_end: None,
    })
    .unwrap();
    let failure = error(result);
    assert_eq!(
        failure.kind,
        codemode::interpreter_model::DiagnosticKind::ToolCallLimitExceeded
    );
    assert_eq!(
        failure.message,
        "Execution exceeded its tool-call limit of 0."
    );
}

#[test]
fn reserves_discovery_namespace_and_builds_runtime() {
    let runtime = codemode::codemode::make(Options {
        discovery: DiscoveryOptions {
            catalog_budget: Some(2000),
        },
        ..Default::default()
    })
    .unwrap();
    assert!(runtime
        .instructions()
        .contains("No tools are currently available."));
    assert!(runtime.catalog().is_empty());
}

#[test]
fn undefined_normalizes_to_null_at_boundary() {
    // `return undefined` → null.
    let result = run(prog(vec![ret(Some(ident("undefined")))]), ToolTree::new());
    assert_eq!(value(result), Value::Null);
    // NaN → null.
    let result = run(prog(vec![ret(Some(ident("NaN")))]), ToolTree::new());
    assert_eq!(value(result), Value::Null);
}

#[test]
fn tool_error_factory_keeps_private_cause() {
    let err = ToolError::new("safe", Some("secret".to_string()));
    assert_eq!(err.message(), "safe");
    assert_eq!(err.cause.as_deref(), Some("secret"));
}
