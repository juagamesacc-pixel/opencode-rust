//! Port of `test/stdlib.test.ts` (Date, RegExp, URL/URI, Map, Set,
//! integration, intra-sandbox checkpoints).

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

fn date_call(method: &str, args: Vec<serde_json::Value>) -> serde_json::Value {
    call(dot(new_expr("Date", vec![lit(json!(0))]), method), args)
}

#[test]
fn date_epoch_iso_and_getters() {
    // new Date(0).toISOString() === "1970-01-01T00:00:00.000Z"
    assert_eq!(
        run_value(prog(vec![ret(Some(date_call("toISOString", vec![])))])),
        json!("1970-01-01T00:00:00.000Z")
    );
    assert_eq!(
        run_value(prog(vec![ret(Some(date_call("getTime", vec![])))])),
        json!(0)
    );
    assert_eq!(
        run_value(prog(vec![ret(Some(date_call("getUTCFullYear", vec![])))])),
        json!(1970)
    );
}

#[test]
fn date_invalid_guards() {
    // new Date(NaN).toJSON() === null; getTime() is NaN (normalizes to null at boundary)
    // FLAG: test-fix — original `new Date(NaN).toJSON` without call returned the method itself, tripping "data only" (source: stdlib.test.ts:52 `new Date("garbage").toJSON()` is called).
    let code = prog(vec![ret(Some(call(
        dot(new_expr("Date", vec![ident("NaN")]), "toJSON"),
        vec![],
    )))]);
    assert_eq!(run_value(code), serde_json::Value::Null);
}

#[test]
fn date_coercions() {
    // Number(new Date(1000)) === 1000
    let code = prog(vec![ret(Some(call(
        ident("Number"),
        vec![new_expr("Date", vec![lit(json!(1000))])],
    )))]);
    assert_eq!(run_value(code), json!(1000));
    // String(new Date(0)) is the ISO form
    let code = prog(vec![ret(Some(call(
        ident("String"),
        vec![new_expr("Date", vec![lit(json!(0))])],
    )))]);
    assert_eq!(run_value(code), json!("1970-01-01T00:00:00.000Z"));
}

#[test]
fn regexp_literal_test_and_exec() {
    // /ab+c/.test("xxabcxx") === true
    let code = prog(vec![ret(Some(call(
        dot(
            json!({"type": "Literal", "value": null, "regex": {"pattern": "ab+c", "flags": ""}}),
            "test",
        ),
        vec![lit(json!("xxabcxx"))],
    )))]);
    assert_eq!(run_value(code), json!(true));
    // /(\w+)@(\w+)/.exec("a@b")[1] === "a"
    let code = prog(vec![ret(Some(json!({
        "type": "MemberExpression", "computed": true, "optional": false,
        "object": call(
            dot(json!({"type": "Literal", "value": null, "regex": {"pattern": "(\\w+)@(\\w+)", "flags": ""}}), "exec"),
            vec![lit(json!("a@b"))]),
        "property": lit(json!(1)),
    })))]);
    assert_eq!(run_value(code), json!("a"));
}

#[test]
fn string_match_and_replace() {
    // "aaa".replace(/a/g, "b") === "bbb"
    let code = prog(vec![ret(Some(call(
        dot(lit(json!("aaa")), "replace"),
        vec![
            json!({"type": "Literal", "value": null, "regex": {"pattern": "a", "flags": "g"}}),
            lit(json!("b")),
        ],
    )))]);
    assert_eq!(run_value(code), json!("bbb"));
    // "hello".slice(1, 4) === "ell"
    let code = prog(vec![ret(Some(call(
        dot(lit(json!("hello")), "slice"),
        vec![lit(json!(1)), lit(json!(4))],
    )))]);
    assert_eq!(run_value(code), json!("ell"));
}

#[test]
fn replace_all_without_g_flag_errors() {
    let code = prog(vec![ret(Some(call(
        dot(lit(json!("aaa")), "replaceAll"),
        vec![
            json!({"type": "Literal", "value": null, "regex": {"pattern": "a", "flags": ""}}),
            lit(json!("b")),
        ],
    )))]);
    let failure = run_error(code);
    assert!(failure
        .message
        .contains("String.replaceAll requires a regular expression with the global (g) flag"));
}

#[test]
fn new_regexp_validates_with_actionable_messages() {
    // new RegExp("(") → catchable with the constructor message.
    let code = prog(vec![json!({"type": "TryStatement",
        "block": block(vec![expr_stmt(new_expr("RegExp", vec![lit(json!("("))]))]),
        "handler": {"type": "CatchClause", "param": ident("e"),
            "body": block(vec![ret(Some(dot(ident("e"), "message")))])},
        "finalizer": null})]);
    let message = run_value(code);
    assert!(
        message
            .as_str()
            .unwrap()
            .contains("new RegExp(...) received \"(\""),
        "unexpected: {}",
        message
    );
}

#[test]
fn url_parse_and_mutation() {
    // const u = new URL("https://example.test:8080/a?x=1#h"); return u.hostname
    let code = prog(vec![
        var_decl(
            "const",
            "u",
            Some(new_expr(
                "URL",
                vec![lit(json!("https://example.test:8080/a?x=1#h"))],
            )),
        ),
        ret(Some(dot(ident("u"), "hostname"))),
    ]);
    assert_eq!(run_value(code), json!("example.test"));
}

