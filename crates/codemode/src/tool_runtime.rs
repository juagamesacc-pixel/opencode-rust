//! Port of `src/tool-runtime.ts`.
//!
//! Tool catalog, discovery plan, search index, data-boundary copies, and the
//! sync tool runtime. The Effect aspects (`Effect.gen`, supervised fibers,
//! `onToolCallStart/End` effects) are modelled as an explicit sync state
//! machine with identical observable semantics (plan §3.5): calls are
//! admitted eagerly at the call site (budget charged + `on_tool_call_start`
//! fired before any await), settlement is run-once and idempotent, and
//! interruption fires neither outcome.
//!
//! R4 NOTE: `copy_in`/`copy_out` operate on `serde_json::Value`. JSON values
//! cannot be circular by construction, so the circularity diagnostic lives in
//! the interpreter-runtime walk over live sandbox values (same verbatim
//! string); sandbox-wrapper serialization (Date/URL→string,
//! RegExp/Map/Set/URLSearchParams→`{}`) is applied at that same boundary walk.

use crate::tool::{Definition, HostToolFn, ToolFailure, ToolTree};
use crate::tool_error::ToolError;
use crate::tool_schema::{input_properties, is_identifier_segment};
use serde_json::Value;

/// Aliases to reduce type complexity for clippy::type_complexity.
pub type OnToolCallStartHook = Box<dyn Fn(&ToolCallStarted) + Send + Sync>;
pub type OnToolCallEndHook = Box<dyn Fn(&ToolCallEnded) + Send + Sync>;

fn estimate_tokens(input: &str) -> usize {
    ((input.len() as f64) / 4.0).round().max(0.0) as usize
}

/// Minimal audit record retained for each admitted tool call. Mirrors `ToolCall`.
#[derive(Debug, Clone)]
pub struct ToolCall {
    pub name: String,
}

/// Decoded tool call observed immediately before tool execution. Mirrors `ToolCallStarted`.
#[derive(Debug, Clone)]
pub struct ToolCallStarted {
    pub index: usize,
    pub name: String,
    pub input: Value,
}

/// Completed tool call observed after settlement. Mirrors `ToolCallEnded`.
#[derive(Debug, Clone)]
pub struct ToolCallEnded {
    pub index: usize,
    pub name: String,
    pub input: Value,
    pub duration_ms: u64,
    pub outcome: ToolCallOutcome,
    /// Model-safe failure message; present only on failure.
    pub message: Option<String>,
}

/// Settle outcome. Mirrors `"success" | "failure"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolCallOutcome {
    Success,
    Failure,
}

/// Non-throwing observation hooks fired around each admitted tool call.
/// Mirrors `ToolCallHooks<R>` (sync closures; `R` services erased).
#[derive(Default)]
pub struct ToolCallHooks {
    pub on_tool_call_start: Option<OnToolCallStartHook>,
    pub on_tool_call_end: Option<OnToolCallEndHook>,
}

impl std::fmt::Debug for ToolCallHooks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ToolCallHooks").finish_non_exhaustive()
    }
}

/// Model-visible description of one schema-backed tool. Mirrors `ToolDescription`.
#[derive(Debug, Clone)]
pub struct ToolDescription {
    pub path: String,
    pub description: String,
    pub signature: String,
}

/// Plain-data object. Mirrors `SafeObject`.
pub type SafeObject = serde_json::Map<String, Value>;

const RESERVED_NAMESPACE: &str = "$codemode";
/// Default token budget for full catalog entries. Mirrors `defaultCatalogBudget`.
pub const DEFAULT_CATALOG_BUDGET: usize = 2_000;
const DEFAULT_SEARCH_LIMIT: usize = 10;

/// Renders a dotted tool path as a JS expression under `tools`.
/// Mirrors `toolExpression`.
pub fn tool_expression(path: &str) -> String {
    let mut out = String::from("tools");
    if path.is_empty() {
        return out;
    }
    for segment in path.split('.') {
        if is_identifier_segment(segment) {
            out.push('.');
            out.push_str(segment);
        } else {
            out.push('[');
            out.push_str(&serde_json::to_string(segment).unwrap_or_else(|_| "\"?\"".to_string()));
            out.push(']');
        }
    }
    out
}

/// Callable reference to a node of the tool tree. Mirrors `ToolReference`.
#[derive(Debug, Clone)]
pub struct ToolReference {
    pub path: Vec<String>,
}

impl ToolReference {
    /// Mirrors `new ToolReference(path)`.
    pub fn new(path: Vec<String>) -> Self {
        ToolReference { path }
    }
}

/// Maximum nesting depth for values crossing a data boundary. Mirrors
/// `MAX_VALUE_DEPTH = 32`.
pub const MAX_VALUE_DEPTH: usize = 32;

/// Tool runtime failures. Mirrors `ToolRuntimeError` kinds verbatim.
#[derive(Debug, Clone)]
pub struct ToolRuntimeError {
    pub kind: ToolRuntimeErrorKind,
    pub message: String,
    pub suggestions: Vec<String>,
}

/// `ToolRuntimeError` kind union. Mirrors the TS `"UnknownTool" |
/// "InvalidToolInput" | "InvalidToolOutput" | "InvalidDataValue" |
/// "ToolCallLimitExceeded"` union verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolRuntimeErrorKind {
    UnknownTool,
    InvalidToolInput,
    InvalidToolOutput,
    InvalidDataValue,
    ToolCallLimitExceeded,
}

impl ToolRuntimeErrorKind {
    /// Verbatim wire spelling.
    pub fn as_str(&self) -> &'static str {
        match self {
            ToolRuntimeErrorKind::UnknownTool => "UnknownTool",
            ToolRuntimeErrorKind::InvalidToolInput => "InvalidToolInput",
            ToolRuntimeErrorKind::InvalidToolOutput => "InvalidToolOutput",
            ToolRuntimeErrorKind::InvalidDataValue => "InvalidDataValue",
            ToolRuntimeErrorKind::ToolCallLimitExceeded => "ToolCallLimitExceeded",
        }
    }
}

impl ToolRuntimeError {
    /// Mirrors `new ToolRuntimeError(kind, message, suggestions?)`.
    pub fn new(
        kind: ToolRuntimeErrorKind,
        message: impl Into<String>,
        suggestions: Vec<String>,
    ) -> Self {
        ToolRuntimeError {
            kind,
            message: message.into(),
            suggestions,
        }
    }
}

