//! Port of `src/openapi/spec.ts`.
//!
//! Pure spec parsing/resolution: `methods`, `isRecord` (local record guard),
//! `nonEmptyString`, `own`, `resolve`, `componentDefinitions`,
//! `operationInput`, `inputSchema`, `operationOutput`, `operationPath`,
//! `specServerUrl`, `validateBaseUrl`, `securityRequirements`,
//! `operationSecurityRequirements`, `securitySchemes`.
//!
//! The Effect `JsonSchema.fromSchemaOpenApi3_0/3_1` projection
//! (`projectSchema`) is modelled with identical observable semantics: schema
//! values pass through as JSON Schema documents with `$defs` hoisting. Full
//! Effect-Schema round-trip fidelity is flagged (R2-adjacent); skip/parse
//! decisions and every verbatim reason string are preserved.

use crate::openapi_types::{
    ApiKeyLocation, Body, BodyMode, Document, InputField, InputLocation, OperationInput,
    ParamStyle, Parsed, SecurityRequirement, SecurityScheme,
};
use crate::tool::JsonSchema;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

/// Supported HTTP methods (lowercase). Mirrors `methods`.
pub fn is_method(method: &str) -> bool {
    matches!(
        method,
        "get" | "put" | "post" | "delete" | "options" | "head" | "patch" | "trace"
    )
}

/// All supported methods in source order.
pub const METHODS: &[&str] = &[
    "get", "put", "post", "delete", "options", "head", "patch", "trace",
];

#[allow(dead_code)]
const PARAMETER_LOCATIONS: &[&str] = &["path", "query", "header"];

fn is_ignored_header(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "accept" | "content-type" | "authorization"
    )
}

/// Local record guard. Mirrors the spec-local `isRecord` (excludes arrays).
pub fn is_record(value: &Value) -> bool {
    matches!(value, Value::Object(_))
}

fn as_array(value: Option<&Value>) -> Vec<&Value> {
    match value {
        Some(Value::Array(items)) => items.iter().collect(),
        _ => vec![],
    }
}

/// Mirrors `nonEmptyString(value)`.
pub fn non_empty_string(value: Option<&Value>) -> Option<String> {
    match value {
        Some(Value::String(s)) if !s.is_empty() => Some(s.clone()),
        _ => None,
    }
}

/// Prototype-safe record lookup. Mirrors `own(record, key)`.
pub fn own<'a>(record: &'a serde_json::Map<String, Value>, key: &str) -> Option<&'a Value> {
    record.get(key)
}

/// Follows local `#/...` JSON-Pointer `$ref`s with cycle protection.
/// Mirrors `resolve(document, value)` (verbatim `~1`→`/`, `~0`→`~` order).
pub fn resolve<'a>(document: &'a Value, value: &'a Value) -> &'a Value {
    fn next<'b>(document: &'b Value, current: &'b Value, seen: &mut BTreeSet<String>) -> &'b Value {
        let obj = match current.as_object() {
            Some(o) => o,
            None => return current,
        };
        let r = match obj.get("$ref").and_then(|v| v.as_str()) {
            Some(r) if !r.is_empty() => r,
            _ => return current,
        };
        if !r.starts_with("#/") || !seen.insert(r.to_string()) {
            return current;
        }
        let mut target: &Value = document;
        for segment in r[2..].split('/') {
            let key = segment.replace("~1", "/").replace("~0", "~");
            match target.as_object().and_then(|m| m.get(&key)) {
                Some(next) => target = next,
                None => return current,
            }
        }
        next(document, target, seen)
    }
    next(document, value, &mut BTreeSet::new())
}

/// Converts an OpenAPI schema value into a render-only [`JsonSchema`]
/// document, hoisting `$defs`. Mirrors `projectSchema` (identity projection
/// over the JSON Schema document; Effect-Schema round-trip is R2-adjacent).
pub fn project_schema(value: &Value) -> JsonSchema {
    let mut schema = json_to_schema(value);
    // Collect nested `$defs`/`definitions` into one hoisted map.
    let mut defs = BTreeMap::new();
    collect_defs(value, &mut defs);
    if !defs.is_empty() {
        let merged = match schema.dollar_defs.take() {
            Some(mut existing) => {
                for (k, v) in defs {
                    existing.entry(k).or_insert(v);
                }
                existing
            }
            None => defs,
        };
        schema.dollar_defs = Some(merged);
    }
    schema
}

