//! Port of `src/tool.ts`.
//!
//! Schema-described tool definitions consumed by a CodeMode tool tree.
//! The Effect-Schema vs JSON-Schema duality is preserved: a validating
//! schema decodes input/output (throwing on failure); a render-only JSON
//! Schema document only shapes the model-visible signature while values pass
//! through unvalidated. A tool without an output schema advertises `unknown`.

use serde_json::Value;

/// Type alias for the validating-schema closure to reduce type complexity (clippy::type_complexity).
pub type ValidatorFn = Box<dyn Fn(&Value) -> Result<Value, String> + Send + Sync>;
/// Type alias for tool `run` closures.
pub type ToolRunFn = Box<dyn Fn(&Value) -> Result<Value, ToolFailure> + Send + Sync>;
/// Type alias for raw host-function closures (`HostTool`).
pub type HostToolFn = Box<dyn Fn(&[Value]) -> Result<Value, ToolFailure> + Send + Sync>;

/// JSON Schema subset accepted for render-only tool schemas.
///
/// A JSON-Schema-described side of a tool is used to generate the
/// model-visible TypeScript signature only — CodeMode performs no validation
/// against it. This is the natural shape for adapter-provided tools (e.g. MCP
/// definitions) whose schemas arrive as JSON Schema documents.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct JsonSchema {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub schema_type: Option<JsonSchemaType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#const: Option<Value>,
    #[serde(rename = "anyOf", skip_serializing_if = "Option::is_none")]
    pub any_of: Option<Vec<JsonSchema>>,
    #[serde(rename = "oneOf", skip_serializing_if = "Option::is_none")]
    pub one_of: Option<Vec<JsonSchema>>,
    #[serde(rename = "allOf", skip_serializing_if = "Option::is_none")]
    pub all_of: Option<Vec<JsonSchema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::BTreeMap<String, JsonSchema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<JsonSchema>>,
    #[serde(
        rename = "additionalProperties",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_properties: Option<AdditionalProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(rename = "minItems", skip_serializing_if = "Option::is_none")]
    pub min_items: Option<u64>,
    #[serde(rename = "maxItems", skip_serializing_if = "Option::is_none")]
    pub max_items: Option<u64>,
    #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    pub dollar_ref: Option<String>,
    #[serde(rename = "$defs", skip_serializing_if = "Option::is_none")]
    pub dollar_defs: Option<std::collections::BTreeMap<String, JsonSchema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<std::collections::BTreeMap<String, JsonSchema>>,
}

/// The `type` keyword: single name or list of names.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum JsonSchemaType {
    Single(String),
    Multiple(Vec<String>),
}

/// The `additionalProperties` keyword: boolean or schema.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum AdditionalProperties {
    Bool(bool),
    Schema(Box<JsonSchema>),
}

/// Either a validating schema or a render-only JSON Schema document.
///
/// The Effect-Schema half is represented as an opaque validator closure plus
/// its pre-rendered JSON Schema document (the same emission signature
/// rendering uses). Validation behavior (decode-or-throw) is preserved;
/// schema-graph introspection goes through the stored document.
pub enum SchemaType {
    /// Validating schema: `validate` decodes (throwing on failure), `document`
    /// is the JSON Schema emission used for signature rendering + search text.
    Validating {
        validate: ValidatorFn,
        document: JsonSchema,
        /// Whether this side represents decoded output (`Schema.toType`).
        decoded: bool,
    },
    /// Render-only JSON Schema document; values pass through unvalidated.
    Json(JsonSchema),
}

impl std::fmt::Debug for SchemaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SchemaType::Validating {
                document, decoded, ..
            } => f
                .debug_struct("Validating")
                .field("document", document)
                .field("decoded", decoded)
                .finish_non_exhaustive(),
            SchemaType::Json(doc) => f.debug_tuple("Json").field(doc).finish(),
        }
    }
}

impl Clone for SchemaType {
    fn clone(&self) -> Self {
        match self {
            SchemaType::Validating {
                document, decoded, ..
            } => SchemaType::Validating {
                // Validators are host behavior, not data: cloning keeps the
                // document and reuses identity validation. Hosts supplying
                // real validators construct fresh `Definition`s; the clone
                // path only serves catalog/search rendering.
                validate: Box::new(|v| Ok(v.clone())),
                document: document.clone(),
                decoded: *decoded,
            },
            SchemaType::Json(doc) => SchemaType::Json(doc.clone()),
        }
    }
}

/// Schema-backed tool definition consumed by a CodeMode tool tree.
///
/// Mirrors `Definition<R>` (`_tag: "CodeModeTool"`, `description`, `input`,
/// `output?`, `run`). The `run` closure receives the decoded input and
/// returns the encoded output; `R` (Effect services) is erased — the host
/// closes over its own environment instead.
pub struct Definition {
    /// Discriminant tag. Always `"CodeModeTool"`.
    pub tag: &'static str,
    /// Model-visible description.
    pub description: String,
    /// Input schema (validating or render-only).
    pub input: SchemaType,
    /// Output schema, if any. Absent advertises `unknown`.
    pub output: Option<SchemaType>,
    /// Tool implementation. Returns `Ok(encoded)` or `Err(failure value)`.
    pub run: ToolRunFn,
}