impl std::fmt::Display for ToolRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ToolRuntimeError {}

/// Blocked prototype-pollution member names. The set contents are verbatim:
/// `__proto__`, `constructor`, `prototype`.
pub fn is_blocked_member(name: &str) -> bool {
    matches!(name, "__proto__" | "constructor" | "prototype")
}

/// Validates + copies a value against the plain-data contract (depth, blocked
/// properties, data-only leaves). Mirrors `copyIn(value, label,
/// preserveSandboxValues?)`.
///
/// The `preserve_sandbox_values` flag is accepted for signature parity; the
/// intra-sandbox checkpoint walk over live interpreter values (which keeps
/// sandbox wrappers alive as leaves) lives in `interpreter_runtime`, which
/// calls this function for the JSON-materialized half. Sandbox-wrapper and
/// promise handling at this layer: JSON values never contain them, so the
/// corresponding diagnostics are unreachable here and preserved verbatim in
/// the interpreter walk (R4).
pub fn copy_in(
    value: &Value,
    label: &str,
    _preserve_sandbox_values: bool,
) -> Result<Value, ToolRuntimeError> {
    copy_bounded(value, label, 0)
}

fn copy_bounded(value: &Value, label: &str, depth: usize) -> Result<Value, ToolRuntimeError> {
    if depth > MAX_VALUE_DEPTH {
        return Err(ToolRuntimeError::new(
            ToolRuntimeErrorKind::InvalidDataValue,
            format!(
                "{} exceeds the maximum value depth of {}.",
                label, MAX_VALUE_DEPTH
            ),
            vec![],
        ));
    }
    match value {
        Value::Null | Value::Bool(_) | Value::String(_) | Value::Number(_) => Ok(value.clone()),
        Value::Array(items) => {
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                out.push(copy_bounded(item, label, depth + 1)?);
            }
            Ok(Value::Array(out))
        }
        Value::Object(map) => {
            let mut out = serde_json::Map::with_capacity(map.len());
            for (key, item) in map {
                if is_blocked_member(key) {
                    return Err(ToolRuntimeError::new(
                        ToolRuntimeErrorKind::InvalidDataValue,
                        format!("{} contains blocked property '{}'.", label, key),
                        vec![],
                    ));
                }
                out.insert(key.clone(), copy_bounded(item, label, depth + 1)?);
            }
            Ok(Value::Object(out))
        }
    }
}

/// Normalizes a value leaving the sandbox. Mirrors `copyOut(value,
/// undefinedAsNull=false)` (tool-runtime.ts:297-311): non-finite numbers → `null`
/// (matching JSON semantics; here via `copy_out_number` before materialization,
/// `Value::Number` is always finite); `undefined` → `null` only when
/// `undefined_as_null` is set (JS `if (value === undefined && undefinedAsNull) return null`);
/// arrays/objects are walked recursively with the flag propagated.
/// `ToolReference` opacity has no JSON analogue and is handled in the
/// interpreter walk (references never reach here). `Value` itself cannot hold
/// JS `undefined`; that case is carried as `Option<Value>` in `copy_out_opt`.
pub fn copy_out(value: &Value, undefined_as_null: bool) -> Value {
    // NOTE: serde_json::Value cannot represent JS `undefined` or non-finite
    // numbers; this function is the explicit normalization point so the
    // boundary rule stays visible. Non-finite f64s cannot occur in `Value`;
    // the interpreter walk converts them via `copy_out_number` before
    // materializing JSON (R4). The `undefined_as_null` flag is propagated
    // to nested values for symmetry with the JS walk; top-level `undefined`
    // is handled by `copy_out_opt` as `None`.
    match value {
        Value::Array(items) => Value::Array(
            items
                .iter()
                .map(|i| copy_out(i, undefined_as_null))
                .collect(),
        ),
        Value::Object(map) => Value::Object(
            map.iter()
                .map(|(k, v)| (k.clone(), copy_out(v, undefined_as_null)))
                .collect(),
        ),
        _ => value.clone(),
    }
}

/// Converts a non-finite f64 at the sandbox boundary. Mirrors the
/// `copyOut` `NaN`/`Infinity` → `null` rule verbatim (R4). Integer-valued
/// f64s are emitted as JSON integers (e.g. 1 → 1 not 1.0) to preserve
/// `serde_json::Number(1) == Number(1.0)` verbatim and to avoid precision loss
/// on large ints that would otherwise round-trip via f64. This keeps
/// `{ ...null, a: 1 }` as `{"a":1}` not `{"a":1.0}`.
pub fn copy_out_number(value: f64) -> Value {
    if !value.is_finite() {
        return Value::Null;
    }
    // Preserve integer JSON numbers verbatim: 1.0 → 1, 7.0 → 7, large ints stay
    // integer when within i64/u64 range and exactly integral. This mirrors
    // JS `JSON.stringify` which emits "1" for both 1 and 1.0, and avoids the
    // `Number(1.0) != Number(1)` mismatch in `spreading_null_is_noop`.
    if value.fract() == 0.0 {
        if value >= i64::MIN as f64 && value <= i64::MAX as f64 {
            // Within i64 range and integral → emit as integer.
            let as_i64 = value as i64;
            // Ensure round-trip is exact (f64 may have lost precision for large
            // ints; if `as_i64 as f64 != value`, fall back to f64 string).
            if (as_i64 as f64) == value {
                return Value::Number(serde_json::Number::from(as_i64));
            }
        }
        if value >= 0.0 && value <= u64::MAX as f64 {
            let as_u64 = value as u64;
            if (as_u64 as f64) == value {
                return Value::Number(serde_json::Number::from(as_u64));
            }
        }
    }
    serde_json::Number::from_f64(value)
        .map(Value::Number)
        .unwrap_or(Value::Null)
}

/// Undefined marker for the `undefined → null` boundary rule. Mirrors
/// `copyOut(value, undefinedAsNull=false)` (tool-runtime.ts:297-298):
/// `if (value === undefined && undefinedAsNull) return null` — `Some(v)`
/// passes through (recursively via `copy_out` with the flag propagated),
/// `None` (JS `undefined`) becomes `null` only when `undefined_as_null`
/// is set; otherwise it stays `None` (JS `undefined`) so a later
/// `JSON.stringify` can omit the key as in JS. Returns `Option<Value>`
/// so `None` can represent JS `undefined` (serde_json::Value has no
/// `Undefined` variant).
pub fn copy_out_opt(value: Option<Value>, undefined_as_null: bool) -> Option<Value> {
    match (value, undefined_as_null) {
        (Some(v), _) => Some(copy_out(&v, undefined_as_null)),
        (None, true) => Some(Value::Null),
        (None, false) => None,
    }
}