fn json_to_schema(value: &Value) -> JsonSchema {
    let mut schema = JsonSchema::default();
    let obj = match value.as_object() {
        Some(o) => o,
        None => return schema,
    };
    if let Some(t) = obj.get("type") {
        match t {
            Value::String(s) => {
                schema.schema_type = Some(crate::tool::JsonSchemaType::Single(s.clone()));
            }
            Value::Array(items) => {
                schema.schema_type = Some(crate::tool::JsonSchemaType::Multiple(
                    items
                        .iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect(),
                ));
            }
            _ => {}
        }
    }
    if let Some(v) = obj.get("enum").and_then(|v| v.as_array()) {
        schema.r#enum = Some(v.clone());
    }
    if let Some(v) = obj.get("const") {
        schema.r#const = Some(v.clone());
    }
    for (key, field) in [
        ("anyOf", &mut schema.any_of),
        ("oneOf", &mut schema.one_of),
        ("allOf", &mut schema.all_of),
    ] {
        if let Some(items) = obj.get(key).and_then(|v| v.as_array()) {
            *field = Some(items.iter().map(json_to_schema).collect());
        }
    }
    if let Some(props) = obj.get("properties").and_then(|v| v.as_object()) {
        schema.properties = Some(
            props
                .iter()
                .map(|(k, v)| (k.clone(), json_to_schema(v)))
                .collect(),
        );
    }
    if let Some(req) = obj.get("required").and_then(|v| v.as_array()) {
        schema.required = Some(
            req.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect(),
        );
    }
    if let Some(items) = obj.get("items") {
        schema.items = Some(Box::new(json_to_schema(items)));
    }
    if let Some(ap) = obj.get("additionalProperties") {
        schema.additional_properties = Some(match ap {
            Value::Bool(b) => crate::tool::AdditionalProperties::Bool(*b),
            _ => crate::tool::AdditionalProperties::Schema(Box::new(json_to_schema(ap))),
        });
    }
    for (key, field) in [
        ("description", &mut schema.description),
        ("format", &mut schema.format),
        ("$ref", &mut schema.dollar_ref),
    ] {
        if let Some(s) = obj.get(key).and_then(|v| v.as_str()) {
            *field = Some(s.to_string());
        }
    }
    if let Some(v) = obj.get("default") {
        schema.default = Some(v.clone());
    }
    if let Some(b) = obj.get("deprecated").and_then(|v| v.as_bool()) {
        schema.deprecated = Some(b);
    }
    if let Some(n) = obj.get("minItems").and_then(|v| v.as_u64()) {
        schema.min_items = Some(n);
    }
    if let Some(n) = obj.get("maxItems").and_then(|v| v.as_u64()) {
        schema.max_items = Some(n);
    }
    if let Some(defs) = obj.get("$defs").and_then(|v| v.as_object()) {
        schema.dollar_defs = Some(
            defs.iter()
                .map(|(k, v)| (k.clone(), json_to_schema(v)))
                .collect(),
        );
    }
    if let Some(defs) = obj.get("definitions").and_then(|v| v.as_object()) {
        schema.definitions = Some(
            defs.iter()
                .map(|(k, v)| (k.clone(), json_to_schema(v)))
                .collect(),
        );
    }
    schema
}

fn collect_defs(value: &Value, out: &mut BTreeMap<String, JsonSchema>) {
    let obj = match value.as_object() {
        Some(o) => o,
        None => return,
    };
    for key in ["$defs", "definitions"] {
        if let Some(defs) = obj.get(key).and_then(|v| v.as_object()) {
            for (k, v) in defs {
                out.entry(k.clone()).or_insert_with(|| json_to_schema(v));
            }
        }
    }
}

/// Component schema definitions. Mirrors `componentDefinitions(document)`.
pub fn component_definitions(document: &Value) -> BTreeMap<String, JsonSchema> {
    let mut out = BTreeMap::new();
    let schemas = document
        .get("components")
        .and_then(|c| c.get("schemas"))
        .and_then(|s| s.as_object());
    if let Some(schemas) = schemas {
        for (name, value) in schemas {
            out.insert(name.clone(), project_schema(value));
        }
    }
    out
}

fn with_definitions(schema: JsonSchema, definitions: &BTreeMap<String, JsonSchema>) -> JsonSchema {
    if definitions.is_empty() {
        return schema;
    }
    let mut out = schema;
    let mut merged = definitions.clone();
    if let Some(local) = out.dollar_defs.take() {
        for (k, v) in local {
            merged.entry(k).or_insert(v);
        }
    }
    out.dollar_defs = Some(merged);
    out
}

fn is_json_media_type(media_type: &str) -> bool {
    let normalized = media_type
        .split(';')
        .next()
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();
    normalized == "application/json" || normalized.ends_with("+json")
}

fn is_binary_media_type(document: &Value, media_type: &str, value: &Value) -> bool {
    let normalized = media_type
        .split(';')
        .next()
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();
    if !is_json_media_type(&normalized) && !normalized.starts_with("text/") {
        return true;
    }
    let obj = match value.as_object() {
        Some(o) => o,
        None => return false,
    };
    let schema = resolve(document, obj.get("schema").unwrap_or(&Value::Null));
    schema.get("format").and_then(|f| f.as_str()) == Some("binary")
}

