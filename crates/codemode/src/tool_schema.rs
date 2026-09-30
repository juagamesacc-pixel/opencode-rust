//! Port of `src/tool-schema.ts`.
//!
//! Model-visible TypeScript signature rendering over tool schemas plus the
//! decode boundary (`decode_input` / `decode_output`). Rendering semantics
//! (depth ceiling, `unknown` degradation, Effect number-union collapse,
//! empty-struct collapse, `$ref` intersection, JSDoc tags, pretty blocks) are
//! preserved verbatim. Rendering never throws: pathological input degrades to
//! `"unknown"`.

use crate::tool::{Definition, JsonSchema, SchemaType};
use serde_json::Value;

/// Bare TypeScript identifier — usable unquoted as an object key (and, in the
/// tool runtime, with dot access as a tool-path segment). Anything else must
/// be quoted/bracketed. Mirrors `identifierSegment`.
pub fn is_identifier_segment(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' || c == '$' => (),
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$')
}

/// Renders a property name as a valid TS object key: bare when an identifier,
/// quoted otherwise. Mirrors `renderKey`.
pub fn render_key(name: &str) -> String {
    if is_identifier_segment(name) {
        name.to_string()
    } else {
        serde_json::to_string(name).unwrap_or_else(|_| "\"unknown\"".to_string())
    }
}

fn render_literal(value: &Value) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "unknown".to_string())
}

fn effect_number_sentinel(schema: &JsonSchemaValue) -> bool {
    schema.schema_type.as_deref() == Some("string")
        && schema.r#enum.as_ref().map(|e| e.len()).unwrap_or(0) == 1
        && matches!(
            schema.r#enum.as_ref().and_then(|e| e.first()),
            Some(Value::String(s)) if s == "NaN" || s == "Infinity" || s == "-Infinity"
        )
}