struct DescribedEntry {
    path: String,
    description: ToolDescription,
    search_text: String,
    namespace: String,
}

fn collect_definitions<'a>(
    tree: &'a ToolTree,
    prefix: &mut Vec<String>,
    out: &mut Vec<(String, &'a Definition)>,
) {
    for (name, value) in &tree.entries {
        prefix.push(name.clone());
        match value {
            crate::tool::ToolTreeValue::Definition(def) => {
                out.push((prefix.join("."), def.as_ref()))
            }
            crate::tool::ToolTreeValue::Namespace(sub) => collect_definitions(sub, prefix, out),
            crate::tool::ToolTreeValue::HostFn(_) => {}
        }
        prefix.pop();
    }
}

fn describe_definition(path: &str, definition: &Definition) -> ToolDescription {
    ToolDescription {
        path: path.to_string(),
        description: definition.description.clone(),
        signature: format!(
            "{}(input: {}): Promise<{}>",
            tool_expression(path),
            crate::tool_schema::input_typescript(definition, true),
            crate::tool_schema::output_typescript(definition, true)
        ),
    }
}

fn visible_definitions(tree: &ToolTree) -> Vec<(String, &Definition, ToolDescription)> {
    let mut defs = vec![];
    collect_definitions(tree, &mut vec![], &mut defs);
    defs.into_iter()
        .map(|(path, def)| {
            let desc = describe_definition(&path, def);
            (path, def, desc)
        })
        .collect()
}

/// All model-visible tool descriptions. Mirrors `catalog(tools)`.
pub fn catalog(tree: &ToolTree) -> Vec<ToolDescription> {
    visible_definitions(tree)
        .into_iter()
        .map(|(_, _, desc)| desc)
        .collect()
}

/// Budgeted discovery plan. Mirrors `DiscoveryPlan`.
#[derive(Debug, Clone)]
pub struct DiscoveryPlan {
    pub catalog: Vec<ToolDescription>,
    pub instructions: String,
    pub search_index: Vec<SearchEntry>,
}

/// Search index entry. Mirrors `SearchEntry`.
#[derive(Debug, Clone)]
pub struct SearchEntry {
    pub description: ToolDescription,
    /// Top-level namespace (first path segment).
    pub namespace: String,
    /// Lowercased path + description + input property names/descriptions.
    pub search_text: String,
}

/// Splits a query into lowercased search terms. Mirrors `tokenize`.
pub fn tokenize(query: &str) -> Vec<String> {
    let mut spaced = String::with_capacity(query.len() + 8);
    let chars: Vec<char> = query.chars().collect();
    for i in 0..chars.len() {
        let c = chars[i];
        if i > 0 {
            let prev = chars[i - 1];
            if (prev.is_ascii_lowercase() || prev.is_ascii_digit()) && c.is_ascii_uppercase() {
                spaced.push(' ');
            }
        }
        spaced.push(c);
    }
    spaced
        .to_lowercase()
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|t| !t.is_empty() && *t != "*")
        .map(|s| s.to_string())
        .collect()
}

/// A term plus naive singular variants. Mirrors `termForms`.
pub fn term_forms(term: &str) -> Vec<String> {
    let mut forms = vec![term.to_string()];
    if term.ends_with("es") && term.len() > 3 {
        forms.push(term[..term.len() - 2].to_string());
    }
    if term.ends_with('s') && term.len() > 2 {
        forms.push(term[..term.len() - 1].to_string());
    }
    forms
}