fn json_content(content: &serde_json::Map<String, Value>) -> Option<(String, Option<Value>)> {
    content
        .iter()
        .find(|(mt, _)| is_json_media_type(mt))
        .map(|(mt, v)| {
            let schema = v.get("schema").cloned();
            (mt.clone(), schema)
        })
}

fn is_flattenable_object_body(schema: &Value, request_required: bool) -> bool {
    let obj = match schema.as_object() {
        Some(o) => o,
        None => return false,
    };
    request_required
        && obj.get("type").and_then(|v| v.as_str()) == Some("object")
        && obj.get("properties").and_then(|v| v.as_object()).is_some()
        && obj.get("additionalProperties") == Some(&Value::Bool(false))
        && obj.get("nullable") != Some(&Value::Bool(true))
        && !obj.contains_key("allOf")
        && !obj.contains_key("anyOf")
        && !obj.contains_key("oneOf")
}

struct PlannedField {
    name: String,
    location: InputLocation,
    required: bool,
    schema: JsonSchema,
    style: Option<ParamStyle>,
    explode: Option<bool>,
}

fn operation_parameters(
    document: &Value,
    path_item: &serde_json::Map<String, Value>,
    operation: &serde_json::Map<String, Value>,
) -> Parsed<Vec<PlannedField>> {
    // Operation-level parameters override path-level ones sharing (location, name).
    let mut declared: BTreeMap<String, (String, String, Value)> = BTreeMap::new();
    let mut raws: Vec<&Value> = vec![];
    raws.extend(as_array(path_item.get("parameters")));
    raws.extend(as_array(operation.get("parameters")));
    for raw in raws {
        let resolved = resolve(document, raw);
        let obj = match resolved.as_object() {
            Some(o) => o,
            None => {
                return Parsed::Err("parameter declaration is invalid or unresolved".to_string())
            }
        };
        let name = match non_empty_string(obj.get("name")) {
            Some(n) => n,
            None => {
                return Parsed::Err("parameter declaration is missing name or location".to_string())
            }
        };
        let location = match non_empty_string(obj.get("in")) {
            Some(l) => l,
            None => {
                return Parsed::Err("parameter declaration is missing name or location".to_string())
            }
        };
        declared.insert(
            format!("{}:{}", location, name),
            (name, location, resolved.clone()),
        );
    }
    let mut unordered: Vec<PlannedField> = vec![];
    for (_, (name, location, resolved)) in declared {
        let obj = resolved.as_object().cloned().unwrap_or_default();
        if location == "cookie" {
            return Parsed::Err(format!("cookie parameter '{}' is not supported", name));
        }
        if location != "path" && location != "query" && location != "header" {
            return Parsed::Err(format!(
                "parameter '{}' uses unsupported location '{}'",
                name, location
            ));
        }
        if location == "header" && is_ignored_header(&name) {
            continue;
        }
        if obj.get("schema").is_none() && obj.get("content").is_none() {
            return Parsed::Err(format!(
                "parameter '{}' declares neither schema nor content",
                name
            ));
        }
        if obj.get("content").is_some() {
            return Parsed::Err(format!(
                "parameter '{}' uses unsupported content encoding",
                name
            ));
        }
        if let Some(style) = obj.get("style") {
            if non_empty_string(Some(style)).is_none() {
                return Parsed::Err(format!("parameter '{}' has an invalid style", name));
            }
        }
        if let Some(explode) = obj.get("explode") {
            if !explode.is_boolean() {
                return Parsed::Err(format!("parameter '{}' has an invalid explode value", name));
            }
        }
        if let Some(allow) = obj.get("allowReserved") {
            if !allow.is_boolean() {
                return Parsed::Err(format!(
                    "parameter '{}' has an invalid allowReserved value",
                    name
                ));
            }
        }
        if obj.get("allowReserved") == Some(&Value::Bool(true)) {
            return Parsed::Err(format!(
                "parameter '{}' uses unsupported allowReserved encoding",
                name
            ));
        }
        let declared_style = non_empty_string(obj.get("style")).unwrap_or_else(|| {
            if location == "query" {
                "form".to_string()
            } else {
                "simple".to_string()
            }
        });
        if location == "query" && declared_style != "form" && declared_style != "deepObject" {
            return Parsed::Err(format!(
                "query parameter '{}' uses unsupported style '{}'",
                name, declared_style
            ));
        }
        if location != "query" && declared_style != "simple" {
            return Parsed::Err(format!(
                "{} parameter '{}' uses unsupported style '{}'",
                location, name, declared_style
            ));
        }
        let style = match declared_style.as_str() {
            "deepObject" => ParamStyle::DeepObject,
            "form" => ParamStyle::Form,
            _ => ParamStyle::Simple,
        };
        let explode = match obj.get("explode").and_then(|v| v.as_bool()) {
            Some(b) => b,
            None => style == ParamStyle::Form,
        };
        if style == ParamStyle::DeepObject && !explode {
            return Parsed::Err(format!(
                "query parameter '{}' uses deepObject with explode=false",
                name
            ));
        }
        let base = project_schema(obj.get("schema").unwrap_or(&Value::Null));
        let description = non_empty_string(obj.get("description"));
        let mut schema = base;
        if schema.description.is_none() {
            schema.description = description;
        }
        let location_enum = match location.as_str() {
            "path" => InputLocation::Path,
            "query" => InputLocation::Query,
            _ => InputLocation::Header,
        };
        unordered.push(PlannedField {
            required: obj.get("required") == Some(&Value::Bool(true)) || location == "path",
            name,
            location: location_enum,
            style: Some(style),
            explode: Some(explode),
            schema,
        });
    }
    // Stable location order: path, query, header (mirrors
    // `parameterLocations.flatMap(...)`).
    let mut ordered: Vec<PlannedField> = vec![];
    // Drain per location without borrow conflicts (n is tiny).
    let mut remaining = unordered;
    for loc in [
        InputLocation::Path,
        InputLocation::Query,
        InputLocation::Header,
    ] {
        let mut kept = vec![];
        for field in remaining.into_iter() {
            if field.location == loc {
                ordered.push(field);
            } else {
                kept.push(field);
            }
        }
        remaining = kept;
    }
    ordered.extend(remaining);
    Parsed::Ok(ordered)
}

