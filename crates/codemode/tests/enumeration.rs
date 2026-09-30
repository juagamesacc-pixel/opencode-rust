//! Port of `test/enumeration.test.ts`.
//!
//! Key enumeration: `Object.keys` and `for...in` share one surface over plain
//! objects, arrays (index strings), and tool references (namespace/tool names
//! from the host tool tree).

mod common;

use codemode::tool::ToolTree;
use common::*;
use serde_json::json;

fn tools() -> ToolTree {
    let mut tree = ToolTree::new();
    insert_tool(
        &mut tree,
        &["github", "list_issues"],
        echo_tool("List issues"),
    );
    insert_tool(
        &mut tree,
        &["github", "get_issue"],
        echo_tool("Get one issue"),
    );
    insert_tool(&mut tree, &["memory", "search"], echo_tool("Search memory"));
    insert_tool(
        &mut tree,
        &["playwright", "navigate"],
        echo_tool("Navigate somewhere"),
    );
    tree
}

fn run_value(code: String) -> serde_json::Value {
    value(run(code, tools()))
}

fn run_error(code: String) -> codemode::codemode::Diagnostic {
    error(run(code, tools()))
}

#[test]
fn object_keys_enumerates_top_level_namespaces() {
    // `const namespaces = Object.keys(tools); return namespaces`
    let namespaces = run_value(prog(vec![ret(Some(call(
        dot(ident("Object"), "keys"),
        vec![ident("tools")],
    )))]));
    assert_eq!(
        namespaces,
        json!(["github", "memory", "playwright", "$codemode"])
    );
}

#[test]
fn object_keys_enumerates_nested_namespace() {
    let result = run_value(prog(vec![ret(Some(call(
        dot(ident("Object"), "keys"),
        vec![dot(ident("tools"), "github")],
    )))]));
    assert_eq!(result, json!(["list_issues", "get_issue"]));
}

#[test]
fn callable_tool_enumerates_as_empty_leaf() {
    let result = run_value(prog(vec![ret(Some(call(
        dot(ident("Object"), "keys"),
        vec![dot(dot(ident("tools"), "github"), "list_issues")],
    )))]));
    assert_eq!(result, json!([]));
}

#[test]
fn discovery_namespace_enumerates_search() {
    let result = run_value(prog(vec![ret(Some(call(
        dot(ident("Object"), "keys"),
        vec![dot(ident("tools"), "$codemode")],
    )))]));
    assert_eq!(result, json!(["search"]));
}

#[test]
fn unknown_namespace_is_unknown_tool_with_idioms() {
    let failure = run_error(prog(vec![ret(Some(call(
        dot(ident("Object"), "keys"),
        vec![dot(ident("tools"), "nonexistent")],
    )))]));
    assert_eq!(
        failure.kind,
        codemode::interpreter_model::DiagnosticKind::UnknownTool
    );
    assert!(failure
        .message
        .contains("Unknown tool namespace 'nonexistent'"));
    assert!(failure
        .suggestions
        .unwrap_or_default()
        .join(" ")
        .contains("Object.keys(tools)"));
}

#[test]
fn object_values_entries_on_tools_point_at_idioms() {
    for method in ["values", "entries"] {
        let failure = run_error(prog(vec![ret(Some(call(
            dot(ident("Object"), method),
            vec![ident("tools")],
        )))]));
        assert_eq!(
            failure.kind,
            codemode::interpreter_model::DiagnosticKind::InvalidDataValue
        );
        assert!(failure.message.contains(&format!(
            "Object.{}(...) cannot read tool references: they are not plain data. Use Object.keys(tools) for names, or tools.$codemode.search({{ query }}) for signatures.",
            method
        )));
    }
}

#[test]
fn object_keys_over_arrays_returns_index_strings() {
    let result = run_value(prog(vec![ret(Some(call(
        dot(ident("Object"), "keys"),
        vec![arr(vec![lit(json!("a")), lit(json!("b")), lit(json!("c"))])],
    )))]));
    assert_eq!(result, json!(["0", "1", "2"]));
    let result = run_value(prog(vec![ret(Some(call(
        dot(ident("Object"), "keys"),
        vec![arr(vec![])],
    )))]));
    assert_eq!(result, json!([]));
}