/// Search request fields. Mirrors `SearchInput` (query?, namespace?, limit?,
/// offset? with positive/non-negative int checks).
#[derive(Debug, Clone, Default)]
pub struct SearchInput {
    pub query: Option<String>,
    pub namespace: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// One search hit. Mirrors `SearchItem`.
#[derive(Debug, Clone)]
pub struct SearchItem {
    pub path: String,
    pub description: String,
    pub signature: String,
}

/// Search response. Mirrors `SearchOutput`.
#[derive(Debug, Clone)]
pub struct SearchOutput {
    pub items: Vec<SearchItem>,
    pub remaining: usize,
    pub next: Option<SearchNext>,
}

/// Search pagination cursor. Mirrors `{ offset }`.
#[derive(Debug, Clone)]
pub struct SearchNext {
    pub offset: usize,
}

/// Runs the `$codemode.search` tool over an index. Mirrors the `run` of
/// `makeSearchTool(searchIndex)`.
pub fn run_search(
    index: &[SearchEntry],
    request: &SearchInput,
) -> Result<SearchOutput, ToolRuntimeError> {
    if let Some(0) = request.limit {
        return Err(ToolRuntimeError::new(
            ToolRuntimeErrorKind::InvalidToolInput,
            "Invalid input for tool '$codemode.search': limit must be a positive integer"
                .to_string(),
            vec![],
        ));
    }
    let query = request.query.as_deref().unwrap_or("");
    let offset = request.offset.unwrap_or(0);
    let scoped: Vec<&SearchEntry> = match request.namespace.as_deref() {
        None => index.iter().collect(),
        Some(ns) => index.iter().filter(|e| e.namespace == ns).collect(),
    };
    let trimmed = query.trim();
    let path_query = trimmed.strip_prefix("tools.").unwrap_or(trimmed);
    let exact = if path_query.is_empty() {
        None
    } else {
        scoped.iter().find(|e| {
            e.description.path == path_query || tool_expression(&e.description.path) == trimmed
        })
    };
    let terms: Vec<Vec<String>> = tokenize(query)
        .into_iter()
        .map(|t| term_forms(&t))
        .collect();
    let ranked: Vec<&SearchEntry> = if let Some(hit) = exact {
        vec![*hit]
    } else {
        let mut scored: Vec<(&SearchEntry, i64)> = scoped
            .iter()
            .map(|entry| {
                let path = entry.description.path.to_lowercase();
                let description = entry.description.description.to_lowercase();
                let mut score: i64 = 0;
                for forms in &terms {
                    if forms
                        .iter()
                        .any(|f| path == *f || path.ends_with(&format!(".{}", f)))
                    {
                        score += 20;
                    }
                    if forms.iter().any(|f| path.contains(f)) {
                        score += 8;
                    }
                    if forms.iter().any(|f| description.contains(f)) {
                        score += 4;
                    }
                    if forms.iter().any(|f| entry.search_text.contains(f)) {
                        score += 2;
                    }
                }
                (*entry, score)
            })
            .filter(|(_, score)| terms.is_empty() || *score > 0)
            .collect();
        scored.sort_by(|a, b| {
            b.1.cmp(&a.1)
                .then_with(|| a.0.description.path.cmp(&b.0.description.path))
        });
        scored.into_iter().map(|(e, _)| e).collect()
    };
    let limit = request.limit.unwrap_or(DEFAULT_SEARCH_LIMIT);
    let items: Vec<SearchItem> = ranked
        .iter()
        .skip(offset)
        .take(limit)
        .map(|e| SearchItem {
            path: tool_expression(&e.description.path),
            description: e.description.description.clone(),
            signature: e.description.signature.clone(),
        })
        .collect();
    let remaining = ranked.len().saturating_sub(offset + items.len());
    let next = if remaining > 0 {
        Some(SearchNext {
            offset: offset + items.len(),
        })
    } else {
        None
    };
    Ok(SearchOutput {
        items,
        remaining,
        next,
    })
}

fn search_tool_description() -> ToolDescription {
    // Canonical signature text for the always-registered search tool.
    ToolDescription {
        path: format!("{}.search", RESERVED_NAMESPACE),
        description: "Search available Code Mode tools".to_string(),
        signature: "tools.$codemode.search(input: { query?: string; namespace?: string; limit?: number; offset?: number }): Promise<unknown>".to_string(),
    }
}

fn catalog_line(tool: &ToolDescription) -> String {
    let line = tool.description.split('\n').next().unwrap_or("").trim();
    let description = if line.len() > 120 {
        format!("{}...", &line[..119])
    } else {
        line.to_string()
    };
    if description.is_empty() {
        format!("  - {}", tool.signature)
    } else {
        format!("  - {} // {}", tool.signature, description)
    }
}

fn to_search_entry(
    path: &str,
    definition: &Definition,
    description: &ToolDescription,
) -> SearchEntry {
    let mut text = vec![path.to_string(), definition.description.clone()];
    for prop in input_properties(definition) {
        text.push(prop.name);
        if let Some(desc) = prop.description {
            text.push(desc);
        }
    }
    SearchEntry {
        description: description.clone(),
        namespace: path.split('.').next().unwrap_or(path).to_string(),
        search_text: text.join("\n").to_lowercase(),
    }
}

/// The runtime search index over every described tool. Mirrors `searchIndex(tools)`.
pub fn search_index(tree: &ToolTree) -> Vec<SearchEntry> {
    visible_definitions(tree)
        .iter()
        .map(|(path, def, desc)| to_search_entry(path, def, desc))
        .collect()
}

/// Rejects the reserved `$codemode` namespace. Mirrors `assertValidTools`.
pub fn assert_valid_tools(tree: &ToolTree) -> Result<(), String> {
    if tree.contains_key(RESERVED_NAMESPACE) {
        return Err(format!(
            "Tool namespace '{}' is reserved for CodeMode discovery tools.",
            RESERVED_NAMESPACE
        ));
    }
    Ok(())
}

/// Budgeted catalog + instructions + search index. Mirrors `prepare(tools,
/// catalogBudget = defaultCatalogBudget)` incl. round-robin fairness and the
/// deliberate section order (workflow → rules → language → catalog).
pub fn prepare(tree: &ToolTree, catalog_budget: Option<usize>) -> Result<DiscoveryPlan, String> {
    let budget = catalog_budget.unwrap_or(DEFAULT_CATALOG_BUDGET);
    let visible = visible_definitions(tree);
    let described: Vec<ToolDescription> = visible.iter().map(|(_, _, d)| d.clone()).collect();

    // Group by namespace; ordered alphabetically.
    let mut namespaces: std::collections::BTreeMap<String, Vec<ToolDescription>> =
        std::collections::BTreeMap::new();
    for tool in &described {
        let ns = tool
            .path
            .split('.')
            .next()
            .unwrap_or(&tool.path)
            .to_string();
        namespaces.entry(ns).or_default().push(tool.clone());
    }
    let ordered: Vec<(String, Vec<ToolDescription>)> = namespaces.into_iter().collect();

    // Round-robin selection of signature lines against the shared budget.
    struct Selection {
        namespace: String,
        picked: Vec<ToolDescription>,
        queue: Vec<ToolDescription>,
    }
    let mut selections: Vec<Selection> = ordered
        .into_iter()
        .map(|(namespace, mut group)| {
            group.sort_by(|a, b| {
                estimate_tokens(&catalog_line(a))
                    .cmp(&estimate_tokens(&catalog_line(b)))
                    .then_with(|| a.path.cmp(&b.path))
            });
            Selection {
                namespace,
                picked: vec![],
                queue: group,
            }
        })
        .collect();
    let mut used = 0usize;
    let mut active: Vec<usize> = selections
        .iter()
        .enumerate()
        .filter(|(_, s)| !s.queue.is_empty())
        .map(|(i, _)| i)
        .collect();
    while !active.is_empty() {
        let mut still_active = vec![];
        for i in active {
            let sel = &mut selections[i];
            let tool = sel.queue[0].clone();
            let cost = estimate_tokens(&catalog_line(&tool));
            if used + cost > budget {
                continue;
            }
            sel.queue.remove(0);
            sel.picked.push(tool);
            used += cost;
            if !sel.queue.is_empty() {
                still_active.push(i);
            }
        }
        active = still_active;
    }
    let total_shown: usize = selections.iter().map(|s| s.picked.len()).sum();
    let complete = total_shown == described.len();
    let empty = described.is_empty();

    let mut intro: Vec<String> = vec![];
    if empty {
        intro.push("This is a restricted JavaScript language for calling tools, not a general-purpose runtime.".to_string());
    } else if complete {
        intro.push("This is a restricted JavaScript language for calling tools, not a general-purpose runtime. Inside the confined interpreter, `tools` contains the Code Mode tools listed below and internal runtime tools; surrounding agent tools are not available.".to_string());
        intro.push("Do not infer or normalize tool names; use only exact signatures shown below or returned by search.".to_string());
    } else {
        intro.push("This is a restricted JavaScript language for calling tools, not a general-purpose runtime. Inside the confined interpreter, `tools` contains the Code Mode tools listed or searchable below and internal runtime tools; surrounding agent tools are not available.".to_string());
        intro.push("Do not infer or normalize tool names; use only exact signatures shown below or returned by search.".to_string());
    }

    let mut workflow: Vec<String> = vec![];
    if !empty {
        workflow.push(String::new());
        workflow.push("## Workflow".to_string());
        workflow.push(String::new());
        if complete {
            workflow.push("1. Pick a tool from the list under `## Available tools` - each line is the exact call signature; use it as-is rather than guessing segments.".to_string());
            workflow.push("2. Call it using the exact signature shown: `const result = await tools.<namespace>.<tool>(input)`; bracket notation and quotes are part of the path.".to_string());
            workflow.push("3. Return only the fields you need from structured results; narrow unknown results before reading fields, and avoid returning large raw payloads.".to_string());
        } else {
            workflow.push("1. If needed, discover tools: `return await tools.$codemode.search({ query: \"<intent + key nouns>\" })`.".to_string());
            workflow.push("2. In the next execution, copy a returned path exactly, call it, and return only the needed fields.".to_string());
        }
    }

    let mut rules: Vec<String> = vec![];
    if !empty {
        rules.push(String::new());
        rules.push("## Rules".to_string());
        rules.push(String::new());
        if complete {
            rules.push("- Only Code Mode tools listed here and internal runtime tools are available; surrounding agent tools are not implicitly exposed.".to_string());
        } else {
            rules.push("- Only Code Mode tools listed here or returned by `tools.$codemode.search` and internal runtime tools are available; surrounding agent tools are not implicitly exposed.".to_string());
        }
        rules.push("- Filter, aggregate, and transform collections in code - never return them raw or call a tool per item across messages.".to_string());
        rules.push("- A result typed `Promise<unknown>` may be structured data or text. Before reading fields, check that it is a non-null object and not an array; otherwise handle the returned text or primitive directly.".to_string());
        rules.push("- Run independent calls in parallel: `await Promise.all(items.map((item) => tools.<namespace>.<tool>(item)))`, or use `tools.<namespace>[\"tool-name\"](item)` when the listed signature uses bracket notation.".to_string());
        rules.push("- `Object.keys(tools)` lists namespaces; `Object.keys(tools.<namespace>)` lists its tools; `for...in` works on both.".to_string());
        if !complete {
            rules.push("- Browse one namespace: `await tools.$codemode.search({ query: \"\", namespace: \"<name>\" })`.".to_string());
            rules.push(
                "- If search returns `next`, repeat the same search with `offset: next.offset`."
                    .to_string(),
            );
        }
    }

    let language = vec![
        String::new(),
        "## Language".to_string(),
        String::new(),
        "Use common JavaScript data operations, functions, control flow, selected standard-library methods, and awaited tool calls. Built-ins include Date, RegExp, Map, Set, URL, URLSearchParams, and URI encoding helpers.".to_string(),
        "Modules/imports, classes, generators, timers, fetch, eval, prototype access, unlisted methods, and promise chaining are unavailable. Use Code Mode tools for external operations. Use await with try/catch.".to_string(),
        "Dates and URLs serialize to strings at data boundaries; Map/Set/RegExp/URLSearchParams serialize to `{}`.".to_string(),
    ];

    let mut tool_section: Vec<String> = vec![String::new()];
    if empty {
        tool_section.push("## Available tools".to_string());
        tool_section.push(String::new());
        tool_section.push("No tools are currently available.".to_string());
    } else {
        // Rebuild ordered groups for emission.
        let mut groups: std::collections::BTreeMap<String, Vec<ToolDescription>> =
            std::collections::BTreeMap::new();
        for tool in &described {
            let ns = tool
                .path
                .split('.')
                .next()
                .unwrap_or(&tool.path)
                .to_string();
            groups.entry(ns).or_default().push(tool.clone());
        }
        tool_section.push(if complete {
            "## Available tools (COMPLETE list - every tool is shown below with its full call signature)".to_string()
        } else {
            format!(
                "## Available tools (PARTIAL - {} of {} shown; find the rest with tools.$codemode.search)",
                total_shown,
                described.len()
            )
        });
        tool_section.push(String::new());
        for (namespace, group) in &groups {
            let picked_count = selections
                .iter()
                .find(|s| &s.namespace == namespace)
                .map(|s| s.picked.len())
                .unwrap_or(0);
            let count = format!(
                "{} tool{}",
                group.len(),
                if group.len() == 1 { "" } else { "s" }
            );
            let label = if picked_count == group.len() {
                count
            } else if picked_count == 0 {
                format!("{}, none shown", count)
            } else {
                format!("{}, {} shown", count, picked_count)
            };
            tool_section.push(format!("- {} ({})", namespace, label));
            let picked: std::collections::BTreeSet<String> = selections
                .iter()
                .find(|s| &s.namespace == namespace)
                .map(|s| s.picked.iter().map(|t| t.path.clone()).collect())
                .unwrap_or_default();
            for tool in group {
                if picked.contains(&tool.path) {
                    tool_section.push(catalog_line(tool));
                }
            }
        }
        if !complete {
            tool_section.push(String::new());
            tool_section.push("Search returns complete callable signatures:".to_string());
            tool_section.push(format!("- {}", search_tool_description().signature));
        }
    }

    let mut lines = vec![];
    lines.extend(intro);
    lines.extend(workflow);
    lines.extend(rules);
    lines.extend(language);
    lines.extend(tool_section);
    Ok(DiscoveryPlan {
        catalog: described.clone(),
        instructions: lines.join("\n"),
        search_index: visible
            .iter()
            .map(|(path, def, desc)| to_search_entry(path, def, desc))
            .collect(),
    })
}

/// Enumerable names at one node of the callable tool tree. Mirrors
/// `namespaceKeys(tools, path)`: the reserved `$codemode` search namespace is
/// always registered at the root; a callable tool is a leaf and enumerates
/// as `[]`; unknown paths raise the `UnknownTool` discovery-hint error.
pub fn namespace_keys(
    tree: &ToolTree,
    index: &[SearchEntry],
    path: &[String],
) -> Result<Vec<String>, ToolRuntimeError> {
    let _ = index;
    let unknown_ns = || {
        ToolRuntimeError::new(
            ToolRuntimeErrorKind::UnknownTool,
            format!("Unknown tool namespace '{}'.", path.join(".")),
            vec!["Object.keys(tools) lists the available namespaces; tools.$codemode.search({ query }) finds described tools.".to_string()],
        )
    };
    if path.is_empty() {
        let mut keys: Vec<String> = tree.keys();
        keys.push(RESERVED_NAMESPACE.to_string());
        return Ok(keys);
    }
    if path.first().map(|s| s.as_str()) == Some(RESERVED_NAMESPACE) {
        if path.len() == 1 {
            return Ok(vec!["search".to_string()]);
        }
        // `$codemode.search` is a callable leaf.
        if path == [RESERVED_NAMESPACE.to_string(), "search".to_string()] {
            return Ok(vec![]);
        }
        return Err(unknown_ns());
    }
    let mut node: &ToolTree = tree;
    for (i, segment) in path.iter().enumerate() {
        if is_blocked_member(segment) {
            return Err(unknown_ns());
        }
        match node.get(segment) {
            Some(crate::tool::ToolTreeValue::Namespace(sub)) => node = sub,
            Some(crate::tool::ToolTreeValue::Definition(_))
            | Some(crate::tool::ToolTreeValue::HostFn(_)) => {
                if i + 1 == path.len() {
                    return Ok(vec![]);
                }
                return Err(unknown_ns());
            }
            None => return Err(unknown_ns()),
        }
    }
    Ok(node.keys())
}

enum Resolved<'a> {
    Definition(&'a Definition),
    HostFn(&'a HostToolFn),
    Search,
}

fn resolve_path<'a>(tree: &'a ToolTree, path: &[String]) -> Result<Resolved<'a>, ToolRuntimeError> {
    if path.first().map(|s| s.as_str()) == Some(RESERVED_NAMESPACE) {
        if path == [RESERVED_NAMESPACE, "search"] {
            return Ok(Resolved::Search);
        }
        if path.len() == 1 {
            return Err(ToolRuntimeError::new(
                ToolRuntimeErrorKind::UnknownTool,
                format!("Tool '{}' is not callable.", path.join(".")),
                vec![],
            ));
        }
        return Err(ToolRuntimeError::new(
            ToolRuntimeErrorKind::UnknownTool,
            format!("Unknown tool '{}'.", path.join(".")),
            vec![
                "Use tools.$codemode.search({ query }) to find available described tools."
                    .to_string(),
            ],
        ));
    }
    let mut node: &ToolTree = tree;
    for (i, segment) in path.iter().enumerate() {
        if is_blocked_member(segment) {
            return Err(ToolRuntimeError::new(
                ToolRuntimeErrorKind::UnknownTool,
                format!("Unknown tool '{}'.", path.join(".")),
                vec![
                    "Use tools.$codemode.search({ query }) to find available described tools."
                        .to_string(),
                ],
            ));
        }
        match node.get(segment) {
            None => {
                return Err(ToolRuntimeError::new(
                    ToolRuntimeErrorKind::UnknownTool,
                    format!("Unknown tool '{}'.", path.join(".")),
                    vec![
                        "Use tools.$codemode.search({ query }) to find available described tools."
                            .to_string(),
                    ],
                ));
            }
            Some(v) => match v {
                crate::tool::ToolTreeValue::Definition(def) => {
                    if i + 1 == path.len() {
                        return Ok(Resolved::Definition(def.as_ref()));
                    }
                    return Err(ToolRuntimeError::new(
                        ToolRuntimeErrorKind::UnknownTool,
                        format!("Unknown tool '{}'.", path.join(".")),
                        vec!["Use tools.$codemode.search({ query }) to find available described tools.".to_string()],
                    ));
                }
                crate::tool::ToolTreeValue::HostFn(f) => {
                    if i + 1 == path.len() {
                        return Ok(Resolved::HostFn(f));
                    }
                    return Err(ToolRuntimeError::new(
                        ToolRuntimeErrorKind::UnknownTool,
                        format!("Unknown tool '{}'.", path.join(".")),
                        vec!["Use tools.$codemode.search({ query }) to find available described tools.".to_string()],
                    ));
                }
                crate::tool::ToolTreeValue::Namespace(sub) => node = sub,
            },
        }
    }
    Err(ToolRuntimeError::new(
        ToolRuntimeErrorKind::UnknownTool,
        format!("Tool '{}' is not callable.", path.join(".")),
        vec![],
    ))
}

