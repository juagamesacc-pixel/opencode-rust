//! Port of `src/interpreter/model.ts`.
//!
//! AST + scope + reference model. The Acorn parse boundary (R1) is
//! represented faithfully as specified: the AST is `serde_json::Value`-based
//! nodes plus the typed helpers below (`is_record`, `as_node`, `get_array`,
//! `get_string`, `get_boolean`, `get_optional_node`, `get_node`,
//! `source_location`, `format_location`). NO parser dependency is added;
//! parser-equivalence is a flagged risk, not silently resolved.

use serde_json::Value;
use std::collections::HashMap;

/// Source line/column (1-based after Acorn→TS offset correction).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SourcePosition {
    pub line: u64,
    pub column: u64,
}

/// Source span.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SourceLocation {
    pub start: SourcePosition,
    pub end: SourcePosition,
}

/// An Acorn AST node: `{ type: string, loc?: SourceLocation, ... }`.
#[derive(Debug, Clone)]
pub struct AstNode {
    /// Node type tag (e.g. `"Program"`, `"CallExpression"`).
    pub node_type: String,
    /// Optional source span.
    pub loc: Option<SourceLocation>,
    /// Raw node object (all other fields).
    pub raw: Value,
}

impl AstNode {
    /// Field read mirroring `node[key]` in TS.
    pub fn get(&self, key: &str) -> &Value {
        self.raw.get(key).unwrap_or(&Value::Null)
    }
}

/// A `Program` node with its statement body.
#[derive(Debug, Clone)]
pub struct ProgramNode {
    pub node: AstNode,
    pub body: Vec<AstNode>,
}

/// Lexical binding in a scope frame.
#[derive(Debug, Clone)]
pub struct Binding {
    pub mutable: bool,
    pub value: Value,
    pub initialized: bool,
}

impl Binding {
    /// Mirrors `{ mutable, value, initialized? }`.
    pub fn new(mutable: bool, value: Value) -> Self {
        Binding {
            mutable,
            value,
            initialized: true,
        }
    }

    /// Un-initialized (`let x;`) binding.
    pub fn uninitialized(mutable: bool) -> Self {
        Binding {
            mutable,
            value: Value::Null,
            initialized: false,
        }
    }
}

/// Result of evaluating one statement.
#[derive(Debug, Clone)]
pub enum StatementResult {
    None,
    Value(Value),
    Return(Value),
    Break(Option<String>),
    Continue(Option<String>),
}

/// Resolved member assignment target (`target[key]`).
#[derive(Debug, Clone)]
pub struct MemberReference {
    pub target: Value,
    pub key: MemberKey,
}

/// Member key: string property or numeric index.
#[derive(Debug, Clone)]
pub enum MemberKey {
    Str(String),
    Num(u64),
}

/// User-defined function value (arrow / function expression / declaration).
#[derive(Debug, Clone)]
pub struct CodeModeFunction {
    pub parameters: Vec<AstNode>,
    pub body: AstNode,
    /// Captured scope chain (cloned frames at definition time).
    pub captured_scopes: Vec<HashMap<String, Binding>>,
}

/// Method reference on a receiver value (`value.method` before call).
#[derive(Debug, Clone)]
pub struct IntrinsicReference {
    pub receiver: Value,
    pub name: String,
}

/// Computed (already-evaluated) callee value.
#[derive(Debug, Clone)]
pub struct ComputedValue {
    pub value: Value,
}

/// The `Promise` namespace object.
#[derive(Debug, Clone, Copy)]
pub struct PromiseNamespace;

/// `Promise.*` method names. Mirrors `PromiseMethodName`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PromiseMethodName {
    All,
    AllSettled,
    Race,
    Resolve,
    Reject,
}