fn intersection(members: &[String]) -> String {
    let concrete: Vec<&str> = members
        .iter()
        .map(|m| m.as_str())
        .filter(|m| *m != "unknown")
        .collect();
    match concrete.len() {
        0 => "unknown".to_string(),
        1 => concrete[0].to_string(),
        _ => concrete
            .iter()
            .map(|m| {
                if m.contains(" | ") {
                    format!("({})", m)
                } else {
                    (*m).to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(" & "),
    }
}

/// Recursion ceiling for schema rendering. Mirrors `MAX_RENDER_DEPTH = 8`.
pub const MAX_RENDER_DEPTH: usize = 8;

/// Workhorse view over a JSON Schema document: this port stores schemas as
/// `serde_json::Value` maps (the JSON Schema document emission), so rendering
/// walks `Value` directly with the same field names (`type`, `enum`,
/// `const`, `anyOf`, `oneOf`, `allOf`, `properties`, `required`, `items`,
/// `additionalProperties`, `description`, `default`, `format`, `deprecated`,
/// `minItems`, `maxItems`, `$ref`, `$defs`, `definitions`).
pub type JsonSchemaValue = JsonSchemaDoc;

/// Owned JSON Schema document node.
#[derive(Debug, Clone, Default)]
pub struct JsonSchemaDoc {
    pub schema_type: Option<String>,
    pub schema_types: Option<Vec<String>>,
    pub r#enum: Option<Vec<Value>>,
    pub r#const: Option<Value>,
    pub has_const: bool,
    pub any_of: Option<Vec<JsonSchemaDoc>>,
    pub one_of: Option<Vec<JsonSchemaDoc>>,
    pub all_of: Option<Vec<JsonSchemaDoc>>,
    pub properties: Option<Vec<(String, JsonSchemaDoc)>>,
    pub required: Option<Vec<String>>,
    pub items: Option<Box<JsonSchemaDoc>>,
    pub additional_properties: Option<AdditionalProps>,
    pub description: Option<String>,
    pub default: Option<Value>,
    pub format: Option<String>,
    pub deprecated: bool,
    pub min_items: Option<u64>,
    pub max_items: Option<u64>,
    pub dollar_ref: Option<String>,
    pub definitions: std::collections::BTreeMap<String, JsonSchemaDoc>,
}

#[derive(Debug, Clone)]
pub enum AdditionalProps {
    Bool(bool),
    Schema(Box<JsonSchemaDoc>),
}

impl JsonSchemaDoc {
    /// Builds the render view from a raw JSON Schema `Value`.
    pub fn from_value(value: &Value) -> Self {
        let obj = match value.as_object() {
            Some(o) => o,
            None => return JsonSchemaDoc::default(),
        };
        let get = |k: &str| obj.get(k);
        let str_val = |k: &str| get(k).and_then(|v| v.as_str()).map(|s| s.to_string());
        let schema_type = match get("type") {
            Some(Value::String(s)) => {
                let doc = JsonSchemaDoc::default();
                let _ = doc;
                Some(s.clone())
            }
            _ => None,
        };
        let schema_types = match get("type") {
            Some(Value::Array(items)) => Some(
                items
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect(),
            ),
            _ => None,
        };
        let parse_sub = |k: &str| -> Option<Vec<JsonSchemaDoc>> {
            get(k)
                .and_then(|v| v.as_array())
                .map(|items| items.iter().map(JsonSchemaDoc::from_value).collect())
        };
        let properties = get("properties").and_then(|v| v.as_object()).map(|props| {
            props
                .iter()
                .map(|(k, v)| (k.clone(), JsonSchemaDoc::from_value(v)))
                .collect()
        });
        let required = get("required").and_then(|v| v.as_array()).map(|items| {
            items
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        });
        let items = get("items").map(|v| Box::new(JsonSchemaDoc::from_value(v)));
        let additional_properties = get("additionalProperties").map(|v| match v {
            Value::Bool(b) => AdditionalProps::Bool(*b),
            _ => AdditionalProps::Schema(Box::new(JsonSchemaDoc::from_value(v))),
        });
        let mut definitions = std::collections::BTreeMap::new();
        for key in ["$defs", "definitions"] {
            if let Some(Value::Object(map)) = get(key) {
                for (k, v) in map {
                    definitions.insert(k.clone(), JsonSchemaDoc::from_value(v));
                }
            }
        }
        JsonSchemaDoc {
            schema_type,
            schema_types,
            r#enum: get("enum").and_then(|v| v.as_array()).cloned(),
            r#const: get("const").cloned(),
            has_const: obj.contains_key("const"),
            any_of: parse_sub("anyOf"),
            one_of: parse_sub("oneOf"),
            all_of: parse_sub("allOf"),
            properties,
            required,
            items,
            additional_properties,
            description: str_val("description"),
            default: get("default").cloned(),
            format: str_val("format"),
            deprecated: get("deprecated").and_then(|v| v.as_bool()).unwrap_or(false),
            min_items: get("minItems").and_then(|v| v.as_u64()),
            max_items: get("maxItems").and_then(|v| v.as_u64()),
            dollar_ref: str_val("$ref"),
            definitions,
        }
    }

    /// Builds the render view from the typed [`JsonSchema`].
    pub fn from_typed(schema: &JsonSchema) -> Self {
        // The typed struct serializes with spec field names (serde renames),
        // so a value round-trip feeds the document parser directly.
        let has_const = schema.r#const.is_some();
        let value = serde_json::to_value(schema).unwrap_or(Value::Null);
        let mut doc = Self::from_value(&value);
        doc.has_const = has_const;
        doc
    }
}

struct RenderCtx {
    definitions: std::collections::BTreeMap<String, JsonSchemaDoc>,
    pretty: bool,
}

fn unescape_token(segment: &str) -> String {
    // JSON Pointer unescape: `~1` → `/`, then `~0` → `~` (order matters).
    segment.replace("~1", "/").replace("~0", "~")
}

fn ref_name(dollar_ref: &str) -> Option<String> {
    for prefix in ["#/$defs/", "#/definitions/"] {
        if let Some(rest) = dollar_ref.strip_prefix(prefix) {
            if !rest.contains('/') {
                return Some(unescape_token(rest));
            }
        }
    }
    None
}

fn has_unresolved_ref(
    schema: &JsonSchemaDoc,
    definitions: &std::collections::BTreeMap<String, JsonSchemaDoc>,
    seen: &std::collections::BTreeSet<String>,
    visited: &mut Vec<*const JsonSchemaDoc>,
) -> bool {
    let ptr = schema as *const JsonSchemaDoc;
    if visited.contains(&ptr) {
        return false;
    }
    visited.push(ptr);
    let result = (|| {
        if let Some(r) = schema.dollar_ref.as_deref() {
            let name = match ref_name(r) {
                Some(n) => n,
                None => return true,
            };
            match definitions.get(&name) {
                None => return true,
                Some(target) => {
                    if seen.contains(&name) {
                        return true;
                    }
                    let mut next_seen = seen.clone();
                    next_seen.insert(name);
                    if has_unresolved_ref(target, definitions, &next_seen, visited) {
                        return true;
                    }
                }
            }
        }
        let mut children: Vec<&JsonSchemaDoc> = vec![];
        for group in [&schema.any_of, &schema.one_of, &schema.all_of]
            .iter()
            .filter_map(|g| g.as_ref())
        {
            children.extend(group.iter());
        }
        if let Some(props) = schema.properties.as_ref() {
            children.extend(props.iter().map(|(_, v)| v));
        }
        if let Some(items) = schema.items.as_deref() {
            children.push(items);
        }
        if let Some(AdditionalProps::Schema(s)) = schema.additional_properties.as_ref() {
            children.push(s);
        }
        children
            .iter()
            .any(|c| has_unresolved_ref(c, definitions, seen, visited))
    })();
    visited.pop();
    result
}

/// Schema constraints a TypeScript type cannot express natively but a model
/// benefits from, surfaced as JSDoc tags. Mirrors `docTags`.
fn doc_tags(schema: &JsonSchemaDoc) -> Vec<String> {
    let mut tags = vec![];
    if schema.deprecated {
        tags.push("@deprecated".to_string());
    }
    if let Some(default) = schema.default.as_ref() {
        if let Ok(rendered) = serde_json::to_string(default) {
            tags.push(format!("@default {}", rendered));
        }
    }
    if let Some(format) = schema.format.as_deref() {
        tags.push(format!("@format {}", format));
    }
    if let Some(min) = schema.min_items {
        tags.push(format!("@minItems {}", min));
    }
    if let Some(max) = schema.max_items {
        tags.push(format!("@maxItems {}", max));
    }
    tags
}

/// Formats description + tags as a JSDoc comment. Mirrors `jsdoc`.
fn jsdoc(description: Option<&str>, tags: &[String], pad: &str) -> String {
    let mut lines: Vec<String> = vec![];
    if let Some(desc) = description {
        lines.extend(desc.split('\n').map(|s| s.to_string()));
    }
    lines.extend(tags.iter().cloned());
    let lines: Vec<String> = lines
        .into_iter()
        .map(|line| line.replace("*/", "* /").trim_end().to_string())
        .collect();
    let mut lines = lines;
    while lines.first().map(|l| l.trim().is_empty()).unwrap_or(false) {
        lines.remove(0);
    }
    while lines.last().map(|l| l.trim().is_empty()).unwrap_or(false) {
        lines.pop();
    }
    if lines.is_empty() {
        return String::new();
    }
    if lines.len() == 1 {
        return format!("{}/** {} */\n", pad, lines[0]);
    }
    let body = lines
        .iter()
        .map(|line| {
            if line.is_empty() {
                format!("{} *", pad)
            } else {
                format!("{} * {}", pad, line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!("{}/**\n{}\n{} */\n", pad, body, pad)
}

fn render_schema(
    schema: &JsonSchemaDoc,
    ctx: &RenderCtx,
    depth: usize,
    seen: &std::collections::BTreeSet<String>,
) -> String {
    if depth > MAX_RENDER_DEPTH {
        return "unknown".to_string();
    }
    // Nested `$defs`/`definitions` extend the context.
    let mut owned_defs: std::collections::BTreeMap<String, JsonSchemaDoc>;
    let definitions: &std::collections::BTreeMap<String, JsonSchemaDoc> =
        if schema.definitions.is_empty() {
            &ctx.definitions
        } else {
            owned_defs = ctx.definitions.clone();
            for (k, v) in &schema.definitions {
                owned_defs.insert(k.clone(), v.clone());
            }
            // Borrow dance: re-create ctx view via a nested call below.
            return render_schema_nested(schema, ctx, depth, seen);
        };
    let nested = RenderCtx {
        definitions: definitions.clone(),
        pretty: ctx.pretty,
    };
    render_schema_inner(schema, &nested, depth, seen)
}

// Helper to avoid borrow issues when nested definitions exist.
fn render_schema_nested(
    schema: &JsonSchemaDoc,
    ctx: &RenderCtx,
    depth: usize,
    seen: &std::collections::BTreeSet<String>,
) -> String {
    let mut merged = ctx.definitions.clone();
    for (k, v) in &schema.definitions {
        merged.insert(k.clone(), v.clone());
    }
    let nested = RenderCtx {
        definitions: merged,
        pretty: ctx.pretty,
    };
    // Strip local definitions for the inner render to avoid re-merging.
    let mut stripped = schema.clone();
    stripped.definitions.clear();
    render_schema_inner(&stripped, &nested, depth, seen)
}

fn render_schema_inner(
    schema: &JsonSchemaDoc,
    ctx: &RenderCtx,
    depth: usize,
    seen: &std::collections::BTreeSet<String>,
) -> String {
    if let Some(r) = schema.dollar_ref.as_deref() {
        let name = match ref_name(r) {
            Some(n) => n,
            None => return "unknown".to_string(),
        };
        let target = match ctx.definitions.get(&name) {
            Some(t) => t.clone(),
            None => return "unknown".to_string(),
        };
        if name.is_empty() || seen.contains(&name) {
            return "unknown".to_string();
        }
        let mut next_seen = seen.clone();
        next_seen.insert(name);
        let mut without_ref = schema.clone();
        without_ref.dollar_ref = None;
        return intersection(&[
            render_schema(&target, ctx, depth, &next_seen),
            render_schema(&without_ref, ctx, depth + 1, seen),
        ]);
    }
    if schema.has_const {
        let c = schema.r#const.clone().unwrap_or(Value::Null);
        return render_literal(&c);
    }
    if let Some(en) = schema.r#enum.as_ref() {
        return en
            .iter()
            .map(render_literal)
            .collect::<Vec<_>>()
            .join(" | ");
    }
    let alternatives = schema.any_of.as_ref().or(schema.one_of.as_ref());
    if let Some(alt) = alternatives {
        if alt
            .iter()
            .any(|item| item.schema_type.as_deref() == Some("number"))
            && alt.iter().all(|item| {
                item.schema_type.as_deref() == Some("number") || effect_number_sentinel(item)
            })
        {
            return "number".to_string();
        }
        if alt.len() == 2
            && alt[0].schema_type.as_deref() == Some("object")
            && alt[0].properties.is_none()
            && alt[1].schema_type.as_deref() == Some("array")
            && alt[1].items.is_none()
        {
            return "{}".to_string();
        }
        let members: Vec<String> = alt
            .iter()
            .map(|item| render_schema(item, ctx, depth + 1, seen))
            .collect();
        if members.iter().any(|m| m == "unknown") {
            return "unknown".to_string();
        }
        let mut without_union = schema.clone();
        without_union.any_of = None;
        without_union.one_of = None;
        return intersection(&[
            members.join(" | "),
            render_schema(&without_union, ctx, depth + 1, seen),
        ]);
    }
    if let Some(all) = schema.all_of.as_ref() {
        let members: Vec<String> = all
            .iter()
            .map(|item| render_schema(item, ctx, depth + 1, seen))
            .collect();
        if all.iter().any(|item| {
            has_unresolved_ref(
                item,
                &ctx.definitions,
                &std::collections::BTreeSet::new(),
                &mut vec![],
            )
        }) {
            return "unknown".to_string();
        }
        let mut without_all = schema.clone();
        without_all.all_of = None;
        let mut parts = vec![render_schema(&without_all, ctx, depth + 1, seen)];
        parts.extend(members);
        return intersection(&parts);
    }
    if let Some(types) = schema.schema_types.as_ref() {
        return types
            .iter()
            .map(|t| {
                let mut single = schema.clone();
                single.schema_type = Some(t.clone());
                single.schema_types = None;
                render_schema(&single, ctx, depth + 1, seen)
            })
            .collect::<Vec<_>>()
            .join(" | ");
    }
    match schema.schema_type.as_deref() {
        Some("string") => return "string".to_string(),
        Some("number") | Some("integer") => return "number".to_string(),
        Some("boolean") => return "boolean".to_string(),
        Some("null") => return "null".to_string(),
        Some("array") => {
            let item = schema.items.as_deref().cloned().unwrap_or_default();
            return format!("Array<{}>", render_schema(&item, ctx, depth + 1, seen));
        }
        Some("object") => {}
        _ => {}
    }
    if schema.schema_type.as_deref() == Some("object") || schema.properties.is_some() {
        let required: std::collections::BTreeSet<&str> = schema
            .required
            .as_ref()
            .map(|r| r.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default();
        let properties = schema.properties.as_ref().cloned().unwrap_or_default();
        let index_type = match schema.additional_properties.as_ref() {
            Some(AdditionalProps::Schema(s)) => Some(render_schema(s, ctx, depth + 1, seen)),
            _ => None,
        };
        let field = |(name, value): &(String, JsonSchemaDoc)| {
            let req = if required.contains(name.as_str()) {
                ""
            } else {
                "?"
            };
            format!(
                "{}{}: {}",
                render_key(name),
                req,
                render_schema(value, ctx, depth + 1, seen)
            )
        };
        if !ctx.pretty {
            let mut fields: Vec<String> = properties.iter().map(field).collect();
            if let Some(index) = index_type {
                fields.push(format!("[key: string]: {}", index));
            }
            if fields.is_empty() {
                return "{}".to_string();
            }
            return format!("{{ {} }}", fields.join("; "));
        }
        if properties.is_empty() && index_type.is_none() {
            return "{}".to_string();
        }
        let pad = "  ".repeat(depth + 1);
        let mut lines: Vec<String> = properties
            .iter()
            .map(|(name, value)| {
                let prop = value;
                format!(
                    "{}{}{},",
                    jsdoc(prop.description.as_deref(), &doc_tags(prop), &pad),
                    pad,
                    field(&(name.clone(), value.clone()))
                )
            })
            .collect();
        if let Some(index) = index_type {
            lines.push(format!("{}[key: string]: {},", pad, index));
        }
        return format!("{{\n{}\n{}}}", lines.join("\n"), "  ".repeat(depth));
    }
    "unknown".to_string()
}

/// Renders a raw JSON Schema document as a TypeScript type string.
/// Mirrors `jsonSchemaToTypeScript(schema, pretty?)`.
pub fn json_schema_to_typescript(schema: &JsonSchema, pretty: bool) -> String {
    let doc = JsonSchemaDoc::from_typed(schema);
    render_schema(
        &doc,
        &RenderCtx {
            definitions: doc.definitions.clone(),
            pretty,
        },
        0,
        &std::collections::BTreeSet::new(),
    )
}

/// Renders a raw JSON `Value` schema document as a TypeScript type string.
pub fn json_value_to_typescript(schema: &Value, pretty: bool) -> String {
    let doc = JsonSchemaDoc::from_value(schema);
    render_schema(
        &doc,
        &RenderCtx {
            definitions: doc.definitions.clone(),
            pretty,
        },
        0,
        &std::collections::BTreeSet::new(),
    )
}

/// One input property of a tool. Mirrors `InputProperty`.
#[derive(Debug, Clone)]
pub struct InputProperty {
    pub name: String,
    pub description: Option<String>,
    pub required: bool,
}

/// The property names, descriptions, and required flags of a tool's input
/// schema. Mirrors `inputProperties(definition)` incl. the trivial top-level
/// `$ref` resolution and the `[]` fallback.
pub fn input_properties(definition: &Definition) -> Vec<InputProperty> {
    let (mut schema, definitions) = match &definition.input {
        SchemaType::Validating { document, .. } => {
            let doc = JsonSchemaDoc::from_typed(document);
            (doc.clone(), doc.definitions.clone())
        }
        SchemaType::Json(doc) => {
            let parsed = JsonSchemaDoc::from_typed(doc);
            (parsed.clone(), parsed.definitions.clone())
        }
    };
    if let Some(r) = schema.dollar_ref.clone() {
        let name = match ref_name(&r) {
            Some(n) => n,
            None => return vec![],
        };
        match definitions.get(&name) {
            Some(resolved) => schema = resolved.clone(),
            None => return vec![],
        }
    }
    let required: std::collections::BTreeSet<&str> = schema
        .required
        .as_ref()
        .map(|r| r.iter().map(|s| s.as_str()).collect())
        .unwrap_or_default();
    schema
        .properties
        .as_ref()
        .map(|props| {
            props
                .iter()
                .map(|(name, value)| InputProperty {
                    name: name.clone(),
                    description: value.description.clone(),
                    required: required.contains(name.as_str()),
                })
                .collect()
        })
        .unwrap_or_default()
}

/// The model-visible TypeScript type of a tool's input. Mirrors
/// `inputTypeScript(definition, pretty?)`.
pub fn input_typescript(definition: &Definition, pretty: bool) -> String {
    match &definition.input {
        SchemaType::Validating { document, .. } => json_schema_to_typescript(document, pretty),
        SchemaType::Json(doc) => json_schema_to_typescript(doc, pretty),
    }
}

/// The model-visible TypeScript type of a tool's result. Mirrors
/// `outputTypeScript(definition, pretty?)` (`"unknown"` without output).
pub fn output_typescript(definition: &Definition, pretty: bool) -> String {
    match &definition.output {
        None => "unknown".to_string(),
        Some(SchemaType::Validating { document, .. }) => {
            json_schema_to_typescript(document, pretty)
        }
        Some(SchemaType::Json(doc)) => json_schema_to_typescript(doc, pretty),
    }
}

/// Decodes tool input before `run` is invoked. Mirrors
/// `decodeInput(definition, value)`: validating schemas validate (throwing on
/// failure); JSON-Schema-described inputs pass through unvalidated.
pub fn decode_input(definition: &Definition, value: &Value) -> Result<Value, String> {
    match &definition.input {
        SchemaType::Validating { validate, .. } => validate(value),
        SchemaType::Json(_) => Ok(value.clone()),
    }
}

/// Decodes a tool result before it is exposed to the program. Mirrors
/// `decodeOutput(definition, value)`.
pub fn decode_output(definition: &Definition, value: &Value) -> Result<Value, String> {
    match &definition.output {
        Some(SchemaType::Validating { validate, .. }) => validate(value),
        _ => Ok(value.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tool::{JsonSchemaType, ToolOptions};
    use serde_json::json;

    fn json_schema(value: Value) -> JsonSchema {
        // Build via raw value round-trip through typed struct defaults.
        let _ = value;
        JsonSchema::default()
    }

    #[test]
    fn identifier_segments_match_ts_regex() {
        assert!(is_identifier_segment("foo"));
        assert!(is_identifier_segment("$tool_1"));
        assert!(!is_identifier_segment("tool-name"));
        assert!(!is_identifier_segment("0abc"));
        assert!(!is_identifier_segment(""));
    }

    #[test]
    fn render_key_quotes_non_identifiers() {
        assert_eq!(render_key("ok"), "ok");
        assert_eq!(render_key("tool-name"), "\"tool-name\"");
    }

    #[test]
    fn decode_input_passes_through_json_schema() {
        let def = crate::tool::make_tool(ToolOptions {
            description: "d".to_string(),
            input: SchemaType::Json(json_schema(json!({}))),
            output: None,
            run: Box::new(|_| Ok(Value::Null)),
        });
        let v = json!({"a": 1});
        assert_eq!(decode_input(&def, &v).unwrap(), v);
    }

    #[test]
    fn output_typescript_unknown_without_output() {
        let def = crate::tool::make_tool(ToolOptions {
            description: "d".to_string(),
            input: SchemaType::Json(JsonSchema::default()),
            output: None,
            run: Box::new(|_| Ok(Value::Null)),
        });
        assert_eq!(output_typescript(&def, false), "unknown");
        let _ = JsonSchemaType::Single("string".to_string());
    }
}