/// Parsed operation input incl. cross-location collision renaming and
/// `__proto__`/`constructor`/`prototype` escaping. Mirrors
/// `operationInput(document, pathItem, operation)`.
pub fn operation_input(
    document: &Value,
    path_item: &serde_json::Map<String, Value>,
    operation: &serde_json::Map<String, Value>,
) -> Parsed<OperationInput> {
    let parameters = operation_parameters(document, path_item, operation);
    let param_fields = match parameters {
        Parsed::Ok(v) => v,
        Parsed::Err(reason) => return Parsed::Err(reason),
    };
    let body = match operation_body(document, operation) {
        Parsed::Ok(v) => v,
        Parsed::Err(reason) => return Parsed::Err(reason),
    };
    let mut fields: Vec<PlannedField> = param_fields;
    fields.extend(body.fields);

    // Cross-location collisions get `location_` prefixes.
    let mut by_name: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for f in &fields {
        by_name
            .entry(f.name.clone())
            .or_default()
            .insert(f.location.as_str().to_string());
    }
    let conflicts: BTreeSet<String> = by_name
        .iter()
        .filter(|(_, locs)| locs.len() > 1)
        .map(|(name, _)| name.clone())
        .collect();
    let mut used: BTreeSet<String> = BTreeSet::new();
    let mut out_fields: Vec<InputField> = vec![];
    for field in fields {
        let visible = if crate::tool_runtime::is_blocked_member(&field.name) {
            format!("{}_2", field.name)
        } else {
            field.name.clone()
        };
        let base = if conflicts.contains(field.name.as_str()) {
            format!("{}_{}", field.location.as_str(), visible)
        } else {
            visible
        };
        let mut index = 1usize;
        let input_name = loop {
            let candidate = if index == 1 {
                base.clone()
            } else {
                format!("{}_{}", base, index)
            };
            if !used.contains(&candidate) {
                break candidate;
            }
            index += 1;
        };
        used.insert(input_name.clone());
        out_fields.push(InputField {
            input_name,
            name: field.name,
            location: field.location,
            required: field.required,
            schema: field.schema,
            style: field.style,
            explode: field.explode,
        });
    }
    Parsed::Ok(OperationInput {
        fields: out_fields,
        body: body.body,
    })
}

struct BodyPlan {
    fields: Vec<PlannedField>,
    body: Option<Body>,
}