impl PromiseMethodName {
    /// Mirrors the TS string names.
    pub fn as_str(&self) -> &'static str {
        match self {
            PromiseMethodName::All => "all",
            PromiseMethodName::AllSettled => "allSettled",
            PromiseMethodName::Race => "race",
            PromiseMethodName::Resolve => "resolve",
            PromiseMethodName::Reject => "reject",
        }
    }

    /// Parses a TS `Promise.<name>` method name.
    pub fn parse(name: &str) -> Option<Self> {
        match name {
            "all" => Some(PromiseMethodName::All),
            "allSettled" => Some(PromiseMethodName::AllSettled),
            "race" => Some(PromiseMethodName::Race),
            "resolve" => Some(PromiseMethodName::Resolve),
            "reject" => Some(PromiseMethodName::Reject),
            _ => None,
        }
    }
}

/// Reference to `Promise.<method>`.
#[derive(Debug, Clone, Copy)]
pub struct PromiseMethodReference {
    pub name: PromiseMethodName,
}

/// Global namespace names. Mirrors `GlobalNamespaceName`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GlobalNamespaceName {
    Object,
    Math,
    Json,
    Array,
    Console,
    Date,
    RegExp,
    Map,
    Set,
    Url,
    UrlSearchParams,
}

impl GlobalNamespaceName {
    /// Mirrors the TS string names.
    pub fn as_str(&self) -> &'static str {
        match self {
            GlobalNamespaceName::Object => "Object",
            GlobalNamespaceName::Math => "Math",
            GlobalNamespaceName::Json => "JSON",
            GlobalNamespaceName::Array => "Array",
            GlobalNamespaceName::Console => "console",
            GlobalNamespaceName::Date => "Date",
            GlobalNamespaceName::RegExp => "RegExp",
            GlobalNamespaceName::Map => "Map",
            GlobalNamespaceName::Set => "Set",
            GlobalNamespaceName::Url => "URL",
            GlobalNamespaceName::UrlSearchParams => "URLSearchParams",
        }
    }

    /// Parses a TS global name.
    pub fn parse(name: &str) -> Option<Self> {
        match name {
            "Object" => Some(GlobalNamespaceName::Object),
            "Math" => Some(GlobalNamespaceName::Math),
            "JSON" => Some(GlobalNamespaceName::Json),
            "Array" => Some(GlobalNamespaceName::Array),
            "console" => Some(GlobalNamespaceName::Console),
            "Date" => Some(GlobalNamespaceName::Date),
            "RegExp" => Some(GlobalNamespaceName::RegExp),
            "Map" => Some(GlobalNamespaceName::Map),
            "Set" => Some(GlobalNamespaceName::Set),
            "URL" => Some(GlobalNamespaceName::Url),
            "URLSearchParams" => Some(GlobalNamespaceName::UrlSearchParams),
            _ => None,
        }
    }
}

/// A global namespace object (`Object`, `Math`, ...).
#[derive(Debug, Clone, Copy)]
pub struct GlobalNamespace {
    pub name: GlobalNamespaceName,
}

/// Reference to `<namespace>.<method>` (incl. `Number.*` / `String.*`).
#[derive(Debug, Clone)]
pub struct GlobalMethodReference {
    pub namespace: String,
    pub name: String,
}

/// Coercion function (`Number`, `String`, `Boolean`, `parseInt`, `parseFloat`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CoercionKind {
    Number,
    String,
    Boolean,
    ParseInt,
    ParseFloat,
}

impl CoercionKind {
    /// Mirrors the TS names.
    pub fn as_str(&self) -> &'static str {
        match self {
            CoercionKind::Number => "Number",
            CoercionKind::String => "String",
            CoercionKind::Boolean => "Boolean",
            CoercionKind::ParseInt => "parseInt",
            CoercionKind::ParseFloat => "parseFloat",
        }
    }
}

/// Wraps a coercion function value.
#[derive(Debug, Clone, Copy)]
pub struct CoercionFunction {
    pub name: CoercionKind,
}

