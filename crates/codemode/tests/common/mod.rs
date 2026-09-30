#![allow(dead_code)] // shared harness: helpers are used by sibling test targets, not every target
//! Shared harness for the `codemode` integration tests (1:1 ports of
//! `packages/codemode/test/*.test.ts`).
//!
//! R1 NOTE: the Rust port consumes Acorn `Program` JSON in `code`
//! (parser-equivalence is a flagged risk; no parser dependency). These builders
//! emit Acorn-shaped AST JSON (with `loc` omitted — `sourceLocation` falls
//! back to the documented defaults), so each ported case exercises the same
//! interpreter path as its TypeScript source.

use codemode::codemode::{ExecuteOptions, ExecutionLimits, ExecutionResult};
use codemode::tool::{SchemaType, ToolFailure, ToolOptions, ToolTree, ToolTreeValue};
use codemode::tool_error::ToolError;
use serde_json::{json, Value};

/// Builds an Acorn `Program` JSON document from statement values.
pub fn prog(body: Vec<Value>) -> String {
    json!({"type": "Program", "body": body}).to_string()
}

/// Runs one program against a tool tree (no limits, real clock).
pub fn run(code: String, tools: ToolTree) -> ExecutionResult {
    codemode::codemode::execute(ExecuteOptions {
        code,
        tools,
        limits: ExecutionLimits::default(),
        on_tool_call_start: None,
        on_tool_call_end: None,
    })
    .expect("valid limits")
}

/// Extracts the success value or panics with the diagnostic.
pub fn value(result: ExecutionResult) -> Value {
    match result {
        ExecutionResult::Success(success) => {
            assert!(success.ok);
            success.value
        }
        ExecutionResult::Failure(failure) => {
            panic!(
                "expected success, got {}: {}",
                failure.error.kind, failure.error.message
            )
        }
    }
}

/// Extracts the failure diagnostic or panics with the value.
pub fn error(result: ExecutionResult) -> codemode::codemode::Diagnostic {
    match result {
        ExecutionResult::Failure(failure) => {
            assert!(!failure.ok);
            failure.error
        }
        ExecutionResult::Success(success) => {
            panic!("expected failure, got value {}", success.value)
        }
    }
}

/// A validating echo tool: decodes `{ value: string }`, returns it.
pub fn echo_tool(description: &str) -> codemode::tool::Definition {
    let validator = |input: &Value| -> Result<Value, String> {
        match input.get("value").and_then(|v| v.as_str()) {
            Some(_) => Ok(input.clone()),
            _ => Err("expected { value: string }".to_string()),
        }
    };
    let output_validator = |output: &Value| -> Result<Value, String> {
        match output.as_str() {
            Some(_) => Ok(output.clone()),
            _ => Err("expected string output".to_string()),
        }
    };
    let input_doc = json_schema_doc(json!({
        "type": "object",
        "properties": {"value": {"type": "string"}},
        "required": ["value"],
    }));
    let output_doc = json_schema_doc(json!({"type": "string"}));
    codemode::tool::make_tool(ToolOptions {
        description: description.to_string(),
        input: SchemaType::Validating {
            validate: Box::new(validator),
            document: input_doc,
            decoded: false,
        },
        output: Some(SchemaType::Validating {
            validate: Box::new(output_validator),
            document: output_doc,
            decoded: true,
        }),
        run: Box::new(|input| Ok(input.get("value").cloned().unwrap_or(Value::Null))),
    })
}

/// Builds a JSON-Schema-document tool definition.
pub fn json_schema_doc(schema: Value) -> codemode::tool::JsonSchema {
    serde_json::from_value(schema).unwrap_or_default()
}

/// A tool that always refuses with a safe message.
pub fn refusing_tool(description: &str, message: &str) -> codemode::tool::Definition {
    let message = message.to_string();
    codemode::tool::make_tool(ToolOptions {
        description: description.to_string(),
        input: SchemaType::Json(json_schema_doc(json!({"type": "object"}))),
        output: Some(SchemaType::Json(json_schema_doc(json!({"type": "string"})))),
        run: Box::new(move |_| {
            Err(ToolFailure::ToolError(ToolError::new(
                message.clone(),
                None,
            )))
        }),
    })
}