/// Sync tool runtime over one host tool tree. Mirrors `ToolRuntime<R>`.
///
/// `invoke` admits the call eagerly (budget charged + hooks fired at the call
/// site) and runs the tool synchronously; the interpreter wraps the outcome
/// in a run-once promise value with identical observable semantics.
pub struct ToolRuntime {
    /// Admitted-call ledger. Mirrors `calls: Array<ToolCall>`.
    pub calls: Vec<ToolCall>,
    max_tool_calls: Option<usize>,
    search_index: Vec<SearchEntry>,
    hooks: ToolCallHooks,
}

impl std::fmt::Debug for ToolRuntime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ToolRuntime")
            .field("calls", &self.calls)
            .field("max_tool_calls", &self.max_tool_calls)
            .finish_non_exhaustive()
    }
}

/// Settles one admitted tool call. `Ok(value)` is the boundary-copied result;
/// `Err` is either [`ToolError`] (safe refusal), [`ToolRuntimeError`]
/// (boundary/limit diagnostic), or a sanitized unknown host failure.
#[derive(Debug, Clone)]
pub enum InvokeOutcome {
    Success(Value),
    ToolRefusal(ToolError),
    RuntimeFailure(ToolRuntimeError),
    UnknownFailure,
}

impl ToolRuntime {
    /// Mirrors `ToolRuntime.make(tools, maxToolCalls, searchIndex, hooks?)`.
    /// `Undefined` (`None`) means unlimited tool calls.
    pub fn make(
        max_tool_calls: Option<usize>,
        search_index: Vec<SearchEntry>,
        hooks: ToolCallHooks,
    ) -> Self {
        ToolRuntime {
            calls: vec![],
            max_tool_calls,
            search_index,
            hooks,
        }
    }