/// URI helper (`encodeURI`, `encodeURIComponent`, `decodeURI`,
/// `decodeURIComponent`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UriKind {
    EncodeUri,
    EncodeUriComponent,
    DecodeUri,
    DecodeUriComponent,
}

impl UriKind {
    /// Mirrors the TS names.
    pub fn as_str(&self) -> &'static str {
        match self {
            UriKind::EncodeUri => "encodeURI",
            UriKind::EncodeUriComponent => "encodeURIComponent",
            UriKind::DecodeUri => "decodeURI",
            UriKind::DecodeUriComponent => "decodeURIComponent",
        }
    }
}

/// Wraps a URI helper value.
#[derive(Debug, Clone, Copy)]
pub struct UriFunction {
    pub name: UriKind,
}

/// A user `throw <value>` in flight.
#[derive(Debug, Clone)]
pub struct ProgramThrow {
    pub value: Value,
}

/// An `Error` constructor reference (`Error`, `TypeError`, ...).
#[derive(Debug, Clone)]
pub struct ErrorConstructorReference {
    pub name: String,
}

/// Stable diagnostic categories. Mirrors the `DiagnosticKind` union
/// (and `CodeMode.DiagnosticKind` schema literals) verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum DiagnosticKind {
    ParseError,
    UnsupportedSyntax,
    UnknownTool,
    InvalidToolInput,
    InvalidToolOutput,
    InvalidDataValue,
    ToolCallLimitExceeded,
    TimeoutExceeded,
    ToolFailure,
    ExecutionFailure,
}

impl DiagnosticKind {
    /// Verbatim wire spelling.
    pub fn as_str(&self) -> &'static str {
        match self {
            DiagnosticKind::ParseError => "ParseError",
            DiagnosticKind::UnsupportedSyntax => "UnsupportedSyntax",
            DiagnosticKind::UnknownTool => "UnknownTool",
            DiagnosticKind::InvalidToolInput => "InvalidToolInput",
            DiagnosticKind::InvalidToolOutput => "InvalidToolOutput",
            DiagnosticKind::InvalidDataValue => "InvalidDataValue",
            DiagnosticKind::ToolCallLimitExceeded => "ToolCallLimitExceeded",
            DiagnosticKind::TimeoutExceeded => "TimeoutExceeded",
            DiagnosticKind::ToolFailure => "ToolFailure",
            DiagnosticKind::ExecutionFailure => "ExecutionFailure",
        }
    }
}

impl std::fmt::Display for DiagnosticKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Marker for optional-chain short-circuit. Mirrors `OptionalShortCircuit`.
#[derive(Debug, Clone, Copy)]
pub struct OptionalShortCircuit;

/// Verbatim supported-syntax message.
pub const SUPPORTED_SYNTAX_MESSAGE: &str = "Supported orchestration syntax: tools.* calls (they return promises - resolve them with await), data literals, destructuring, optional chaining, template literals, conditionals, switch, loops (incl. for...of and for...in over object/array/tools keys), arrow functions, spread, try/catch, array methods (map/filter/find/findIndex/some/every/reduce/flatMap/forEach/sort/slice/concat/indexOf/lastIndexOf/at/flat/reverse/includes/join), string methods (incl. match/matchAll/replace/split with regular expressions), Date/RegExp/Map/Set/URL/URLSearchParams, URI encoding helpers, Object/Math/JSON helpers, captured console.log/warn/error/dir/table, and Promise.all/allSettled/race/resolve/reject over arrays mixing promises and plain values for parallel tool calls (promise chaining with .then/.catch is not supported - use await with try/catch).";

/// Runtime error raised by the interpreter. Mirrors
/// `InterpreterRuntimeError` incl. the `.as(errorName)` brand mapping and the
/// default `"ExecutionFailure"` kind.
#[derive(Debug, Clone)]
pub struct InterpreterRuntimeError {
    pub message: String,
    pub node: Option<AstNode>,
    pub kind: DiagnosticKind,
    pub suggestions: Option<Vec<String>>,
    /// JS-visible error name (`"Error"` default; set via [`as_error`]).
    pub error_name: String,
}