fn operation_body(
    document: &Value,
    operation: &serde_json::Map<String, Value>,
) -> Parsed<BodyPlan> {
    let resolved = match operation.get("requestBody") {
        None => {
            return Parsed::Ok(BodyPlan {
                fields: vec![],
                body: None,
            })
        }
        Some(v) => resolve(document, v).clone(),
    };
    let obj = match resolved.as_object() {
        Some(o) => o.clone(),
        None => {
            return Parsed::Ok(BodyPlan {
                fields: vec![],
                body: None,
            })
        }
    };
    let content = obj
        .get("content")
        .and_then(|v| v.as_object())
        .cloned()
        .unwrap_or_default();
    let selected = match json_content(&content) {
        Some(s) => s,
        None => {
            return Parsed::Err(format!(
                "request body has no JSON content (declared: {})",
                if content.is_empty() {
                    "none".to_string()
                } else {
                    content.keys().cloned().collect::<Vec<_>>().join(", ")
                }
            ))
        }
    };
    let schema = resolve(document, selected.1.as_ref().unwrap_or(&Value::Null)).clone();
    let required = obj.get("required") == Some(&Value::Bool(true));
    if !is_flattenable_object_body(&schema, required) {
        return Parsed::Ok(BodyPlan {
            fields: vec![PlannedField {
                name: "body".to_string(),
                location: InputLocation::Body,
                required,
                schema: project_schema(selected.1.as_ref().unwrap_or(&Value::Null)),
                style: None,
                explode: None,
            }],
            body: Some(Body {
                required,
                mode: BodyMode::Value,
                media_type: selected.0,
            }),
        });
    }
    let schema_obj = schema.as_object().cloned().unwrap_or_default();
    let required_props: BTreeSet<String> = schema_obj
        .get("required")
        .and_then(|v| v.as_array())
        .map(|items| {
            items
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();
    let properties = schema_obj
        .get("properties")
        .and_then(|v| v.as_object())
        .cloned()
        .unwrap_or_default();
    Parsed::Ok(BodyPlan {
        fields: properties
            .iter()
            .map(|(name, value)| PlannedField {
                required: required && required_props.contains(name),
                name: name.clone(),
                location: InputLocation::Body,
                schema: project_schema(value),
                style: None,
                explode: None,
            })
            .collect(),
        body: Some(Body {
            required,
            mode: BodyMode::Object,
            media_type: selected.0,
        }),
    })
}

/// Builds the tool input JSON Schema from fields + definitions. Mirrors
/// `inputSchema(fields, definitions)`.
pub fn input_schema(
    fields: &[InputField],
    definitions: &BTreeMap<String, JsonSchema>,
) -> JsonSchema {
    let required: Vec<String> = fields
        .iter()
        .filter(|f| f.required)
        .map(|f| f.input_name.clone())
        .collect();
    let mut schema = JsonSchema {
        schema_type: Some(crate::tool::JsonSchemaType::Single("object".to_string())),
        properties: Some(
            fields
                .iter()
                .map(|f| (f.input_name.clone(), f.schema.clone()))
                .collect(),
        ),
        required: if required.is_empty() {
            None
        } else {
            Some(required)
        },
        ..Default::default()
    };
    schema = with_definitions(schema, definitions);
    schema
}

fn successful_responses(
    document: &Value,
    operation: &serde_json::Map<String, Value>,
) -> Parsed<Vec<serde_json::Map<String, Value>>> {
    let responses = match operation.get("responses").and_then(|v| v.as_object()) {
        None => return Parsed::Ok(vec![]),
        Some(r) => r,
    };
    let mut entries: Vec<(&String, &Value)> = responses.iter().collect();
    entries.sort_by(|a, b| a.0.cmp(b.0));
    let twos: Vec<(&String, &Value)> = entries
        .iter()
        .filter(|(status, _)| {
            status.len() == 3
                && status.starts_with('2')
                && status.chars().all(|c| c.is_ascii_digit())
        })
        .map(|(s, v)| (*s, *v))
        .collect();
    let wildcards: Vec<(&String, &Value)> = entries
        .iter()
        .filter(|(status, _)| status.eq_ignore_ascii_case("2XX"))
        .map(|(s, v)| (*s, *v))
        .collect();
    let mut responses_out = vec![];
    for (_, value) in twos.into_iter().chain(wildcards) {
        let resolved = resolve(document, value);
        match resolved.as_object() {
            Some(o) if o.get("$ref").and_then(|v| v.as_str()).is_none() => {
                responses_out.push(o.clone())
            }
            _ => {
                return Parsed::Err(
                    "successful response declaration is invalid or unresolved".to_string(),
                )
            }
        }
    }
    Parsed::Ok(responses_out)
}

/// Infers the tool output schema. Mirrors `operationOutput(document,
/// operation, definitions)` incl. verbatim skip reasons.
pub fn operation_output(
    document: &Value,
    operation: &serde_json::Map<String, Value>,
    definitions: &BTreeMap<String, JsonSchema>,
) -> Parsed<Option<JsonSchema>> {
    if operation.get("x-websocket") == Some(&Value::Bool(true)) {
        return Parsed::Err("WebSocket operations are not supported".to_string());
    }
    let responses = match successful_responses(document, operation) {
        Parsed::Ok(v) => v,
        Parsed::Err(reason) => return Parsed::Err(reason),
    };
    let streams = responses.iter().any(|response| {
        response
            .get("content")
            .and_then(|c| c.as_object())
            .map(|content| {
                content.keys().any(|mt| {
                    mt.split(';')
                        .next()
                        .unwrap_or("")
                        .trim()
                        .eq_ignore_ascii_case("text/event-stream")
                })
            })
            .unwrap_or(false)
    });
    if streams {
        return Parsed::Err("SSE operations are not supported".to_string());
    }
    let binary = responses.iter().any(|response| {
        response
            .get("content")
            .and_then(|c| c.as_object())
            .map(|content| {
                content
                    .iter()
                    .any(|(mt, value)| is_binary_media_type(document, mt, value))
            })
            .unwrap_or(false)
    });
    if binary {
        return Parsed::Err("binary responses are not supported".to_string());
    }
    let mut outcomes: Vec<JsonSchema> = vec![];
    for response in &responses {
        match response.get("content") {
            Some(c) if !is_record(c) => return Parsed::Ok(None),
            _ => {}
        }
        let content = response
            .get("content")
            .and_then(|c| c.as_object())
            .cloned()
            .unwrap_or_default();
        if content.is_empty() {
            outcomes.push(JsonSchema {
                schema_type: Some(crate::tool::JsonSchemaType::Single("null".to_string())),
                ..Default::default()
            });
            continue;
        }
        for (media_type, value) in &content {
            if !is_json_media_type(media_type) {
                outcomes.push(JsonSchema {
                    schema_type: Some(crate::tool::JsonSchemaType::Single("string".to_string())),
                    ..Default::default()
                });
                continue;
            }
            let schema_val = value.get("schema");
            if !matches!(value, Value::Object(_)) || schema_val.is_none() {
                return Parsed::Ok(None);
            }
            outcomes.push(project_schema(schema_val.unwrap()));
        }
    }
    if outcomes.is_empty() {
        return Parsed::Ok(None);
    }
    let combined = if outcomes.len() == 1 {
        outcomes.into_iter().next().unwrap()
    } else {
        JsonSchema {
            any_of: Some(outcomes),
            ..Default::default()
        }
    };
    Parsed::Ok(Some(with_definitions(combined, definitions)))
}

fn sanitize_operation_segment(raw: &str) -> String {
    let mut base: String = raw
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' || c == '$' {
                c
            } else {
                '_'
            }
        })
        .collect::<String>()
        .trim_matches('_')
        .to_string();
    if let Some(first) = base.chars().next() {
        if first.is_ascii_digit() {
            base = format!("_{}", base);
        }
    }
    if base.is_empty() {
        base = "operation".to_string();
    }
    if crate::tool_runtime::is_blocked_member(&base) {
        base = format!("{}_2", base);
    }
    base
}