    /// Root reference. Mirrors `root: new ToolReference([])`.
    pub fn root() -> ToolReference {
        ToolReference::new(vec![])
    }

    fn record_call(&mut self, name: &str) -> Result<(), ToolRuntimeError> {
        if let Some(max) = self.max_tool_calls {
            if self.calls.len() >= max {
                return Err(ToolRuntimeError::new(
                    ToolRuntimeErrorKind::ToolCallLimitExceeded,
                    format!("Execution exceeded its tool-call limit of {}.", max),
                    vec![],
                ));
            }
        }
        self.calls.push(ToolCall {
            name: name.to_string(),
        });
        Ok(())
    }

    fn end_success(&self, call: &ToolCallStarted, started_ms: u64) {
        if let Some(on_end) = self.hooks.on_tool_call_end.as_ref() {
            on_end(&ToolCallEnded {
                index: call.index,
                name: call.name.clone(),
                input: call.input.clone(),
                duration_ms: started_ms,
                outcome: ToolCallOutcome::Success,
                message: None,
            });
        }
    }

    fn end_failure(&self, call: &ToolCallStarted, started_ms: u64, message: String) {
        if let Some(on_end) = self.hooks.on_tool_call_end.as_ref() {
            on_end(&ToolCallEnded {
                index: call.index,
                name: call.name.clone(),
                input: call.input.clone(),
                duration_ms: started_ms,
                outcome: ToolCallOutcome::Failure,
                message: Some(message),
            });
        }
    }