// Manual impl: AstNode carries serde_json::Value (already Debug+Clone).
impl InterpreterRuntimeError {
    /// Mirrors `new InterpreterRuntimeError(message, node?, kind?, suggestions?)`.
    pub fn new(
        message: impl Into<String>,
        node: Option<AstNode>,
        kind: DiagnosticKind,
        suggestions: Option<Vec<String>>,
    ) -> Self {
        InterpreterRuntimeError {
            message: message.into(),
            node,
            kind,
            suggestions,
            error_name: "Error".to_string(),
        }
    }

    /// Execution-failure shorthand (default kind, no node).
    pub fn execution(message: impl Into<String>) -> Self {
        Self::new(message, None, DiagnosticKind::ExecutionFailure, None)
    }

    /// Mirrors `.as(errorName)`.
    pub fn as_error(mut self, error_name: impl Into<String>) -> Self {
        self.error_name = error_name.into();
        self
    }
}

impl std::fmt::Display for InterpreterRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for InterpreterRuntimeError {}

/// Mirrors `unsupportedSyntax(kind, node)`.
pub fn unsupported_syntax(kind: &str, node: AstNode) -> InterpreterRuntimeError {
    InterpreterRuntimeError::new(
        format!(
            "Syntax '{}' is not supported in CodeMode. {}",
            kind, SUPPORTED_SYNTAX_MESSAGE
        ),
        Some(node),
        DiagnosticKind::UnsupportedSyntax,
        Some(vec![SUPPORTED_SYNTAX_MESSAGE.to_string()]),
    )
}

/// Mirrors `isRecord(value)`.
pub fn is_record(value: &Value) -> bool {
    value.is_object()
}

/// Mirrors `asNode(value, context)`.
pub fn as_node(value: &Value, context: &str) -> Result<AstNode, InterpreterRuntimeError> {
    match value {
        Value::Object(map) => match map.get("type").and_then(|t| t.as_str()) {
            Some(t) => Ok(AstNode {
                node_type: t.to_string(),
                loc: parse_loc(map.get("loc")),
                raw: value.clone(),
            }),
            None => Err(InterpreterRuntimeError::execution(format!(
                "Invalid AST node while reading {}.",
                context
            ))),
        },
        _ => Err(InterpreterRuntimeError::execution(format!(
            "Invalid AST node while reading {}.",
            context
        ))),
    }
}

fn parse_loc(loc: Option<&Value>) -> Option<SourceLocation> {
    let obj = loc?.as_object()?;
    let start = obj.get("start")?.as_object()?;
    let end = obj.get("end")?.as_object()?;
    Some(SourceLocation {
        start: SourcePosition {
            line: start.get("line")?.as_u64()?,
            column: start.get("column")?.as_u64()?,
        },
        end: SourcePosition {
            line: end.get("line")?.as_u64()?,
            column: end.get("column")?.as_u64()?,
        },
    })
}

/// Mirrors `getArray(node, key)`.
pub fn get_array(node: &AstNode, key: &str) -> Result<Vec<Value>, InterpreterRuntimeError> {
    match node.get(key) {
        Value::Array(items) => Ok(items.clone()),
        _ => Err(InterpreterRuntimeError::new(
            format!("Expected '{}' to be an array.", key),
            Some(node.clone()),
            DiagnosticKind::ExecutionFailure,
            None,
        )),
    }
}

/// Mirrors `getString(node, key)`.
pub fn get_string(node: &AstNode, key: &str) -> Result<String, InterpreterRuntimeError> {
    match node.get(key) {
        Value::String(s) => Ok(s.clone()),
        _ => Err(InterpreterRuntimeError::new(
            format!("Expected '{}' to be a string.", key),
            Some(node.clone()),
            DiagnosticKind::ExecutionFailure,
            None,
        )),
    }
}