fn fallback_operation_id(method: &str, path: &str) -> String {
    let mut words: Vec<String> = vec![method.to_string()];
    for part in path.split('/').filter(|p| !p.is_empty()) {
        if part.starts_with('{') && part.ends_with('}') {
            words.push("by".to_string());
            words.push(part[1..part.len() - 1].to_string());
        } else {
            words.push(part.to_string());
        }
    }
    let mut flat: Vec<String> = vec![];
    for word in words {
        for piece in word
            .split(|c: char| !c.is_ascii_alphanumeric())
            .filter(|w| !w.is_empty())
        {
            flat.push(piece.to_string());
        }
    }
    flat.iter()
        .enumerate()
        .map(|(i, word)| {
            let lower = word.to_ascii_lowercase();
            if i == 0 {
                lower
            } else {
                let mut chars = lower.chars();
                match chars.next() {
                    Some(first) => format!(
                        "{}{}",
                        first.to_ascii_uppercase(),
                        chars.collect::<String>()
                    ),
                    None => String::new(),
                }
            }
        })
        .collect::<String>()
}

/// Allocates a collision-free tool path. Mirrors `operationPath(method, path,
/// operation, used, namespaces)` verbatim (incl. collapse + `_2` suffix).
pub fn operation_path(
    method: &str,
    path: &str,
    operation: &serde_json::Map<String, Value>,
    used: &BTreeSet<String>,
    namespaces: &BTreeSet<String>,
) -> Vec<String> {
    let raw = non_empty_string(operation.get("operationId"));
    let segments: Vec<String> = match raw {
        None => vec![sanitize_operation_segment(&fallback_operation_id(
            method, path,
        ))],
        Some(id) => id.split('.').map(sanitize_operation_segment).collect(),
    };
    if is_operation_path_available(&segments, used, namespaces) {
        return segments;
    }
    let conflict = segments
        .iter()
        .enumerate()
        .take(segments.len().saturating_sub(1))
        .find(|(index, _)| used.contains(&segments[..index + 1].join(".")))
        .map(|(index, _)| index);
    if let Some(conflict) = conflict {
        if conflict + 1 < segments.len() {
            let mut collapsed: Vec<String> = vec![];
            for (index, segment) in segments.iter().enumerate() {
                if index == conflict {
                    let next = segments.get(index + 1).cloned().unwrap_or_default();
                    let mut chars = next.chars();
                    let merged = match chars.next() {
                        Some(first) => {
                            format!(
                                "{}{}{}",
                                segment,
                                first.to_ascii_uppercase(),
                                chars.collect::<String>()
                            )
                        }
                        None => segment.clone(),
                    };
                    collapsed.push(merged);
                } else if index == conflict + 1 {
                    continue;
                } else {
                    collapsed.push(segment.clone());
                }
            }
            if is_operation_path_available(&collapsed, used, namespaces) {
                return collapsed;
            }
        }
    }
    let fallback = segments.join("_");
    let mut index = 2usize;
    loop {
        let candidate = format!("{}_{}", fallback, index);
        if is_operation_path_available(std::slice::from_ref(&candidate), used, namespaces) {
            return vec![candidate];
        }
        index += 1;
    }
}

