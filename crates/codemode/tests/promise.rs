//! Port of `test/promise.test.ts` (first-class promises, boundaries,
//! combinators, race, resolve/reject, unsupported surface).

mod common;

use codemode::tool::ToolTree;
use common::*;
use serde_json::json;

fn ns_tools() -> ToolTree {
    let mut tree = ToolTree::new();
    insert_tool(&mut tree, &["ns", "tool"], echo_tool("Echo"));
    insert_tool(&mut tree, &["ns", "fail"], refusing_tool("Fail", "Refused"));
    tree
}

fn tool_call(path: &[&str], input: serde_json::Value) -> serde_json::Value {
    let mut callee = ident("tools");
    for segment in path {
        callee = dot(callee, segment);
    }
    await_(call(callee, vec![input]))
}

#[test]
fn eager_start_before_await_and_single_settlement() {
    // const p = tools.ns.tool({value:"a"}); const first = await p; const second = await p; return [first, second]
    let call = call(
        dot(dot(ident("tools"), "ns"), "tool"),
        vec![obj(vec![("value".to_string(), lit(json!("a")))])],
    );
    let code = prog(vec![
        var_decl("const", "p", Some(call)),
        var_decl("const", "first", Some(await_(ident("p")))),
        var_decl("const", "second", Some(await_(ident("p")))),
        ret(Some(arr(vec![ident("first"), ident("second")]))),
    ]);
    assert_eq!(run_value(code), json!(["a", "a"]));
}

#[test]
fn await_of_non_promise_is_passthrough() {
    let code = prog(vec![ret(Some(await_(lit(json!(7)))))]);
    assert_eq!(run_value(code), json!(7));
}

#[test]
fn returning_tool_call_resolves_it() {
    let code = prog(vec![ret(Some(tool_call(
        &["ns", "tool"],
        obj(vec![("value".to_string(), lit(json!("v")))]),
    )))]);
    assert_eq!(run_value(code), json!("v"));
}

#[test]
fn typeof_promise_is_object() {
    let code = prog(vec![
        var_decl(
            "const",
            "p",
            Some(call(
                dot(dot(ident("tools"), "ns"), "tool"),
                vec![obj(vec![("value".to_string(), lit(json!("x")))])],
            )),
        ),
        ret(Some(
            json!({"type": "UnaryExpression", "operator": "typeof", "argument": ident("p"), "prefix": true}),
        )),
    ]);
    assert_eq!(run_value(code), json!("object"));
}

#[test]
fn awaited_failure_is_catchable() {
    // try { await tools.ns.fail({value:"x"}) } catch (e) { return e.message }
    let code = prog(vec![json!({"type": "TryStatement",
        "block": block(vec![expr_stmt(tool_call(&["ns", "fail"], obj(vec![("value".to_string(), lit(json!("x")))])))]),
        "handler": {"type": "CatchClause", "param": ident("e"),
            "body": block(vec![ret(Some(dot(ident("e"), "message")))])},
        "finalizer": null})]);
    assert_eq!(run_value(code), json!("Refused"));
}

#[test]
fn fire_and_forget_completes_before_end() {
    use std::sync::{Arc, Mutex};
    let calls: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    let seen = calls.clone();
    let mut tree = ToolTree::new();
    insert_tool(
        &mut tree,
        &["ns", "tool"],
        codemode::tool::make_tool(codemode::tool::ToolOptions {
            description: "Echo".to_string(),
            input: codemode::tool::SchemaType::Json(json_schema_doc(json!({"type": "object"}))),
            output: None,
            run: Box::new(move |input| {
                seen.lock().unwrap().push("ran".to_string());
                Ok(input.clone())
            }),
        }),
    );
    // tools.ns.tool({}); return "done" — the abandoned call still runs.
    let code = prog(vec![
        expr_stmt(call(
            dot(dot(ident("tools"), "ns"), "tool"),
            vec![obj(vec![])],
        )),
        ret(Some(lit(json!("done")))),
    ]);
    assert_eq!(value(run(code, tree)), json!("done"));
    assert_eq!(*calls.lock().unwrap(), vec!["ran".to_string()]);
}

#[test]
fn unawaited_failing_call_is_unhandled_rejection() {
    // tools.ns.fail({value:"x"}); return "no"
    let code = prog(vec![
        expr_stmt(call(
            dot(dot(ident("tools"), "ns"), "fail"),
            vec![obj(vec![("value".to_string(), lit(json!("x")))])],
        )),
        ret(Some(lit(json!("no")))),
    ]);
    let failure = run_error(code);
    assert!(failure
        .message
        .starts_with("Unhandled rejection from an un-awaited tool call: "));
    assert!(failure.message.contains("Refused"));
}

#[test]
fn promise_all_mixes_values_preserving_order() {
    // return await Promise.all([tools.ns.tool({value:"a"}), "b", 3])
    let all = call(
        dot(ident("Promise"), "all"),
        vec![arr(vec![
            tool_call(
                &["ns", "tool"],
                obj(vec![("value".to_string(), lit(json!("a")))]),
            ),
            lit(json!("b")),
            lit(json!(3)),
        ])],
    );
    assert_eq!(
        run_value(prog(vec![ret(Some(await_(all)))])),
        json!(["a", "b", 3])
    );
}