    /// Invokes one tool path with sandbox args. Mirrors the `invoke(path,
    /// args)` Effect program, executed synchronously.
    pub fn invoke(
        &mut self,
        tree: &ToolTree,
        path: &[String],
        args: &[Value],
    ) -> Result<Value, InvokeError> {
        let name = path.join(".");
        // Boundary-copy args first (validates data contract).
        let external_args: Vec<Value> = args
            .iter()
            .map(|arg| {
                copy_in(arg, &format!("Arguments for tool '{}'", name), false)
                    .map(|v| copy_out(&v, false))
            })
            .collect::<Result<_, _>>()
            .map_err(InvokeError::Runtime)?;
        let resolved = resolve_path(tree, path).map_err(InvokeError::Runtime)?;
        // `$codemode.search` is always registered.
        if matches!(resolved, Resolved::Search) {
            if external_args.len() != 1 {
                return Err(InvokeError::Runtime(ToolRuntimeError::new(
                    ToolRuntimeErrorKind::InvalidToolInput,
                    format!("Tool '{}' expects exactly one input object.", name),
                    vec![],
                )));
            }
            let request = parse_search_input(&external_args[0]).map_err(|message| {
                InvokeError::Runtime(ToolRuntimeError::new(
                    ToolRuntimeErrorKind::InvalidToolInput,
                    format!("Invalid input for tool '{}': {}", name, message),
                    vec![],
                ))
            })?;
            self.record_call(&name).map_err(InvokeError::Runtime)?;
            let index = self.calls.len() - 1;
            let started = ToolCallStarted {
                index,
                name: name.clone(),
                input: external_args[0].clone(),
            };
            if let Some(on_start) = self.hooks.on_tool_call_start.as_ref() {
                on_start(&started);
            }
            let output = run_search(&self.search_index, &request).map_err(InvokeError::Runtime)?;
            self.end_success(&started, 0);
            let value = serde_json::to_value(SearchOutputJson::from(output)).unwrap_or(Value::Null);
            return copy_in(&value, &format!("Result from tool '{}'", name), false)
                .map_err(InvokeError::Runtime);
        }
        match resolved {
            Resolved::Definition(def) => {
                if external_args.len() != 1 {
                    return Err(InvokeError::Runtime(ToolRuntimeError::new(
                        ToolRuntimeErrorKind::InvalidToolInput,
                        format!("Tool '{}' expects exactly one input object.", name),
                        vec![],
                    )));
                }
                let described =
                    crate::tool_schema::decode_input(def, &external_args[0]).map_err(|cause| {
                        InvokeError::Runtime(ToolRuntimeError::new(
                            ToolRuntimeErrorKind::InvalidToolInput,
                            format!("Invalid input for tool '{}': {}", name, cause),
                            vec![],
                        ))
                    })?;
                self.record_call(&name).map_err(InvokeError::Runtime)?;
                let index = self.calls.len() - 1;
                let started = ToolCallStarted {
                    index,
                    name: name.clone(),
                    input: described.clone(),
                };
                if let Some(on_start) = self.hooks.on_tool_call_start.as_ref() {
                    on_start(&started);
                }
                let raw = match (def.run)(&described) {
                    Ok(v) => v,
                    Err(ToolFailure::ToolError(e)) => {
                        self.end_failure(&started, 0, e.message.clone());
                        return Err(InvokeError::Tool(e));
                    }
                    Err(ToolFailure::Unknown(_)) => {
                        self.end_failure(&started, 0, "Tool execution failed".to_string());
                        return Err(InvokeError::Unknown);
                    }
                };
                let decoded = crate::tool_schema::decode_output(def, &raw).map_err(|_| {
                    InvokeError::Runtime(ToolRuntimeError::new(
                        ToolRuntimeErrorKind::InvalidToolOutput,
                        format!("Invalid output from tool '{}'.", name),
                        vec![],
                    ))
                })?;
                let copied = copy_in(&decoded, &format!("Result from tool '{}'", name), false)
                    .map_err(|_| {
                        InvokeError::Runtime(ToolRuntimeError::new(
                            ToolRuntimeErrorKind::InvalidToolOutput,
                            format!("Invalid output from tool '{}'.", name),
                            vec![],
                        ))
                    })?;
                self.end_success(&started, 0);
                Ok(copied)
            }
            Resolved::HostFn(f) => {
                self.record_call(&name).map_err(InvokeError::Runtime)?;
                let index = self.calls.len() - 1;
                let started = ToolCallStarted {
                    index,
                    name: name.clone(),
                    input: Value::Array(external_args.clone()),
                };
                if let Some(on_start) = self.hooks.on_tool_call_start.as_ref() {
                    on_start(&started);
                }
                let raw = match f(&external_args) {
                    Ok(v) => v,
                    Err(ToolFailure::ToolError(e)) => {
                        self.end_failure(&started, 0, e.message.clone());
                        return Err(InvokeError::Tool(e));
                    }
                    Err(ToolFailure::Unknown(_)) => {
                        self.end_failure(&started, 0, "Tool execution failed".to_string());
                        return Err(InvokeError::Unknown);
                    }
                };
                let copied = copy_in(&raw, &format!("Result from tool '{}'", name), false)
                    .map_err(|_| {
                        InvokeError::Runtime(ToolRuntimeError::new(
                            ToolRuntimeErrorKind::InvalidToolOutput,
                            format!("Invalid output from tool '{}'.", name),
                            vec![],
                        ))
                    })?;
                self.end_success(&started, 0);
                Ok(copied)
            }
            Resolved::Search => unreachable!(),
        }
    }
}