fn is_operation_path_available(
    segments: &[String],
    used: &BTreeSet<String>,
    namespaces: &BTreeSet<String>,
) -> bool {
    let key = segments.join(".");
    if used.contains(&key) || namespaces.contains(&key) {
        return false;
    }
    segments[..segments.len().saturating_sub(1)]
        .iter()
        .enumerate()
        .all(|(index, _)| !used.contains(&segments[..index + 1].join(".")))
}

/// Reads the first server URL. Mirrors `specServerUrl(source)` verbatim.
pub fn spec_server_url(source: &serde_json::Map<String, Value>) -> Parsed<String> {
    let server = source
        .get("servers")
        .and_then(|v| v.as_array())
        .map(|items| items.iter().find(|v| v.is_object()).cloned())
        .unwrap_or(None);
    let url = server.as_ref().and_then(|s| non_empty_string(s.get("url")));
    match url {
        None => Parsed::Err("spec declares no servers; pass baseUrl".to_string()),
        Some(url) => {
            if contains_template(&url) {
                return Parsed::Err(format!(
                    "server URL '{}' is not an absolute URL; pass baseUrl",
                    url
                ));
            }
            validate_base_url(&url)
        }
    }
}

fn contains_template(url: &str) -> bool {
    let mut rest = url;
    while let Some(start) = rest.find('{') {
        let after = &rest[start + 1..];
        match after.find('}') {
            Some(end) => {
                let inner = &after[..end];
                if !inner.contains('{') && !inner.contains('}') {
                    return true;
                }
                rest = &after[end + 1..];
            }
            None => break,
        }
    }
    false
}

/// Validates an absolute HTTP(S) base URL. Mirrors `validateBaseUrl(value)`
/// verbatim incl. the `^https?://` gate and query/fragment rejection.
pub fn validate_base_url(value: &str) -> Parsed<String> {
    let lower = value.to_ascii_lowercase();
    if !(lower.starts_with("http://") || lower.starts_with("https://")) {
        return Parsed::Err(format!(
            "server URL '{}' is not an absolute HTTP(S) URL",
            value
        ));
    }
    let after_scheme = match value.find("://") {
        Some(i) => &value[i + 3..],
        None => {
            return Parsed::Err(format!(
                "server URL '{}' is not an absolute HTTP(S) URL",
                value
            ));
        }
    };
    if after_scheme.is_empty() {
        return Parsed::Err(format!(
            "server URL '{}' is not an absolute HTTP(S) URL",
            value
        ));
    }
    // Reject query/fragment (verbatim reason).
    let authority_end = after_scheme
        .find(['/', '?', '#'])
        .unwrap_or(after_scheme.len());
    let remainder = &after_scheme[authority_end..];
    if remainder.contains('?') || remainder.contains('#') {
        return Parsed::Err(format!(
            "server URL '{}' contains an unsupported query string or fragment",
            value
        ));
    }
    Parsed::Ok(value.to_string())
}

/// Parses top-level security requirements. Mirrors
/// `securityRequirements(value)` verbatim.
pub fn security_requirements(value: Option<&Value>) -> Parsed<Vec<SecurityRequirement>> {
    match value {
        None => Parsed::Ok(vec![]),
        Some(v) => {
            let items = match v.as_array() {
                Some(a) => a,
                None => return Parsed::Err("security declaration is not an array".to_string()),
            };
            let mut out = vec![];
            for item in items {
                let obj = match item.as_object() {
                    Some(o) => o,
                    None => {
                        return Parsed::Err("security requirement is not an object".to_string())
                    }
                };
                let mut requirement = SecurityRequirement::new();
                for (name, scopes) in obj {
                    let arr = match scopes.as_array() {
                        Some(a) => a,
                        None => {
                            return Parsed::Err(
                                "security requirement scopes are not string arrays".to_string(),
                            )
                        }
                    };
                    let mut parsed = vec![];
                    for scope in arr {
                        match scope.as_str() {
                            Some(s) => parsed.push(s.to_string()),
                            None => {
                                return Parsed::Err(
                                    "security requirement scopes are not string arrays".to_string(),
                                )
                            }
                        }
                    }
                    requirement.insert(name.clone(), parsed);
                }
                out.push(requirement);
            }
            Parsed::Ok(out)
        }
    }
}