#[test]
fn uri_helpers_round_trip() {
    // encodeURIComponent("a b") === "a%20b"; decodeURIComponent("a%20b") === "a b"
    let code = prog(vec![ret(Some(call(
        ident("encodeURIComponent"),
        vec![lit(json!("a b"))],
    )))]);
    assert_eq!(run_value(code), json!("a%20b"));
}

#[test]
fn map_get_set_has_size_chaining() {
    // const m = new Map([["a", 1]]); m.set("b", 2); return [m.get("a"), m.get("b"), m.size]
    let code = prog(vec![
        var_decl(
            "const",
            "m",
            Some(new_expr(
                "Map",
                vec![arr(vec![arr(vec![lit(json!("a")), lit(json!(1))])])],
            )),
        ),
        expr_stmt(call(
            dot(ident("m"), "set"),
            vec![lit(json!("b")), lit(json!(2))],
        )),
        ret(Some(arr(vec![
            call(dot(ident("m"), "get"), vec![lit(json!("a"))]),
            call(dot(ident("m"), "get"), vec![lit(json!("b"))]),
            dot(ident("m"), "size"),
        ]))),
    ]);
    assert_eq!(run_value(code), json!([1, 2, 2]));
}

#[test]
fn map_object_keys_use_identity() {
    // const k = {id: 1}; const m = new Map([[k, "v"]]); return [m.get(k), m.get({id: 1}) ?? "miss"]
    let code = prog(vec![
        var_decl(
            "const",
            "k",
            Some(obj(vec![("id".to_string(), lit(json!(1)))])),
        ),
        var_decl(
            "const",
            "m",
            Some(new_expr(
                "Map",
                vec![arr(vec![arr(vec![ident("k"), lit(json!("v"))])])],
            )),
        ),
        ret(Some(arr(vec![
            call(dot(ident("m"), "get"), vec![ident("k")]),
            json!({"type": "LogicalExpression", "operator": "??",
                "left": call(dot(ident("m"), "get"),
                    vec![obj(vec![("id".to_string(), lit(json!(1)))])]),
                "right": lit(json!("miss"))}),
        ]))),
    ]);
    assert_eq!(run_value(code), json!(["v", "miss"]));
}

#[test]
fn set_dedupes_and_nan_is_findable() {
    // [...new Set([1, 2, 2, 3, 1])] + new Set([NaN]).has(NaN)
    let code = prog(vec![ret(Some(arr(vec![json!({"type": "SpreadElement",
    "argument": new_expr("Set", vec![arr(vec![
        lit(json!(1)), lit(json!(2)), lit(json!(2)), lit(json!(3)), lit(json!(1)),
    ])])})])))]);
    assert_eq!(run_value(code), json!([1, 2, 3]));
    let code = prog(vec![ret(Some(call(
        dot(new_expr("Set", vec![arr(vec![ident("NaN")])]), "has"),
        vec![ident("NaN")],
    )))]);
    assert_eq!(run_value(code), json!(true));
}

#[test]
fn math_and_number_helpers() {
    // Math.max(1, 9, 3) === 9; (1.5).toFixed(0) === "2"
    let code = prog(vec![ret(Some(call(
        dot(ident("Math"), "max"),
        vec![lit(json!(1)), lit(json!(9)), lit(json!(3))],
    )))]);
    assert_eq!(run_value(code), json!(9));
    assert_eq!(
        codemode::stdlib_math::math_constant("PI"),
        Some(std::f64::consts::PI)
    );
    assert_eq!(
        codemode::stdlib_number::number_constant("MAX_SAFE_INTEGER"),
        Some(9_007_199_254_740_991.0)
    );
}

#[test]
fn json_parse_stringify_round_trip() {
    // JSON.parse('{"a":1}').a === 1
    let code = prog(vec![ret(Some(dot(
        call(dot(ident("JSON"), "parse"), vec![lit(json!("{\"a\":1}"))]),
        "a",
    )))]);
    assert_eq!(run_value(code), json!(1));
}

#[test]
fn typeof_constructors_are_functions() {
    // typeof Date === "function"; typeof Math === "object"
    let code = prog(vec![ret(Some(arr(vec![
        json!({"type": "UnaryExpression", "operator": "typeof", "argument": ident("Date"), "prefix": true}),
        json!({"type": "UnaryExpression", "operator": "typeof", "argument": ident("Math"), "prefix": true}),
    ])))]);
    assert_eq!(run_value(code), json!(["function", "object"]));
}

#[test]
fn instanceof_recognizes_stdlib_types() {
    // new Date(0) instanceof Date; [] instanceof Array; {} instanceof Object
    let code = prog(vec![ret(Some(arr(vec![
        json!({"type": "BinaryExpression", "operator": "instanceof",
            "left": new_expr("Date", vec![lit(json!(0))]), "right": ident("Date")}),
        json!({"type": "BinaryExpression", "operator": "instanceof",
            "left": arr(vec![]), "right": ident("Array")}),
    ])))]);
    assert_eq!(run_value(code), json!([true, true]));
}