/// Mirrors `getBoolean(node, key)`.
pub fn get_boolean(node: &AstNode, key: &str) -> Result<bool, InterpreterRuntimeError> {
    match node.get(key) {
        Value::Bool(b) => Ok(*b),
        _ => Err(InterpreterRuntimeError::new(
            format!("Expected '{}' to be a boolean.", key),
            Some(node.clone()),
            DiagnosticKind::ExecutionFailure,
            None,
        )),
    }
}

/// Mirrors `getOptionalNode(node, key)`.
pub fn get_optional_node(
    node: &AstNode,
    key: &str,
) -> Result<Option<AstNode>, InterpreterRuntimeError> {
    match node.get(key) {
        Value::Null => Ok(None),
        v if v.is_null() => Ok(None),
        // serde_json has no Undefined; missing keys surface as Null above.
        value => as_node(value, key).map(Some),
    }
}

/// Mirrors `getNode(node, key)`.
pub fn get_node(node: &AstNode, key: &str) -> Result<AstNode, InterpreterRuntimeError> {
    as_node(node.get(key), key)
}

/// Mirrors `sourceLocation(node)`: Acorn 0-based `loc` shifted to the
/// 1-based diagnostic position (`line - 1`, `column - 3` with floor 1).
pub fn source_location(node: &AstNode) -> SourcePosition {
    let line = node.loc.map(|l| l.start.line).unwrap_or(2);
    let column = node.loc.map(|l| l.start.column).unwrap_or(4);
    SourcePosition {
        line: line.saturating_sub(1).max(1),
        column: column.saturating_sub(3).max(1),
    }
}

/// Mirrors `formatLocation(node?)`.
pub fn format_location(node: Option<&AstNode>) -> String {
    match node.and_then(|n| n.loc.as_ref()) {
        None => String::new(),
        Some(_) => {
            let location = source_location(node.expect("loc checked"));
            format!(" (line {}, col {})", location.line, location.column)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn diagnostic_kind_spellings_verbatim() {
        assert_eq!(DiagnosticKind::ParseError.as_str(), "ParseError");
        assert_eq!(
            DiagnosticKind::UnsupportedSyntax.as_str(),
            "UnsupportedSyntax"
        );
        assert_eq!(DiagnosticKind::UnknownTool.as_str(), "UnknownTool");
        assert_eq!(
            DiagnosticKind::InvalidToolInput.as_str(),
            "InvalidToolInput"
        );
        assert_eq!(
            DiagnosticKind::InvalidToolOutput.as_str(),
            "InvalidToolOutput"
        );
        assert_eq!(
            DiagnosticKind::InvalidDataValue.as_str(),
            "InvalidDataValue"
        );
        assert_eq!(
            DiagnosticKind::ToolCallLimitExceeded.as_str(),
            "ToolCallLimitExceeded"
        );
        assert_eq!(DiagnosticKind::TimeoutExceeded.as_str(), "TimeoutExceeded");
        assert_eq!(DiagnosticKind::ToolFailure.as_str(), "ToolFailure");
        assert_eq!(
            DiagnosticKind::ExecutionFailure.as_str(),
            "ExecutionFailure"
        );
    }

    #[test]
    fn node_helpers_read_typed_fields() {
        let node = as_node(&json!({"type": "Program", "body": []}), "root").unwrap();
        assert_eq!(node.node_type, "Program");
        assert!(get_array(&node, "body").is_ok());
        assert!(get_string(&node, "body").is_err());
    }

    #[test]
    fn source_location_applies_acorn_offset() {
        let node = as_node(
            &json!({"type": "X", "loc": {"start": {"line": 2, "column": 4}, "end": {"line": 2, "column": 5}}}),
            "x",
        )
        .unwrap();
        let loc = source_location(&node);
        assert_eq!((loc.line, loc.column), (1, 1));
    }
}