/// Resolves operation security against known schemes. Mirrors
/// `operationSecurityRequirements(value, defaults, schemes)` verbatim incl.
/// cookie-scheme fail-closed behavior.
pub fn operation_security_requirements(
    value: Option<&Value>,
    defaults: &Parsed<Vec<SecurityRequirement>>,
    schemes: &BTreeMap<String, SecurityScheme>,
) -> Parsed<Vec<SecurityRequirement>> {
    let parsed: Parsed<Vec<SecurityRequirement>> = match value {
        None => match defaults {
            Parsed::Ok(v) => Parsed::Ok(v.clone()),
            Parsed::Err(reason) => Parsed::Err(reason.clone()),
        },
        Some(_) => security_requirements(value),
    };
    let requirements = match parsed {
        Parsed::Ok(v) => v,
        Parsed::Err(reason) => return Parsed::Err(reason),
    };
    let supported: Vec<SecurityRequirement> = requirements
        .iter()
        .filter(|requirement| {
            requirement
                .keys()
                .iter()
                .all(|name| match schemes.get(name.as_str()) {
                    Some(SecurityScheme::ApiKey {
                        location: ApiKeyLocation::Cookie,
                        ..
                    }) => false,
                    Some(_) => true,
                    None => false,
                })
        })
        .cloned()
        .collect();
    if requirements.is_empty() || !supported.is_empty() {
        return Parsed::Ok(supported);
    }
    let mut names: Vec<String> = vec![];
    for requirement in &requirements {
        for name in requirement.keys() {
            if !names.contains(name) {
                names.push(name.clone());
            }
        }
    }
    let cookie_scheme = names.iter().find(|name| {
        matches!(
            schemes.get(*name),
            Some(SecurityScheme::ApiKey {
                location: ApiKeyLocation::Cookie,
                ..
            })
        )
    });
    match cookie_scheme {
        None => Parsed::Err(format!(
            "security requirement references missing or malformed scheme: {}",
            names.join(", ")
        )),
        Some(name) => Parsed::Err(format!("cookie authentication '{}' is not supported", name)),
    }
}

/// Collects valid security schemes. Mirrors `securitySchemes(document)`:
/// malformed declarations are dropped (empty result), never thrown.
pub fn security_schemes(document: &Value) -> BTreeMap<String, SecurityScheme> {
    let mut out = BTreeMap::new();
    let declared = document
        .get("components")
        .and_then(|c| c.get("securitySchemes"))
        .and_then(|s| s.as_object());
    let declared = match declared {
        Some(d) => d,
        None => return out,
    };
    for (name, value) in declared {
        let resolved = resolve(document, value);
        let obj = match resolved.as_object() {
            Some(o) => o,
            None => continue,
        };
        let scheme_type = non_empty_string(obj.get("type"));
        match scheme_type.as_deref() {
            Some("apiKey") => {
                let carrier = non_empty_string(obj.get("in"));
                let parameter = non_empty_string(obj.get("name"));
                let location = match (parameter, carrier.as_deref()) {
                    (Some(p), Some("header")) => Some((p, ApiKeyLocation::Header)),
                    (Some(p), Some("query")) => Some((p, ApiKeyLocation::Query)),
                    (Some(p), Some("cookie")) => Some((p, ApiKeyLocation::Cookie)),
                    _ => None,
                };
                if let Some((p, location)) = location {
                    out.insert(name.clone(), SecurityScheme::ApiKey { name: p, location });
                }
            }
            Some("http") => {
                if let Some(scheme) = non_empty_string(obj.get("scheme")) {
                    out.insert(
                        name.clone(),
                        SecurityScheme::Http {
                            scheme: scheme.to_ascii_lowercase(),
                        },
                    );
                }
            }
            Some("oauth2") => {
                out.insert(name.clone(), SecurityScheme::OAuth2);
            }
            Some("openIdConnect") => {
                out.insert(name.clone(), SecurityScheme::OpenIdConnect);
            }
            _ => {}
        }
    }
    out
}

#[allow(dead_code)]
fn document_map(_doc: &Document) {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn methods_verbatim() {
        assert!(is_method("get"));
        assert!(!is_method("connect"));
    }

    #[test]
    fn validate_base_url_gate_verbatim() {
        let err = match validate_base_url("ftp://example.test") {
            Parsed::Err(reason) => reason,
            _ => panic!("expected err"),
        };
        assert_eq!(
            err,
            "server URL 'ftp://example.test' is not an absolute HTTP(S) URL"
        );
        let err = match validate_base_url("https://example.test/?x=1") {
            Parsed::Err(reason) => reason,
            _ => panic!("expected err"),
        };
        assert!(err.contains("unsupported query string or fragment"));
    }

    #[test]
    fn resolve_follows_local_refs() {
        let doc = json!({"components": {"schemas": {"A": {"type": "string"}}}});
        let value = json!({"$ref": "#/components/schemas/A"});
        let resolved = resolve(&doc, &value).clone();
        assert_eq!(resolved, json!({"type": "string"}));
    }

    #[test]
    fn own_reads_existing_keys() {
        let doc = json!({"a": 1});
        let map = doc.as_object().unwrap();
        assert!(own(map, "a").is_some());
        assert!(own(map, "b").is_none());
    }
}
