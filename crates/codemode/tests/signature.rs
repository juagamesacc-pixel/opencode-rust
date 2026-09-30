//! Port of `test/signature.test.ts` (TypeScript signature rendering:
//! pretty/JSDoc, quoted keys, unions, catalog/search JSDoc, bracket paths).

use codemode::tool::{JsonSchema, SchemaType, ToolOptions, ToolTree};
use codemode::tool_schema::{
    input_typescript, is_identifier_segment, json_schema_to_typescript, output_typescript,
};
use serde_json::json;

fn json_tool(
    description: &str,
    input: serde_json::Value,
    output: Option<serde_json::Value>,
) -> codemode::tool::Definition {
    let input_schema: JsonSchema = serde_json::from_value(input).unwrap();
    let output_schema: Option<JsonSchema> = output.map(|v| serde_json::from_value(v).unwrap());
    codemode::tool::make_tool(ToolOptions {
        description: description.to_string(),
        input: SchemaType::Json(input_schema),
        output: output_schema.map(SchemaType::Json),
        run: Box::new(|input| Ok(input.clone())),
    })
}

#[test]
fn identifier_segments_match_ts_regex() {
    assert!(is_identifier_segment("foo"));
    assert!(is_identifier_segment("$tool_1"));
    assert!(!is_identifier_segment("tool-name"));
    assert!(!is_identifier_segment("0abc"));
}

#[test]
fn compact_rendering_quotes_non_identifier_keys() {
    let rendered = json_schema_to_typescript(
        &serde_json::from_value(json!({
            "type": "object",
            "properties": {
                "ok": {"type": "string"},
                "tool-name": {"type": "string"},
            },
            "required": ["ok", "tool-name"],
        }))
        .unwrap(),
        false,
    );
    assert!(rendered.contains("ok: string"), "unexpected: {}", rendered);
    assert!(
        rendered.contains("\"tool-name\": string"),
        "unexpected: {}",
        rendered
    );
}

#[test]
fn pretty_rendering_adds_jsdoc_with_tags() {
    let rendered = json_schema_to_typescript(
        &serde_json::from_value(json!({
            "type": "object",
            "properties": {
                "query": {"type": "string", "description": "Search text", "default": "x"},
            },
            "required": ["query"],
        }))
        .unwrap(),
        true,
    );
    // FLAG: test-fix — source jsdoc() emits multi-line for description + tags (tool-schema.ts:95-104: lines.length>1 → "/**\n * ..."), so "/** Search text" single-line never occurs; the description + @default pair renders as "/**\n * Search text\n * @default \"x\"\n */" (cf. signature.test.ts:46-49). Assert the content, not the single-line layout.
    assert!(rendered.contains("Search text"), "unexpected: {}", rendered);
    assert!(
        rendered.contains("@default \"x\""),
        "unexpected: {}",
        rendered
    );
}

#[test]
fn union_schemas_render_every_alternative() {
    let rendered = json_schema_to_typescript(
        &serde_json::from_value(json!({
            "anyOf": [{"type": "string"}, {"type": "number"}],
        }))
        .unwrap(),
        false,
    );
    assert_eq!(rendered, "string | number");
}

#[test]
fn effect_number_artifact_collapses_to_number() {
    // Effect's number emission: anyOf number + NaN/Infinity consts.
    let rendered = json_schema_to_typescript(
        &serde_json::from_value(json!({
            "anyOf": [
                {"type": "number"},
                {"type": "string", "enum": ["NaN"]},
                {"type": "string", "enum": ["Infinity"]},
                {"type": "string", "enum": ["-Infinity"]},
            ],
        }))
        .unwrap(),
        false,
    );
    assert_eq!(rendered, "number");
}

#[test]
fn unresolved_refs_render_unknown() {
    let rendered = json_schema_to_typescript(
        &serde_json::from_value(json!({"$ref": "#/$defs/Missing"})).unwrap(),
        false,
    );
    assert_eq!(rendered, "unknown");
    // Cyclic refs degrade rather than recurse forever.
    let rendered = json_schema_to_typescript(
        &serde_json::from_value(json!({
            "$ref": "#/$defs/A",
            "$defs": {"A": {"$ref": "#/$defs/A"}},
        }))
        .unwrap(),
        false,
    );
    assert_eq!(rendered, "unknown");
}

#[test]
fn jsdoc_neutralizes_comment_close() {
    let rendered = json_schema_to_typescript(
        &serde_json::from_value(json!({
            "type": "object",
            "properties": {"a": {"type": "string", "description": "ends */ early"}},
        }))
        .unwrap(),
        true,
    );
    assert!(!rendered.contains("*/ early"), "unexpected: {}", rendered);
    assert!(rendered.contains("* / early"), "unexpected: {}", rendered);
}

#[test]
fn missing_output_advertises_unknown() {
    let tool = json_tool("Do it", json!({"type": "object"}), None);
    assert_eq!(output_typescript(&tool, false), "unknown");
}

#[test]
fn input_signature_uses_compact_single_line_by_default() {
    let tool = json_tool(
        "Look up",
        json!({
            "type": "object",
            "properties": {"id": {"type": "string"}},
            "required": ["id"],
        }),
        Some(json!({"type": "string"})),
    );
    assert_eq!(input_typescript(&tool, false), "{ id: string }");
    assert_eq!(output_typescript(&tool, false), "string");
}

#[test]
fn bracket_notation_for_dashed_tool_names() {
    let mut tree = ToolTree::new();
    let tool = json_tool("Dashed", json!({"type": "object"}), None);
    let mut sub = ToolTree::new();
    sub.insert(
        "tool-name".to_string(),
        codemode::tool::ToolTreeValue::Definition(Box::new(tool)),
    );
    tree.insert(
        "ns".to_string(),
        codemode::tool::ToolTreeValue::Namespace(sub),
    );
    let catalog = codemode::tool_runtime::catalog(&tree);
    assert_eq!(catalog.len(), 1);
    assert!(
        catalog[0]
            .signature
            .starts_with("tools.ns[\"tool-name\"](input: "),
        "unexpected: {}",
        catalog[0].signature
    );
}

#[test]
fn search_returns_callable_bracket_paths() {
    let mut tree = ToolTree::new();
    let tool = json_tool("Dashed", json!({"type": "object"}), None);
    let mut sub = ToolTree::new();
    sub.insert(
        "tool-name".to_string(),
        codemode::tool::ToolTreeValue::Definition(Box::new(tool)),
    );
    tree.insert(
        "ns".to_string(),
        codemode::tool::ToolTreeValue::Namespace(sub),
    );
    let index = codemode::tool_runtime::search_index(&tree);
    let output = codemode::tool_runtime::run_search(
        &index,
        &codemode::tool_runtime::SearchInput {
            query: Some("dashed".to_string()),
            namespace: None,
            limit: None,
            offset: None,
        },
    )
    .unwrap();
    assert_eq!(output.items.len(), 1);
    assert_eq!(output.items[0].path, "tools.ns[\"tool-name\"]");
}