/// Host failure value from `run`: either a safe [`crate::tool_error::ToolError`]
/// refusal or an opaque unknown failure (sanitized to `"Tool execution failed"`).
#[derive(Debug, Clone)]
pub enum ToolFailure {
    /// Safe model-visible refusal.
    ToolError(crate::tool_error::ToolError),
    /// Unknown host failure; sanitized by default.
    Unknown(String),
}

impl std::fmt::Debug for Definition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Definition")
            .field("tag", &self.tag)
            .field("description", &self.description)
            .field("input", &self.input)
            .field("output", &self.output)
            .finish_non_exhaustive()
    }
}

/// Options for defining one CodeMode tool. Mirrors `Tool.Options`.
pub struct ToolOptions {
    pub description: String,
    pub input: SchemaType,
    pub output: Option<SchemaType>,
    pub run: ToolRunFn,
}

/// Mirrors `isDefinition(value)`.
pub fn is_definition(value: &ToolTreeValue) -> bool {
    matches!(value, ToolTreeValue::Definition(_))
}

/// A node of the host tool tree: definition, subtree, or raw host function.
///
/// Raw host functions mirror `HostTool` (`(...args) => Effect`): they receive
/// positional sandbox args and return host values.
pub enum ToolTreeValue {
    Definition(Box<Definition>),
    Namespace(ToolTree),
    HostFn(HostToolFn),
}

impl std::fmt::Debug for ToolTreeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolTreeValue::Definition(d) => f.debug_tuple("Definition").field(d).finish(),
            ToolTreeValue::Namespace(_) => f.debug_tuple("Namespace").finish_non_exhaustive(),
            ToolTreeValue::HostFn(_) => f.debug_tuple("HostFn").finish_non_exhaustive(),
        }
    }
}

/// The host tool tree (`tools` argument to `execute`/`make`).
///
/// Insertion-ordered like a JS object: enumeration (`Object.keys`,
/// `for...in`) and catalog walks observe definition order, not sorted order.
#[derive(Debug, Default)]
pub struct ToolTree {
    pub entries: Vec<(String, ToolTreeValue)>,
}

impl ToolTree {
    /// Empty tree.
    pub fn new() -> Self {
        ToolTree { entries: vec![] }
    }

    /// Looks up a child by name.
    pub fn get(&self, key: &str) -> Option<&ToolTreeValue> {
        self.entries.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    /// Looks up a child by name, mutably.
    pub fn get_mut(&mut self, key: &str) -> Option<&mut ToolTreeValue> {
        self.entries
            .iter_mut()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }

    /// Inserts or replaces a child, preserving first-insertion position.
    pub fn insert(&mut self, key: String, value: ToolTreeValue) {
        match self.entries.iter_mut().find(|(k, _)| *k == key) {
            Some(slot) => slot.1 = value,
            None => self.entries.push((key, value)),
        }
    }

    /// Removes a child by name.
    pub fn remove(&mut self, key: &str) {
        self.entries.retain(|(k, _)| k != key);
    }

    /// Whether a child exists.
    pub fn contains_key(&self, key: &str) -> bool {
        self.entries.iter().any(|(k, _)| k == key)
    }

    /// Child names in definition order.
    pub fn keys(&self) -> Vec<String> {
        self.entries.iter().map(|(k, _)| k.clone()).collect()
    }
}

/// Defines one schema-described tool available to a CodeMode program through
/// `tools.*`. Mirrors `Tool.make(options)`.
///
/// `input` and `output` each accept a validating schema or a render-only JSON
/// Schema document. Validating input is decoded before `run` is invoked, and
/// `run` returns the encoded representation of a validating `output`, which
/// CodeMode decodes before returning it to the program. JSON Schemas only
/// shape the model-visible signature; values pass through unvalidated.
/// `output` is optional — without it the signature advertises `unknown` and
/// the host result is exposed as-is. The host tool remains responsible for
/// authorization and durable side-effect handling.
pub fn make_tool(options: ToolOptions) -> Definition {
    Definition {
        tag: "CodeModeTool",
        description: options.description,
        input: options.input,
        output: options.output,
        run: options.run,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn make_tool_sets_codemode_tag() {
        let def = make_tool(ToolOptions {
            description: "Look up an order".to_string(),
            input: SchemaType::Json(JsonSchema::default()),
            output: None,
            run: Box::new(|_| Ok(json!({"status": "open"}))),
        });
        assert_eq!(def.tag, "CodeModeTool");
        assert!(def.output.is_none());
    }

    #[test]
    fn is_definition_matches_definitions_only() {
        let def = make_tool(ToolOptions {
            description: "d".to_string(),
            input: SchemaType::Json(JsonSchema::default()),
            output: None,
            run: Box::new(|_| Ok(Value::Null)),
        });
        assert!(is_definition(&ToolTreeValue::Definition(Box::new(def))));
        assert!(!is_definition(&ToolTreeValue::Namespace(ToolTree::new())));
    }
}