#[test]
fn promise_all_rejects_with_first_failure() {
    let all = call(
        dot(ident("Promise"), "all"),
        vec![arr(vec![
            tool_call(
                &["ns", "tool"],
                obj(vec![("value".to_string(), lit(json!("a")))]),
            ),
            tool_call(
                &["ns", "fail"],
                obj(vec![("value".to_string(), lit(json!("x")))]),
            ),
        ])],
    );
    let code = prog(vec![json!({"type": "TryStatement",
        "block": block(vec![expr_stmt(await_(all)), ret(Some(lit(json!("no"))))]),
        "handler": {"type": "CatchClause", "param": ident("e"),
            "body": block(vec![ret(Some(dot(ident("e"), "message")))])},
        "finalizer": null})]);
    assert_eq!(run_value(code), json!("Refused"));
}

#[test]
fn promise_all_rejects_non_collections() {
    let code = prog(vec![ret(Some(await_(call(
        dot(ident("Promise"), "all"),
        vec![lit(json!(42))],
    ))))]);
    let failure = run_error(code);
    assert!(failure.message.contains(
        "Promise.all expects an array of promises or plain values (e.g. Promise.all(items.map((item) => tools.ns.tool(item))))."
    ));
}

#[test]
fn promise_allsettled_reports_outcomes() {
    let all = call(
        dot(ident("Promise"), "allSettled"),
        vec![arr(vec![
            // Pending (un-awaited) promises per source promise.test.ts:286-291:
            // an inner await would throw during array construction.
            call(
                dot(dot(ident("tools"), "ns"), "tool"),
                vec![obj(vec![("value".to_string(), lit(json!("a")))])],
            ),
            call(
                dot(dot(ident("tools"), "ns"), "fail"),
                vec![obj(vec![("value".to_string(), lit(json!("x")))])],
            ),
        ])],
    );
    let result = run_value(prog(vec![ret(Some(await_(all)))]));
    assert_eq!(
        result,
        json!([
            {"status": "fulfilled", "value": "a"},
            {"status": "rejected", "reason": {"name": "Error", "message": "Refused"}},
        ])
    );
}

#[test]
fn promise_race_first_wins_and_losers_interrupt() {
    // const slow = tools.ns.tool({value:"slow"}); const fast = "now";
    // const winner = await Promise.race([slow, fast]); return winner
    let code = prog(vec![
        var_decl(
            "const",
            "slow",
            Some(call(
                dot(dot(ident("tools"), "ns"), "tool"),
                vec![obj(vec![("value".to_string(), lit(json!("slow")))])],
            )),
        ),
        var_decl(
            "const",
            "winner",
            Some(await_(call(
                dot(ident("Promise"), "race"),
                vec![arr(vec![ident("slow"), lit(json!("now"))])],
            ))),
        ),
        ret(Some(ident("winner"))),
    ]);
    // Sync port: every admitted call is settled, so the first item wins.
    assert_eq!(run_value(code), json!("slow"));
}

#[test]
fn promise_race_empty_is_clear_error() {
    let code = prog(vec![ret(Some(await_(call(
        dot(ident("Promise"), "race"),
        vec![arr(vec![])],
    ))))]);
    let failure = run_error(code);
    assert_eq!(
        failure.message,
        "Promise.race([]) would never settle; provide at least one promise or value."
    );
}

#[test]
fn promise_resolve_reject_round_trip() {
    // await Promise.resolve(5) === 5; try { await Promise.reject("nope") } catch (e) { return e }
    let code = prog(vec![ret(Some(await_(call(
        dot(ident("Promise"), "resolve"),
        vec![lit(json!(5))],
    ))))]);
    assert_eq!(run_value(code), json!(5));
    let code = prog(vec![json!({"type": "TryStatement",
        "block": block(vec![expr_stmt(await_(call(
            dot(ident("Promise"), "reject"),
            vec![lit(json!("nope"))],
        )))]),
        "handler": {"type": "CatchClause", "param": ident("e"),
            "body": block(vec![ret(Some(ident("e")))])},
        "finalizer": null})]);
    assert_eq!(run_value(code), json!("nope"));
}

#[test]
fn then_catch_finally_hint_at_await() {
    for method in ["then", "catch", "finally"] {
        let code = prog(vec![
            var_decl(
                "const",
                "p",
                Some(call(
                    dot(dot(ident("tools"), "ns"), "tool"),
                    vec![obj(vec![("value".to_string(), lit(json!("x")))])],
                )),
            ),
            ret(Some(dot(ident("p"), method))),
        ]);
        let failure = run_error(code);
        assert!(failure.message.contains(&format!(
            "Promise.prototype.{} is not supported in CodeMode; use await instead",
            method
        )));
    }
}

#[test]
fn unknown_promise_statics_list_available() {
    let code = prog(vec![ret(Some(dot(ident("Promise"), "any")))]);
    let failure = run_error(code);
    assert!(failure.message.contains(
        "Promise.any is not available in CodeMode. Available: Promise.all, Promise.allSettled, Promise.race, Promise.resolve, and Promise.reject; consume promises with await."
    ));
}

#[test]
fn new_promise_points_at_tool_calls() {
    let code = prog(vec![ret(Some(json!({
        "type": "NewExpression", "callee": ident("Promise"), "arguments": []
    })))]);
    let failure = run_error(code);
    assert!(failure
        .message
        .contains("new Promise(...) is not supported"));
}

#[test]
fn tool_call_concurrency_constant_is_eight() {
    assert_eq!(codemode::stdlib_promise::TOOL_CALL_CONCURRENCY, 8);
}

fn run_value(code: String) -> serde_json::Value {
    value(run(code, ns_tools()))
}

fn run_error(code: String) -> codemode::codemode::Diagnostic {
    error(run(code, ns_tools()))
}