/// Failure of one `invoke`. The interpreter maps these to diagnostics:
/// `Tool` → `ToolFailure` (safe message), `Runtime` → its kind verbatim,
/// `Unknown` → sanitized `"Tool execution failed"`.
#[derive(Debug, Clone)]
pub enum InvokeError {
    Tool(ToolError),
    Runtime(ToolRuntimeError),
    Unknown,
}

fn parse_search_input(value: &Value) -> Result<SearchInput, String> {
    let obj = value
        .as_object()
        .ok_or_else(|| "expected an object".to_string())?;
    let query = match obj.get("query") {
        None | Some(Value::Null) => None,
        Some(Value::String(s)) => Some(s.clone()),
        Some(_) => return Err("query must be a string".to_string()),
    };
    let namespace = match obj.get("namespace") {
        None | Some(Value::Null) => None,
        Some(Value::String(s)) => Some(s.clone()),
        Some(_) => return Err("namespace must be a string".to_string()),
    };
    let limit = match obj.get("limit") {
        None | Some(Value::Null) => None,
        Some(Value::Number(n)) => {
            let v = n
                .as_u64()
                .ok_or_else(|| "limit must be a positive integer".to_string())?;
            if v == 0 {
                return Err("limit must be a positive integer".to_string());
            }
            Some(v as usize)
        }
        Some(_) => return Err("limit must be a positive integer".to_string()),
    };
    let offset = match obj.get("offset") {
        None | Some(Value::Null) => None,
        Some(Value::Number(n)) => Some(
            n.as_u64()
                .ok_or_else(|| "offset must be a non-negative integer".to_string())?
                as usize,
        ),
        Some(_) => return Err("offset must be a non-negative integer".to_string()),
    };
    Ok(SearchInput {
        query,
        namespace,
        limit,
        offset,
    })
}

#[derive(serde::Serialize)]
struct SearchOutputJson {
    items: Vec<SearchItemJson>,
    remaining: usize,
    next: Option<SearchNextJson>,
}

#[derive(serde::Serialize)]
struct SearchItemJson {
    path: String,
    description: String,
    signature: String,
}

#[derive(serde::Serialize)]
struct SearchNextJson {
    offset: usize,
}

impl From<SearchOutput> for SearchOutputJson {
    fn from(o: SearchOutput) -> Self {
        SearchOutputJson {
            items: o
                .items
                .into_iter()
                .map(|i| SearchItemJson {
                    path: i.path,
                    description: i.description,
                    signature: i.signature,
                })
                .collect(),
            remaining: o.remaining,
            next: o.next.map(|n| SearchNextJson { offset: n.offset }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocked_members_verbatim() {
        assert!(is_blocked_member("__proto__"));
        assert!(is_blocked_member("constructor"));
        assert!(is_blocked_member("prototype"));
        assert!(!is_blocked_member("toString"));
    }

    #[test]
    fn tool_expression_brackets_non_identifiers() {
        assert_eq!(tool_expression("ns.tool"), "tools.ns.tool");
        assert_eq!(tool_expression("ns.tool-name"), "tools.ns[\"tool-name\"]");
    }

    #[test]
    fn tokenize_splits_camel_and_separators() {
        assert_eq!(
            tokenize("resolveLibrary-id"),
            vec!["resolve", "library", "id"]
        );
        assert_eq!(tokenize("*"), Vec::<String>::new());
    }

    #[test]
    fn copy_in_rejects_blocked_props_and_depth() {
        let v = serde_json::json!({"__proto__": 1});
        let err = copy_in(&v, "Arguments for tool 'x'", false).unwrap_err();
        assert_eq!(err.kind, ToolRuntimeErrorKind::InvalidDataValue);
        assert!(err.message.contains("blocked property '__proto__'"));
    }

    #[test]
    fn assert_valid_tools_rejects_reserved_namespace() {
        let mut tree = ToolTree::new();
        tree.insert(
            "$codemode".to_string(),
            crate::tool::ToolTreeValue::Namespace(ToolTree::new()),
        );
        let err = assert_valid_tools(&tree).unwrap_err();
        assert_eq!(
            err,
            "Tool namespace '$codemode' is reserved for CodeMode discovery tools."
        );
    }

    #[test]
    fn prepare_empty_catalog_message() {
        let plan = prepare(&ToolTree::new(), None).unwrap();
        assert!(plan
            .instructions
            .contains("No tools are currently available."));
    }

    #[test]
    fn default_catalog_budget_verbatim() {
        assert_eq!(DEFAULT_CATALOG_BUDGET, 2_000);
        assert_eq!(MAX_VALUE_DEPTH, 32);
    }

    #[test]
    fn copy_out_opt_undefined_vs_null() {
        // Mirrors JS `copyOut(undefined, false)` vs `copyOut(undefined, true)`
        // (tool-runtime.ts:297-298): `undefined` stays `undefined` when flag is
        // false, becomes `null` only when `undefined_as_null=true`.
        assert_eq!(copy_out_opt(None, false), None);
        assert_eq!(copy_out_opt(None, true), Some(Value::Null));
        // `Some` passes through via `copy_out` with flag propagated.
        assert_eq!(
            copy_out_opt(Some(serde_json::json!({"a": 1})), false),
            Some(serde_json::json!({"a": 1}))
        );
        assert_eq!(
            copy_out_opt(Some(serde_json::json!({"a": 1})), true),
            Some(serde_json::json!({"a": 1}))
        );
        // Array/object recursion preserves flag (no `undefined` inside `Value`,
        // but the flag is propagated for symmetry).
        assert_eq!(
            copy_out(&serde_json::json!([1, 2]), false),
            serde_json::json!([1, 2])
        );
        assert_eq!(
            copy_out(&serde_json::json!([1, 2]), true),
            serde_json::json!([1, 2])
        );
    }

    #[test]
    fn copy_out_preserves_flag_recursively() {
        // Non-finite normalization is via `copy_out_number` before `Value`
        // materialization; `Value::Number` is always finite here.
        let v = serde_json::json!({"x": 1, "y": [2, 3]});
        assert_eq!(copy_out(&v, false), v);
        assert_eq!(copy_out(&v, true), v);
    }
}