/// A tool that fails opaquely (sanitized to `"Tool execution failed"`).
pub fn opaque_failing_tool(description: &str) -> codemode::tool::Definition {
    codemode::tool::make_tool(ToolOptions {
        description: description.to_string(),
        input: SchemaType::Json(json_schema_doc(json!({"type": "object"}))),
        output: Some(SchemaType::Json(json_schema_doc(json!({"type": "string"})))),
        run: Box::new(|_| Err(ToolFailure::Unknown("defect".to_string()))),
    })
}

/// Inserts a definition at a dotted path of a tool tree.
pub fn insert_tool(tree: &mut ToolTree, path: &[&str], definition: codemode::tool::Definition) {
    let (head, rest) = path.split_first().expect("non-empty path");
    if rest.is_empty() {
        tree.insert(
            head.to_string(),
            ToolTreeValue::Definition(Box::new(definition)),
        );
        return;
    }
    if !matches!(tree.get(head), Some(ToolTreeValue::Namespace(_))) {
        tree.insert(head.to_string(), ToolTreeValue::Namespace(ToolTree::new()));
    }
    match tree.get_mut(head) {
        Some(ToolTreeValue::Namespace(sub)) => insert_tool(sub, rest, definition),
        _ => unreachable!("namespace ensured above"),
    }
}

// --- Acorn AST builders -------------------------------------------------------

/// `"literal"` node.
pub fn lit(value: Value) -> Value {
    json!({"type": "Literal", "value": value})
}

/// Identifier node.
pub fn ident(name: &str) -> Value {
    json!({"type": "Identifier", "name": name})
}

/// `object.property` / `object[prop]` node.
pub fn member(object: Value, property: Value, computed: bool) -> Value {
    json!({"type": "MemberExpression", "object": object, "property": property, "computed": computed, "optional": false})
}

/// `object.name` node.
pub fn dot(object: Value, name: &str) -> Value {
    member(object, ident(name), false)
}

/// `callee(args)` node.
pub fn call(callee: Value, args: Vec<Value>) -> Value {
    json!({"type": "CallExpression", "callee": callee, "arguments": args, "optional": false})
}

/// `await expr` node.
pub fn await_(argument: Value) -> Value {
    json!({"type": "AwaitExpression", "argument": argument})
}

/// `return expr?` statement.
pub fn ret(argument: Option<Value>) -> Value {
    json!({"type": "ReturnStatement", "argument": argument})
}

/// Expression statement.
pub fn expr_stmt(expression: Value) -> Value {
    json!({"type": "ExpressionStatement", "expression": expression})
}

/// `const/let name = init` statement.
pub fn var_decl(kind: &str, name: &str, init: Option<Value>) -> Value {
    json!({"type": "VariableDeclaration", "kind": kind,
        "declarations": [{"type": "VariableDeclarator", "id": ident(name), "init": init}]})
}

/// Object expression from entries.
pub fn obj(entries: Vec<(String, Value)>) -> Value {
    json!({"type": "ObjectExpression", "properties": entries.into_iter().map(|(key, value)| {
        json!({"type": "Property", "kind": "init", "computed": false,
            "key": ident(&key), "value": value})
    }).collect::<Vec<_>>()})
}

/// Array expression.
pub fn arr(elements: Vec<Value>) -> Value {
    json!({"type": "ArrayExpression", "elements": elements})
}

/// `(params) => body` node.
pub fn arrow(params: Vec<&str>, body: Value, expression: bool) -> Value {
    json!({"type": "ArrowFunctionExpression", "params": params.into_iter().map(ident).collect::<Vec<_>>(),
        "body": body, "expression": expression, "generator": false})
}

/// Block statement.
pub fn block(body: Vec<Value>) -> Value {
    json!({"type": "BlockStatement", "body": body})
}

/// Binary expression.
pub fn binary(operator: &str, left: Value, right: Value) -> Value {
    json!({"type": "BinaryExpression", "operator": operator, "left": left, "right": right})
}

/// Template literal with a single interpolation.
pub fn template(cooked: &str, expression: Value) -> Value {
    json!({"type": "TemplateLiteral",
        "quasis": [{"type": "TemplateElement", "value": {"cooked": cooked}}],
        "expressions": [expression]})
}

/// `new Name(args)` node.
pub fn new_expr(name: &str, args: Vec<Value>) -> Value {
    json!({"type": "NewExpression", "callee": ident(name), "arguments": args})
}