#[test]
fn objects_keep_own_enumerable_keys() {
    let result = run_value(prog(vec![ret(Some(call(
        dot(ident("Object"), "keys"),
        vec![obj(vec![
            ("a".to_string(), lit(json!(1))),
            ("b".to_string(), lit(json!(2))),
        ])],
    )))]));
    assert_eq!(result, json!(["a", "b"]));
}

#[test]
fn non_object_inputs_fail_clearly() {
    let failure = run_error(prog(vec![ret(Some(call(
        dot(ident("Object"), "keys"),
        vec![lit(json!("nope"))],
    )))]));
    assert!(failure
        .message
        .contains("Object.keys expects a data object or array"));
}

#[test]
fn for_in_iterates_object_keys_with_break_continue() {
    // for (const key in {a,b,c,d}) { if (key === "b") continue; if (key === "d") break; seen.push(key) }
    let seen_push = expr_stmt(call(dot(ident("seen"), "push"), vec![ident("key")]));
    let body = block(vec![
        json!({"type": "IfStatement",
            "test": binary("===", ident("key"), lit(json!("b"))),
            "consequent": json!({"type": "ContinueStatement"}),
            "alternate": null}),
        json!({"type": "IfStatement",
            "test": binary("===", ident("key"), lit(json!("d"))),
            "consequent": json!({"type": "BreakStatement"}),
            "alternate": null}),
        seen_push,
    ]);
    let code = prog(vec![
        var_decl("const", "seen", Some(arr(vec![]))),
        json!({"type": "ForInStatement",
            "left": {"type": "VariableDeclaration", "kind": "const",
                "declarations": [{"type": "VariableDeclarator", "id": ident("key"), "init": null}]},
            "right": obj(vec![
                ("a".to_string(), lit(json!(1))),
                ("b".to_string(), lit(json!(2))),
                ("c".to_string(), lit(json!(3))),
                ("d".to_string(), lit(json!(4))),
            ]),
            "body": body}),
        ret(Some(ident("seen"))),
    ]);
    assert_eq!(run_value(code), json!(["a", "c"]));
}

#[test]
fn for_in_supports_let_and_bare_identifiers() {
    let loop1 = json!({"type": "ForInStatement",
        "left": {"type": "VariableDeclaration", "kind": "let",
            "declarations": [{"type": "VariableDeclarator", "id": ident("key"), "init": null}]},
        "right": obj(vec![("a".to_string(), lit(json!(1))), ("b".to_string(), lit(json!(2)))]),
        "body": expr_stmt(json!({"type": "AssignmentExpression", "operator": "=",
            "left": ident("last"), "right": ident("key")}))});
    let code = prog(vec![
        var_decl("let", "last", Some(lit(json!("")))),
        loop1,
        ret(Some(ident("last"))),
    ]);
    assert_eq!(run_value(code), json!("b"));
}

#[test]
fn for_in_rejects_unsupported_values_with_hint() {
    for expression in [
        lit(json!("text")),
        new_expr(
            "Map",
            vec![arr(vec![arr(vec![lit(json!(1)), lit(json!(2))])])],
        ),
        new_expr("Set", vec![arr(vec![lit(json!(1))])]),
        lit(json!(42)),
        lit(json!(null)),
    ] {
        let code = prog(vec![
            json!({"type": "ForInStatement",
                "left": {"type": "VariableDeclaration", "kind": "const",
                    "declarations": [{"type": "VariableDeclarator", "id": ident("key"), "init": null}]},
                "right": expression,
                "body": block(vec![])}),
            ret(Some(lit(json!("no")))),
        ]);
        let failure = run_error(code);
        assert!(
            failure
                .message
                .contains("for...in requires a plain object, array, or tools reference"),
            "unexpected: {}",
            failure.message
        );
        assert!(
            failure
                .message
                .contains("Use for...of for arrays/strings/Maps/Sets, or Object.keys(value)"),
            "unexpected: {}",
            failure.message
        );
    }
}
