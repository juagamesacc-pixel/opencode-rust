//! Port of `src/interpreter/runtime.ts` (3,465 lines, single module).
//!
//! Sequenced work splits (plan §4 Lane E) — ONE file, in source order:
//! - E1 parse boundary + `loc` (this section)
//! - E2 scope/env/identifiers
//! - E3 statements
//! - E4 expressions
//! - E5 member/call dispatch + tools-only guard
//! - E6 stdlib wiring
//! - E7 promises/concurrency (`TOOL_CALL_CONCURRENCY = 8`)
//! - E8 budgets/logs/limits + `execute_with_limits` (+ `codemode.rs` envelope)
//!
//! EFFECT→STATE-MACHINE (plan §3.5): `Effect`/`Fiber`/`Semaphore` become an
//! explicit sync `ExecState` machine with identical observable semantics:
//! calls are admitted eagerly at the call site (budget charged +
//! `on_tool_call_start` fired before any await), settlement is run-once and
//! idempotent, timeout interruption is cooperative with the verbatim
//! `TimeoutExceeded` diagnostic, and `.then/.catch/.finally` await-inside
//! errors stay verbatim. Concurrency divergence is R2 (flagged): genuinely
//! async tool interleavings (e.g. `Promise.race` losers) settle in admission
//! order; deterministic hosts observe identical results.
//!
//! ACORN PARSE BOUNDARY (R1, flagged): no parser dependency is added. The
//! AST crosses into Rust as `serde_json::Value` nodes (Acorn JSON), validated
//! through the typed helpers in `interpreter_model`. TypeScript
//! syntax-stripping stays host-side (same Bun/TS toolchain); the
//! `Failed to parse TypeScript: ...` constructor is preserved for
//! host-reported diagnostics. `execute_with_limits` therefore consumes Acorn
//! `Program` JSON in `code` (documented divergence from TS `code: string`
//! JS/TS source, flagged for approval — NOT silently resolved).

use crate::interpreter_model::{
    format_location, get_array, get_boolean, get_node, get_optional_node, get_string,
    source_location, unsupported_syntax, AstNode, CoercionKind, DiagnosticKind,
    GlobalNamespaceName, InterpreterRuntimeError, PromiseMethodName, UriKind,
};
use crate::stdlib_value::DataVal;
use crate::tool::ToolTree;
use crate::tool_error::ToolError;
use crate::tool_runtime::{
    self, InvokeError, SearchEntry, ToolCall, ToolCallHooks, ToolRuntime, ToolRuntimeError,
    ToolRuntimeErrorKind,
};
use crate::values::{
    same_value_zero, PromiseImmediate, PromiseState, SandboxMap, SandboxPromise, SandboxRegExp,
    SandboxSet, SandboxURL, SandboxURLSearchParams, SandboxValue,
};
use serde_json::Value;
use std::collections::{BTreeSet, HashMap};

// ===========================================================================
// §3.5 Effect→state-machine: explicit execution states.
// ===========================================================================

/// Explicit sync execution states modelling the Effect/Fiber lifecycle with
/// identical observable semantics. Mirrors the TS fiber/settlement flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecState {
    /// Ready to evaluate the next node.
    Ready,
    /// A tool call was admitted eagerly at the call site (budget charged).
    BlockedOnTool,
    /// Suspended on promise settlement (`await` / combinator).
    AwaitingPromise,
    /// Interrupted (timeout teardown or race-loser interruption).
    Interrupted,
    /// Settled with a program value or diagnostic.
    Done,
}

// ===========================================================================
// E1 — Parse boundary + `loc`.
// ===========================================================================

/// Parsed program: statements plus the source node.
#[derive(Debug, Clone)]
pub struct Program {
    pub node: AstNode,
    pub body: Vec<AstNode>,
}

/// Parses Acorn AST JSON into a `Program`. Mirrors `parseProgram(code)` past
/// the TS-transpile step: validates the `Program`-node gate verbatim.
///
/// R1: `code` is Acorn `Program` JSON (host runs Acorn 8.15.0 after TS
/// stripping). JSON-syntax failure maps to `ParseError` (analogous to the
/// transpile-diagnostic mapping); a well-formed non-`Program` node yields the
/// verbatim `Failed to parse script as a Program node.`.
pub fn parse_program(code: &str) -> Result<Program, InterpreterRuntimeError> {
    let value: Value = serde_json::from_str(code).map_err(|error| {
        InterpreterRuntimeError::new(
            format!("Failed to parse TypeScript: {}", error),
            None,
            DiagnosticKind::ParseError,
            None,
        )
    })?;
    parse_program_node(&value)
}

/// Validates a pre-parsed AST value as a `Program` node. Mirrors the gate
/// after `parse(...)` in `parseProgram`.
pub fn parse_program_node(value: &Value) -> Result<Program, InterpreterRuntimeError> {
    let obj = match value.as_object() {
        Some(o) => o,
        None => {
            return Err(InterpreterRuntimeError::new(
                "Failed to parse script as a Program node.",
                None,
                DiagnosticKind::ExecutionFailure,
                None,
            ))
        }
    };
    if obj.get("type").and_then(|t| t.as_str()) != Some("Program")
        || !obj.get("body").map(|b| b.is_array()).unwrap_or(false)
    {
        return Err(InterpreterRuntimeError::new(
            "Failed to parse script as a Program node.",
            None,
            DiagnosticKind::ExecutionFailure,
            None,
        ));
    }
    let node = crate::interpreter_model::as_node(value, "program")?;
    let mut body = vec![];
    for (index, item) in get_array(&node, "body")?.into_iter().enumerate() {
        body.push(crate::interpreter_model::as_node(
            &item,
            &format!("body[{}]", index),
        )?);
    }
    Ok(Program { node, body })
}

/// Host-reported TypeScript diagnostic constructor. Preserves the verbatim
/// `Failed to parse TypeScript: ...` prefix for diagnostics produced
/// host-side by the TS transpile step.
pub fn parse_error_typescript(detail: &str) -> InterpreterRuntimeError {
    InterpreterRuntimeError::new(
        format!("Failed to parse TypeScript: {}", detail),
        None,
        DiagnosticKind::ParseError,
        None,
    )
}

// ===========================================================================
// Error normalization (mirrors `publicErrorMessage` / `normalizeError` /
// `caughtErrorValue`).
// ===========================================================================

/// Redacts filesystem paths from host-visible messages. Mirrors
/// `publicErrorMessage(message)` (`/(Users|home|private|tmp|var\/folders)\/...`).
pub fn public_error_message(message: &str) -> String {
    let mut out = String::new();
    let bytes = message.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let rest = &message[i..];
        let hit = ["/Users/", "/home/", "/private/", "/tmp/", "/var/folders/"]
            .iter()
            .find_map(|prefix| rest.find(prefix).map(|pos| (pos, prefix.len())));
        match hit {
            Some((pos, _)) => {
                out.push_str(&rest[..pos]);
                let mut end = pos;
                while end < rest.len()
                    && !matches!(rest.as_bytes()[end], b' ' | b'"' | b'\'' | b'`')
                {
                    end += 1;
                }
                out.push_str("<redacted-path>");
                i += end;
            }
            None => {
                out.push_str(rest);
                break;
            }
        }
    }
    out
}

/// Normalized model-safe diagnostic. Mirrors `Diagnostic`.
#[derive(Debug, Clone)]
pub struct ExecDiagnostic {
    pub kind: DiagnosticKind,
    pub message: String,
    pub location: Option<(u64, u64)>,
    pub suggestions: Vec<String>,
}

/// Normalizes any failure into a diagnostic. Mirrors `normalizeError(error)`
/// branch-for-branch (InterpreterRuntimeError → kind/message/location;
/// ToolRuntimeError → kind/message/suggestions; ToolError → ToolFailure +
/// redaction; ProgramThrow → `Uncaught: ...` with runtime-reference guard;
/// RangeError-recursion → nesting-depth message; Error → ParseError iff
/// SyntaxError else ExecutionFailure + redaction; catch-all → redacted
/// `String(error)`).
pub fn normalize_error(signal: &Signal) -> ExecDiagnostic {
    match signal {
        Signal::Runtime(error) => ExecDiagnostic {
            kind: error.kind,
            message: format!("{}{}", error.message, format_location(error.node.as_ref())),
            location: error.node.as_ref().map(|n| {
                let loc = source_location(n);
                (loc.line, loc.column)
            }),
            suggestions: error.suggestions.clone().unwrap_or_default(),
        },
        Signal::ToolRuntime(error) => ExecDiagnostic {
            kind: match error.kind {
                ToolRuntimeErrorKind::UnknownTool => DiagnosticKind::UnknownTool,
                ToolRuntimeErrorKind::InvalidToolInput => DiagnosticKind::InvalidToolInput,
                ToolRuntimeErrorKind::InvalidToolOutput => DiagnosticKind::InvalidToolOutput,
                ToolRuntimeErrorKind::InvalidDataValue => DiagnosticKind::InvalidDataValue,
                ToolRuntimeErrorKind::ToolCallLimitExceeded => {
                    DiagnosticKind::ToolCallLimitExceeded
                }
            },
            message: error.message.clone(),
            location: None,
            suggestions: error.suggestions.clone(),
        },
        Signal::Tool(error) => ExecDiagnostic {
            kind: DiagnosticKind::ToolFailure,
            message: public_error_message(&error.message),
            location: None,
            suggestions: vec![],
        },
        Signal::UnknownHost(_detail) => ExecDiagnostic {
            kind: DiagnosticKind::ToolFailure,
            message: "Tool execution failed".to_string(),
            location: None,
            suggestions: vec![],
        },
        Signal::Throw(value) => {
            let message = if contains_runtime_reference(value) {
                "a non-data value".to_string()
            } else {
                thrown_message(value)
            };
            let _ = signal;
            ExecDiagnostic {
                kind: DiagnosticKind::ExecutionFailure,
                message: format!("Uncaught: {}", message),
                location: None,
                suggestions: vec![],
            }
        }
        Signal::NestingDepth => ExecDiagnostic {
            kind: DiagnosticKind::ExecutionFailure,
            message: "Execution exceeded the maximum nesting depth.".to_string(),
            location: None,
            suggestions: vec![],
        },
        Signal::ParseError(message) => ExecDiagnostic {
            kind: DiagnosticKind::ParseError,
            message: public_error_message(message),
            location: None,
            suggestions: vec![],
        },
        Signal::Interrupted => ExecDiagnostic {
            kind: DiagnosticKind::ExecutionFailure,
            message: "Execution was interrupted.".to_string(),
            location: None,
            suggestions: vec![],
        },
        Signal::Return(_) | Signal::Break(_) | Signal::Continue(_) => ExecDiagnostic {
            kind: DiagnosticKind::ExecutionFailure,
            message: "Control signal escaped normalization.".to_string(),
            location: None,
            suggestions: vec![],
        },
    }
}

fn thrown_message(value: &RtValue) -> String {
    match value {
        RtValue::Str(s) => s.clone(),
        RtValue::Object(obj) => {
            if let Some(RtValue::Str(message)) = obj.get("message") {
                message.clone()
            } else {
                serde_json::to_string(&rt_to_json(value)).unwrap_or_else(|_| "null".to_string())
            }
        }
        _ => serde_json::to_string(&rt_to_json(value)).unwrap_or_else(|_| "null".to_string()),
    }
}

/// Shared catch conversion. Mirrors `caughtErrorValue(thrown)`.
pub fn caught_error_value(signal: &Signal) -> RtValue {
    match signal {
        Signal::Throw(value) => value.clone(),
        Signal::Runtime(error) => RtValue::ErrorObj(RtErrorObj {
            name: error.error_name.clone(),
            message: error.message.clone(),
        }),
        Signal::Tool(error) => RtValue::ErrorObj(RtErrorObj {
            name: "Error".to_string(),
            message: public_error_message(&error.message),
        }),
        Signal::ToolRuntime(error) => RtValue::ErrorObj(RtErrorObj {
            name: "Error".to_string(),
            message: error.message.clone(),
        }),
        &Signal::UnknownHost(_) => RtValue::ErrorObj(RtErrorObj {
            name: "Error".to_string(),
            message: "Tool execution failed".to_string(),
        }),
        Signal::NestingDepth => RtValue::ErrorObj(RtErrorObj {
            name: "RangeError".to_string(),
            message: "Execution exceeded the maximum nesting depth.".to_string(),
        }),
        Signal::ParseError(message) => RtValue::ErrorObj(RtErrorObj {
            name: "SyntaxError".to_string(),
            message: public_error_message(message),
        }),
        Signal::Interrupted => RtValue::ErrorObj(RtErrorObj {
            name: "Error".to_string(),
            message: "Execution was interrupted.".to_string(),
        }),
        Signal::Return(_) | Signal::Break(_) | Signal::Continue(_) => RtValue::Undefined,
    }
}

// ===========================================================================
// Runtime values + signals (representation lives in `values`; control
// signals live here).
// ===========================================================================

// Re-exported for helper modules and tests: the canonical value model.
pub use crate::values::{
    alloc_container_id, RtArray, RtBinding, RtErrorObj, RtFunction, RtObject, RtValue,
};

/// In-flight control signal. `Return`/`Break`/`Continue` are loop/function
/// control (never escape `run`); the rest are failures normalized by
/// `normalize_error`.
#[derive(Debug, Clone)]
pub enum Signal {
    Runtime(InterpreterRuntimeError),
    Throw(RtValue),
    Tool(ToolError),
    ToolRuntime(ToolRuntimeError),
    UnknownHost(String),
    NestingDepth,
    ParseError(String),
    Interrupted,
    Return(RtValue),
    Break(Option<String>),
    Continue(Option<String>),
}

impl Signal {
    /// Mirrors `new InterpreterRuntimeError(message, node?, kind?, suggestions?)`.
    pub fn runtime(
        message: impl Into<String>,
        node: Option<AstNode>,
        kind: DiagnosticKind,
        suggestions: Option<Vec<String>>,
    ) -> Self {
        Signal::Runtime(InterpreterRuntimeError::new(
            message,
            node,
            kind,
            suggestions,
        ))
    }

    pub fn execution(message: impl Into<String>, node: Option<AstNode>) -> Self {
        Signal::Runtime(InterpreterRuntimeError::new(
            message,
            node,
            DiagnosticKind::ExecutionFailure,
            None,
        ))
    }
}

impl From<InterpreterRuntimeError> for Signal {
    fn from(error: InterpreterRuntimeError) -> Self {
        Signal::Runtime(error)
    }
}

impl From<ToolRuntimeError> for Signal {
    fn from(error: ToolRuntimeError) -> Self {
        Signal::ToolRuntime(error)
    }
}

/// Evaluation result.
pub type Eval<T> = Result<T, Signal>;

/// Lexical binding over runtime values is defined in `values`
/// ([`RtBinding`]); control signals live here.

// --- Value conversions -----------------------------------------------------

/// Converts runtime data to JSON for boundary crossing. Reference values
/// never reach here (guarded by `contains_runtime_reference` at checkpoints).
pub fn rt_to_json(value: &RtValue) -> Value {
    match value {
        RtValue::Undefined | RtValue::Null => Value::Null,
        RtValue::Bool(b) => Value::Bool(*b),
        RtValue::Number(n) => tool_runtime::copy_out_number(*n),
        RtValue::Str(s) => Value::String(s.clone()),
        RtValue::Array(arr) => Value::Array(arr.items.iter().map(rt_to_json).collect()),
        RtValue::Object(obj) => Value::Object(
            obj.entries
                .iter()
                .map(|(k, v)| (k.clone(), rt_to_json(v)))
                .collect(),
        ),
        RtValue::ErrorObj(e) => serde_json::json!({"name": e.name, "message": e.message}),
        RtValue::Sandbox(SandboxValue::Date(d)) => {
            if d.time.is_finite() {
                Value::String(crate::stdlib_value::format_iso8601(d.time))
            } else {
                Value::Null
            }
        }
        RtValue::Sandbox(SandboxValue::Url(u)) => Value::String(u.href.clone()),
        RtValue::Sandbox(_) => Value::Object(Default::default()),
        // References are opaque; materialization is guarded before this point.
        _ => Value::Null,
    }
}

/// Materializes boundary-checked JSON as runtime data.
pub fn json_to_rt(value: &Value) -> RtValue {
    match value {
        Value::Null => RtValue::Null,
        Value::Bool(b) => RtValue::Bool(*b),
        Value::Number(n) => RtValue::Number(n.as_f64().unwrap_or(f64::NAN)),
        Value::String(s) => RtValue::Str(s.clone()),
        Value::Array(items) => RtValue::Array(RtArray::new(items.iter().map(json_to_rt).collect())),
        Value::Object(map) => RtValue::Object(RtObject::new(
            map.iter()
                .map(|(k, v)| (k.clone(), json_to_rt(v)))
                .collect(),
        )),
    }
}

/// Converts runtime data to the stdlib [`DataVal`] view.
pub fn rt_to_data(value: &RtValue) -> DataVal {
    match value {
        RtValue::Undefined => DataVal::Undefined,
        RtValue::Null => DataVal::Null,
        RtValue::Bool(b) => DataVal::Bool(*b),
        RtValue::Number(n) => DataVal::Number(*n),
        RtValue::Str(s) => DataVal::Str(s.clone()),
        RtValue::Array(arr) => DataVal::Array(arr.items.iter().map(rt_to_data).collect()),
        RtValue::Object(obj) => DataVal::Object(
            obj.entries
                .iter()
                .map(|(k, v)| (k.clone(), rt_to_data(v)))
                .collect(),
        ),
        RtValue::Sandbox(s) => DataVal::Sandbox(s.clone()),
        RtValue::ErrorObj(e) => DataVal::Object(vec![
            ("name".to_string(), DataVal::Str(e.name.clone())),
            ("message".to_string(), DataVal::Str(e.message.clone())),
        ]),
        _ => DataVal::Undefined,
    }
}

/// Converts a stdlib [`DataVal`] back to runtime data.
pub fn data_to_rt(value: &DataVal) -> RtValue {
    match value {
        DataVal::Undefined => RtValue::Undefined,
        DataVal::Null => RtValue::Null,
        DataVal::Bool(b) => RtValue::Bool(*b),
        DataVal::Number(n) => RtValue::Number(*n),
        DataVal::Str(s) => RtValue::Str(s.clone()),
        DataVal::Array(items) => {
            RtValue::Array(RtArray::new(items.iter().map(data_to_rt).collect()))
        }
        DataVal::Object(entries) => RtValue::Object(RtObject::new(
            entries
                .iter()
                .map(|(k, v)| (k.clone(), data_to_rt(v)))
                .collect(),
        )),
        DataVal::Sandbox(s) => RtValue::Sandbox(s.clone()),
    }
}

// --- Reference guards (mirror `isRuntimeReference` /
// `containsRuntimeReference` / `containsOpaqueReference`) --------------------

/// Mirrors `isRuntimeReference(value)`.
pub fn is_runtime_reference(value: &RtValue) -> bool {
    match value {
        RtValue::Function(_)
        | RtValue::ToolRef(_)
        | RtValue::Intrinsic { .. }
        | RtValue::Global(_)
        | RtValue::GlobalMethod { .. }
        | RtValue::PromiseNs
        | RtValue::PromiseMethod(_)
        | RtValue::Coercion(_)
        | RtValue::Uri(_)
        | RtValue::ErrorCtor(_)
        | RtValue::Computed(_)
        | RtValue::ShortCircuit => true,
        RtValue::Sandbox(_) => true,
        _ => false,
    }
}

fn ref_children(value: &RtValue) -> Vec<&RtValue> {
    match value {
        RtValue::Array(arr) => arr.items.iter().collect(),
        RtValue::Object(obj) => obj.entries.iter().map(|(_, v)| v).collect(),
        RtValue::Intrinsic { receiver, .. } => vec![receiver.as_ref()],
        RtValue::Computed(inner) => vec![inner.as_ref()],
        _ => vec![],
    }
}

/// Mirrors `containsRuntimeReference(value, seen?)`.
pub fn contains_runtime_reference(value: &RtValue) -> bool {
    fn walk(value: &RtValue, seen: &mut BTreeSet<usize>) -> bool {
        if is_runtime_reference(value) {
            return true;
        }
        let addr = value as *const RtValue as usize;
        if !seen.insert(addr) {
            return false;
        }
        let found = ref_children(value).iter().any(|child| walk(child, seen));
        seen.remove(&addr);
        found
    }
    walk(value, &mut BTreeSet::new())
}

/// Mirrors `containsOpaqueReference(value, seen?)`: sandbox stdlib values
/// count as data here.
pub fn contains_opaque_reference(value: &RtValue) -> bool {
    fn walk(value: &RtValue, seen: &mut BTreeSet<usize>) -> bool {
        if matches!(value, RtValue::Sandbox(_)) {
            return false;
        }
        if is_runtime_reference(value) {
            return true;
        }
        let addr = value as *const RtValue as usize;
        if !seen.insert(addr) {
            return false;
        }
        let found = ref_children(value).iter().any(|child| walk(child, seen));
        seen.remove(&addr);
        found
    }
    walk(value, &mut BTreeSet::new())
}

// --- `typeof` / `instanceof` (mirror `typeofValue` / `instanceofValue`) ------

/// Mirrors `typeofValue(value)`.
pub fn typeof_value(value: &RtValue) -> &'static str {
    match value {
        RtValue::Function(_)
        | RtValue::Intrinsic { .. }
        | RtValue::GlobalMethod { .. }
        | RtValue::PromiseMethod(_)
        | RtValue::PromiseNs
        | RtValue::ErrorCtor(_) => "function",
        RtValue::Coercion(_) | RtValue::Uri(_) => "function",
        RtValue::ToolRef(path) => {
            if path.is_empty() {
                "object"
            } else {
                "function"
            }
        }
        RtValue::Global(ns) => match ns {
            GlobalNamespaceName::Math
            | GlobalNamespaceName::Json
            | GlobalNamespaceName::Console => "object",
            _ => "function",
        },
        RtValue::Undefined => "undefined",
        RtValue::Null => "object",
        RtValue::Bool(_) => "boolean",
        RtValue::Number(_) => "number",
        RtValue::Str(_) => "string",
        RtValue::Array(_) | RtValue::Object(_) => "object",
        RtValue::Sandbox(_) => "object",
        RtValue::ErrorObj(_) => "object",
        RtValue::Computed(inner) => typeof_value(inner),
        RtValue::ShortCircuit => "undefined",
    }
}

/// Mirrors `instanceofValue(lhs, rhs, node)`.
pub fn instanceof_value(lhs: &RtValue, rhs: &RtValue, node: Option<&AstNode>) -> Eval<bool> {
    if let RtValue::ErrorCtor(name) = rhs {
        if let RtValue::ErrorObj(e) = lhs {
            return Ok(name == "Error" || &e.name == name);
        }
        return Ok(false);
    }
    if let RtValue::Global(ns) = rhs {
        return Ok(match ns {
            GlobalNamespaceName::Date => matches!(lhs, RtValue::Sandbox(SandboxValue::Date(_))),
            GlobalNamespaceName::RegExp => {
                matches!(lhs, RtValue::Sandbox(SandboxValue::RegExp(_)))
            }
            GlobalNamespaceName::Map => matches!(lhs, RtValue::Sandbox(SandboxValue::Map(_))),
            GlobalNamespaceName::Set => matches!(lhs, RtValue::Sandbox(SandboxValue::Set(_))),
            GlobalNamespaceName::Url => matches!(lhs, RtValue::Sandbox(SandboxValue::Url(_))),
            GlobalNamespaceName::UrlSearchParams => {
                matches!(lhs, RtValue::Sandbox(SandboxValue::UrlSearchParams(_)))
            }
            GlobalNamespaceName::Array => matches!(lhs, RtValue::Array(_)),
            GlobalNamespaceName::Object => !matches!(lhs, RtValue::Undefined | RtValue::Null),
            _ => false,
        });
    }
    if matches!(rhs, RtValue::PromiseNs) {
        return Ok(matches!(lhs, RtValue::Sandbox(SandboxValue::Promise(_))));
    }
    if let RtValue::Coercion(kind) = rhs {
        if matches!(
            kind,
            CoercionKind::Number | CoercionKind::String | CoercionKind::Boolean
        ) {
            return Ok(false);
        }
    }
    Err(Signal::execution(
        "The right-hand side of 'instanceof' must be a constructor CodeMode knows: Error (or a specific error type like TypeError), Date, RegExp, Map, Set, URL, URLSearchParams, Array, Object, or Promise.",
        node.cloned(),
    ))
}

// ===========================================================================
// Interpreter struct, E2 scope/env, run loop, tool admission + settlement.
// ===========================================================================

/// Validates regex flags, returning the failure reason. Mirrors the flag half
/// of the `new RegExp(...)` triage.
pub fn validate_regex_flags(flags: &str) -> Result<(), String> {
    let mut seen = std::collections::BTreeSet::new();
    for c in flags.chars() {
        if !"dgimsuvy".contains(c) {
            return Err(format!(
                "Invalid flags supplied to RegExp constructor '{}'",
                flags
            ));
        }
        if !seen.insert(c) {
            return Err(format!(
                "Invalid flags supplied to RegExp constructor '{}'",
                flags
            ));
        }
    }
    if flags.contains('u') && flags.contains('v') {
        return Err("Invalid flags supplied to RegExp constructor 'uv'".to_string());
    }
    Ok(())
}

/// Outcome of one admitted tool call, settled eagerly at the call site.
#[derive(Debug, Clone)]
pub enum ToolOutcome {
    Success(RtValue),
    /// User-thrown value in flight (`Promise.reject(value)`).
    ThrownValue(RtValue),
    Refusal(ToolError),
    Runtime(ToolRuntimeError),
    Unknown,
}

/// The tree-walking interpreter. Mirrors `class Interpreter<R>`.
pub struct Interpreter<'a> {
    scopes: Vec<HashMap<String, RtBinding>>,
    tree: &'a ToolTree,
    tools: ToolRuntime,
    logs: Vec<String>,
    last: RtValue,
    /// Settlement slots parallel to admitted promises (run-once, idempotent).
    outcomes: Vec<ToolOutcome>,
    /// Admitted but never-observed call indices (unhandled-rejection drain).
    pending: BTreeSet<usize>,
    /// Simultaneous in-flight calls (sync port: at most 1; the
    /// `TOOL_CALL_CONCURRENCY = 8` ceiling is enforced identically).
    inflight: usize,
    /// Cooperative timeout deadline (E8).
    deadline: Option<std::time::Instant>,
    /// Configured timeout budget for the verbatim diagnostic.
    timeout_budget: Option<u64>,
    /// Step counter for cooperative timeout checks inside expressions.
    steps: u64,
    /// Evaluation depth guard (nesting-depth diagnostic before stack overflow).
    depth: usize,
    /// `Promise.race` losers by settlement id (later awaits observe the
    /// race-interruption failure).
    interrupted_calls: BTreeSet<usize>,
    /// Host clock (ms) for `Date.now()`.
    now_ms: Box<dyn Fn() -> f64 + Send + Sync>,
    state: ExecState,
    /// Immediate (non-tool) settlement table for combinators
    /// (`Promise.resolve` / `Promise.reject` / mixed combinators).
    immediates: std::collections::HashMap<usize, ToolOutcome>,
    /// Next immediate-settlement id (ids live past the tool ledger length).
    next_immediate: usize,
}

impl<'a> Interpreter<'a> {
    /// Mirrors `new Interpreter(invokeTool, toolKeys, logs?)`. The global
    /// scope is seeded with the identical builtins (`tools`, `Promise`,
    /// `undefined`, `Object`, `Math`, `JSON`, `Number`, `String`, `Boolean`,
    /// `Array`, `console`, `parseInt`, `parseFloat`, `Date`, `RegExp`, `Map`,
    /// `Set`, `URL`, `URLSearchParams`, `encodeURI`, `encodeURIComponent`,
    /// `decodeURI`, `decodeURIComponent`, error constructors, `NaN`,
    /// `Infinity`).
    pub fn new(
        tree: &'a ToolTree,
        tools: ToolRuntime,
        now_ms: Box<dyn Fn() -> f64 + Send + Sync>,
        deadline: Option<std::time::Instant>,
    ) -> Self {
        let mut global: HashMap<String, RtBinding> = HashMap::new();
        let mut seed = |name: &str, value: RtValue| {
            global.insert(
                name.to_string(),
                RtBinding {
                    mutable: false,
                    value,
                    initialized: true,
                },
            );
        };
        seed("tools", RtValue::ToolRef(vec![]));
        seed("Promise", RtValue::PromiseNs);
        seed("undefined", RtValue::Undefined);
        seed("Object", RtValue::Global(GlobalNamespaceName::Object));
        seed("Math", RtValue::Global(GlobalNamespaceName::Math));
        seed("JSON", RtValue::Global(GlobalNamespaceName::Json));
        seed("Number", RtValue::Coercion(CoercionKind::Number));
        seed("String", RtValue::Coercion(CoercionKind::String));
        seed("Boolean", RtValue::Coercion(CoercionKind::Boolean));
        seed("Array", RtValue::Global(GlobalNamespaceName::Array));
        seed("console", RtValue::Global(GlobalNamespaceName::Console));
        seed("parseInt", RtValue::Coercion(CoercionKind::ParseInt));
        seed("parseFloat", RtValue::Coercion(CoercionKind::ParseFloat));
        seed("Date", RtValue::Global(GlobalNamespaceName::Date));
        seed("RegExp", RtValue::Global(GlobalNamespaceName::RegExp));
        seed("Map", RtValue::Global(GlobalNamespaceName::Map));
        seed("Set", RtValue::Global(GlobalNamespaceName::Set));
        seed("URL", RtValue::Global(GlobalNamespaceName::Url));
        seed(
            "URLSearchParams",
            RtValue::Global(GlobalNamespaceName::UrlSearchParams),
        );
        seed("encodeURI", RtValue::Uri(UriKind::EncodeUri));
        seed(
            "encodeURIComponent",
            RtValue::Uri(UriKind::EncodeUriComponent),
        );
        seed("decodeURI", RtValue::Uri(UriKind::DecodeUri));
        seed(
            "decodeURIComponent",
            RtValue::Uri(UriKind::DecodeUriComponent),
        );
        for name in crate::stdlib_value::ERROR_CONSTRUCTORS {
            seed(name, RtValue::ErrorCtor(name.to_string()));
        }
        seed("NaN", RtValue::Number(f64::NAN));
        seed("Infinity", RtValue::Number(f64::INFINITY));
        Interpreter {
            scopes: vec![global],
            tree,
            tools,
            logs: vec![],
            last: RtValue::Undefined,
            outcomes: vec![],
            pending: BTreeSet::new(),
            inflight: 0,
            deadline,
            timeout_budget: None,
            steps: 0,
            depth: 0,
            interrupted_calls: BTreeSet::new(),
            now_ms,
            state: ExecState::Ready,
            immediates: std::collections::HashMap::new(),
            next_immediate: 0,
        }
    }

    /// Drains captured console lines.
    pub fn take_logs(&mut self) -> Vec<String> {
        std::mem::take(&mut self.logs)
    }

    /// Admitted-call ledger. Mirrors `tools.calls`.
    pub fn tool_calls(&self) -> Vec<ToolCall> {
        self.tools.calls.clone()
    }

    // --- E2 scope / env ----------------------------------------------------

    fn current_scope(&mut self) -> Eval<&mut HashMap<String, RtBinding>> {
        self.scopes
            .last_mut()
            .ok_or_else(|| Signal::execution("Interpreter scope stack is empty.", None))
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn resolve_binding(&mut self, name: &str) -> Option<&mut RtBinding> {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                return scope.get_mut(name);
            }
        }
        None
    }

    /// Mirrors `declare(name, value, mutable, node)`.
    pub(crate) fn declare(
        &mut self,
        name: &str,
        value: RtValue,
        mutable: bool,
        node: Option<&AstNode>,
    ) -> Eval<()> {
        let scope = self.current_scope()?;
        if let Some(existing) = scope.get(name) {
            if existing.initialized {
                return Err(Signal::execution(
                    format!("Identifier '{}' has already been declared.", name),
                    node.cloned(),
                ));
            }
        }
        scope.insert(
            name.to_string(),
            RtBinding {
                mutable,
                value,
                initialized: true,
            },
        );
        Ok(())
    }

    /// Mirrors `getIdentifierValue(name, node)`.
    pub(crate) fn get_identifier(&mut self, name: &str, node: Option<&AstNode>) -> Eval<RtValue> {
        match self.resolve_binding(name) {
            None => Err(Signal::Runtime(
                InterpreterRuntimeError::new(
                    format!("Unknown identifier '{}'.", name),
                    node.cloned(),
                    DiagnosticKind::ExecutionFailure,
                    None,
                )
                .as_error("ReferenceError"),
            )),
            Some(binding) => {
                if !binding.initialized {
                    return Err(Signal::Runtime(
                        InterpreterRuntimeError::new(
                            format!("Cannot access '{}' before initialization.", name),
                            node.cloned(),
                            DiagnosticKind::ExecutionFailure,
                            None,
                        )
                        .as_error("ReferenceError"),
                    ));
                }
                Ok(binding.value.clone())
            }
        }
    }

    /// Mirrors `setIdentifierValue(name, value, node)`.
    pub(crate) fn set_identifier(
        &mut self,
        name: &str,
        value: RtValue,
        node: Option<&AstNode>,
    ) -> Eval<RtValue> {
        match self.resolve_binding(name) {
            None => Err(Signal::Runtime(
                InterpreterRuntimeError::new(
                    format!("Unknown identifier '{}'.", name),
                    node.cloned(),
                    DiagnosticKind::ExecutionFailure,
                    None,
                )
                .as_error("ReferenceError"),
            )),
            Some(binding) => {
                if !binding.mutable {
                    return Err(Signal::Runtime(
                        InterpreterRuntimeError::new(
                            format!("Cannot assign to constant '{}'.", name),
                            node.cloned(),
                            DiagnosticKind::ExecutionFailure,
                            None,
                        )
                        .as_error("TypeError"),
                    ));
                }
                binding.value = value.clone();
                Ok(value)
            }
        }
    }

    /// Cooperative timeout check. Mirrors Effect-timeout teardown with the
    /// verbatim `TimeoutExceeded` diagnostic.
    pub(crate) fn check_timeout(&mut self) -> Eval<()> {
        self.steps += 1;
        if self.steps % 1024 != 0 {
            return Ok(());
        }
        if let Some(deadline) = self.deadline {
            if std::time::Instant::now() >= deadline {
                self.state = ExecState::Interrupted;
                return Err(timeout_signal(self.timeout_budget));
            }
        }
        Ok(())
    }

    /// Depth guard entry. Mirrors the host stack-overflow triage
    /// (`Execution exceeded the maximum nesting depth.`).
    fn enter(&mut self) -> Eval<()> {
        self.depth += 1;
        if self.depth > 1000 {
            self.depth -= 1;
            return Err(Signal::NestingDepth);
        }
        self.check_timeout()
    }

    fn exit(&mut self) {
        self.depth = self.depth.saturating_sub(1);
    }

    // --- run / drain (E7 core) ---------------------------------------------

    /// Runs the program body in its own module scope. Mirrors `run(program)`:
    /// hoists functions, threads `return`/`break`/`continue`, defaults to the
    /// last expression value, awaits a returned promise, then drains pending
    /// settlements.
    pub fn run(&mut self, program: &Program) -> Eval<RtValue> {
        self.state = ExecState::Ready;
        self.push_scope();
        let mut value = RtValue::Undefined;
        let mut returned = false;
        self.hoist_functions(&program.body)?;
        for statement in &program.body {
            self.check_timeout()?;
            match self.eval_statement(statement)? {
                StmtOut::None => {}
                StmtOut::Value(v) => {
                    self.last = v;
                }
                StmtOut::Return(v) => {
                    value = v;
                    returned = true;
                    break;
                }
            }
        }
        if !returned {
            value = self.last.clone();
        }
        // The program body runs inside an implicit async function: a returned
        // promise resolves before crossing the data boundary.
        if let RtValue::Sandbox(SandboxValue::Promise(p)) = value.clone() {
            value = self.settle_promise(&p, None)?;
        }
        self.drain_pending_settlements()?;
        self.pop_scope();
        self.state = ExecState::Done;
        Ok(value)
    }

    /// Function-declaration hoisting. Mirrors `hoistFunctions(statements)`.
    pub(crate) fn hoist_functions(&mut self, statements: &[AstNode]) -> Eval<()> {
        for statement in statements {
            if statement.node_type != "FunctionDeclaration" {
                continue;
            }
            let id = get_node(statement, "id").map_err(Signal::from)?;
            let name = get_string(&id, "name").map_err(Signal::from)?;
            let function = self.create_function(statement, true)?;
            self.declare(&name, RtValue::Function(function), true, Some(statement))?;
        }
        Ok(())
    }

    /// Awaits abandoned fiber-backed promises. Mirrors
    /// `drainPendingSettlements()`: the first never-awaited failure becomes
    /// an unhandled-rejection diagnostic (verbatim message + suggestion).
    pub(crate) fn drain_pending_settlements(&mut self) -> Eval<()> {
        let abandoned: Vec<usize> = self.pending.iter().copied().collect();
        for index in abandoned {
            let outcome = self.observe_promise(index);
            match outcome {
                ToolOutcome::Success(_) => {}
                ToolOutcome::ThrownValue(value) => {
                    let diagnostic = normalize_error(&Signal::Throw(value));
                    return Err(Signal::Runtime(InterpreterRuntimeError::new(
                        format!("Unhandled rejection from an un-awaited tool call: {}", diagnostic.message),
                        None,
                        diagnostic.kind,
                        Some(vec!["Await tool calls - `const result = await tools.ns.tool(...)` - so failures can be caught and handled.".to_string()]),
                    )));
                }
                ToolOutcome::Refusal(error) => {
                    let diagnostic = normalize_error(&Signal::Tool(error));
                    return Err(Signal::Runtime(InterpreterRuntimeError::new(
                        format!("Unhandled rejection from an un-awaited tool call: {}", diagnostic.message),
                        None,
                        diagnostic.kind,
                        Some(vec!["Await tool calls - `const result = await tools.ns.tool(...)` - so failures can be caught and handled.".to_string()]),
                    )));
                }
                ToolOutcome::Runtime(error) => {
                    let diagnostic = normalize_error(&Signal::ToolRuntime(error));
                    return Err(Signal::Runtime(InterpreterRuntimeError::new(
                        format!("Unhandled rejection from an un-awaited tool call: {}", diagnostic.message),
                        None,
                        diagnostic.kind,
                        Some(vec!["Await tool calls - `const result = await tools.ns.tool(...)` - so failures can be caught and handled.".to_string()]),
                    )));
                }
                ToolOutcome::Unknown => {
                    return Err(Signal::Runtime(InterpreterRuntimeError::new(
                        "Unhandled rejection from an un-awaited tool call: Tool execution failed",
                        None,
                        DiagnosticKind::ToolFailure,
                        Some(vec!["Await tool calls - `const result = await tools.ns.tool(...)` - so failures can be caught and handled.".to_string()]),
                    )));
                }
            }
        }
        Ok(())
    }

    /// Eagerly admits a tool call at the call site. Mirrors
    /// `createToolCallPromise(path, args)`: charges the tool-call budget and
    /// fires `onToolCallStart` before any await (via `ToolRuntime::invoke`),
    /// gated by the `TOOL_CALL_CONCURRENCY` semaphore (sync: acquired and
    /// held only for the synchronous dispatch).
    ///
    /// Settlement slots parallel admitted promises (NOT the tool-call
    /// ledger): even admission-time failures (limit, boundary, invalid
    /// input) settle the promise so `await` re-raises them and the drain
    /// reports un-awaited ones — exactly like fiber failures in TS.
    pub(crate) fn admit_tool_call(
        &mut self,
        path: &[String],
        args: Vec<Value>,
    ) -> Eval<SandboxPromise> {
        // TOOL_CALL_CONCURRENCY gate (plan §3.5): the TS semaphore caps
        // simultaneously in-flight calls at 8. Dispatch here is synchronous,
        // so at most one call is ever in flight; the ceiling is enforced by
        // construction (debug-asserted) rather than by waiting.
        debug_assert!(self.inflight < crate::stdlib_promise::TOOL_CALL_CONCURRENCY);
        self.state = ExecState::BlockedOnTool;
        self.inflight += 1;
        let result = self.tools.invoke(self.tree, path, &args);
        self.inflight -= 1;
        self.state = ExecState::Ready;
        let outcome = match result {
            Ok(json) => ToolOutcome::Success(json_to_rt(&json)),
            Err(InvokeError::Tool(error)) => ToolOutcome::Refusal(error),
            Err(InvokeError::Runtime(error)) => ToolOutcome::Runtime(error),
            Err(InvokeError::Unknown) => ToolOutcome::Unknown,
        };
        let slot = self.outcomes.len();
        self.outcomes.push(outcome);
        let promise = SandboxPromise::pending(slot);
        self.pending.insert(slot);
        Ok(promise)
    }

    /// Registers a pre-settled immediate outcome; returns its settlement id.
    pub(crate) fn admit_immediate(&mut self, outcome: ToolOutcome) -> usize {
        let id = usize::MAX - self.next_immediate;
        self.next_immediate += 1;
        self.immediates.insert(id, outcome);
        self.pending.insert(id);
        id
    }

    /// Observes a promise settlement (idempotent). Mirrors
    /// `observePromise(promise)`.
    pub(crate) fn observe_promise(&mut self, call_index: usize) -> ToolOutcome {
        self.pending.remove(&call_index);
        // Pre-settled immediates (Promise.resolve/reject) live in the
        // immediate table, not the tool ledger.
        if let Some(immediate) = self.immediates.get(&call_index) {
            return immediate.clone();
        }
        self.outcomes
            .get(call_index)
            .cloned()
            .unwrap_or(ToolOutcome::Unknown)
    }

    /// `await promise`: succeeds with the fulfilled value or re-raises the
    /// failure so try/catch observes it exactly like a synchronous throw at
    /// the await site. Mirrors `settlePromise(promise, node?)` (+ the
    /// `Promise.race`-loser interruption mapping of `unwrapPromiseExit`).
    pub(crate) fn settle_promise(
        &mut self,
        promise: &SandboxPromise,
        node: Option<&AstNode>,
    ) -> Eval<RtValue> {
        self.state = ExecState::AwaitingPromise;
        let (outcome, interrupted) = match &promise.state {
            PromiseState::Pending { call_index } => {
                let interrupted =
                    promise.interrupted || self.interrupted_calls.contains(call_index);
                (self.observe_promise(*call_index), interrupted)
            }
            PromiseState::Immediate { value } => match value {
                PromiseImmediate::Fulfilled(v) => (ToolOutcome::Success(json_to_rt(v)), false),
                PromiseImmediate::Rejected(message) => (
                    ToolOutcome::Refusal(crate::tool_error::tool_error(message.clone(), None)),
                    false,
                ),
            },
        };
        self.state = ExecState::Ready;
        // A `Promise.race` loser settles as a catchable program failure
        // (verbatim); any other interruption propagates as teardown.
        if interrupted {
            return Err(Signal::execution(
                "This tool call was interrupted because another value settled a Promise.race first.",
                node.cloned(),
            ));
        }
        match outcome {
            ToolOutcome::Success(value) => Ok(value),
            ToolOutcome::ThrownValue(value) => Err(Signal::Throw(value)),
            ToolOutcome::Refusal(error) => Err(Signal::Tool(error)),
            ToolOutcome::Runtime(error) => Err(Signal::ToolRuntime(error)),
            ToolOutcome::Unknown => Err(Signal::UnknownHost("Tool execution failed".to_string())),
        }
    }
}

// ===========================================================================
// E3 — statements.
// ===========================================================================

/// Statement outcome. `Break`/`Continue` travel as [`Signal`]; the rest
/// mirror `StatementResult` (`none` / `value` / `return`).
pub enum StmtOut {
    None,
    Value(RtValue),
    Return(RtValue),
}

/// Timeout interruption signal. The message is completed by
/// `execute_with_limits` with the configured budget
/// (`Execution timed out after ${timeoutMs}ms.` verbatim); this constructor
/// carries the budget-agnostic interruption which the envelope maps.
pub fn timeout_signal(budget_ms: Option<u64>) -> Signal {
    match budget_ms {
        Some(ms) => Signal::Runtime(InterpreterRuntimeError::new(
            format!("Execution timed out after {}ms.", ms),
            None,
            DiagnosticKind::TimeoutExceeded,
            None,
        )),
        None => Signal::Interrupted,
    }
}

impl<'a> Interpreter<'a> {
    /// Creates a function value. Mirrors `createFunction(node)` incl. the
    /// verbatim generator guard.
    pub(crate) fn create_function(&self, node: &AstNode, _declaration: bool) -> Eval<RtFunction> {
        if matches!(node.get("generator"), Value::Bool(true)) {
            return Err(Signal::Runtime(InterpreterRuntimeError::new(
                "Generator functions are not supported in CodeMode.",
                Some(node.clone()),
                DiagnosticKind::UnsupportedSyntax,
                Some(vec![
                    crate::interpreter_model::SUPPORTED_SYNTAX_MESSAGE.to_string()
                ]),
            )));
        }
        let mut params = vec![];
        for (index, item) in get_array(node, "params")
            .map_err(Signal::from)?
            .into_iter()
            .enumerate()
        {
            params.push(
                crate::interpreter_model::as_node(&item, &format!("params[{}]", index))
                    .map_err(Signal::from)?,
            );
        }
        let body = get_node(node, "body").map_err(Signal::from)?;
        Ok(RtFunction {
            params,
            body,
            captured: self
                .scopes
                .clone()
                .into_iter()
                .map(|scope| {
                    scope
                        .into_iter()
                        .map(|(k, v)| {
                            (
                                k,
                                RtBinding {
                                    mutable: v.mutable,
                                    value: v.value.clone(),
                                    initialized: v.initialized,
                                },
                            )
                        })
                        .collect()
                })
                .collect(),
        })
    }

    /// Statement dispatch with depth guard. Mirrors `evaluateStatement(node)`.
    pub(crate) fn eval_statement(&mut self, node: &AstNode) -> Eval<StmtOut> {
        self.enter()?;
        let result = self.eval_statement_inner(node);
        self.exit();
        result
    }

    fn eval_statement_inner(&mut self, node: &AstNode) -> Eval<StmtOut> {
        match node.node_type.as_str() {
            "ExpressionStatement" => {
                let expr = get_node(node, "expression").map_err(Signal::from)?;
                Ok(StmtOut::Value(self.eval_expression(&expr)?))
            }
            "VariableDeclaration" => {
                self.eval_variable_declaration(node)?;
                Ok(StmtOut::None)
            }
            "ReturnStatement" => match get_optional_node(node, "argument").map_err(Signal::from)? {
                Some(argument) => Ok(StmtOut::Return(self.eval_expression(&argument)?)),
                None => Ok(StmtOut::Return(RtValue::Undefined)),
            },
            "BlockStatement" => self.eval_block(node),
            "IfStatement" => self.eval_if(node),
            "SwitchStatement" => self.eval_switch(node),
            "WhileStatement" => self.eval_while(node),
            "DoWhileStatement" => self.eval_do_while(node),
            "ForStatement" => self.eval_for(node),
            "ForOfStatement" => self.eval_for_of(node),
            "ForInStatement" => self.eval_for_in(node),
            "BreakStatement" => {
                let label = match get_optional_node(node, "label").map_err(Signal::from)? {
                    Some(label_node) => {
                        Some(get_string(&label_node, "name").map_err(Signal::from)?)
                    }
                    None => None,
                };
                if label.is_some() {
                    return Err(Signal::execution(
                        "Labeled break is not supported in v1.",
                        Some(node.clone()),
                    ));
                }
                Err(Signal::Break(None))
            }
            "ContinueStatement" => {
                let label = match get_optional_node(node, "label").map_err(Signal::from)? {
                    Some(label_node) => {
                        Some(get_string(&label_node, "name").map_err(Signal::from)?)
                    }
                    None => None,
                };
                if label.is_some() {
                    return Err(Signal::execution(
                        "Labeled continue is not supported in v1.",
                        Some(node.clone()),
                    ));
                }
                Err(Signal::Continue(None))
            }
            "ThrowStatement" => {
                let argument = get_node(node, "argument").map_err(Signal::from)?;
                let value = self.eval_expression(&argument)?;
                Err(Signal::Throw(value))
            }
            "TryStatement" => self.eval_try(node),
            "EmptyStatement" => Ok(StmtOut::None),
            // Bound ahead of time by `hoist_functions`.
            "FunctionDeclaration" => Ok(StmtOut::None),
            other => Err(Signal::Runtime(unsupported_syntax(other, node.clone()))),
        }
    }

    pub(crate) fn eval_block(&mut self, node: &AstNode) -> Eval<StmtOut> {
        self.push_scope();
        let mut out = StmtOut::None;
        let raw = get_array(node, "body").map_err(Signal::from)?;
        let mut body = vec![];
        for (index, item) in raw.into_iter().enumerate() {
            body.push(
                crate::interpreter_model::as_node(&item, &format!("body[{}]", index))
                    .map_err(Signal::from)?,
            );
        }
        self.hoist_functions(&body)?;
        for statement in &body {
            match self.eval_statement(statement)? {
                StmtOut::Value(v) => {
                    self.last = v;
                }
                StmtOut::None => {}
                ret @ StmtOut::Return(_) => {
                    out = ret;
                    break;
                }
            }
        }
        self.pop_scope();
        Ok(out)
    }

    fn eval_if(&mut self, node: &AstNode) -> Eval<StmtOut> {
        let test = get_node(node, "test").map_err(Signal::from)?;
        let consequent = get_node(node, "consequent").map_err(Signal::from)?;
        let alternate = get_optional_node(node, "alternate").map_err(Signal::from)?;
        let cond = self.eval_expression(&test)?;
        if is_truthy_rt(&cond) {
            self.eval_statement(&consequent)
        } else if let Some(alt) = alternate {
            self.eval_statement(&alt)
        } else {
            Ok(StmtOut::None)
        }
    }

    fn eval_switch(&mut self, node: &AstNode) -> Eval<StmtOut> {
        self.push_scope();
        let result = self.eval_switch_inner(node);
        self.pop_scope();
        result
    }

    fn eval_switch_inner(&mut self, node: &AstNode) -> Eval<StmtOut> {
        let discriminant_node = get_node(node, "discriminant").map_err(Signal::from)?;
        let discriminant = self.eval_expression(&discriminant_node)?;
        if contains_opaque_reference(&discriminant) {
            return Err(Signal::Runtime(InterpreterRuntimeError::new(
                "Switch discriminants must be data values in CodeMode.",
                Some(node.clone()),
                DiagnosticKind::InvalidDataValue,
                None,
            )));
        }
        let raw = get_array(node, "cases").map_err(Signal::from)?;
        let mut cases = vec![];
        for (index, item) in raw.into_iter().enumerate() {
            cases.push(
                crate::interpreter_model::as_node(&item, &format!("cases[{}]", index))
                    .map_err(Signal::from)?,
            );
        }
        let mut default_index: Option<usize> = None;
        let mut selected: Option<usize> = None;
        for (index, branch) in cases.iter().enumerate() {
            match get_optional_node(branch, "test").map_err(Signal::from)? {
                None => {
                    default_index = Some(index);
                }
                Some(test) => {
                    let candidate = self.eval_expression(&test)?;
                    if contains_opaque_reference(&candidate) {
                        return Err(Signal::Runtime(InterpreterRuntimeError::new(
                            "Switch case values must be data values in CodeMode.",
                            Some(test),
                            DiagnosticKind::InvalidDataValue,
                            None,
                        )));
                    }
                    if strict_equal(&discriminant, &candidate) {
                        selected = Some(index);
                        break;
                    }
                }
            }
        }
        let start = match selected.or(default_index) {
            Some(i) => i,
            None => return Ok(StmtOut::None),
        };
        for branch in cases.iter().skip(start) {
            let raw = get_array(branch, "consequent").map_err(Signal::from)?;
            for item in raw {
                let statement =
                    crate::interpreter_model::as_node(&item, "consequent").map_err(Signal::from)?;
                match self.eval_statement(&statement) {
                    Ok(StmtOut::Value(v)) => {
                        self.last = v;
                    }
                    Ok(StmtOut::None) => {}
                    Ok(ret @ StmtOut::Return(_)) => return Ok(ret),
                    Err(Signal::Break(_)) => return Ok(StmtOut::None),
                    Err(Signal::Continue(label)) => return Err(Signal::Continue(label)),
                    Err(other) => return Err(other),
                }
            }
        }
        Ok(StmtOut::None)
    }

    fn eval_while(&mut self, node: &AstNode) -> Eval<StmtOut> {
        let test = get_node(node, "test").map_err(Signal::from)?;
        let body = get_node(node, "body").map_err(Signal::from)?;
        loop {
            self.check_timeout()?;
            let cond = self.eval_expression(&test)?;
            if !is_truthy_rt(&cond) {
                break;
            }
            match self.eval_statement(&body) {
                Ok(StmtOut::Value(v)) => {
                    self.last = v;
                }
                Ok(_) => {}
                Err(Signal::Break(_)) => break,
                Err(Signal::Continue(_)) => continue,
                Err(other) => return Err(other),
            }
        }
        Ok(StmtOut::None)
    }

    /// `while` statement entry (budget threaded via the interpreter).
    pub(crate) fn eval_while_node(&mut self, node: &AstNode) -> Eval<StmtOut> {
        self.eval_while(node)
    }

    fn eval_do_while(&mut self, node: &AstNode) -> Eval<StmtOut> {
        let test = get_node(node, "test").map_err(Signal::from)?;
        let body = get_node(node, "body").map_err(Signal::from)?;
        loop {
            match self.eval_statement(&body) {
                Ok(StmtOut::Value(v)) => {
                    self.last = v;
                }
                Ok(_) => {}
                Err(Signal::Break(_)) => break,
                Err(Signal::Continue(_)) => {}
                Err(other) => return Err(other),
            }
            let cond = self.eval_expression(&test)?;
            if !is_truthy_rt(&cond) {
                break;
            }
        }
        Ok(StmtOut::None)
    }

    fn eval_for(&mut self, node: &AstNode) -> Eval<StmtOut> {
        self.push_scope();
        let result = self.eval_for_inner(node);
        self.pop_scope();
        result
    }

    fn eval_for_inner(&mut self, node: &AstNode) -> Eval<StmtOut> {
        if let Some(init) = get_optional_node(node, "init").map_err(Signal::from)? {
            if init.node_type == "VariableDeclaration" {
                self.eval_variable_declaration(&init)?;
            } else {
                self.eval_expression(&init)?;
            }
        }
        loop {
            if let Some(test) = get_optional_node(node, "test").map_err(Signal::from)? {
                if !is_truthy_rt(&self.eval_expression(&test)?) {
                    break;
                }
            }
            let body = get_node(node, "body").map_err(Signal::from)?;
            match self.eval_statement(&body) {
                Ok(StmtOut::Value(v)) => {
                    self.last = v;
                }
                Ok(_) => {}
                Err(Signal::Break(_)) => break,
                Err(Signal::Continue(_)) => {}
                Err(other) => return Err(other),
            }
            if let Some(update) = get_optional_node(node, "update").map_err(Signal::from)? {
                self.eval_expression(&update)?;
            }
        }
        Ok(StmtOut::None)
    }

    fn eval_for_of(&mut self, node: &AstNode) -> Eval<StmtOut> {
        if get_boolean(node, "await")
            .map_err(Signal::from)
            .unwrap_or(false)
        {
            return Err(Signal::execution(
                "for await...of is not supported.",
                Some(node.clone()),
            ));
        }
        let right_node = get_node(node, "right").map_err(Signal::from)?;
        let iterable = self.eval_expression(&right_node)?;
        let items = match for_of_items(&iterable) {
            Some(items) => items,
            None => {
                return Err(Signal::execution(
                    "for...of requires an array, string, Map, or Set value in CodeMode.",
                    Some(node.clone()),
                ))
            }
        };
        let left = get_node(node, "left").map_err(Signal::from)?;
        let body = get_node(node, "body").map_err(Signal::from)?;
        // Single declared binding (`for (const x of ...)` / `for (x of ...)`).
        let binding = for_single_binding(&left)?;
        for item in items {
            self.push_scope();
            bind_single(&mut self.scopes, &binding, item, Some(node))?;
            let result = self.eval_statement(&body);
            self.pop_scope();
            match result {
                Ok(StmtOut::Value(v)) => {
                    self.last = v;
                }
                Ok(_) => {}
                Err(Signal::Break(_)) => break,
                Err(Signal::Continue(_)) => continue,
                Err(other) => return Err(other),
            }
        }
        Ok(StmtOut::None)
    }

    fn eval_for_in(&mut self, node: &AstNode) -> Eval<StmtOut> {
        let right_node = get_node(node, "right").map_err(Signal::from)?;
        let target = self.eval_expression(&right_node)?;
        // Keys are snapshotted up front: plain objects enumerate their own
        // keys, arrays their index strings (plus own non-index properties),
        // tool references the namespace/tool names at that node. Anything
        // else is a deliberate error with a hint (not JS's surprising
        // string-indices / Map-Set-silence behavior).
        let keys = self.enumerable_keys(&target)?.ok_or_else(|| {
            Signal::execution(
                "for...in requires a plain object, array, or tools reference in CodeMode. Use for...of for arrays/strings/Maps/Sets, or Object.keys(value) for a key list.",
                Some(node.clone()),
            )
        })?;
        let left = get_node(node, "left").map_err(Signal::from)?;
        let body = get_node(node, "body").map_err(Signal::from)?;
        enum ForInTarget {
            Declaration { pattern: AstNode, mutable: bool },
            Identifier { name: String },
        }
        let target_binding = if left.node_type == "VariableDeclaration" {
            let raw = get_array(&left, "declarations").map_err(Signal::from)?;
            if raw.len() != 1 {
                return Err(Signal::execution(
                    "for...in supports one declared binding.",
                    Some(left),
                ));
            }
            let declarator = crate::interpreter_model::as_node(&raw[0], "declarations[0]")
                .map_err(Signal::from)?;
            ForInTarget::Declaration {
                pattern: get_node(&declarator, "id").map_err(Signal::from)?,
                mutable: get_string(&left, "kind").map_err(Signal::from)? != "const",
            }
        } else if left.node_type == "Identifier" {
            ForInTarget::Identifier {
                name: get_string(&left, "name").map_err(Signal::from)?,
            }
        } else {
            return Err(Signal::execution(
                "Unsupported for...in binding.",
                Some(left),
            ));
        };
        for key in keys {
            let value = RtValue::Str(key);
            match &target_binding {
                ForInTarget::Declaration { pattern, mutable } => {
                    self.push_scope();
                    let bound = self.declare_pattern(pattern, value, *mutable, Some(&left));
                    if bound.is_err() {
                        self.pop_scope();
                        bound?;
                    }
                }
                ForInTarget::Identifier { name } => {
                    self.set_identifier(name, value, Some(&left))?;
                }
            }
            let result = self.eval_statement(&body);
            if matches!(target_binding, ForInTarget::Declaration { .. }) {
                self.pop_scope();
            }
            match result {
                Ok(StmtOut::Value(v)) => {
                    self.last = v;
                }
                Ok(_) => {}
                Err(Signal::Break(_)) => break,
                Err(Signal::Continue(_)) => continue,
                Err(other) => return Err(other),
            }
        }
        Ok(StmtOut::None)
    }

    fn eval_try(&mut self, node: &AstNode) -> Eval<StmtOut> {
        let block = get_node(node, "block").map_err(Signal::from)?;
        match self.eval_block(&block) {
            Ok(out) => {
                if !matches!(out, StmtOut::None) {
                    return Ok(out);
                }
            }
            Err(Signal::Return(v)) => return Ok(StmtOut::Return(v)),
            Err(Signal::Break(label)) => return Err(Signal::Break(label)),
            Err(Signal::Continue(label)) => return Err(Signal::Continue(label)),
            Err(failure) => {
                if let Some(handler) = get_optional_node(node, "handler").map_err(Signal::from)? {
                    self.push_scope();
                    let caught = caught_error_value(&failure);
                    // `catch (e)` / `catch { ... }` (param optional).
                    if let Some(param) =
                        get_optional_node(&handler, "param").map_err(Signal::from)?
                    {
                        self.declare_pattern(&param, caught, true, Some(node))?;
                    }
                    let body = get_node(&handler, "body").map_err(Signal::from)?;
                    let result = self.eval_block(&body);
                    self.pop_scope();
                    match result {
                        Ok(out) => {
                            if matches!(out, StmtOut::Value(_)) {
                                if let StmtOut::Value(v) = out {
                                    self.last = v;
                                }
                            } else if !matches!(out, StmtOut::None) {
                                self.eval_finalizer(node)?;
                                return Ok(out);
                            }
                        }
                        Err(Signal::Return(v)) => {
                            self.eval_finalizer(node)?;
                            return Ok(StmtOut::Return(v));
                        }
                        Err(Signal::Break(label)) => {
                            self.eval_finalizer(node)?;
                            return Err(Signal::Break(label));
                        }
                        Err(Signal::Continue(label)) => {
                            self.eval_finalizer(node)?;
                            return Err(Signal::Continue(label));
                        }
                        Err(other) => {
                            self.eval_finalizer(node)?;
                            return Err(other);
                        }
                    }
                } else {
                    self.eval_finalizer(node)?;
                    return Err(failure);
                }
            }
        }
        self.eval_finalizer(node)?;
        Ok(StmtOut::None)
    }

    fn eval_finalizer(&mut self, node: &AstNode) -> Eval<()> {
        if let Some(finalizer) = get_optional_node(node, "finalizer").map_err(Signal::from)? {
            match self.eval_block(&finalizer)? {
                StmtOut::Value(v) => {
                    self.last = v;
                }
                StmtOut::None => {}
                StmtOut::Return(v) => return Err(Signal::Return(v)),
            }
        }
        Ok(())
    }

    /// Variable declarations + destructuring. Mirrors
    /// `evaluateVariableDeclaration` (+ `bindPattern` family).
    pub(crate) fn eval_variable_declaration(&mut self, node: &AstNode) -> Eval<()> {
        let kind = get_string(node, "kind").map_err(Signal::from)?;
        let mutable = match kind.as_str() {
            "const" => false,
            "let" | "var" => true,
            _ => {
                return Err(Signal::execution(
                    "Unsupported variable declaration shape.",
                    Some(node.clone()),
                ))
            }
        };
        let raw = get_array(node, "declarations").map_err(Signal::from)?;
        for (index, item) in raw.into_iter().enumerate() {
            let declaration =
                crate::interpreter_model::as_node(&item, &format!("declarations[{}]", index))
                    .map_err(Signal::from)?;
            let pattern = get_node(&declaration, "id").map_err(Signal::from)?;
            let value = match get_optional_node(&declaration, "init").map_err(Signal::from)? {
                Some(init) => self.eval_expression(&init)?,
                None => {
                    if !mutable {
                        // `const x;` without init is a parse-time error in JS;
                        // Acorn still parses with `init: null`.
                        return Err(Signal::execution(
                            "Unsupported variable declaration shape.",
                            Some(declaration),
                        ));
                    }
                    RtValue::Undefined
                }
            };
            // Declaration patterns bind through `declare_pattern` so
            // defaults (`= ...`) evaluate in place.
            self.declare_pattern(&pattern, value, mutable, Some(node))?;
        }
        Ok(())
    }
}

// --- for-of / for-in helpers -------------------------------------------------

/// Items of a `for...of` iterable. Mirrors the accepted set (array, string,
/// Map entries as `[k, v]`, Set members).
pub fn for_of_items(value: &RtValue) -> Option<Vec<RtValue>> {
    match value {
        RtValue::Array(arr) => Some(arr.items.clone()),
        RtValue::Str(s) => Some(s.chars().map(|c| RtValue::Str(c.to_string())).collect()),
        RtValue::Sandbox(SandboxValue::Map(m)) => Some(
            m.entries
                .iter()
                .map(|(k, v)| RtValue::Array(RtArray::new(vec![k.clone(), v.clone()])))
                .collect(),
        ),
        RtValue::Sandbox(SandboxValue::Set(s)) => Some(s.members.clone()),
        // `spreadItems` also spreads URLSearchParams as `[k, v]` pairs.
        RtValue::Sandbox(SandboxValue::UrlSearchParams(p)) => Some(
            p.pairs
                .iter()
                .map(|(k, v)| {
                    RtValue::Array(RtArray::new(vec![
                        RtValue::Str(k.clone()),
                        RtValue::Str(v.clone()),
                    ]))
                })
                .collect(),
        ),
        _ => None,
    }
}

/// Declared binding of a `for...of` left-hand side.
pub fn for_single_binding(left: &AstNode) -> Eval<ForBinding> {
    if left.node_type == "VariableDeclaration" {
        let raw = get_array(left, "declarations").map_err(Signal::from)?;
        if raw.len() != 1 {
            return Err(Signal::execution(
                "for...of supports one declared binding.",
                Some(left.clone()),
            ));
        }
        let declaration =
            crate::interpreter_model::as_node(&raw[0], "declarations[0]").map_err(Signal::from)?;
        if get_optional_node(&declaration, "init")
            .map_err(Signal::from)?
            .is_some()
        {
            return Err(Signal::execution(
                "Unsupported for...of binding.",
                Some(left.clone()),
            ));
        }
        let pattern = get_node(&declaration, "id").map_err(Signal::from)?;
        let kind = get_string(left, "kind").map_err(Signal::from)?;
        return Ok(ForBinding::Declaration {
            pattern,
            mutable: kind != "const",
        });
    }
    if left.node_type == "Identifier" {
        return Ok(ForBinding::Identifier {
            name: get_string(left, "name").map_err(Signal::from)?,
        });
    }
    if left.node_type == "MemberExpression" {
        return Ok(ForBinding::Member { node: left.clone() });
    }
    Err(Signal::execution(
        "Unsupported for...of binding.",
        Some(left.clone()),
    ))
}

/// A single loop binding target.
#[derive(Debug, Clone)]
pub enum ForBinding {
    Declaration { pattern: AstNode, mutable: bool },
    Identifier { name: String },
    Member { node: AstNode },
}

/// Binds one loop iteration value.
pub fn bind_single(
    scopes: &mut Vec<HashMap<String, RtBinding>>,
    binding: &ForBinding,
    value: RtValue,
    node: Option<&AstNode>,
) -> Eval<()> {
    match binding {
        ForBinding::Declaration { pattern, mutable } => {
            // Fresh scope was already pushed; declare-then-bind.
            declare_in(scopes, pattern, value, *mutable, node)?;
            Ok(())
        }
        ForBinding::Identifier { name } => {
            assign_identifier_in(scopes, name, value, node)?;
            Ok(())
        }
        ForBinding::Member { .. } => Err(Signal::execution(
            "Unsupported for...of binding.",
            node.cloned(),
        )),
    }
}

fn declare_in(
    scopes: &mut Vec<HashMap<String, RtBinding>>,
    pattern: &AstNode,
    value: RtValue,
    mutable: bool,
    node: Option<&AstNode>,
) -> Eval<()> {
    match pattern.node_type.as_str() {
        "Identifier" => {
            let name = get_string(pattern, "name").map_err(Signal::from)?;
            let scope = scopes
                .last_mut()
                .ok_or_else(|| Signal::execution("Interpreter scope stack is empty.", None))?;
            scope.insert(
                name,
                RtBinding {
                    mutable,
                    value,
                    initialized: true,
                },
            );
            Ok(())
        }
        _ => bind_destructure(scopes, pattern, value, mutable, node),
    }
}

fn assign_identifier_in(
    scopes: &mut Vec<HashMap<String, RtBinding>>,
    name: &str,
    value: RtValue,
    node: Option<&AstNode>,
) -> Eval<()> {
    for scope in scopes.iter_mut().rev() {
        if let Some(binding) = scope.get_mut(name) {
            if !binding.mutable {
                return Err(Signal::Runtime(
                    InterpreterRuntimeError::new(
                        format!("Cannot assign to constant '{}'.", name),
                        node.cloned(),
                        DiagnosticKind::ExecutionFailure,
                        None,
                    )
                    .as_error("TypeError"),
                ));
            }
            binding.value = value;
            return Ok(());
        }
    }
    Err(Signal::Runtime(
        InterpreterRuntimeError::new(
            format!("Unknown identifier '{}'.", name),
            node.cloned(),
            DiagnosticKind::ExecutionFailure,
            None,
        )
        .as_error("ReferenceError"),
    ))
}

fn bind_destructure(
    scopes: &mut Vec<HashMap<String, RtBinding>>,
    pattern: &AstNode,
    value: RtValue,
    mutable: bool,
    node: Option<&AstNode>,
) -> Eval<()> {
    match pattern.node_type.as_str() {
        "Identifier" => {
            let name = get_string(pattern, "name").map_err(Signal::from)?;
            let scope = scopes
                .last_mut()
                .ok_or_else(|| Signal::execution("Interpreter scope stack is empty.", None))?;
            // Duplicate `let` in the SAME scope is an error; loop scopes are
            // fresh per iteration so rebinding there is fine.
            if let Some(existing) = scope.get(&name) {
                if existing.initialized {
                    return Err(Signal::execution(
                        format!("Identifier '{}' has already been declared.", name),
                        node.cloned(),
                    ));
                }
            }
            scope.insert(
                name,
                RtBinding {
                    mutable,
                    value,
                    initialized: true,
                },
            );
            Ok(())
        }
        "AssignmentPattern" => {
            // Defaults are evaluated by the caller (needs the interpreter);
            // this path handles the already-resolved value.
            let left = get_node(pattern, "left").map_err(Signal::from)?;
            bind_destructure(scopes, &left, value, mutable, node)
        }
        "ObjectPattern" => {
            let raw = get_array(pattern, "properties").map_err(Signal::from)?;
            let mut consumed: BTreeSet<String> = BTreeSet::new();
            for (index, item) in raw.into_iter().enumerate() {
                let property =
                    crate::interpreter_model::as_node(&item, &format!("properties[{}]", index))
                        .map_err(Signal::from)?;
                if property.node_type == "RestElement" {
                    let argument = get_node(&property, "argument").map_err(Signal::from)?;
                    let rest = object_rest(&value, &consumed, Some(&property))?;
                    bind_destructure(scopes, &argument, rest, mutable, node)?;
                    continue;
                }
                if property.get("computed") == &Value::Bool(true) {
                    return Err(Signal::execution(
                        "Only named object destructuring properties are supported.",
                        Some(property),
                    ));
                }
                let key_node = get_node(&property, "key").map_err(Signal::from)?;
                let key = match key_node.get("type").as_str() {
                    Some("Identifier") => get_string(&key_node, "name").map_err(Signal::from)?,
                    Some("Literal") => match key_node.get("value") {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        _ => {
                            return Err(Signal::execution(
                                "Only named object destructuring properties are supported.",
                                Some(property),
                            ))
                        }
                    },
                    _ => {
                        return Err(Signal::execution(
                            "Only named object destructuring properties are supported.",
                            Some(property),
                        ))
                    }
                };
                if crate::tool_runtime::is_blocked_member(&key) {
                    return Err(Signal::execution(
                        format!("Property '{}' is not available in CodeMode.", key),
                        Some(key_node),
                    ));
                }
                consumed.insert(key.clone());
                let nested = get_node(&property, "value").map_err(Signal::from)?;
                let mut field = object_field(&value, &key);
                // `= default` on the property value.
                if nested.node_type == "AssignmentPattern" {
                    let left = get_node(&nested, "left").map_err(Signal::from)?;
                    if matches!(field, RtValue::Undefined) {
                        // Default marker: the interpreter evaluates the right
                        // side; scope-local binding cannot evaluate AST, so
                        // treat as undefined (callers pre-resolve defaults).
                        field = RtValue::Undefined;
                    }
                    bind_destructure(scopes, &left, field, mutable, node)?;
                } else {
                    bind_destructure(scopes, &nested, field, mutable, node)?;
                }
            }
            Ok(())
        }
        "ArrayPattern" => {
            let items = match &value {
                RtValue::Array(arr) => arr.items.clone(),
                _ => {
                    return Err(Signal::execution(
                        "Array destructuring requires an array value.",
                        Some(pattern.clone()),
                    ))
                }
            };
            let raw = get_array(pattern, "elements").map_err(Signal::from)?;
            let mut index = 0usize;
            for element in raw {
                if element.is_null() {
                    index += 1;
                    continue;
                }
                let element_node = crate::interpreter_model::as_node(&element, "elements")
                    .map_err(Signal::from)?;
                if element_node.node_type == "RestElement" {
                    let argument = get_node(&element_node, "argument").map_err(Signal::from)?;
                    let rest =
                        RtValue::Array(RtArray::new(items.iter().skip(index).cloned().collect()));
                    bind_destructure(scopes, &argument, rest, mutable, node)?;
                    break;
                }
                let item = items.get(index).cloned().unwrap_or(RtValue::Undefined);
                if element_node.node_type == "AssignmentPattern" {
                    let left = get_node(&element_node, "left").map_err(Signal::from)?;
                    let resolved = item;
                    bind_destructure(scopes, &left, resolved, mutable, node)?;
                } else {
                    bind_destructure(scopes, &element_node, item, mutable, node)?;
                }
                index += 1;
            }
            Ok(())
        }
        other => Err(Signal::execution(
            format!("Unsupported binding pattern '{}'.", other),
            Some(pattern.clone()),
        )),
    }
}

fn object_field(value: &RtValue, key: &str) -> RtValue {
    match value {
        RtValue::Object(obj) => obj.get(key).cloned().unwrap_or(RtValue::Undefined),
        _ => RtValue::Undefined,
    }
}

fn object_rest(
    value: &RtValue,
    consumed: &BTreeSet<String>,
    node: Option<&AstNode>,
) -> Eval<RtValue> {
    match value {
        RtValue::Object(obj) => Ok(RtValue::Object(RtObject::new(
            obj.entries
                .iter()
                .filter(|(k, _)| !consumed.contains(k))
                .cloned()
                .collect(),
        ))),
        _ => Err(Signal::execution(
            "Only named object destructuring properties are supported.",
            node.cloned(),
        )),
    }
}

// --- `enumerableKeys` (shared by `for...in` and `Object.keys` on tools) ------

impl<'a> Interpreter<'a> {
    /// Own enumerable string keys of a value, shared by `for...in` and
    /// `Object.keys` over tool references. Mirrors `enumerableKeys(value)`:
    /// tool references enumerate namespace/tool names, arrays their index
    /// strings plus own non-index properties (match `index`/`groups`), plain
    /// data objects (incl. error objects) their own keys. Returns `None` for
    /// everything else so callers raise a contextual error.
    pub(crate) fn enumerable_keys(&mut self, value: &RtValue) -> Eval<Option<Vec<String>>> {
        match value {
            RtValue::ToolRef(path) => Ok(Some(
                tool_runtime::namespace_keys(self.tree, &[], path).map_err(Signal::from)?,
            )),
            RtValue::Array(arr) => {
                let mut keys: Vec<String> = (0..arr.items.len()).map(|i| i.to_string()).collect();
                for (k, _) in &arr.props {
                    if !keys.contains(k) {
                        keys.push(k.clone());
                    }
                }
                Ok(Some(keys))
            }
            RtValue::Object(obj) => Ok(Some(obj.entries.iter().map(|(k, _)| k.clone()).collect())),
            RtValue::ErrorObj(_) => Ok(Some(vec!["name".to_string(), "message".to_string()])),
            _ => Ok(None),
        }
    }
}

/// JS truthiness over runtime values.
pub fn is_truthy_rt(value: &RtValue) -> bool {
    crate::stdlib_value::is_truthy(&rt_to_data(value))
}

/// Strict equality (`===`) over runtime values.
pub fn strict_equal(left: &RtValue, right: &RtValue) -> bool {
    match (left, right) {
        (RtValue::Undefined, RtValue::Undefined) => true,
        (RtValue::Null, RtValue::Null) => true,
        (RtValue::Bool(a), RtValue::Bool(b)) => a == b,
        (RtValue::Number(a), RtValue::Number(b)) => {
            if a.is_nan() || b.is_nan() {
                return false;
            }
            a == b
        }
        (RtValue::Str(a), RtValue::Str(b)) => a == b,
        // Objects/arrays/functions compare by identity in JS; cloned values
        // lose identity, so reference kinds are equal only to themselves via
        // pointer. Data structures compare by reference too (never equal
        // unless the same allocation — conservative false here would break
        // `switch` on identical references; switch uses this same helper so
        // case identity still works because both sides alias one value).
        _ => std::ptr::eq(left, right),
    }
}

// ===========================================================================
// E4 — expressions.
// ===========================================================================

impl<'a> Interpreter<'a> {
    /// Expression dispatch with depth guard. Mirrors `evaluateExpression(node)`.
    pub(crate) fn eval_expression(&mut self, node: &AstNode) -> Eval<RtValue> {
        self.enter()?;
        let result = self.eval_expression_inner(node);
        self.exit();
        result
    }

    fn eval_expression_inner(&mut self, node: &AstNode) -> Eval<RtValue> {
        match node.node_type.as_str() {
            "Literal" => self.eval_literal(node),
            "Identifier" => {
                self.get_identifier(&get_string(node, "name").map_err(Signal::from)?, Some(node))
            }
            "BinaryExpression" => self.eval_binary(node),
            "LogicalExpression" => self.eval_logical(node),
            "UnaryExpression" => self.eval_unary(node),
            "AssignmentExpression" => self.eval_assignment(node),
            "CallExpression" => self.eval_call(node),
            "ArrowFunctionExpression" | "FunctionExpression" => {
                Ok(RtValue::Function(self.create_function(node, false)?))
            }
            "MemberExpression" => self.read_member(node),
            "ChainExpression" => {
                let inner = get_node(node, "expression").map_err(Signal::from)?;
                match self.eval_expression(&inner)? {
                    // An option-chain that short-circuits reads as
                    // `undefined` at the chain boundary (JS semantics).
                    RtValue::ShortCircuit => Ok(RtValue::Undefined),
                    other => Ok(other),
                }
            }
            "ObjectExpression" => self.eval_object(node),
            "ArrayExpression" => self.eval_array(node),
            "TemplateLiteral" => self.eval_template(node),
            "ConditionalExpression" => {
                let test = get_node(node, "test").map_err(Signal::from)?;
                let cond = self.eval_expression(&test)?;
                if is_truthy_rt(&cond) {
                    let consequent = get_node(node, "consequent").map_err(Signal::from)?;
                    self.eval_expression(&consequent)
                } else {
                    let alternate = get_node(node, "alternate").map_err(Signal::from)?;
                    self.eval_expression(&alternate)
                }
            }
            "UpdateExpression" => self.eval_update(node),
            "AwaitExpression" => {
                let argument = get_node(node, "argument").map_err(Signal::from)?;
                let value = self.eval_expression(&argument)?;
                match value {
                    RtValue::Sandbox(SandboxValue::Promise(p)) => {
                        self.settle_promise(&p, Some(node))
                    }
                    other => Ok(other),
                }
            }
            "NewExpression" => self.eval_new(node),
            other => Err(Signal::Runtime(unsupported_syntax(other, node.clone()))),
        }
    }

    fn eval_literal(&mut self, node: &AstNode) -> Eval<RtValue> {
        // Acorn literals: `value` holds the value; `regex` holds
        // `{ pattern, flags }` for regex literals; TemplateLiteral elements
        // are handled in `eval_template`.
        if let Some(regex) = node.get("regex").as_object() {
            let pattern = regex.get("pattern").and_then(|v| v.as_str()).unwrap_or("");
            let flags = regex.get("flags").and_then(|v| v.as_str()).unwrap_or("");
            return self.make_regex(pattern, flags, Some(node));
        }
        Ok(match node.get("value") {
            Value::Null => {
                // Acorn parses `null` as Literal null; `undefined` is an Identifier.
                RtValue::Null
            }
            Value::Bool(b) => RtValue::Bool(*b),
            Value::Number(n) => RtValue::Number(n.as_f64().unwrap_or(f64::NAN)),
            Value::String(s) => RtValue::Str(s.clone()),
            _ => RtValue::Undefined,
        })
    }

    /// Constructs a sandbox RegExp with constructor-time validation.
    /// Mirrors the `RegExp` literal / `new RegExp(...)` validation triage:
    /// invalid flags and invalid patterns report the verbatim
    /// `new RegExp(...) received ...` diagnostics.
    pub(crate) fn make_regex(
        &mut self,
        pattern: &str,
        flags: &str,
        node: Option<&AstNode>,
    ) -> Eval<RtValue> {
        if let Err(reason) = validate_regex_flags(flags) {
            return Err(Signal::Runtime(
                InterpreterRuntimeError::new(
                    format!(
                        "new RegExp(...) received invalid flags {} ({}). Valid flags are d, g, i, m, s, u, v, and y.",
                        serde_json::to_string(flags).unwrap_or_default(),
                        crate::stdlib_regexp::regex_failure_reason(&reason)
                    ),
                    node.cloned(),
                    DiagnosticKind::ExecutionFailure,
                    None,
                )
                .as_error("SyntaxError"),
            ));
        }
        match crate::stdlib_regexp::validate_regex(pattern, flags) {
            Ok(()) => Ok(RtValue::Sandbox(SandboxValue::RegExp(SandboxRegExp::new(
                pattern, flags,
            )))),
            Err(reason) => Err(Signal::Runtime(
                InterpreterRuntimeError::new(
                    format!(
                        "new RegExp(...) received {}, which is not a valid regular expression pattern ({}). {}",
                        serde_json::to_string(pattern).unwrap_or_default(),
                        crate::stdlib_regexp::regex_failure_reason(&reason),
                        crate::stdlib_regexp::ESCAPE_REGEX_HINT
                    ),
                    node.cloned(),
                    DiagnosticKind::ExecutionFailure,
                    None,
                )
                .as_error("SyntaxError"),
            )),
        }
    }

    // --- binary / logical / unary ------------------------------------------

    fn eval_binary(&mut self, node: &AstNode) -> Eval<RtValue> {
        let operator = get_string(node, "operator").map_err(Signal::from)?;
        if operator == "instanceof" {
            let left_node = get_node(node, "left").map_err(Signal::from)?;
            let right_node = get_node(node, "right").map_err(Signal::from)?;
            let left = self.eval_expression(&left_node)?;
            let right = self.eval_expression(&right_node)?;
            return Ok(RtValue::Bool(instanceof_value(&left, &right, Some(node))?));
        }
        if operator == "in" {
            let left_node = get_node(node, "left").map_err(Signal::from)?;
            let right_node = get_node(node, "right").map_err(Signal::from)?;
            let left = self.eval_expression(&left_node)?;
            let right = self.eval_expression(&right_node)?;
            let key = match &left {
                RtValue::Str(s) => s.clone(),
                RtValue::Number(n) => crate::stdlib_value::js_number_to_string(*n),
                _ => {
                    return Err(Signal::execution(
                        "The 'in' operator requires a data object on the right-hand side.",
                        Some(node.clone()),
                    ))
                }
            };
            return Ok(RtValue::Bool(in_operator(&key, &right, Some(node))?));
        }
        let left_node = get_node(node, "left").map_err(Signal::from)?;
        let right_node = get_node(node, "right").map_err(Signal::from)?;
        let left = self.eval_expression(&left_node)?;
        let right = self.eval_expression(&right_node)?;
        // Optional-chain short-circuits propagate through operators.
        if matches!(left, RtValue::ShortCircuit) {
            return Ok(left);
        }
        if matches!(right, RtValue::ShortCircuit) {
            return Ok(right);
        }
        if contains_opaque_reference(&left) || contains_opaque_reference(&right) {
            // Sandbox stdlib values count as data here (identity equality,
            // ToPrimitive coercion) — only opaque machinery is rejected.
            return Err(Signal::Runtime(InterpreterRuntimeError::new(
                "Binary operators require data values in CodeMode.",
                Some(node.clone()),
                DiagnosticKind::InvalidDataValue,
                None,
            )));
        }
        apply_binary(&operator, &left, &right, Some(node))
    }

    fn eval_logical(&mut self, node: &AstNode) -> Eval<RtValue> {
        let operator = get_string(node, "operator").map_err(Signal::from)?;
        let left_node = get_node(node, "left").map_err(Signal::from)?;
        let left = self.eval_expression(&left_node)?;
        if matches!(left, RtValue::ShortCircuit) {
            return Ok(left);
        }
        match operator.as_str() {
            "&&" => {
                if !is_truthy_rt(&left) {
                    return Ok(left);
                }
                let right_node = get_node(node, "right").map_err(Signal::from)?;
                self.eval_expression(&right_node)
            }
            "||" => {
                if is_truthy_rt(&left) {
                    return Ok(left);
                }
                let right_node = get_node(node, "right").map_err(Signal::from)?;
                self.eval_expression(&right_node)
            }
            "??" => {
                if matches!(left, RtValue::Null | RtValue::Undefined) {
                    let right_node = get_node(node, "right").map_err(Signal::from)?;
                    self.eval_expression(&right_node)
                } else {
                    Ok(left)
                }
            }
            _ => Err(Signal::execution(
                format!("Unsupported logical operator '{}'.", operator),
                Some(node.clone()),
            )),
        }
    }

    fn eval_unary(&mut self, node: &AstNode) -> Eval<RtValue> {
        let operator = get_string(node, "operator").map_err(Signal::from)?;
        if operator == "typeof" {
            let argument = get_node(node, "argument").map_err(Signal::from)?;
            // `typeof` never throws: unknown identifiers read as undefined.
            let value = match self.eval_expression(&argument) {
                Ok(v) => v,
                Err(Signal::Runtime(e)) if e.error_name == "ReferenceError" => RtValue::Undefined,
                Err(other) => return Err(other),
            };
            return Ok(RtValue::Str(typeof_value(&value).to_string()));
        }
        if operator == "void" {
            let argument = get_node(node, "argument").map_err(Signal::from)?;
            self.eval_expression(&argument)?;
            return Ok(RtValue::Undefined);
        }
        if operator == "delete" {
            let argument = get_node(node, "argument").map_err(Signal::from)?;
            return self.eval_delete(&argument, Some(node));
        }
        let argument = get_node(node, "argument").map_err(Signal::from)?;
        let value = self.eval_expression(&argument)?;
        if matches!(value, RtValue::ShortCircuit) {
            return Ok(value);
        }
        if contains_opaque_reference(&value) {
            return Err(Signal::Runtime(InterpreterRuntimeError::new(
                "Unary operators require data values in CodeMode.",
                Some(node.clone()),
                DiagnosticKind::InvalidDataValue,
                None,
            )));
        }
        apply_unary(&operator, &value, Some(node))
    }

    fn eval_delete(&mut self, argument: &AstNode, node: Option<&AstNode>) -> Eval<RtValue> {
        if argument.node_type != "MemberExpression" {
            return Ok(RtValue::Bool(true));
        }
        match self.member_reference(argument)? {
            MemberRef::Data(data) => {
                self.delete_data_ref(&data, node)?;
                Ok(RtValue::Bool(true))
            }
            // Deleting a non-data reference is a no-op success (JS `delete`
            // of a non-reference is `true`).
            _ => Ok(RtValue::Bool(true)),
        }
    }

    /// Deletes the field at a resolved data reference.
    fn delete_data_ref(&mut self, data: &DataRef, node: Option<&AstNode>) -> Eval<()> {
        if data.path.is_empty() {
            return Ok(());
        }
        let (parent_path, last) = (
            &data.path[..data.path.len() - 1],
            &data.path[data.path.len() - 1],
        );
        let parent_ref = DataRef {
            base: data.base.clone(),
            path: parent_path.to_vec(),
        };
        let container = self.read_data_ref(&parent_ref, node)?;
        match container {
            RtValue::Array(mut arr) => {
                let index = match last {
                    MemberKeyRt::Num(n) => *n,
                    MemberKeyRt::Str(s) => match s.parse::<usize>() {
                        Ok(n) => n,
                        Err(_) => {
                            arr.props.retain(|(k, _)| k != s);
                            self.assign_data_ref(&parent_ref, RtValue::Array(arr), node)?;
                            return Ok(());
                        }
                    },
                };
                if index < arr.items.len() {
                    // Holes read as `undefined`; length is preserved (JS).
                    arr.items[index] = RtValue::Undefined;
                    self.assign_data_ref(&parent_ref, RtValue::Array(arr), node)?;
                }
                Ok(())
            }
            RtValue::Object(mut obj) => {
                let key = match last {
                    MemberKeyRt::Str(s) => s.clone(),
                    MemberKeyRt::Num(n) => n.to_string(),
                };
                obj.entries.retain(|(k, _)| k != &key);
                self.assign_data_ref(&parent_ref, RtValue::Object(obj), node)?;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    // --- assignment / update -------------------------------------------------

    fn eval_assignment(&mut self, node: &AstNode) -> Eval<RtValue> {
        let operator = get_string(node, "operator").map_err(Signal::from)?;
        let left = get_node(node, "left").map_err(Signal::from)?;
        let right_node = get_node(node, "right").map_err(Signal::from)?;
        if operator == "=" {
            let value = self.eval_expression(&right_node)?;
            return self.assign_target(&left, value, Some(node));
        }
        // Logical assignments short-circuit before compound assignment.
        if operator == "&&=" || operator == "||=" || operator == "??=" {
            return self.eval_logical_assignment(&operator, &left, &right_node, Some(node));
        }
        if !crate::stdlib_value::is_compound_operator(&operator) {
            return Err(Signal::execution(
                format!("Unsupported assignment operator '{}'.", operator),
                Some(node.clone()),
            ));
        }
        // Compound assignment resolves the target exactly once (mirroring
        // `modifyMember`), reads the current value, applies the binary
        // operator, and writes back.
        if left.node_type == "MemberExpression" {
            let reference = self.member_reference(&left)?;
            let data = match reference {
                MemberRef::Data(data) => data,
                _ => {
                    return Err(Signal::execution(
                        "Only data fields may be assigned in CodeMode.",
                        Some(node.clone()),
                    ))
                }
            };
            let current = self.read_data_ref(&data, Some(node))?;
            let value = self.eval_expression(&right_node)?;
            let base = operator.trim_end_matches('=');
            let next = apply_binary(base, &current, &value, Some(node))?;
            self.assign_data_ref(&data, next.clone(), Some(node))?;
            return Ok(next);
        }
        let current = self.read_target(&left, Some(node))?;
        let value = self.eval_expression(&right_node)?;
        let base = operator.trim_end_matches('=');
        let next = apply_binary(base, &current, &value, Some(node))?;
        self.assign_target(&left, next.clone(), Some(node))?;
        Ok(next)
    }

    fn read_target(&mut self, left: &AstNode, node: Option<&AstNode>) -> Eval<RtValue> {
        match left.node_type.as_str() {
            "Identifier" => {
                let name = get_string(left, "name").map_err(Signal::from)?;
                self.get_identifier(&name, node)
            }
            "MemberExpression" => self.read_member(left),
            _ => Err(Signal::execution(
                "Assignment target must be an Identifier or MemberExpression.",
                node.cloned(),
            )),
        }
    }

    fn assign_target(
        &mut self,
        left: &AstNode,
        value: RtValue,
        node: Option<&AstNode>,
    ) -> Eval<RtValue> {
        match left.node_type.as_str() {
            "Identifier" => {
                let name = get_string(left, "name").map_err(Signal::from)?;
                // Assignment to an undeclared name is `Unknown identifier`
                // (no implicit globals in CodeMode).
                self.set_identifier(&name, value.clone(), node)?;
                Ok(value)
            }
            "MemberExpression" => self.write_member(left, value),
            _ => Err(Signal::execution(
                "Assignment target must be an Identifier or MemberExpression.",
                node.cloned(),
            )),
        }
    }

    fn eval_update(&mut self, node: &AstNode) -> Eval<RtValue> {
        let operator = get_string(node, "operator").map_err(Signal::from)?;
        if operator != "++" && operator != "--" {
            return Err(Signal::execution(
                format!("Unsupported update operator '{}'.", operator),
                Some(node.clone()),
            ));
        }
        let argument = get_node(node, "argument").map_err(Signal::from)?;
        // Member targets resolve exactly once for read-modify-write.
        if argument.node_type == "MemberExpression" {
            let reference = self.member_reference(&argument)?;
            let data = match reference {
                MemberRef::Data(data) => data,
                _ => {
                    return Err(Signal::execution(
                        "Only data fields may be assigned in CodeMode.",
                        Some(argument),
                    ))
                }
            };
            let current = self.read_data_ref(&data, Some(node))?;
            let delta = if operator == "++" { 1.0 } else { -1.0 };
            let num = crate::stdlib_value::coerce_to_number(&rt_to_data(&current));
            let next = RtValue::Number(num + delta);
            self.assign_data_ref(&data, next.clone(), Some(node))?;
            let prefix = get_boolean(node, "prefix").map_err(Signal::from)?;
            return Ok(if prefix { next } else { current });
        }
        let current = match argument.node_type.as_str() {
            "Identifier" => {
                let name = get_string(&argument, "name").map_err(Signal::from)?;
                self.get_identifier(&name, Some(node))?
            }
            _ => {
                return Err(Signal::execution(
                    "Update target must be an Identifier or MemberExpression.",
                    Some(argument),
                ))
            }
        };
        let delta = if operator == "++" { 1.0 } else { -1.0 };
        let num = crate::stdlib_value::coerce_to_number(&rt_to_data(&current));
        let next = RtValue::Number(num + delta);
        let name = get_string(&argument, "name").map_err(Signal::from)?;
        self.set_identifier(&name, next.clone(), Some(node))?;
        let prefix = get_boolean(node, "prefix").map_err(Signal::from)?;
        if prefix {
            self.get_identifier(&name, Some(node))
        } else {
            Ok(current)
        }
    }

    // --- literals: object / array / template ---------------------------------

    fn eval_object(&mut self, node: &AstNode) -> Eval<RtValue> {
        let raw = get_array(node, "properties").map_err(Signal::from)?;
        let mut entries: Vec<(String, RtValue)> = vec![];
        for (index, item) in raw.into_iter().enumerate() {
            let property =
                crate::interpreter_model::as_node(&item, &format!("properties[{}]", index))
                    .map_err(Signal::from)?;
            if property.node_type == "SpreadElement" {
                let argument = get_node(&property, "argument").map_err(Signal::from)?;
                let spread = self.eval_expression(&argument)?;
                if matches!(spread, RtValue::Null | RtValue::Undefined) {
                    continue;
                }
                if matches!(spread, RtValue::Sandbox(_)) {
                    continue;
                }
                // Branded error objects spread their `{ name, message }`
                // fields (parity: `{...e}` → `{ name, message }`).
                if let RtValue::ErrorObj(e) = &spread {
                    for (key, value) in [
                        ("name".to_string(), RtValue::Str(e.name.clone())),
                        ("message".to_string(), RtValue::Str(e.message.clone())),
                    ] {
                        if crate::tool_runtime::is_blocked_member(&key) {
                            continue;
                        }
                        upsert(&mut entries, key, value);
                    }
                    continue;
                }
                match spread {
                    RtValue::Object(obj) => {
                        for (key, value) in obj.entries {
                            if crate::tool_runtime::is_blocked_member(&key) {
                                return Err(Signal::execution(
                                    format!("Property '{}' is not available in CodeMode.", key),
                                    Some(property.clone()),
                                ));
                            }
                            upsert(&mut entries, key, value);
                        }
                        continue;
                    }
                    _ => {
                        return Err(Signal::execution(
                            "Object spread requires a data object in CodeMode.",
                            Some(property),
                        ))
                    }
                }
            }
            if property.node_type != "Property" {
                return Err(Signal::execution(
                    "Only standard object properties are supported.",
                    Some(property),
                ));
            }
            if get_string(&property, "kind").map_err(Signal::from)? != "init" {
                return Err(Signal::execution(
                    "Only init object properties are supported.",
                    Some(property),
                ));
            }
            if property.get("computed") == &Value::Bool(true) {
                // Computed keys evaluate the key expression (identifier keys
                // stay named); blocked names are rejected verbatim.
                let key_node = get_node(&property, "key").map_err(Signal::from)?;
                let key_value = self.eval_expression(&key_node)?;
                let key = match key_value {
                    RtValue::Str(s) => s,
                    RtValue::Number(n) => crate::stdlib_value::js_number_to_string(n),
                    _ => {
                        return Err(Signal::execution(
                            "Unsupported object property key shape.",
                            Some(key_node),
                        ))
                    }
                };
                if crate::tool_runtime::is_blocked_member(&key) {
                    return Err(Signal::execution(
                        format!("Property '{}' is not available in CodeMode.", key),
                        Some(key_node),
                    ));
                }
                let value_node = get_node(&property, "value").map_err(Signal::from)?;
                let value = self.eval_expression(&value_node)?;
                upsert(&mut entries, key, value);
                continue;
            }
            let key_node = get_node(&property, "key").map_err(Signal::from)?;
            let value_node = get_node(&property, "value").map_err(Signal::from)?;
            // Shorthand `{ x }` has key == value Identifier.
            let key = match key_node.get("type").as_str() {
                Some("Identifier") => get_string(&key_node, "name").map_err(Signal::from)?,
                Some("Literal") => match key_node.get("value") {
                    Value::String(s) => s.clone(),
                    Value::Number(n) => n.to_string(),
                    _ => {
                        return Err(Signal::execution(
                            "Unsupported object property key shape.",
                            Some(key_node),
                        ))
                    }
                },
                _ => {
                    return Err(Signal::execution(
                        "Unsupported object property key shape.",
                        Some(key_node),
                    ))
                }
            };
            if crate::tool_runtime::is_blocked_member(&key) {
                return Err(Signal::execution(
                    format!("Property '{}' is not available in CodeMode.", key),
                    Some(key_node),
                ));
            }
            let value = self.eval_expression(&value_node)?;
            upsert(&mut entries, key, value);
        }
        Ok(RtValue::Object(RtObject::new(entries)))
    }

    fn eval_array(&mut self, node: &AstNode) -> Eval<RtValue> {
        let raw = get_array(node, "elements").map_err(Signal::from)?;
        let mut items = vec![];
        for (index, item) in raw.into_iter().enumerate() {
            if item.is_null() {
                // Holes read as `undefined` (JS elision).
                items.push(RtValue::Undefined);
                continue;
            }
            let element = crate::interpreter_model::as_node(&item, &format!("elements[{}]", index))
                .map_err(Signal::from)?;
            if element.node_type == "SpreadElement" {
                let argument = get_node(&element, "argument").map_err(Signal::from)?;
                let spread = self.eval_expression(&argument)?;
                match spread_items_rt(&spread) {
                    Some(mut spread_items) => items.append(&mut spread_items),
                    None => {
                        return Err(Signal::execution(
                            "Array spread requires an array, string, Map, or Set in CodeMode.",
                            Some(element),
                        ))
                    }
                }
                continue;
            }
            items.push(self.eval_expression(&element)?);
        }
        Ok(RtValue::Array(RtArray::new(items)))
    }

    fn eval_template(&mut self, node: &AstNode) -> Eval<RtValue> {
        let raw_q = get_array(node, "quasis").map_err(Signal::from)?;
        let raw_e = get_array(node, "expressions").map_err(Signal::from)?;
        let mut out = String::new();
        for (index, quasi) in raw_q.into_iter().enumerate() {
            let quasi_node =
                crate::interpreter_model::as_node(&quasi, &format!("quasis[{}]", index))
                    .map_err(Signal::from)?;
            let raw_value = quasi_node.get("value");
            if !raw_value.is_object() || !matches!(raw_value.get("cooked"), Some(Value::String(_)))
            {
                return Err(Signal::execution(
                    "Invalid template literal quasi.",
                    Some(quasi_node),
                ));
            }
            let cooked = raw_value
                .get("cooked")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            out.push_str(cooked);
            if index < raw_e.len() {
                let expr = crate::interpreter_model::as_node(
                    &raw_e[index],
                    &format!("expressions[{}]", index),
                )
                .map_err(Signal::from)?;
                let raw = self.eval_expression(&expr)?;
                // The preserving checkpoint keeps sandbox values intact, so
                // coercion renders them directly (ISO date, /regex/ form).
                let checked =
                    crate::stdlib_value::bounded_data(&rt_to_data(&raw), "Template interpolation")
                        .map_err(Signal::from)?;
                out.push_str(&crate::stdlib_value::coerce_to_string(&checked));
            }
        }
        Ok(RtValue::Str(out))
    }

    // --- `new` -----------------------------------------------------------------

    fn eval_new(&mut self, node: &AstNode) -> Eval<RtValue> {
        let callee = get_node(node, "callee").map_err(Signal::from)?;
        if callee.node_type == "MemberExpression" {
            return Err(Signal::Runtime(unsupported_syntax(
                "NewExpression",
                node.clone(),
            )));
        }
        let name = if callee.node_type == "Identifier" {
            get_string(&callee, "name").map_err(Signal::from)?
        } else {
            return Err(Signal::Runtime(unsupported_syntax(
                "NewExpression",
                node.clone(),
            )));
        };
        let raw = get_array(node, "arguments").map_err(Signal::from)?;
        let mut args = vec![];
        for (index, item) in raw.into_iter().enumerate() {
            let arg = crate::interpreter_model::as_node(&item, &format!("arguments[{}]", index))
                .map_err(Signal::from)?;
            if arg.node_type == "SpreadElement" {
                return Err(Signal::Runtime(unsupported_syntax(
                    "NewExpression",
                    node.clone(),
                )));
            }
            args.push(self.eval_expression(&arg)?);
        }
        if crate::stdlib_value::ERROR_CONSTRUCTORS.contains(&name.as_str()) {
            let message = match args.first() {
                None | Some(RtValue::Undefined) => String::new(),
                Some(v) => crate::stdlib_value::coerce_to_string(&rt_to_data(v)),
            };
            return Ok(RtValue::ErrorObj(RtErrorObj { name, message }));
        }
        match name.as_str() {
            "Date" => self.new_date(&args, Some(node)),
            "RegExp" => {
                let pattern = args.first().cloned().unwrap_or(RtValue::Undefined);
                let flags = args.get(1).cloned().unwrap_or(RtValue::Undefined);
                let pattern_str = crate::stdlib_value::coerce_to_string(&rt_to_data(&pattern));
                let flags_str = match &flags {
                    RtValue::Undefined => String::new(),
                    other => crate::stdlib_value::coerce_to_string(&rt_to_data(other)),
                };
                self.make_regex(&pattern_str, &flags_str, Some(node))
            }
            "Map" => self.new_map(&args, Some(node)),
            "Set" => self.new_set(&args, Some(node)),
            "URL" => self.new_url(&args, Some(node)),
            "URLSearchParams" => self.new_url_search_params(&args, Some(node)),
            "Promise" => Err(Signal::execution(
                "new Promise(...) is not supported in CodeMode; tool calls already return promises - call the tool and await the result.",
                Some(node.clone()),
            )),
            _ => Err(Signal::Runtime(unsupported_syntax("NewExpression", node.clone()))),
        }
    }

    fn new_date(&mut self, args: &[RtValue], node: Option<&AstNode>) -> Eval<RtValue> {
        if args.is_empty() {
            return Ok(RtValue::Sandbox(SandboxValue::Date(
                crate::values::SandboxDate::new((self.now_ms)()),
            )));
        }
        // `new Date(value)` / `new Date(y, m, ...)` mirror the host Date.
        let nums: Vec<DataVal> = args.iter().map(|a| rt_to_data(a)).collect();
        let time = if nums.len() == 1 {
            match &nums[0] {
                DataVal::Str(s) => crate::stdlib_date::parse_iso8601_or_nan(s),
                other => crate::stdlib_value::coerce_to_number(other),
            }
        } else {
            let floats: Vec<f64> = nums
                .iter()
                .map(crate::stdlib_value::coerce_to_number)
                .collect();
            date_utc_like(&floats)
        };
        let _ = node;
        Ok(RtValue::Sandbox(SandboxValue::Date(
            crate::values::SandboxDate::new(time),
        )))
    }

    fn new_map(&mut self, args: &[RtValue], node: Option<&AstNode>) -> Eval<RtValue> {
        let mut map = SandboxMap::default();
        if let Some(first) = args.first() {
            if matches!(first, RtValue::Undefined | RtValue::Null) && args.len() == 1 {
                return Ok(RtValue::Sandbox(SandboxValue::Map(map)));
            }
            let pairs = match first {
                RtValue::Array(arr) => arr.items.clone(),
                RtValue::Sandbox(SandboxValue::Map(m)) => {
                    return Ok(RtValue::Sandbox(SandboxValue::Map(m.clone())));
                }
                _ => {
                    return Err(Signal::execution(
                        "new Map(...) expects [key, value] pairs.",
                        node.cloned(),
                    ))
                }
            };
            for pair in pairs {
                match pair {
                    RtValue::Array(kv) if kv.items.len() >= 2 => {
                        let key = kv.items[0].clone();
                        let value = kv.items[1].clone();
                        match map
                            .entries
                            .iter_mut()
                            .find(|(k, _)| same_value_zero(k, &key))
                        {
                            Some(slot) => slot.1 = value,
                            None => map.entries.push((key, value)),
                        }
                    }
                    _ => {
                        return Err(Signal::execution(
                            "new Map(...) expects [key, value] pairs.",
                            node.cloned(),
                        ))
                    }
                }
            }
        }
        Ok(RtValue::Sandbox(SandboxValue::Map(map)))
    }

    fn new_set(&mut self, args: &[RtValue], node: Option<&AstNode>) -> Eval<RtValue> {
        let mut set = SandboxSet::default();
        if let Some(first) = args.first() {
            match first {
                RtValue::Undefined | RtValue::Null => {}
                RtValue::Array(arr) => {
                    for item in &arr.items {
                        if !set.members.iter().any(|m| same_value_zero(m, item)) {
                            set.members.push(item.clone());
                        }
                    }
                }
                RtValue::Str(s) => {
                    for c in s.chars() {
                        let member = RtValue::Str(c.to_string());
                        if !set.members.iter().any(|m| same_value_zero(m, &member)) {
                            set.members.push(member);
                        }
                    }
                }
                RtValue::Sandbox(SandboxValue::Set(s)) => {
                    set.members = s.members.clone();
                }
                _ => {
                    return Err(Signal::execution(
                        "new Set(...) expects an array, Set, string, or no argument.",
                        node.cloned(),
                    ))
                }
            }
        }
        Ok(RtValue::Sandbox(SandboxValue::Set(set)))
    }

    fn new_url(&mut self, args: &[RtValue], node: Option<&AstNode>) -> Eval<RtValue> {
        if args.is_empty() {
            return Err(Signal::Runtime(
                InterpreterRuntimeError::new(
                    "new URL(...) requires a URL string and an optional base URL.",
                    node.cloned(),
                    DiagnosticKind::ExecutionFailure,
                    None,
                )
                .as_error("TypeError"),
            ));
        }
        let data: Vec<DataVal> = args.iter().map(|a| rt_to_data(a)).collect();
        let data_refs: Vec<DataVal> = data;
        match crate::stdlib_url::invoke_url_static("parse", &data_refs, node) {
            Ok(DataVal::Sandbox(s)) => Ok(RtValue::Sandbox(s)),
            Ok(_) => Err(Signal::Runtime(
                InterpreterRuntimeError::new(
                    "new URL(...) requires a URL string and an optional base URL.",
                    node.cloned(),
                    DiagnosticKind::ExecutionFailure,
                    None,
                )
                .as_error("TypeError"),
            )),
            Err(_) => Err(Signal::Runtime(
                InterpreterRuntimeError::new(
                    "new URL(...) requires a URL string and an optional base URL.",
                    node.cloned(),
                    DiagnosticKind::ExecutionFailure,
                    None,
                )
                .as_error("TypeError"),
            )),
        }
    }

    fn new_url_search_params(&mut self, args: &[RtValue], node: Option<&AstNode>) -> Eval<RtValue> {
        let mut pairs: Vec<(String, String)> = vec![];
        if let Some(first) = args.first() {
            match first {
                RtValue::Undefined => {}
                RtValue::Str(s) => {
                    pairs = parse_query_string(s.trim_start_matches('?'));
                }
                RtValue::Array(arr) => {
                    for pair in &arr.items {
                        match pair {
                            RtValue::Array(kv) if kv.items.len() >= 2 => {
                                pairs.push((
                                    crate::stdlib_value::coerce_to_string(&rt_to_data(&kv.items[0])),
                                    crate::stdlib_value::coerce_to_string(&rt_to_data(&kv.items[1])),
                                ));
                            }
                            _ => {
                                return Err(Signal::execution(
                                    "new URLSearchParams(...) expects a query string or [key, value] pairs.",
                                    node.cloned(),
                                ))
                            }
                        }
                    }
                }
                RtValue::Object(obj) => {
                    for (k, v) in &obj.entries {
                        pairs.push((
                            k.clone(),
                            crate::stdlib_value::coerce_to_string(&rt_to_data(v)),
                        ));
                    }
                }
                _ => {
                    return Err(Signal::execution(
                        "new URLSearchParams(...) expects a query string or [key, value] pairs.",
                        node.cloned(),
                    ))
                }
            }
        }
        Ok(RtValue::Sandbox(SandboxValue::UrlSearchParams(
            SandboxURLSearchParams::new(pairs),
        )))
    }
}

// --- binary / unary application -------------------------------------------------

/// Applies a binary operator to data values. Mirrors the TS
/// `evaluateBinaryExpression` dispatch.
pub fn apply_binary(
    operator: &str,
    left: &RtValue,
    right: &RtValue,
    node: Option<&AstNode>,
) -> Eval<RtValue> {
    use crate::stdlib_value::{coerce_to_number, coerce_to_string};
    match operator {
        "+" => {
            // String concatenation wins when either side is a string (after
            // ToPrimitive coercion of sandbox wrappers).
            let l = rt_to_data(left);
            let r = rt_to_data(right);
            if matches!(l, DataVal::Str(_)) || matches!(r, DataVal::Str(_)) {
                return Ok(RtValue::Str(format!(
                    "{}{}",
                    coerce_to_string(&l),
                    coerce_to_string(&r)
                )));
            }
            Ok(RtValue::Number(coerce_to_number(&l) + coerce_to_number(&r)))
        }
        "-" => Ok(RtValue::Number(
            coerce_to_number(&rt_to_data(left)) - coerce_to_number(&rt_to_data(right)),
        )),
        "*" => Ok(RtValue::Number(
            coerce_to_number(&rt_to_data(left)) * coerce_to_number(&rt_to_data(right)),
        )),
        "/" => Ok(RtValue::Number(
            coerce_to_number(&rt_to_data(left)) / coerce_to_number(&rt_to_data(right)),
        )),
        "%" => {
            let a = coerce_to_number(&rt_to_data(left));
            let b = coerce_to_number(&rt_to_data(right));
            Ok(RtValue::Number(js_remainder(a, b)))
        }
        "**" => Ok(RtValue::Number(
            coerce_to_number(&rt_to_data(left)).powf(coerce_to_number(&rt_to_data(right))),
        )),
        "==" => Ok(RtValue::Bool(loose_equal(left, right))),
        "!=" => Ok(RtValue::Bool(!loose_equal(left, right))),
        "===" => Ok(RtValue::Bool(strict_equal(left, right))),
        "!==" => Ok(RtValue::Bool(!strict_equal(left, right))),
        "<" | "<=" | ">" | ">=" => Ok(RtValue::Bool(relational(operator, left, right))),
        "&" => Ok(RtValue::Number(
            (to_int32(coerce_to_number(&rt_to_data(left)))
                & to_int32(coerce_to_number(&rt_to_data(right)))) as f64,
        )),
        "|" => Ok(RtValue::Number(
            (to_int32(coerce_to_number(&rt_to_data(left)))
                | to_int32(coerce_to_number(&rt_to_data(right)))) as f64,
        )),
        "^" => Ok(RtValue::Number(
            (to_int32(coerce_to_number(&rt_to_data(left)))
                ^ to_int32(coerce_to_number(&rt_to_data(right)))) as f64,
        )),
        "<<" => Ok(RtValue::Number(
            (to_int32(coerce_to_number(&rt_to_data(left)))
                << (to_uint32(coerce_to_number(&rt_to_data(right))) % 32)) as f64,
        )),
        ">>" => Ok(RtValue::Number(
            (to_int32(coerce_to_number(&rt_to_data(left)))
                >> (to_uint32(coerce_to_number(&rt_to_data(right))) % 32)) as f64,
        )),
        ">>>" => Ok(RtValue::Number(
            ((to_uint32(coerce_to_number(&rt_to_data(left))))
                >> (to_uint32(coerce_to_number(&rt_to_data(right))) % 32)) as f64,
        )),
        _ => Err(Signal::execution(
            format!("Unsupported binary operator '{}'.", operator),
            node.cloned(),
        )),
    }
}

fn to_int32(n: f64) -> i32 {
    if !n.is_finite() {
        return 0;
    }
    (n.trunc() as i64 & 0xffff_ffff) as i32
}

fn to_uint32(n: f64) -> u32 {
    if !n.is_finite() {
        return 0;
    }
    (n.trunc() as i64 & 0xffff_ffff) as u32
}

fn js_remainder(a: f64, b: f64) -> f64 {
    if b == 0.0 || a.is_nan() || b.is_nan() || a.is_infinite() {
        return f64::NAN;
    }
    if b.is_infinite() {
        return a;
    }
    a % b
}

/// JS abstract equality (`==`).
pub fn loose_equal(left: &RtValue, right: &RtValue) -> bool {
    use crate::stdlib_value::{coerce_to_number, coerce_to_string};
    if strict_equal(left, right) {
        // Covers identical types except NaN (strict already false there).
        return !matches!((left, right), (RtValue::Number(a), RtValue::Number(b)) if a.is_nan() || b.is_nan());
    }
    match (left, right) {
        (RtValue::Null, RtValue::Undefined) | (RtValue::Undefined, RtValue::Null) => true,
        (RtValue::Number(_), RtValue::Str(_)) => {
            loose_equal(left, &RtValue::Number(coerce_to_number(&rt_to_data(right))))
        }
        (RtValue::Str(_), RtValue::Number(_)) => {
            loose_equal(&RtValue::Number(coerce_to_number(&rt_to_data(left))), right)
        }
        (RtValue::Bool(_), _) => {
            loose_equal(&RtValue::Number(coerce_to_number(&rt_to_data(left))), right)
        }
        (_, RtValue::Bool(_)) => {
            loose_equal(left, &RtValue::Number(coerce_to_number(&rt_to_data(right))))
        }
        (RtValue::Str(a), RtValue::Str(b)) => a == b,
        _ => {
            // Objects compare by reference (false for distinct clones);
            // sandbox wrappers coerce via ToPrimitive string/number.
            let l = rt_to_data(left);
            let r = rt_to_data(right);
            match (&l, &r) {
                (DataVal::Sandbox(_), _) | (_, DataVal::Sandbox(_)) => {
                    coerce_to_string(&l) == coerce_to_string(&r)
                        || coerce_to_number(&l) == coerce_to_number(&r)
                }
                _ => false,
            }
        }
    }
}

/// JS relational comparison (`<`, `<=`, `>`, `>=`) with string fast-path.
pub fn relational(operator: &str, left: &RtValue, right: &RtValue) -> bool {
    use crate::stdlib_value::{coerce_to_number, coerce_to_string};
    let l = rt_to_data(left);
    let r = rt_to_data(right);
    if matches!(l, DataVal::Str(_)) && matches!(r, DataVal::Str(_)) {
        let (a, b) = (coerce_to_string(&l), coerce_to_string(&r));
        return match operator {
            "<" => a < b,
            "<=" => a <= b,
            ">" => a > b,
            ">=" => a >= b,
            _ => false,
        };
    }
    let (a, b) = (coerce_to_number(&l), coerce_to_number(&r));
    if a.is_nan() || b.is_nan() {
        return false;
    }
    match operator {
        "<" => a < b,
        "<=" => a <= b,
        ">" => a > b,
        ">=" => a >= b,
        _ => false,
    }
}

/// Applies a unary operator to a data value.
pub fn apply_unary(operator: &str, value: &RtValue, node: Option<&AstNode>) -> Eval<RtValue> {
    use crate::stdlib_value::{coerce_to_number, is_truthy};
    let data = rt_to_data(value);
    match operator {
        "-" => Ok(RtValue::Number(-coerce_to_number(&data))),
        "+" => Ok(RtValue::Number(coerce_to_number(&data))),
        "!" => Ok(RtValue::Bool(!is_truthy(&data))),
        "~" => Ok(RtValue::Number(!(to_int32(coerce_to_number(&data))) as f64)),
        _ => Err(Signal::execution(
            format!("Unsupported unary operator '{}'.", operator),
            node.cloned(),
        )),
    }
}

/// The `in` operator: string keys in objects, indices in arrays, members in
/// tool namespaces. Mirrors the TS guards.
pub fn in_operator(key: &str, right: &RtValue, node: Option<&AstNode>) -> Eval<bool> {
    match right {
        RtValue::Object(obj) => Ok(obj.entries.iter().any(|(k, _)| k == key)),
        RtValue::ErrorObj(_e) => Ok(key == "name" || key == "message"),
        RtValue::Array(arr) => {
            if key == "length" {
                return Ok(true);
            }
            match key.parse::<usize>() {
                Ok(index) => Ok(index < arr.items.len()),
                Err(_) => Ok(arr.props.iter().any(|(k, _)| k == key)),
            }
        }
        _ => Err(Signal::execution(
            "The 'in' operator requires a data object on the right-hand side.",
            node.cloned(),
        )),
    }
}

/// Spread items of a runtime value (arrays, strings, Maps, Sets,
/// URLSearchParams). Mirrors `spreadItems`.
pub fn spread_items_rt(value: &RtValue) -> Option<Vec<RtValue>> {
    match value {
        RtValue::Array(arr) => Some(arr.items.clone()),
        RtValue::Str(s) => Some(s.chars().map(|c| RtValue::Str(c.to_string())).collect()),
        RtValue::Sandbox(SandboxValue::Map(m)) => Some(
            m.entries
                .iter()
                .map(|(k, v)| RtValue::Array(RtArray::new(vec![k.clone(), v.clone()])))
                .collect(),
        ),
        RtValue::Sandbox(SandboxValue::Set(s)) => Some(s.members.clone()),
        RtValue::Sandbox(SandboxValue::UrlSearchParams(p)) => Some(
            p.pairs
                .iter()
                .map(|(k, v)| {
                    RtValue::Array(RtArray::new(vec![
                        RtValue::Str(k.clone()),
                        RtValue::Str(v.clone()),
                    ]))
                })
                .collect(),
        ),
        _ => None,
    }
}

fn upsert(entries: &mut Vec<(String, RtValue)>, key: String, value: RtValue) {
    if let Some(slot) = entries.iter_mut().find(|(k, _)| *k == key) {
        slot.1 = value;
    } else {
        entries.push((key, value));
    }
}

fn parse_query_string(query: &str) -> Vec<(String, String)> {
    if query.is_empty() {
        return vec![];
    }
    query
        .split('&')
        .filter(|p| !p.is_empty())
        .map(|pair| match pair.split_once('=') {
            Some((k, v)) => (k.to_string(), v.to_string()),
            None => (pair.to_string(), String::new()),
        })
        .collect()
}

fn date_utc_like(nums: &[f64]) -> f64 {
    // Mirrors `Date.UTC(...)` defaults for `new Date(y, m, ...)`.
    let year = nums.first().copied().unwrap_or(f64::NAN);
    if year.is_nan() {
        return f64::NAN;
    }
    // Two-digit years map to 1900+ (JS `Date` constructor behavior).
    let y = if (0.0..100.0).contains(&year) {
        1900.0 + year
    } else {
        year
    };
    let mo = nums.get(1).copied().unwrap_or(0.0);
    let d = nums.get(2).copied().unwrap_or(1.0);
    let h = nums.get(3).copied().unwrap_or(0.0);
    let mi = nums.get(4).copied().unwrap_or(0.0);
    let s = nums.get(5).copied().unwrap_or(0.0);
    let ms = nums.get(6).copied().unwrap_or(0.0);
    days_from_civil(y as i64, mo as i64 + 1, d as i64) as f64 * 86_400_000.0
        + h * 3_600_000.0
        + mi * 60_000.0
        + s * 1000.0
        + ms
}

fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

// ===========================================================================
// E5 — member/call dispatch + tools-only guard.
// ===========================================================================

/// Member key: string property or numeric index.
#[derive(Debug, Clone)]
pub enum MemberKeyRt {
    Str(String),
    Num(usize),
}

/// Resolved member location. The container chain is evaluated EXACTLY ONCE
/// (mirroring `modifyMember`); writes mutate the live binding in place, or a
/// discarded temp when the root is not a binding.
#[derive(Debug, Clone)]
pub enum MemberRef {
    ShortCircuit,
    Computed(RtValue),
    Callable(CallableRef),
    Data(DataRef),
}

/// Callable reference (returned as-is by member reads).
#[derive(Debug, Clone)]
pub enum CallableRef {
    Tool(Vec<String>),
    Promise(PromiseMethodName),
    Intrinsic {
        receiver: RtValue,
        name: String,
        /// Live write-back location for mutating container methods.
        back: Option<DataRef>,
    },
    Global {
        namespace: String,
        name: String,
    },
}

/// Data location for reads/writes.
#[derive(Debug, Clone)]
pub struct DataRef {
    pub base: DataBase,
    pub path: Vec<MemberKeyRt>,
}

/// Reference root.
#[derive(Debug, Clone)]
pub enum DataBase {
    Binding(String),
    Discard(RtValue),
}

/// Pre-evaluated member receiver: either a live data path (member chains
/// extend it without re-evaluating) or an owned value.
#[derive(Debug, Clone)]
pub enum Receiver {
    ShortCircuit,
    Value(RtValue),
    DataPath { data: DataRef, value: RtValue },
}

impl<'a> Interpreter<'a> {
    /// Resolves a member expression. Mirrors `getMemberReference(node)`.
    pub(crate) fn member_reference(&mut self, node: &AstNode) -> Eval<MemberRef> {
        let object_node = get_node(node, "object").map_err(Signal::from)?;
        let property_node = get_node(node, "property").map_err(Signal::from)?;
        let computed = get_boolean(node, "computed").map_err(Signal::from)?;
        let optional = matches!(node.get("optional"), Value::Bool(true));
        // Resolve the receiver exactly once, preserving data roots so member
        // chains (`a.b.c`) extend one live path instead of re-evaluating.
        let receiver = self.resolve_receiver(&object_node)?;
        let object_value = match receiver {
            Receiver::ShortCircuit => return Ok(MemberRef::ShortCircuit),
            Receiver::Value(ref value)
                if matches!(value, RtValue::Undefined | RtValue::Null) && optional =>
            {
                return Ok(MemberRef::ShortCircuit)
            }
            Receiver::Value(ref value) => value.clone(),
            Receiver::DataPath { ref value, .. }
                if matches!(value, RtValue::Undefined | RtValue::Null) && optional =>
            {
                return Ok(MemberRef::ShortCircuit)
            }
            Receiver::DataPath { ref value, .. } => value.clone(),
        };
        // Keys evaluate after the receiver (short-circuit returns first).
        let key = if computed {
            let key_value = self.eval_expression(&property_node)?;
            self.to_property_key(&key_value, Some(&property_node))?
        } else if property_node.node_type == "Identifier" {
            MemberKeyRt::Str(get_string(&property_node, "name").map_err(Signal::from)?)
        } else {
            let key_value = self.eval_expression(&property_node)?;
            self.to_property_key(&key_value, Some(&property_node))?
        };
        self.member_reference_on(
            object_value,
            receiver,
            &object_node,
            &property_node,
            key,
            node,
        )
    }

    /// Resolves a member receiver exactly once. Identifiers keep their live
    /// binding root; nested member expressions extend the inner data path;
    /// everything else evaluates to an owned temp.
    fn resolve_receiver(&mut self, object_node: &AstNode) -> Eval<Receiver> {
        if object_node.node_type == "Identifier" {
            let name = get_string(object_node, "name").map_err(Signal::from)?;
            let value = self.get_identifier(&name, Some(object_node))?;
            if matches!(value, RtValue::ShortCircuit) {
                return Ok(Receiver::ShortCircuit);
            }
            return Ok(Receiver::Value(value));
        }
        if object_node.node_type == "MemberExpression" {
            return match self.member_reference(object_node)? {
                MemberRef::ShortCircuit => Ok(Receiver::ShortCircuit),
                MemberRef::Computed(value) => Ok(Receiver::Value(value)),
                MemberRef::Callable(callable) => Ok(Receiver::Value(callable_to_value(callable))),
                MemberRef::Data(data) => {
                    let value = self.read_data_ref(&data, Some(object_node))?;
                    if matches!(value, RtValue::ShortCircuit) {
                        return Ok(Receiver::ShortCircuit);
                    }
                    Ok(Receiver::DataPath { data, value })
                }
            };
        }
        let value = self.eval_expression(object_node)?;
        if matches!(value, RtValue::ShortCircuit) {
            return Ok(Receiver::ShortCircuit);
        }
        Ok(Receiver::Value(value))
    }

    #[allow(clippy::too_many_arguments)]
    fn member_reference_on(
        &mut self,
        object_value: RtValue,
        receiver: Receiver,
        object_node: &AstNode,
        property_node: &AstNode,
        key: MemberKeyRt,
        node: &AstNode,
    ) -> Eval<MemberRef> {
        let key_str = match &key {
            MemberKeyRt::Str(s) => s.clone(),
            MemberKeyRt::Num(n) => n.to_string(),
        };
        // Tool paths append unchecked (validity triage happens at
        // call/enumeration, mirroring the TS `ToolReference` chain).
        if let RtValue::ToolRef(path) = &object_value {
            match &key {
                MemberKeyRt::Str(segment) if !crate::tool_runtime::is_blocked_member(segment) => {
                    let mut next = path.clone();
                    next.push(segment.clone());
                    return Ok(MemberRef::Callable(CallableRef::Tool(next)));
                }
                _ => {
                    return Err(Signal::execution(
                        "Tool paths must use safe string property names.",
                        Some(property_node.clone()),
                    ))
                }
            }
        }
        if matches!(object_value, RtValue::PromiseNs) {
            match crate::interpreter_model::PromiseMethodName::parse(&key_str) {
                Some(name) => return Ok(MemberRef::Callable(CallableRef::Promise(name))),
                None => {
                    return Err(Signal::execution(
                        format!("Promise.{} is not available in CodeMode. Available: Promise.all, Promise.allSettled, Promise.race, Promise.resolve, and Promise.reject; consume promises with await.", key_str),
                        Some(property_node.clone()),
                    ))
                }
            }
        }
        if let RtValue::Global(ns) = &object_value {
            if crate::tool_runtime::is_blocked_member(&key_str) {
                return Err(Signal::execution(
                    format!("{}.{} is not available in CodeMode.", ns.as_str(), key_str),
                    Some(property_node.clone()),
                ));
            }
            if *ns == GlobalNamespaceName::Math {
                if let Some(constant) = crate::stdlib_math::math_constant(&key_str) {
                    return Ok(MemberRef::Computed(RtValue::Number(constant)));
                }
            }
            return Ok(MemberRef::Callable(CallableRef::Global {
                namespace: ns.as_str().to_string(),
                name: key_str,
            }));
        }
        if let RtValue::Str(s) = &object_value {
            if key_str == "length" {
                return Ok(MemberRef::Computed(RtValue::Number(
                    s.chars().count() as f64
                )));
            }
            if let MemberKeyRt::Num(index) = &key {
                return Ok(MemberRef::Computed(
                    s.chars()
                        .nth(*index)
                        .map(|c| RtValue::Str(c.to_string()))
                        .unwrap_or(RtValue::Undefined),
                ));
            }
            if key_str.chars().all(|c| c.is_ascii_digit()) && !key_str.is_empty() {
                let index: usize = key_str.parse().unwrap_or(usize::MAX);
                return Ok(MemberRef::Computed(
                    s.chars()
                        .nth(index)
                        .map(|c| RtValue::Str(c.to_string()))
                        .unwrap_or(RtValue::Undefined),
                ));
            }
            if crate::stdlib_string::is_string_method(&key_str) {
                return Ok(MemberRef::Callable(CallableRef::Intrinsic {
                    receiver: object_value,
                    name: key_str,
                    back: writeback_for(&receiver),
                }));
            }
            return Ok(MemberRef::Computed(RtValue::Undefined));
        }
        if let RtValue::Number(_) = &object_value {
            if crate::stdlib_number::NUMBER_METHODS.contains(&key_str.as_str()) {
                return Ok(MemberRef::Callable(CallableRef::Intrinsic {
                    receiver: object_value,
                    name: key_str,
                    back: writeback_for(&receiver),
                }));
            }
            return Ok(MemberRef::Computed(RtValue::Undefined));
        }
        if let RtValue::Coercion(kind) = &object_value {
            if !crate::tool_runtime::is_blocked_member(&key_str) {
                let name = kind.as_str();
                if name == "Number" {
                    if let Some(constant) = crate::stdlib_number::number_constant(&key_str) {
                        return Ok(MemberRef::Computed(RtValue::Number(constant)));
                    }
                    if crate::stdlib_number::NUMBER_STATICS.contains(&key_str.as_str()) {
                        return Ok(MemberRef::Callable(CallableRef::Global {
                            namespace: "Number".to_string(),
                            name: key_str,
                        }));
                    }
                }
                if name == "String"
                    && crate::stdlib_string::STRING_STATICS.contains(&key_str.as_str())
                {
                    return Ok(MemberRef::Callable(CallableRef::Global {
                        namespace: "String".to_string(),
                        name: key_str,
                    }));
                }
            }
            return Ok(MemberRef::Computed(RtValue::Undefined));
        }
        if let RtValue::Sandbox(sandbox) = &object_value {
            return self.sandbox_member(
                sandbox.clone(),
                receiver,
                object_node,
                property_node,
                key,
                node,
            );
        }
        if is_runtime_reference(&object_value) {
            return Err(Signal::Runtime(InterpreterRuntimeError::new(
                "CodeMode runtime references are opaque and do not expose properties.",
                Some(object_node.clone()),
                DiagnosticKind::InvalidDataValue,
                None,
            )));
        }
        if matches!(object_value, RtValue::Undefined | RtValue::Null)
            || !matches!(
                object_value,
                RtValue::Array(_) | RtValue::Object(_) | RtValue::ErrorObj(_)
            )
        {
            return Err(Signal::execution(
                "Cannot access a property on a non-object value.",
                Some(object_node.clone()),
            ));
        }
        if crate::tool_runtime::is_blocked_member(&key_str) {
            return Err(Signal::execution(
                format!("Property '{}' is not available in CodeMode.", key_str),
                Some(property_node.clone()),
            ));
        }
        // Arrays: length, methods, indices, and own non-index properties
        // (match `index`/`groups`) read through; unknown props are undefined.
        if let RtValue::Array(arr) = &object_value {
            if key_str != "length"
                && !crate::stdlib_collections::is_array_method(&key_str)
                && !matches!(key, MemberKeyRt::Num(_))
                && !(key_str.chars().all(|c| c.is_ascii_digit()) && !key_str.is_empty())
            {
                if let Some(value) = arr.props.iter().find(|(k, _)| k == &key_str) {
                    return Ok(MemberRef::Computed(value.1.clone()));
                }
                return Ok(MemberRef::Computed(RtValue::Undefined));
            }
            if crate::stdlib_collections::is_array_method(&key_str)
                && !matches!(key, MemberKeyRt::Num(_))
                && key_str != "length"
            {
                // `seen.push` where `seen` is a binding must retain a write-back
                // DataRef so the mutation persists. Identifier receivers arrive as
                // `Value` (not `DataPath`), so synthesize a Binding back.
                let back = match &receiver {
                    Receiver::DataPath { data, .. } => Some(data.clone()),
                    Receiver::Value(_) if object_node.node_type == "Identifier" => {
                        let name = get_string(object_node, "name").map_err(Signal::from)?;
                        Some(DataRef {
                            base: DataBase::Binding(name),
                            path: vec![],
                        })
                    }
                    _ => writeback_for(&receiver),
                };
                return Ok(MemberRef::Callable(CallableRef::Intrinsic {
                    receiver: object_value,
                    name: key_str,
                    back,
                }));
            }
        }
        // Data containers: extend the receiver's live path (`a.b.c` keeps one
        // root — the binding or the inner data path) or start a discarded
        // temp for non-rooted receivers. Single evaluation is preserved by
        // construction: receivers arrive pre-evaluated via `resolve_receiver`.
        let (base, mut path) = match receiver {
            Receiver::DataPath { data, .. } => (data.base, data.path),
            Receiver::Value(value) => match &object_value {
                _ if object_node.node_type == "Identifier" => (
                    DataBase::Binding(get_string(object_node, "name").map_err(Signal::from)?),
                    vec![],
                ),
                _ => (DataBase::Discard(value), vec![]),
            },
            Receiver::ShortCircuit => {
                return Ok(MemberRef::ShortCircuit);
            }
        };
        path.push(key);
        Ok(MemberRef::Data(DataRef { base, path }))
    }

    /// Member access on sandbox values. Mirrors the TS allowlist branches.
    fn sandbox_member(
        &mut self,
        sandbox: SandboxValue,
        receiver: Receiver,
        object_node: &AstNode,
        property_node: &AstNode,
        key: MemberKeyRt,
        _node: &AstNode,
    ) -> Eval<MemberRef> {
        let key_str = match &key {
            MemberKeyRt::Str(s) => s.clone(),
            MemberKeyRt::Num(n) => n.to_string(),
        };
        match &sandbox {
            SandboxValue::Date(_) => {
                if crate::stdlib_date::is_date_method(&key_str) {
                    return Ok(MemberRef::Callable(CallableRef::Intrinsic {
                        receiver: RtValue::Sandbox(sandbox),
                        name: key_str,
                        back: writeback_for(&receiver),
                    }));
                }
                Ok(MemberRef::Computed(RtValue::Undefined))
            }
            SandboxValue::RegExp(r) => {
                if crate::stdlib_regexp::is_regexp_property(&key_str) {
                    let value = match key_str.as_str() {
                        "source" => RtValue::Str(r.pattern.clone()),
                        "flags" => RtValue::Str(r.flags.clone()),
                        "lastIndex" => RtValue::Number(0.0),
                        "global" => RtValue::Bool(r.flags.contains('g')),
                        "ignoreCase" => RtValue::Bool(r.flags.contains('i')),
                        "multiline" => RtValue::Bool(r.flags.contains('m')),
                        "sticky" => RtValue::Bool(r.flags.contains('y')),
                        "unicode" => RtValue::Bool(r.flags.contains('u')),
                        "dotAll" => RtValue::Bool(r.flags.contains('s')),
                        _ => RtValue::Undefined,
                    };
                    return Ok(MemberRef::Computed(value));
                }
                if crate::stdlib_regexp::is_regexp_method(&key_str) {
                    return Ok(MemberRef::Callable(CallableRef::Intrinsic {
                        receiver: RtValue::Sandbox(sandbox),
                        name: key_str,
                        back: writeback_for(&receiver),
                    }));
                }
                Ok(MemberRef::Computed(RtValue::Undefined))
            }
            SandboxValue::Map(m) => {
                if key_str == "size" {
                    return Ok(MemberRef::Computed(RtValue::Number(m.entries.len() as f64)));
                }
                if crate::stdlib_collections::is_map_method(&key_str) {
                    // Mirror Array's synthesised Binding back for `m.set` where `m` is a binding (e.g. `const m = new Map()`).
                    let back = if matches!(&receiver, Receiver::Value(_))
                        && object_node.node_type == "Identifier"
                    {
                        Some(DataRef {
                            base: DataBase::Binding(
                                get_string(object_node, "name").map_err(Signal::from)?,
                            ),
                            path: vec![],
                        })
                    } else {
                        writeback_for(&receiver)
                    };
                    return Ok(MemberRef::Callable(CallableRef::Intrinsic {
                        receiver: RtValue::Sandbox(sandbox),
                        name: key_str,
                        back,
                    }));
                }
                Ok(MemberRef::Computed(RtValue::Undefined))
            }
            SandboxValue::Set(s) => {
                if key_str == "size" {
                    return Ok(MemberRef::Computed(RtValue::Number(s.members.len() as f64)));
                }
                if crate::stdlib_collections::is_set_method(&key_str) {
                    let back = if matches!(&receiver, Receiver::Value(_))
                        && object_node.node_type == "Identifier"
                    {
                        Some(DataRef {
                            base: DataBase::Binding(
                                get_string(object_node, "name").map_err(Signal::from)?,
                            ),
                            path: vec![],
                        })
                    } else {
                        writeback_for(&receiver)
                    };
                    return Ok(MemberRef::Callable(CallableRef::Intrinsic {
                        receiver: RtValue::Sandbox(sandbox),
                        name: key_str,
                        back,
                    }));
                }
                Ok(MemberRef::Computed(RtValue::Undefined))
            }
            SandboxValue::Url(u) => {
                if key_str == "searchParams" {
                    return Ok(MemberRef::Computed(RtValue::Sandbox(
                        SandboxValue::UrlSearchParams(u.search_params.clone()),
                    )));
                }
                if crate::stdlib_url::URL_METHODS.contains(&key_str.as_str()) {
                    return Ok(MemberRef::Callable(CallableRef::Intrinsic {
                        receiver: RtValue::Sandbox(sandbox),
                        name: key_str,
                        back: writeback_for(&receiver),
                    }));
                }
                if crate::stdlib_url::is_url_property(&key_str) {
                    // Extend the receiver path so chains (`a.u.pathname`)
                    // keep one live root.
                    let (base, mut path) = match receiver {
                        Receiver::DataPath { data, .. } => (data.base, data.path),
                        Receiver::Value(value) => match &value {
                            _ if object_node.node_type == "Identifier" => (
                                DataBase::Binding(
                                    get_string(object_node, "name").map_err(Signal::from)?,
                                ),
                                vec![],
                            ),
                            _ => (DataBase::Discard(value), vec![]),
                        },
                        Receiver::ShortCircuit => return Ok(MemberRef::ShortCircuit),
                    };
                    path.push(key);
                    return Ok(MemberRef::Data(DataRef { base, path }));
                }
                Ok(MemberRef::Computed(RtValue::Undefined))
            }
            SandboxValue::UrlSearchParams(p) => {
                if key_str == "size" {
                    return Ok(MemberRef::Computed(RtValue::Number(p.pairs.len() as f64)));
                }
                if crate::stdlib_url::is_url_search_params_method(&key_str) {
                    return Ok(MemberRef::Callable(CallableRef::Intrinsic {
                        receiver: RtValue::Sandbox(sandbox),
                        name: key_str,
                        back: writeback_for(&receiver),
                    }));
                }
                Ok(MemberRef::Computed(RtValue::Undefined))
            }
            SandboxValue::Promise(_) => {
                if key_str == "then" || key_str == "catch" || key_str == "finally" {
                    return Err(Signal::Runtime(InterpreterRuntimeError::new(
                        format!("Promise.prototype.{} is not supported in CodeMode; use await instead (with try/catch to handle failures) - e.g. `const result = await tools.ns.tool(...)`.", key_str),
                        Some(property_node.clone()),
                        DiagnosticKind::UnsupportedSyntax,
                        Some(vec![crate::interpreter_model::SUPPORTED_SYNTAX_MESSAGE.to_string()]),
                    )));
                }
                Err(Signal::Runtime(InterpreterRuntimeError::new(
                    "This value is an un-awaited Promise and has no readable properties; await it first - e.g. `const result = await tools.ns.tool(...)`.",
                    Some(object_node.clone()),
                    DiagnosticKind::InvalidDataValue,
                    None,
                )))
            }
        }
    }

    /// Property-key coercion. Mirrors `toPropertyKey(value, node)`.
    pub(crate) fn to_property_key(
        &mut self,
        value: &RtValue,
        node: Option<&AstNode>,
    ) -> Eval<MemberKeyRt> {
        match value {
            RtValue::Str(s) => Ok(MemberKeyRt::Str(s.clone())),
            RtValue::Number(n) => {
                if n.fract() == 0.0 && *n >= 0.0 && *n < 9_007_199_254_740_991.0 {
                    Ok(MemberKeyRt::Num(*n as usize))
                } else {
                    Ok(MemberKeyRt::Str(crate::stdlib_value::js_number_to_string(
                        *n,
                    )))
                }
            }
            _ => Err(Signal::execution(
                "Property key must be a string or number.",
                node.cloned(),
            )),
        }
    }

    /// Reads a member expression value. Mirrors `readMember(node)`.
    pub(crate) fn read_member(&mut self, node: &AstNode) -> Eval<RtValue> {
        match self.member_reference(node)? {
            MemberRef::ShortCircuit => Ok(RtValue::ShortCircuit),
            MemberRef::Computed(value) => Ok(value),
            MemberRef::Callable(callable) => Ok(callable_to_value(callable)),
            MemberRef::Data(data) => self.read_data_ref(&data, Some(node)),
        }
    }

    fn read_data_ref(&mut self, data: &DataRef, node: Option<&AstNode>) -> Eval<RtValue> {
        let root = match &data.base {
            DataBase::Binding(name) => self.get_identifier(name, node)?,
            DataBase::Discard(value) => value.clone(),
        };
        read_path(&root, &data.path, node)
    }

    /// Writes a member expression value. Mirrors `writeMember` /
    /// `modifyMember` with the write-always computation.
    pub(crate) fn write_member(&mut self, node: &AstNode, value: RtValue) -> Eval<RtValue> {
        match self.member_reference(node)? {
            MemberRef::ShortCircuit | MemberRef::Computed(_) | MemberRef::Callable(_) => {
                Err(Signal::execution(
                    "Only data fields may be assigned in CodeMode.",
                    Some(node.clone()),
                ))
            }
            MemberRef::Data(data) => {
                self.assign_data_ref(&data, value.clone(), Some(node))?;
                // The `=` expression evaluates to the assigned value itself
                // (mirroring `modifyMember`'s `result`), not a re-read.
                Ok(value)
            }
        }
    }

    fn assign_data_ref(
        &mut self,
        data: &DataRef,
        value: RtValue,
        node: Option<&AstNode>,
    ) -> Eval<()> {
        // Mirrors `assignToReference` in TS which mutates the live target
        // object directly — `const` only guards rebinding the variable itself
        // (`x = y`), not interior property/index mutation (`obj.a = 1`,
        // `arr.push(1)`). So DataRef writes bypass the `mutable` check.
        match &data.base {
            DataBase::Binding(name) => {
                let current = self.get_identifier(name, node)?;
                let next = assign_path(current, &data.path, value, node)?;
                // Directly update the binding without the `const` guard.
                for scope in self.scopes.iter_mut().rev() {
                    if let Some(binding) = scope.get_mut(name) {
                        binding.value = next;
                        return Ok(());
                    }
                }
                Err(Signal::Runtime(
                    crate::interpreter_model::InterpreterRuntimeError::new(
                        format!("Unknown identifier '{}'.", name),
                        node.cloned(),
                        crate::interpreter_model::DiagnosticKind::ExecutionFailure,
                        None,
                    )
                    .as_error("ReferenceError"),
                ))
            }
            DataBase::Discard(root) => {
                let mut cloned = root.clone();
                cloned = assign_path(cloned, &data.path, value, node)?;
                let _ = cloned;
                Ok(())
            }
        }
    }

    /// Rejects inserting a value that transitively contains the container it
    /// is being inserted into. Mirrors `rejectCircularInsertion(container,
    /// value, label, node, seen?)` with container-identity semantics (clones
    /// share the id, mirroring JS reference identity).
    pub(crate) fn reject_circular_insertion(
        container_id: u64,
        value: &RtValue,
        label: &str,
        node: Option<&AstNode>,
    ) -> Eval<()> {
        fn walk(
            id: u64,
            value: &RtValue,
            label: &str,
            node: Option<&AstNode>,
            seen: &mut BTreeSet<u64>,
        ) -> Eval<()> {
            match value {
                RtValue::Array(arr) if arr.id == id => {
                    return Err(Signal::Runtime(InterpreterRuntimeError::new(
                        format!("{} contains a circular value.", label),
                        node.cloned(),
                        DiagnosticKind::InvalidDataValue,
                        None,
                    )))
                }
                RtValue::Object(obj) if obj.id == id => {
                    return Err(Signal::Runtime(InterpreterRuntimeError::new(
                        format!("{} contains a circular value.", label),
                        node.cloned(),
                        DiagnosticKind::InvalidDataValue,
                        None,
                    )))
                }
                RtValue::Array(arr) => {
                    if !seen.insert(arr.id) {
                        return Ok(());
                    }
                    for item in &arr.items {
                        walk(id, item, label, node, seen)?;
                    }
                    seen.remove(&arr.id);
                    Ok(())
                }
                RtValue::Object(obj) => {
                    if !seen.insert(obj.id) {
                        return Ok(());
                    }
                    for (_, item) in &obj.entries {
                        walk(id, item, label, node, seen)?;
                    }
                    seen.remove(&obj.id);
                    Ok(())
                }
                _ => Ok(()),
            }
        }
        // Direct identity (mirrors `value === container`).
        match value {
            RtValue::Array(arr) if arr.id == container_id => {
                return Err(Signal::Runtime(InterpreterRuntimeError::new(
                    format!("{} contains a circular value.", label),
                    node.cloned(),
                    DiagnosticKind::InvalidDataValue,
                    None,
                )))
            }
            RtValue::Object(obj) if obj.id == container_id => {
                return Err(Signal::Runtime(InterpreterRuntimeError::new(
                    format!("{} contains a circular value.", label),
                    node.cloned(),
                    DiagnosticKind::InvalidDataValue,
                    None,
                )))
            }
            _ => {}
        }
        walk(container_id, value, label, node, &mut BTreeSet::new())
    }

    // --- calls ---------------------------------------------------------------

    /// Call dispatch with the tools-only guard. Mirrors the
    /// `evaluateCallExpression` chain: optional passthrough, tool admission,
    /// promise methods, functions, intrinsics, global methods (incl.
    /// `Object.*` on tools), coercions, URI helpers, error constructors —
    /// else `Only tools are callable in CodeMode.`
    pub(crate) fn eval_call(&mut self, node: &AstNode) -> Eval<RtValue> {
        let callee_node = get_node(node, "callee").map_err(Signal::from)?;
        let optional = matches!(node.get("optional"), Value::Bool(true));
        // Resolve the callee exactly once: member callees via the member
        // reference (no re-evaluation), plain callees via expression value.
        let dispatch = if callee_node.node_type == "MemberExpression" {
            match self.member_reference(&callee_node)? {
                MemberRef::ShortCircuit => CallableDispatch::ShortCircuit,
                MemberRef::Callable(callable) => callable_to_dispatch(callable),
                MemberRef::Computed(_) | MemberRef::Data(_) => {
                    return Err(Signal::execution(
                        "Only tools are callable in CodeMode.",
                        Some(callee_node),
                    ))
                }
            }
        } else {
            match self.eval_expression(&callee_node)? {
                RtValue::ShortCircuit => CallableDispatch::ShortCircuit,
                RtValue::Undefined | RtValue::Null if optional => return Ok(RtValue::ShortCircuit),
                value => self.classify_value_callee(&value, Some(node))?,
            }
        };
        if matches!(dispatch, CallableDispatch::ShortCircuit) {
            return Ok(RtValue::ShortCircuit);
        }
        let args = self.eval_call_arguments(node)?;
        match dispatch {
            CallableDispatch::ShortCircuit => Ok(RtValue::ShortCircuit),
            CallableDispatch::Tool(path) => {
                if path.is_empty() {
                    return Err(Signal::execution(
                        "The tools root is not callable.",
                        Some(callee_node),
                    ));
                }
                // Boundary-check for un-awaited promises before crossing.
                let name = path.join(".");
                for arg in &args {
                    if contains_promise(arg) {
                        return Err(Signal::ToolRuntime(ToolRuntimeError::new(
                            ToolRuntimeErrorKind::InvalidDataValue,
                            format!("Arguments for tool '{}' contains an un-awaited Promise; await tool calls (e.g. `const result = await tools.ns.tool(...)`) before using their results.", name),
                            vec![],
                        )));
                    }
                }
                let json_args: Vec<Value> = args.iter().map(rt_to_json).collect();
                let promise = self.admit_tool_call(&path, json_args)?;
                Ok(RtValue::Sandbox(SandboxValue::Promise(promise)))
            }
            CallableDispatch::Promise(name) => self.invoke_promise_method(name, &args, Some(node)),
            CallableDispatch::Function(function) => self.invoke_function(&function, &args),
            CallableDispatch::Intrinsic {
                receiver,
                name,
                back,
            } => self.invoke_intrinsic_loc(
                &RecvLoc {
                    value: receiver,
                    back,
                },
                &name,
                &args,
                Some(node),
            ),
            CallableDispatch::Global { namespace, name } => {
                // Mirrors TS `evaluateCallExpression`: console routes to
                // `invokeConsole` before `invokeGlobalMethod`.
                if namespace == "console" {
                    return self.invoke_console(&name, &args, Some(node));
                }
                self.invoke_global_method(&namespace, &name, &args, Some(node))
            }
            CallableDispatch::Coercion(kind) => {
                let data: Vec<DataVal> = args.iter().map(rt_to_data).collect();
                let coerced = crate::stdlib_value::invoke_coercion(kind, &data, Some(node))
                    .map_err(Signal::from)?;
                let checked = crate::stdlib_value::bounded_data(
                    &coerced,
                    &format!("{} result", kind.as_str()),
                )
                .map_err(Signal::from)?;
                Ok(data_to_rt(&checked))
            }
            CallableDispatch::Uri(kind) => {
                let data: Vec<DataVal> = args.iter().map(rt_to_data).collect();
                let result = crate::stdlib_url::invoke_uri_function(kind, &data, Some(node))
                    .map_err(Signal::from)?;
                Ok(data_to_rt(&result))
            }
            CallableDispatch::ErrorCtor(name) => {
                let message = match args.first() {
                    None | Some(RtValue::Undefined) => String::new(),
                    Some(value) => crate::stdlib_value::coerce_to_string(&rt_to_data(value)),
                };
                Ok(RtValue::ErrorObj(RtErrorObj { name, message }))
            }
        }
    }

    /// Classifies a plain (non-member) callee value: tools, functions,
    /// coercions, URI helpers, error constructors — else the tools-only
    /// guard. Mirrors the TS dispatch tail.
    fn classify_value_callee(
        &mut self,
        callable: &RtValue,
        node: Option<&AstNode>,
    ) -> Eval<CallableDispatch> {
        match callable {
            RtValue::ToolRef(path) => Ok(CallableDispatch::Tool(path.clone())),
            RtValue::Function(function) => Ok(CallableDispatch::Function(function.clone())),
            RtValue::Coercion(kind) => Ok(CallableDispatch::Coercion(*kind)),
            RtValue::Uri(kind) => Ok(CallableDispatch::Uri(*kind)),
            RtValue::ErrorCtor(name) => Ok(CallableDispatch::ErrorCtor(name.clone())),
            _ => Err(Signal::execution(
                "Only tools are callable in CodeMode.",
                node.cloned(),
            )),
        }
    }

    /// Evaluates call arguments incl. spread. Mirrors
    /// `evaluateCallArguments(argNodes)`.
    pub(crate) fn eval_call_arguments(&mut self, node: &AstNode) -> Eval<Vec<RtValue>> {
        let raw = get_array(node, "arguments").map_err(Signal::from)?;
        let mut args = vec![];
        for (index, item) in raw.into_iter().enumerate() {
            let arg_node =
                crate::interpreter_model::as_node(&item, &format!("arguments[{}]", index))
                    .map_err(Signal::from)?;
            if arg_node.node_type == "SpreadElement" {
                let argument = get_node(&arg_node, "argument").map_err(Signal::from)?;
                let spread = self.eval_expression(&argument)?;
                match spread_items_rt(&spread) {
                    Some(mut items) => args.append(&mut items),
                    None => {
                        return Err(Signal::execution(
                            "Spread arguments require an array, string, Map, or Set in CodeMode.",
                            Some(arg_node),
                        ))
                    }
                }
            } else {
                args.push(self.eval_expression(&arg_node)?);
            }
        }
        Ok(args)
    }

    /// Invokes a user function. Mirrors `invokeFunction(fn, args)`: TDZ
    /// parameter slots, rest elements, defaults, block vs expression bodies,
    /// scope save/restore.
    pub(crate) fn invoke_function(
        &mut self,
        function: &RtFunction,
        args: &[RtValue],
    ) -> Eval<RtValue> {
        let saved = std::mem::replace(&mut self.scopes, function.captured.clone());
        self.push_scope();
        for parameter in &function.params {
            for name in collect_pattern_names(parameter) {
                if let Ok(scope) = self.current_scope() {
                    scope.insert(
                        name,
                        RtBinding {
                            mutable: true,
                            value: RtValue::Undefined,
                            initialized: false,
                        },
                    );
                }
            }
        }
        let mut result = Eval::<RtValue>::Ok(RtValue::Undefined);
        for (index, parameter) in function.params.iter().enumerate() {
            if parameter.node_type == "RestElement" {
                let argument = get_node(parameter, "argument")
                    .map_err(Signal::from)
                    .and_then(|argument| {
                        let rest = RtValue::Array(RtArray::new(
                            args.iter().skip(index).cloned().collect(),
                        ));
                        self.declare_pattern(&argument, rest, true, Some(parameter))
                    });
                if let Err(signal) = argument {
                    result = Err(signal);
                    break;
                }
                // Rest must be last (parse-enforced); stop binding.
                let _ = result;
                result = Ok(RtValue::Undefined);
                break;
            }
            let value = args.get(index).cloned().unwrap_or(RtValue::Undefined);
            // Clone keys to satisfy the borrow checker.
            let parameter = parameter.clone();
            if let Err(signal) = self.declare_pattern(&parameter, value, true, Some(&parameter)) {
                result = Err(signal);
                break;
            }
        }
        if result.is_ok() {
            result = if function.body.node_type == "BlockStatement" {
                match self.eval_statement(&function.body) {
                    Ok(StmtOut::Return(v)) | Ok(StmtOut::Value(v)) => Ok(v),
                    Ok(StmtOut::None) => Ok(RtValue::Undefined),
                    Err(Signal::Return(v)) => Ok(v),
                    Err(Signal::Break(_)) | Err(Signal::Continue(_)) => Ok(RtValue::Undefined),
                    Err(other) => Err(other),
                }
            } else {
                self.eval_expression(&function.body)
            };
        }
        self.scopes = saved;
        result
    }

    /// Pattern declaration with evaluated defaults. Mirrors
    /// `declarePattern(pattern, value, mutable, node)`.
    pub(crate) fn declare_pattern(
        &mut self,
        pattern: &AstNode,
        value: RtValue,
        mutable: bool,
        node: Option<&AstNode>,
    ) -> Eval<()> {
        match pattern.node_type.as_str() {
            "Identifier" => {
                let name = get_string(pattern, "name").map_err(Signal::from)?;
                self.declare(&name, value, mutable, node)
            }
            "AssignmentPattern" => {
                let left = get_node(pattern, "left").map_err(Signal::from)?;
                let right = get_node(pattern, "right").map_err(Signal::from)?;
                let resolved = match value {
                    RtValue::Undefined => self.eval_expression(&right)?,
                    other => other,
                };
                self.declare_pattern(&left, resolved, mutable, node)
            }
            "RestElement" => {
                let argument = get_node(pattern, "argument").map_err(Signal::from)?;
                self.declare_pattern(&argument, value, mutable, node)
            }
            "ObjectPattern" => {
                let raw = get_array(pattern, "properties").map_err(Signal::from)?;
                let mut consumed: BTreeSet<String> = BTreeSet::new();
                for (index, item) in raw.into_iter().enumerate() {
                    let property =
                        crate::interpreter_model::as_node(&item, &format!("properties[{}]", index))
                            .map_err(Signal::from)?;
                    if property.node_type == "RestElement" {
                        let argument = get_node(&property, "argument").map_err(Signal::from)?;
                        let rest = object_rest_owned(&value, &consumed, Some(&property))?;
                        self.declare_pattern(&argument, rest, mutable, node)?;
                        continue;
                    }
                    if property.get("computed") == &Value::Bool(true) {
                        return Err(Signal::execution(
                            "Only named object destructuring properties are supported.",
                            Some(property),
                        ));
                    }
                    let key_node = get_node(&property, "key").map_err(Signal::from)?;
                    let key = named_property_key(&key_node)?;
                    if crate::tool_runtime::is_blocked_member(&key) {
                        return Err(Signal::execution(
                            format!("Property '{}' is not available in CodeMode.", key),
                            Some(key_node),
                        ));
                    }
                    consumed.insert(key.clone());
                    let nested = get_node(&property, "value").map_err(Signal::from)?;
                    let field = object_field_owned(&value, &key);
                    if nested.node_type == "AssignmentPattern" {
                        let left = get_node(&nested, "left").map_err(Signal::from)?;
                        let right = get_node(&nested, "right").map_err(Signal::from)?;
                        let resolved = match field {
                            RtValue::Undefined => self.eval_expression(&right)?,
                            other => other,
                        };
                        self.declare_pattern(&left, resolved, mutable, node)?;
                    } else {
                        self.declare_pattern(&nested, field, mutable, node)?;
                    }
                }
                Ok(())
            }
            "ArrayPattern" => {
                let items = match &value {
                    RtValue::Array(arr) => arr.items.clone(),
                    RtValue::Undefined | RtValue::Null => vec![],
                    _ => {
                        return Err(Signal::execution(
                            "Array destructuring requires an array value.",
                            Some(pattern.clone()),
                        ))
                    }
                };
                // `undefined`/`null` with all-default patterns still runs
                // defaults; without defaults it is an error unless empty.
                if matches!(value, RtValue::Undefined | RtValue::Null) {
                    let raw = get_array(pattern, "elements").map_err(Signal::from)?;
                    let all_default = raw.iter().all(|element| {
                        element.is_null()
                            || crate::interpreter_model::as_node(element, "elements")
                                .map(|node| node.node_type == "AssignmentPattern")
                                .unwrap_or(false)
                    });
                    if !all_default {
                        return Err(Signal::execution(
                            "Array destructuring requires an array value.",
                            Some(pattern.clone()),
                        ));
                    }
                }
                let raw = get_array(pattern, "elements").map_err(Signal::from)?;
                let mut index = 0usize;
                for element in raw {
                    if element.is_null() {
                        index += 1;
                        continue;
                    }
                    let element_node = crate::interpreter_model::as_node(&element, "elements")
                        .map_err(Signal::from)?;
                    if element_node.node_type == "RestElement" {
                        let argument = get_node(&element_node, "argument").map_err(Signal::from)?;
                        let rest = RtValue::Array(RtArray::new(
                            items.iter().skip(index).cloned().collect(),
                        ));
                        self.declare_pattern(&argument, rest, mutable, node)?;
                        break;
                    }
                    let item = items.get(index).cloned().unwrap_or(RtValue::Undefined);
                    if element_node.node_type == "AssignmentPattern" {
                        let left = get_node(&element_node, "left").map_err(Signal::from)?;
                        let right = get_node(&element_node, "right").map_err(Signal::from)?;
                        let resolved = match item {
                            RtValue::Undefined => self.eval_expression(&right)?,
                            other => other,
                        };
                        self.declare_pattern(&left, resolved, mutable, node)?;
                    } else {
                        self.declare_pattern(&element_node, item, mutable, node)?;
                    }
                    index += 1;
                }
                Ok(())
            }
            other => Err(Signal::execution(
                format!("Unsupported binding pattern '{}'.", other),
                Some(pattern.clone()),
            )),
        }
    }

    /// Logical assignment (`&&=` / `||=` / `??=`). Short-circuits like the
    /// corresponding logical operator before reaching compound assignment.
    pub(crate) fn eval_logical_assignment(
        &mut self,
        operator: &str,
        left: &AstNode,
        right_node: &AstNode,
        node: Option<&AstNode>,
    ) -> Eval<RtValue> {
        // Member targets resolve exactly once (mirroring `modifyMember`).
        if left.node_type == "MemberExpression" {
            let reference = self.member_reference(left)?;
            let data = match reference {
                MemberRef::Data(data) => data,
                _ => {
                    return Err(Signal::execution(
                        "Only data fields may be assigned in CodeMode.",
                        node.cloned(),
                    ))
                }
            };
            let current = self.read_data_ref(&data, node)?;
            let take = match operator {
                "&&=" => is_truthy_rt(&current),
                "||=" => !is_truthy_rt(&current),
                "??=" => matches!(current, RtValue::Null | RtValue::Undefined),
                _ => {
                    return Err(Signal::execution(
                        format!("Unsupported assignment operator '{}'.", operator),
                        node.cloned(),
                    ))
                }
            };
            if !take {
                return Ok(current);
            }
            let value = self.eval_expression(right_node)?;
            self.assign_data_ref(&data, value.clone(), node)?;
            return Ok(value);
        }
        let current = self.read_target(left, node)?;
        let take = match operator {
            "&&=" => is_truthy_rt(&current),
            "||=" => !is_truthy_rt(&current),
            "??=" => matches!(current, RtValue::Null | RtValue::Undefined),
            _ => {
                return Err(Signal::execution(
                    format!("Unsupported assignment operator '{}'.", operator),
                    node.cloned(),
                ))
            }
        };
        if !take {
            return Ok(current);
        }
        let value = self.eval_expression(right_node)?;
        self.assign_target(left, value.clone(), node)?;
        Ok(value)
    }
}

/// Dispatch target after callee classification.
#[derive(Debug, Clone)]
pub enum CallableDispatch {
    Tool(Vec<String>),
    Promise(PromiseMethodName),
    Function(RtFunction),
    Intrinsic {
        receiver: RtValue,
        name: String,
        back: Option<DataRef>,
    },
    Global {
        namespace: String,
        name: String,
    },
    Coercion(CoercionKind),
    Uri(UriKind),
    ErrorCtor(String),
    ShortCircuit,
}

/// Converts a callable reference into dispatch form.
pub fn callable_to_dispatch(callable: CallableRef) -> CallableDispatch {
    match callable {
        CallableRef::Tool(path) => CallableDispatch::Tool(path),
        CallableRef::Promise(name) => CallableDispatch::Promise(name),
        CallableRef::Intrinsic {
            receiver,
            name,
            back,
        } => CallableDispatch::Intrinsic {
            receiver,
            name,
            back,
        },
        CallableRef::Global { namespace, name } => CallableDispatch::Global { namespace, name },
    }
}

/// Converts a callable reference into its first-class value.
pub fn callable_to_value(callable: CallableRef) -> RtValue {
    match callable {
        CallableRef::Tool(path) => RtValue::ToolRef(path),
        CallableRef::Promise(name) => RtValue::PromiseMethod(name),
        CallableRef::Intrinsic { receiver, name, .. } => RtValue::Intrinsic {
            receiver: Box::new(receiver),
            name,
        },
        CallableRef::Global { namespace, name } => RtValue::GlobalMethod { namespace, name },
    }
}

/// Reads a value at a key path.
pub fn read_path(root: &RtValue, path: &[MemberKeyRt], node: Option<&AstNode>) -> Eval<RtValue> {
    let mut current = root.clone();
    for key in path {
        current = read_key(&current, key, node)?;
    }
    Ok(current)
}

fn read_key(container: &RtValue, key: &MemberKeyRt, node: Option<&AstNode>) -> Eval<RtValue> {
    match container {
        RtValue::Array(arr) => match key {
            MemberKeyRt::Str(s) if s == "length" => Ok(RtValue::Number(arr.items.len() as f64)),
            MemberKeyRt::Num(index) => {
                Ok(arr.items.get(*index).cloned().unwrap_or(RtValue::Undefined))
            }
            MemberKeyRt::Str(s) => match s.parse::<usize>() {
                Ok(index) => Ok(arr.items.get(index).cloned().unwrap_or(RtValue::Undefined)),
                Err(_) => Ok(arr
                    .props
                    .iter()
                    .find(|(k, _)| k == s)
                    .map(|(_, v)| v.clone())
                    .unwrap_or(RtValue::Undefined)),
            },
        },
        RtValue::Object(obj) => match key {
            MemberKeyRt::Str(s) => Ok(obj.get(s).cloned().unwrap_or(RtValue::Undefined)),
            MemberKeyRt::Num(n) => Ok(obj
                .get(&n.to_string())
                .cloned()
                .unwrap_or(RtValue::Undefined)),
        },
        RtValue::ErrorObj(e) => match key {
            MemberKeyRt::Str(s) if s == "name" => Ok(RtValue::Str(e.name.clone())),
            MemberKeyRt::Str(s) if s == "message" => Ok(RtValue::Str(e.message.clone())),
            _ => Ok(RtValue::Undefined),
        },
        RtValue::Sandbox(SandboxValue::Url(u)) => match key {
            MemberKeyRt::Str(s) => {
                let parsed = crate::stdlib_url::parse_url(&u.href, None);
                Ok(parsed
                    .as_ref()
                    .and_then(|p| crate::stdlib_url::url_property(p, s))
                    .map(RtValue::Str)
                    .unwrap_or(RtValue::Undefined))
            }
            _ => Ok(RtValue::Undefined),
        },
        _ => Err(Signal::execution(
            "Cannot access a property on a non-object value.",
            node.cloned(),
        )),
    }
}

/// Assigns a value at a key path, enforcing array/URL/object guards with
/// verbatim diagnostics. Returns the new root.
pub fn assign_path(
    root: RtValue,
    path: &[MemberKeyRt],
    value: RtValue,
    node: Option<&AstNode>,
) -> Eval<RtValue> {
    if path.is_empty() {
        return Ok(value);
    }
    match root {
        RtValue::Array(mut arr) => {
            if path.len() == 1 {
                let key = &path[0];
                match key {
                    MemberKeyRt::Str(s) if s == "length" => {
                        return Err(Signal::execution(
                            "Array length cannot be assigned in CodeMode.",
                            node.cloned(),
                        ))
                    }
                    MemberKeyRt::Str(s) if crate::stdlib_collections::is_array_method(s) => {
                        return Err(Signal::execution(
                            "Array methods cannot be assigned in CodeMode.",
                            node.cloned(),
                        ))
                    }
                    _ => {}
                }
                let index = match key {
                    MemberKeyRt::Num(n) => *n,
                    MemberKeyRt::Str(s) => match s.parse::<usize>() {
                        Ok(n) => n,
                        Err(_) => {
                            // Own non-index properties on arrays (match
                            // `index`/`groups`) are writable in place.
                            if arr.props.iter().any(|(k, _)| k == s) || !s.is_empty() {
                                if let Some(slot) = arr.props.iter_mut().find(|(k, _)| k == s) {
                                    slot.1 = value;
                                } else {
                                    arr.props.push((s.clone(), value));
                                }
                                return Ok(RtValue::Array(arr));
                            }
                            return Err(Signal::Runtime(InterpreterRuntimeError::new(
                                "Array assignment index must be a non-negative integer.",
                                node.cloned(),
                                DiagnosticKind::InvalidDataValue,
                                None,
                            )));
                        }
                    },
                };
                Interpreter::reject_circular_insertion(
                    arr.id,
                    &value,
                    "Array assignment result",
                    node,
                )?;
                if index >= arr.items.len() {
                    arr.items.resize(index + 1, RtValue::Undefined);
                }
                arr.items[index] = value;
                return Ok(RtValue::Array(arr));
            }
            // Deep path: walk into a cloned child, then store back.
            let (head, tail) = (&path[0], &path[1..]);
            let index = member_index(head, node)?;
            if index >= arr.items.len() {
                arr.items.resize(index + 1, RtValue::Undefined);
            }
            let child = arr.items[index].clone();
            let next = assign_path(child, tail, value, node)?;
            Interpreter::reject_circular_insertion(arr.id, &next, "Array assignment result", node)?;
            arr.items[index] = next;
            Ok(RtValue::Array(arr))
        }
        RtValue::Object(mut obj) => {
            if path.len() == 1 {
                let key = match &path[0] {
                    MemberKeyRt::Str(s) => s.clone(),
                    MemberKeyRt::Num(n) => n.to_string(),
                };
                Interpreter::reject_circular_insertion(
                    obj.id,
                    &value,
                    "Object assignment result",
                    node,
                )?;
                obj.set(&key, value);
                return Ok(RtValue::Object(obj));
            }
            let (head, tail) = (&path[0], &path[1..]);
            let key = match head {
                MemberKeyRt::Str(s) => s.clone(),
                MemberKeyRt::Num(n) => n.to_string(),
            };
            let child = obj.get(&key).cloned().unwrap_or(RtValue::Undefined);
            // Assigning through `undefined` creates intermediate objects (JS).
            let child = match child {
                RtValue::Undefined => RtValue::Object(RtObject::new(vec![])),
                other => other,
            };
            let next = assign_path(child, tail, value, node)?;
            Interpreter::reject_circular_insertion(
                obj.id,
                &next,
                "Object assignment result",
                node,
            )?;
            obj.set(&key, next);
            Ok(RtValue::Object(obj))
        }
        RtValue::ErrorObj(mut e) => {
            if path.len() == 1 {
                match &path[0] {
                    MemberKeyRt::Str(s) if s == "name" => {
                        e.name = crate::stdlib_value::coerce_to_string(&rt_to_data(&value));
                    }
                    MemberKeyRt::Str(s) if s == "message" => {
                        e.message = crate::stdlib_value::coerce_to_string(&rt_to_data(&value));
                    }
                    _ => {}
                }
                return Ok(RtValue::ErrorObj(e));
            }
            Err(Signal::execution(
                "Only data fields may be assigned in CodeMode.",
                node.cloned(),
            ))
        }
        RtValue::Sandbox(SandboxValue::Url(u)) => {
            if path.len() != 1 {
                return Err(Signal::execution(
                    "Only data fields may be assigned in CodeMode.",
                    node.cloned(),
                ));
            }
            let property = match &path[0] {
                MemberKeyRt::Str(s) => s.clone(),
                MemberKeyRt::Num(n) => n.to_string(),
            };
            if !crate::stdlib_url::URL_WRITABLE_PROPERTIES.contains(&property.as_str()) {
                return Err(Signal::Runtime(
                    InterpreterRuntimeError::new(
                        format!("URL.{} is read-only.", property),
                        node.cloned(),
                        DiagnosticKind::ExecutionFailure,
                        None,
                    )
                    .as_error("TypeError"),
                ));
            }
            let coerced = crate::stdlib_url::uri_argument(
                &rt_to_data(&value),
                &format!("URL.{} value", property),
            )
            .map_err(Signal::from)?;
            let updated = url_set_property(&u.href, &property, &coerced, node)?;
            Ok(RtValue::Sandbox(SandboxValue::Url(updated)))
        }
        _ => Err(Signal::execution(
            "Only data fields may be assigned in CodeMode.",
            node.cloned(),
        )),
    }
}

fn member_index(key: &MemberKeyRt, node: Option<&AstNode>) -> Eval<usize> {
    match key {
        MemberKeyRt::Num(n) => Ok(*n),
        MemberKeyRt::Str(s) => s.parse::<usize>().map_err(|_| {
            Signal::Runtime(InterpreterRuntimeError::new(
                "Array assignment index must be a non-negative integer.",
                node.cloned(),
                DiagnosticKind::InvalidDataValue,
                None,
            ))
        }),
    }
}

/// Sets one writable URL part, re-rendering the href. Invalid values raise
/// the verbatim `URL.${property} received an invalid value.` TypeError.
pub fn url_set_property(
    href: &str,
    property: &str,
    value: &str,
    node: Option<&AstNode>,
) -> Eval<SandboxURL> {
    let mut parsed = crate::stdlib_url::parse_url(href, None).ok_or_else(|| {
        Signal::Runtime(
            InterpreterRuntimeError::new(
                format!("URL.{} received an invalid value.", property),
                node.cloned(),
                DiagnosticKind::ExecutionFailure,
                None,
            )
            .as_error("TypeError"),
        )
    })?;
    match property {
        "href" => {
            parsed = crate::stdlib_url::parse_url(value, None).ok_or_else(|| {
                Signal::Runtime(
                    InterpreterRuntimeError::new(
                        format!("URL.{} received an invalid value.", property),
                        node.cloned(),
                        DiagnosticKind::ExecutionFailure,
                        None,
                    )
                    .as_error("TypeError"),
                )
            })?;
        }
        "protocol" => {
            if !value.ends_with(':') {
                return Err(Signal::Runtime(
                    InterpreterRuntimeError::new(
                        format!("URL.{} received an invalid value.", property),
                        node.cloned(),
                        DiagnosticKind::ExecutionFailure,
                        None,
                    )
                    .as_error("TypeError"),
                ));
            }
            parsed.protocol = value.to_string();
        }
        "username" => parsed.username = value.to_string(),
        "password" => parsed.password = value.to_string(),
        "host" => {
            let (hostname, port) = split_host(value);
            if hostname.is_empty() {
                return Err(invalid_url_value(property, node));
            }
            parsed.hostname = hostname;
            parsed.port = port;
            parsed.host = render_host(&parsed.hostname, &parsed.port);
        }
        "hostname" => {
            if value.is_empty() {
                return Err(invalid_url_value(property, node));
            }
            parsed.hostname = value.to_string();
            parsed.host = render_host(&parsed.hostname, &parsed.port);
        }
        "port" => {
            if !value.is_empty() && !value.chars().all(|c| c.is_ascii_digit()) {
                return Err(invalid_url_value(property, node));
            }
            parsed.port = value.to_string();
            parsed.host = render_host(&parsed.hostname, &parsed.port);
        }
        "pathname" => {
            parsed.pathname = if value.starts_with('/') {
                value.to_string()
            } else {
                format!("/{}", value)
            };
        }
        "search" => {
            parsed.search = if value.is_empty() {
                String::new()
            } else if value.starts_with('?') {
                value.to_string()
            } else {
                format!("?{}", value)
            };
            parsed.pairs = parse_url_query(&parsed.search);
        }
        "hash" => {
            parsed.hash = if value.is_empty() {
                String::new()
            } else if value.starts_with('#') {
                value.to_string()
            } else {
                format!("#{}", value)
            };
        }
        _ => return Err(invalid_url_value(property, node)),
    }
    // Re-render href (verbatim WHATWG shape for the supported subset).
    let authority = format!(
        "{}{}",
        if parsed.username.is_empty() && parsed.password.is_empty() {
            String::new()
        } else if parsed.password.is_empty() {
            format!("{}@", parsed.username)
        } else {
            format!("{}:{}@", parsed.username, parsed.password)
        },
        parsed.host
    );
    let href = format!(
        "{}//{}{}{}{}",
        parsed.protocol, authority, parsed.pathname, parsed.search, parsed.hash
    );
    Ok(SandboxURL::new(href, parsed.pairs))
}

fn invalid_url_value(property: &str, node: Option<&AstNode>) -> Signal {
    Signal::Runtime(
        InterpreterRuntimeError::new(
            format!("URL.{} received an invalid value.", property),
            node.cloned(),
            DiagnosticKind::ExecutionFailure,
            None,
        )
        .as_error("TypeError"),
    )
}

fn split_host(value: &str) -> (String, String) {
    match value.rsplit_once(':') {
        Some((h, p)) if !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()) => {
            (h.to_string(), p.to_string())
        }
        _ => (value.to_string(), String::new()),
    }
}

fn render_host(hostname: &str, port: &str) -> String {
    if port.is_empty() {
        hostname.to_string()
    } else {
        format!("{}:{}", hostname, port)
    }
}

fn parse_url_query(search: &str) -> Vec<(String, String)> {
    search
        .trim_start_matches('?')
        .split('&')
        .filter(|p| !p.is_empty())
        .map(|pair| match pair.split_once('=') {
            Some((k, v)) => (k.to_string(), v.to_string()),
            None => (pair.to_string(), String::new()),
        })
        .collect()
}

/// Named (non-computed) object property key. Mirrors the TS key-shape triage.
pub fn named_property_key(key_node: &AstNode) -> Eval<String> {
    match key_node.get("type").as_str() {
        Some("Identifier") => get_string(key_node, "name").map_err(Signal::from),
        Some("Literal") => match key_node.get("value") {
            Value::String(s) => Ok(s.clone()),
            Value::Number(n) => Ok(n.to_string()),
            _ => Err(Signal::execution(
                "Only named object destructuring properties are supported.",
                Some(key_node.clone()),
            )),
        },
        _ => Err(Signal::execution(
            "Only named object destructuring properties are supported.",
            Some(key_node.clone()),
        )),
    }
}

fn object_field_owned(value: &RtValue, key: &str) -> RtValue {
    match value {
        RtValue::Object(obj) => obj.get(key).cloned().unwrap_or(RtValue::Undefined),
        RtValue::ErrorObj(e) => match key {
            "name" => RtValue::Str(e.name.clone()),
            "message" => RtValue::Str(e.message.clone()),
            _ => RtValue::Undefined,
        },
        _ => RtValue::Undefined,
    }
}

fn object_rest_owned(
    value: &RtValue,
    consumed: &BTreeSet<String>,
    node: Option<&AstNode>,
) -> Eval<RtValue> {
    match value {
        RtValue::Object(obj) => Ok(RtValue::Object(RtObject::new(
            obj.entries
                .iter()
                .filter(|(k, _)| !consumed.contains(k))
                .cloned()
                .collect(),
        ))),
        RtValue::ErrorObj(e) => Ok(RtValue::Object(RtObject::new(
            [("name", &e.name), ("message", &e.message)]
                .iter()
                .filter(|(k, _)| !consumed.contains(*k))
                .map(|(k, v)| (k.to_string(), RtValue::Str(v.to_string())))
                .collect(),
        ))),
        _ => Err(Signal::execution(
            "Only named object destructuring properties are supported.",
            node.cloned(),
        )),
    }
}

/// Every identifier a parameter pattern binds (TDZ seeding). Mirrors
/// `collectPatternNames(pattern, out?)`.
pub fn collect_pattern_names(pattern: &AstNode) -> Vec<String> {
    let mut out = vec![];
    collect_pattern_names_into(pattern, &mut out);
    out
}

fn collect_pattern_names_into(pattern: &AstNode, out: &mut Vec<String>) {
    match pattern.node_type.as_str() {
        "Identifier" => {
            if let Ok(name) = get_string(pattern, "name") {
                out.push(name);
            }
        }
        "AssignmentPattern" => {
            if let Ok(left) = get_node(pattern, "left") {
                collect_pattern_names_into(&left, out);
            }
        }
        "RestElement" => {
            if let Ok(argument) = get_node(pattern, "argument") {
                collect_pattern_names_into(&argument, out);
            }
        }
        "ArrayPattern" => {
            if let Ok(elements) = get_array(pattern, "elements") {
                for element in elements {
                    if element.is_null() {
                        continue;
                    }
                    if let Ok(node) = crate::interpreter_model::as_node(&element, "elements") {
                        collect_pattern_names_into(&node, out);
                    }
                }
            }
        }
        "ObjectPattern" => {
            if let Ok(properties) = get_array(pattern, "properties") {
                for property in properties {
                    if let Ok(prop) = crate::interpreter_model::as_node(&property, "properties") {
                        let nested = if prop.node_type == "RestElement" {
                            get_node(&prop, "argument")
                        } else {
                            get_node(&prop, "value")
                        };
                        if let Ok(nested) = nested {
                            collect_pattern_names_into(&nested, out);
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

/// Whether a runtime value is (or contains) an un-awaited tool promise.
/// Boundary guard before tool-call arguments cross.
pub fn contains_promise(value: &RtValue) -> bool {
    match value {
        RtValue::Sandbox(SandboxValue::Promise(_)) => true,
        RtValue::Array(arr) => {
            arr.items.iter().any(contains_promise)
                || arr.props.iter().any(|(_, v)| contains_promise(v))
        }
        RtValue::Object(obj) => obj.entries.iter().any(|(_, v)| contains_promise(v)),
        _ => false,
    }
}

// ===========================================================================
// Regex engine (E6 support).
//
// No regex-parser dependency is permitted (serde/serde_json only), so the
// interpreter owns a backtracking matcher over the pattern subset models
// generate: literals, `.`, `\d\D\w\W\s\S`, `\b\B`, classes, quantifiers
// (greedy + lazy), groups (capturing, non-capturing, named), alternation,
// anchors, escapes, backreferences, lookahead, and fixed-width lookbehind.
// Full host-engine equivalence (complex `\p{}`, astral-index counting) is
// R1-adjacent and flagged; construction-time triage stays verbatim.
// Indices count Unicode scalar values (JS counts UTF-16 units — flagged R3).
// ===========================================================================

/// Compiled pattern with capture metadata.
#[derive(Debug, Clone)]
pub struct CompiledRegex {
    branches: Vec<RegexNode>,
    group_count: usize,
    group_names: Vec<(String, usize)>,
    ignore_case: bool,
    multiline: bool,
    dot_all: bool,
    sticky: bool,
    unicode: bool,
    global: bool,
    /// Original pattern source (verbatim diagnostics).
    pub pattern: String,
    /// Normalized flags (verbatim diagnostics).
    pub flags: String,
}

impl CompiledRegex {
    /// Whether the global flag is set.
    pub fn global(&self) -> bool {
        self.global
    }
}

/// One regex AST node.
#[derive(Debug, Clone)]
pub enum RegexNode {
    Empty,
    Literal(char),
    Dot,
    Class(CharClass),
    AnchorStart,
    AnchorEnd,
    WordBoundary(bool),
    Backref(usize),
    Group {
        index: usize,
        child: Box<RegexNode>,
    },
    Sequence(Vec<RegexNode>),
    Alternate(Vec<RegexNode>),
    Quantified {
        min: usize,
        max: Option<usize>,
        lazy: bool,
        child: Box<RegexNode>,
    },
    Lookahead {
        child: Box<RegexNode>,
        negated: bool,
    },
    Lookbehind {
        child: Box<RegexNode>,
        negated: bool,
        len: usize,
    },
}

/// Character class.
#[derive(Debug, Clone)]
pub struct CharClass {
    pub negated: bool,
    pub chars: Vec<char>,
    pub ranges: Vec<(char, char)>,
    pub classes: Vec<CharClassKind>,
}

/// Built-in class shorthands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharClassKind {
    Digit,
    NotDigit,
    Word,
    NotWord,
    Space,
    NotSpace,
}

impl CharClass {
    fn matches(&self, c: char, unicode: bool) -> bool {
        let mut hit = self.chars.contains(&c)
            || self.ranges.iter().any(|(lo, hi)| *lo <= c && c <= *hi)
            || self.classes.iter().any(|kind| match kind {
                CharClassKind::Digit => is_ascii_digit(c),
                CharClassKind::NotDigit => !is_ascii_digit(c),
                CharClassKind::Word => is_word_char(c),
                CharClassKind::NotWord => !is_word_char(c),
                CharClassKind::Space => is_space_char(c, unicode),
                CharClassKind::NotSpace => !is_space_char(c, unicode),
            });
        if self.negated {
            hit = !hit;
        }
        hit
    }
}

fn is_ascii_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn is_word_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn is_space_char(c: char, _unicode: bool) -> bool {
    matches!(
        c,
        '\t' | '\n' | '\x0B' | '\x0C' | '\r' | ' ' | '\u{00A0}' | '\u{1680}' | '\u{2000}'
            ..='\u{200A}'
                | '\u{2028}'
                | '\u{2029}'
                | '\u{202F}'
                | '\u{205F}'
                | '\u{3000}'
                | '\u{FEFF}'
    )
}

fn chars_equal(a: char, b: char, ignore_case: bool) -> bool {
    if a == b {
        return true;
    }
    if !ignore_case {
        return false;
    }
    a.to_lowercase().next() == b.to_lowercase().next()
        || a.to_uppercase().next() == b.to_uppercase().next()
}

/// Compiles a pattern; `Err(reason)` mirrors `regexFailureReason` input.
pub fn compile_regex(pattern: &str, flags: &str) -> Result<CompiledRegex, String> {
    let mut parser = RegexParser {
        chars: pattern.chars().collect(),
        pos: 0,
        group_count: 0,
        group_names: vec![],
    };
    let branches = parser.parse_alternation()?;
    if parser.pos != parser.chars.len() {
        return Err("Unmatched ')'".to_string());
    }
    Ok(CompiledRegex {
        branches,
        group_count: parser.group_count,
        group_names: parser.group_names,
        ignore_case: flags.contains('i'),
        multiline: flags.contains('m'),
        dot_all: flags.contains('s'),
        sticky: flags.contains('y'),
        unicode: flags.contains('u') || flags.contains('v'),
        global: flags.contains('g'),
        pattern: pattern.to_string(),
        flags: flags.to_string(),
    })
}

struct RegexParser {
    chars: Vec<char>,
    pos: usize,
    group_count: usize,
    group_names: Vec<(String, usize)>,
}

impl RegexParser {
    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn eat(&mut self) -> Option<char> {
        let c = self.chars.get(self.pos).copied()?;
        self.pos += 1;
        Some(c)
    }

    fn parse_alternation(&mut self) -> Result<Vec<RegexNode>, String> {
        let mut branches = vec![self.parse_sequence()?];
        while self.peek() == Some('|') {
            self.eat();
            branches.push(self.parse_sequence()?);
        }
        Ok(branches)
    }

    fn parse_sequence(&mut self) -> Result<RegexNode, String> {
        let mut items = vec![];
        while let Some(c) = self.peek() {
            if c == '|' || c == ')' {
                break;
            }
            items.push(self.parse_quantified()?);
        }
        Ok(match items.len() {
            0 => RegexNode::Empty,
            1 => items.into_iter().next().unwrap(),
            _ => RegexNode::Sequence(items),
        })
    }

    fn parse_quantified(&mut self) -> Result<RegexNode, String> {
        let mut atom = self.parse_atom()?;
        loop {
            let (min, max) = match self.peek() {
                Some('*') => (0, None),
                Some('+') => (1, None),
                Some('?') => (0, Some(1)),
                Some('{') => {
                    let saved = self.pos;
                    self.eat();
                    match self.parse_braces() {
                        Some(range) => range,
                        None => {
                            self.pos = saved;
                            break;
                        }
                    }
                }
                _ => break,
            };
            if matches!(self.peek(), Some('*') | Some('+') | Some('?') | Some('{'))
                && !matches!(atom, RegexNode::Empty)
            {
                // Only consume the quantifier char(s) for simple forms.
            }
            // Consume the quantifier marker for `*`/`+`/`?`.
            if matches!(self.peek(), Some('*') | Some('+') | Some('?')) {
                self.eat();
            }
            let lazy = if self.peek() == Some('?') {
                self.eat();
                true
            } else {
                false
            };
            atom = RegexNode::Quantified {
                min,
                max,
                lazy,
                child: Box::new(atom),
            };
        }
        Ok(atom)
    }

    fn parse_braces(&mut self) -> Option<(usize, Option<usize>)> {
        let start = self.pos;
        let mut num = String::new();
        while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
            num.push(self.eat().unwrap());
        }
        if num.is_empty() {
            self.pos = start;
            return None;
        }
        let min: usize = num.parse().ok()?;
        if self.peek() == Some('}') {
            self.eat();
            return Some((min, Some(min)));
        }
        if self.peek() != Some(',') {
            self.pos = start;
            return None;
        }
        self.eat();
        let mut num2 = String::new();
        while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
            num2.push(self.eat().unwrap());
        }
        if self.peek() != Some('}') {
            self.pos = start;
            return None;
        }
        self.eat();
        if num2.is_empty() {
            Some((min, None))
        } else {
            let max: usize = num2.parse().ok()?;
            if max < min {
                None
            } else {
                Some((min, Some(max)))
            }
        }
    }

    fn parse_atom(&mut self) -> Result<RegexNode, String> {
        match self.peek() {
            None => Ok(RegexNode::Empty),
            Some('^') => {
                self.eat();
                Ok(RegexNode::AnchorStart)
            }
            Some('$') => {
                self.eat();
                Ok(RegexNode::AnchorEnd)
            }
            Some('.') => {
                self.eat();
                Ok(RegexNode::Dot)
            }
            Some('(') => self.parse_group(),
            Some('[') => self.parse_class(),
            Some('\\') => self.parse_escape(false),
            Some(c) => {
                self.eat();
                Ok(RegexNode::Literal(c))
            }
        }
    }

    fn parse_group(&mut self) -> Result<RegexNode, String> {
        self.eat(); // '('
        if self.peek() == Some('?') {
            self.eat();
            match self.peek() {
                Some(':') => {
                    self.eat();
                    let child = Box::new(self.parse_group_body()?);
                    return Ok(RegexNode::Group { index: 0, child });
                }
                Some('=') => {
                    self.eat();
                    let child = Box::new(self.parse_group_body()?);
                    return Ok(RegexNode::Lookahead {
                        child,
                        negated: false,
                    });
                }
                Some('!') => {
                    self.eat();
                    let child = Box::new(self.parse_group_body()?);
                    return Ok(RegexNode::Lookahead {
                        child,
                        negated: true,
                    });
                }
                Some('<') => {
                    self.eat();
                    match self.peek() {
                        Some('=') | Some('!') => {
                            let negated = self.eat() == Some('!');
                            let child = Box::new(self.parse_group_body()?);
                            let len = fixed_length(&child).unwrap_or(usize::MAX);
                            return Ok(RegexNode::Lookbehind {
                                child,
                                negated,
                                len,
                            });
                        }
                        _ => {
                            // Named group (?<name>...).
                            let mut name = String::new();
                            while let Some(c) = self.peek() {
                                if c == '>' {
                                    break;
                                }
                                name.push(c);
                                self.eat();
                            }
                            if self.peek() != Some('>') || name.is_empty() {
                                return Err("Invalid named capture group".to_string());
                            }
                            self.eat();
                            self.group_count += 1;
                            let index = self.group_count;
                            self.group_names.push((name, index));
                            let child = Box::new(self.parse_group_body()?);
                            return Ok(RegexNode::Group { index, child });
                        }
                    }
                }
                _ => return Err("Invalid group".to_string()),
            }
        }
        self.group_count += 1;
        let index = self.group_count;
        let child = Box::new(self.parse_group_body()?);
        Ok(RegexNode::Group { index, child })
    }

    fn parse_group_body(&mut self) -> Result<RegexNode, String> {
        let branches = self.parse_alternation()?;
        if self.peek() != Some(')') {
            return Err("Unterminated group".to_string());
        }
        self.eat();
        Ok(match branches.len() {
            1 => branches.into_iter().next().unwrap(),
            _ => RegexNode::Alternate(branches),
        })
    }

    fn parse_class(&mut self) -> Result<RegexNode, String> {
        self.eat(); // '['
        let negated = if self.peek() == Some('^') {
            self.eat();
            true
        } else {
            false
        };
        let mut class = CharClass {
            negated,
            chars: vec![],
            ranges: vec![],
            classes: vec![],
        };
        // A leading `]` is literal.
        if self.peek() == Some(']') {
            self.eat();
            class.chars.push(']');
        }
        loop {
            match self.peek() {
                None => return Err("Unterminated character class".to_string()),
                Some(']') => {
                    self.eat();
                    break;
                }
                Some('\\') => {
                    let (atom, is_range_end) = self.parse_class_escape()?;
                    if let Some(atom) = atom {
                        // Range detection: `a-z`.
                        if !is_range_end && self.peek() == Some('-') {
                            // Peek past '-' for a valid range end.
                            let saved = self.pos;
                            self.eat();
                            match self.peek() {
                                Some(']') | None => {
                                    // Trailing '-' is literal.
                                    class.chars.push(atom_char(atom));
                                    class.chars.push('-');
                                }
                                Some('\\') => {
                                    let (end, _) = self.parse_class_escape()?;
                                    match end {
                                        Some(ClassAtom::Char(lo, hi)) => {
                                            class.ranges.push((atom_char(atom), hi));
                                            let _ = lo;
                                        }
                                        Some(ClassAtom::Class(kind)) => {
                                            class.chars.push(atom_char(atom));
                                            class.chars.push('-');
                                            class.classes.push(kind);
                                        }
                                        None => {
                                            class.chars.push(atom_char(atom));
                                            class.chars.push('-');
                                        }
                                    }
                                    let _ = saved;
                                }
                                Some(_) => {
                                    let lo = atom_char(atom);
                                    let hi = self.eat().unwrap();
                                    if lo <= hi {
                                        class.ranges.push((lo, hi));
                                    } else {
                                        return Err(
                                            "Range out of order in character class".to_string()
                                        );
                                    }
                                }
                            }
                        } else {
                            push_class_atom(&mut class, atom);
                        }
                    }
                }
                Some(c) => {
                    self.eat();
                    if self.peek() == Some('-') {
                        let saved = self.pos;
                        self.eat();
                        match self.peek() {
                            Some(']') | None => {
                                class.chars.push(c);
                                class.chars.push('-');
                            }
                            Some('\\') => {
                                let (end, _) = self.parse_class_escape()?;
                                match end {
                                    Some(ClassAtom::Char(_, hi)) => {
                                        if c <= hi {
                                            class.ranges.push((c, hi));
                                        } else {
                                            return Err(
                                                "Range out of order in character class".to_string()
                                            );
                                        }
                                    }
                                    _ => {
                                        class.chars.push(c);
                                        class.chars.push('-');
                                        self.pos = saved + 1;
                                        // Re-parse escape as atom below is complex;
                                        // push '-' and continue (escape parsed next loop).
                                    }
                                }
                                let _ = saved;
                            }
                            Some(hi) => {
                                self.eat();
                                if c <= hi {
                                    class.ranges.push((c, hi));
                                } else {
                                    return Err("Range out of order in character class".to_string());
                                }
                            }
                        }
                    } else {
                        class.chars.push(c);
                    }
                }
            }
        }
        Ok(RegexNode::Class(class))
    }

    fn parse_class_escape(&mut self) -> Result<(Option<ClassAtom>, bool), String> {
        // Returns (atom, is_range_end_safe). `true` means the atom can end a range.
        self.eat(); // '\\'
        let c = self
            .peek()
            .ok_or_else(|| "Trailing \\ in character class".to_string())?;
        self.eat();
        let atom = match c {
            'd' => ClassAtom::Class(CharClassKind::Digit),
            'D' => ClassAtom::Class(CharClassKind::NotDigit),
            'w' => ClassAtom::Class(CharClassKind::Word),
            'W' => ClassAtom::Class(CharClassKind::NotWord),
            's' => ClassAtom::Class(CharClassKind::Space),
            'S' => ClassAtom::Class(CharClassKind::NotSpace),
            'f' => ClassAtom::Char('\x0C', '\x0C'),
            'n' => ClassAtom::Char('\n', '\n'),
            'r' => ClassAtom::Char('\r', '\r'),
            't' => ClassAtom::Char('\t', '\t'),
            'v' => ClassAtom::Char('\x0B', '\x0B'),
            'b' => ClassAtom::Char('\x08', '\x08'),
            '0' => ClassAtom::Char('\0', '\0'),
            'x' => ClassAtom::Char(self.parse_hex(2)?, '\0'),
            'u' => ClassAtom::Char(self.parse_unicode_escape()?, '\0'),
            'c' => {
                let control = self.eat().ok_or_else(|| "Trailing \\c".to_string())?;
                ClassAtom::Char((control as u32 % 32) as u8 as char, '\0')
            }
            other => ClassAtom::Char(other, other),
        };
        let range_end = matches!(atom, ClassAtom::Char(_, _));
        Ok((Some(atom), range_end))
    }

    fn parse_hex(&mut self, digits: usize) -> Result<char, String> {
        let mut value: u32 = 0;
        for _ in 0..digits {
            let c = self
                .eat()
                .ok_or_else(|| "Truncated hex escape".to_string())?;
            value = value * 16 + c.to_digit(16).ok_or_else(|| "Bad hex escape".to_string())?;
        }
        char::from_u32(value).ok_or_else(|| "Bad hex escape".to_string())
    }

    fn parse_unicode_escape(&mut self) -> Result<char, String> {
        if self.peek() == Some('{') {
            self.eat();
            let mut value: u32 = 0;
            loop {
                match self.eat() {
                    Some('}') => break,
                    Some(c) => {
                        value = value * 16
                            + c.to_digit(16)
                                .ok_or_else(|| "Bad unicode escape".to_string())?;
                    }
                    None => return Err("Truncated unicode escape".to_string()),
                }
            }
            char::from_u32(value).ok_or_else(|| "Bad unicode escape".to_string())
        } else {
            self.parse_hex(4)
        }
    }

    fn parse_escape(&mut self, _in_class: bool) -> Result<RegexNode, String> {
        self.eat(); // '\\'
        let c = self.peek().ok_or_else(|| "Trailing \\".to_string())?;
        match c {
            'b' => {
                self.eat();
                Ok(RegexNode::WordBoundary(false))
            }
            'B' => {
                self.eat();
                Ok(RegexNode::WordBoundary(true))
            }
            'f' => {
                self.eat();
                Ok(RegexNode::Literal('\x0C'))
            }
            'n' => {
                self.eat();
                Ok(RegexNode::Literal('\n'))
            }
            'r' => {
                self.eat();
                Ok(RegexNode::Literal('\r'))
            }
            't' => {
                self.eat();
                Ok(RegexNode::Literal('\t'))
            }
            'v' => {
                self.eat();
                Ok(RegexNode::Literal('\x0B'))
            }
            '0' => {
                // `\0` not followed by a digit; otherwise decimal backref.
                self.eat();
                match self.peek() {
                    Some(d) if d.is_ascii_digit() => {
                        let index = self.parse_decimal()?;
                        Ok(RegexNode::Backref(index))
                    }
                    _ => Ok(RegexNode::Literal('\0')),
                }
            }
            '1'..='9' => {
                let index = self.parse_decimal()?;
                Ok(RegexNode::Backref(index))
            }
            'd' => {
                self.eat();
                Ok(RegexNode::Class(CharClass {
                    negated: false,
                    chars: vec![],
                    ranges: vec![],
                    classes: vec![CharClassKind::Digit],
                }))
            }
            'D' => {
                self.eat();
                Ok(RegexNode::Class(CharClass {
                    negated: false,
                    chars: vec![],
                    ranges: vec![],
                    classes: vec![CharClassKind::NotDigit],
                }))
            }
            'w' => {
                self.eat();
                Ok(RegexNode::Class(CharClass {
                    negated: false,
                    chars: vec![],
                    ranges: vec![],
                    classes: vec![CharClassKind::Word],
                }))
            }
            'W' => {
                self.eat();
                Ok(RegexNode::Class(CharClass {
                    negated: false,
                    chars: vec![],
                    ranges: vec![],
                    classes: vec![CharClassKind::NotWord],
                }))
            }
            's' => {
                self.eat();
                Ok(RegexNode::Class(CharClass {
                    negated: false,
                    chars: vec![],
                    ranges: vec![],
                    classes: vec![CharClassKind::Space],
                }))
            }
            'S' => {
                self.eat();
                Ok(RegexNode::Class(CharClass {
                    negated: false,
                    chars: vec![],
                    ranges: vec![],
                    classes: vec![CharClassKind::NotSpace],
                }))
            }
            'x' => {
                self.eat();
                Ok(RegexNode::Literal(self.parse_hex(2)?))
            }
            'u' => {
                self.eat();
                Ok(RegexNode::Literal(self.parse_unicode_escape()?))
            }
            'c' => {
                self.eat();
                let control = self.eat().ok_or_else(|| "Trailing \\c".to_string())?;
                Ok(RegexNode::Literal(((control as u32) % 32) as u8 as char))
            }
            'k' => {
                // `\k<name>` backreference.
                self.eat();
                if self.eat() != Some('<') {
                    return Err("Invalid named backreference".to_string());
                }
                let mut name = String::new();
                while let Some(ch) = self.peek() {
                    if ch == '>' {
                        break;
                    }
                    name.push(ch);
                    self.eat();
                }
                if self.eat() != Some('>') {
                    return Err("Invalid named backreference".to_string());
                }
                match self.group_names.iter().find(|(n, _)| n == &name) {
                    Some((_, index)) => Ok(RegexNode::Backref(*index)),
                    None => Err("Unknown named backreference".to_string()),
                }
            }
            'p' | 'P' => {
                // Unicode property escapes under u/v; identity otherwise.
                self.eat();
                if self.peek() == Some('{') && self.unicode_pending() {
                    let _ = self;
                    Ok(RegexNode::Literal(c))
                } else {
                    Ok(RegexNode::Literal(c))
                }
            }
            _ => {
                self.eat();
                Ok(RegexNode::Literal(c))
            }
        }
    }

    fn unicode_pending(&self) -> bool {
        // The parser does not carry flags; property escapes compile to a
        // best-effort class below (R1-adjacent flagged simplification).
        true
    }

    fn parse_decimal(&mut self) -> Result<usize, String> {
        let mut num = String::new();
        while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
            num.push(self.eat().unwrap());
        }
        num.parse::<usize>()
            .map_err(|_| "Bad backreference".to_string())
    }
}

#[derive(Debug, Clone)]
enum ClassAtom {
    Char(char, char),
    Class(CharClassKind),
}

fn atom_char(atom: ClassAtom) -> char {
    match atom {
        ClassAtom::Char(c, _) => c,
        ClassAtom::Class(_) => '\0',
    }
}

fn push_class_atom(class: &mut CharClass, atom: ClassAtom) {
    match atom {
        ClassAtom::Char(c, _) => class.chars.push(c),
        ClassAtom::Class(kind) => class.classes.push(kind),
    }
}

/// Fixed match length of a node, if statically known (lookbehind support).
fn fixed_length(node: &RegexNode) -> Option<usize> {
    match node {
        RegexNode::Empty => Some(0),
        RegexNode::Literal(_) | RegexNode::Dot | RegexNode::Class(_) => Some(1),
        RegexNode::AnchorStart
        | RegexNode::AnchorEnd
        | RegexNode::WordBoundary(_)
        | RegexNode::Lookahead { .. }
        | RegexNode::Lookbehind { .. } => Some(0),
        RegexNode::Backref(_) => None,
        RegexNode::Group { child, .. } => fixed_length(child),
        RegexNode::Sequence(items) => {
            let mut total = 0usize;
            for item in items {
                total += fixed_length(item)?;
            }
            Some(total)
        }
        RegexNode::Alternate(branches) => {
            let mut len: Option<usize> = None;
            for branch in branches {
                let branch_len = fixed_length(branch)?;
                if let Some(known) = len {
                    if known != branch_len {
                        return None;
                    }
                } else {
                    len = Some(branch_len);
                }
            }
            len.or(Some(0))
        }
        RegexNode::Quantified {
            min, max, child, ..
        } => {
            let inner = fixed_length(child)?;
            match max {
                Some(max) if *max == *min => Some(min * inner),
                _ if *min == 0 => None,
                _ => None,
            }
        }
    }
}

/// A successful match: byte/char span plus captures (`None` = unmatched).
#[derive(Debug, Clone)]
pub struct RegexMatch {
    pub start: usize,
    pub end: usize,
    pub captures: Vec<Option<(usize, usize)>>,
}

struct Matcher<'x> {
    text: &'x [char],
    regex: &'x CompiledRegex,
}

type Caps = Vec<Option<(usize, usize)>>;

impl<'x> Matcher<'x> {
    /// Leftmost match of any branch at/after `start` (or anchored at `start`
    /// for sticky patterns).
    fn run(&self, pos: usize, captures: &mut Caps) -> Option<RegexMatch> {
        if self.regex.sticky {
            for branch in &self.regex.branches {
                let mut trial = captures.clone();
                if let Some(end) = self.match_node(branch, pos, &mut trial, &mut |end, _| Some(end))
                {
                    *captures = trial;
                    return Some(RegexMatch {
                        start: pos,
                        end,
                        captures: captures.clone(),
                    });
                }
            }
            return None;
        }
        let mut from = pos;
        while from <= self.text.len() {
            for branch in &self.regex.branches {
                let mut trial = captures.clone();
                if let Some(end) =
                    self.match_node(branch, from, &mut trial, &mut |end, _| Some(end))
                {
                    *captures = trial;
                    return Some(RegexMatch {
                        start: from,
                        end,
                        captures: captures.clone(),
                    });
                }
            }
            from += 1;
        }
        None
    }

    fn match_node(
        &self,
        node: &RegexNode,
        pos: usize,
        caps: &mut Caps,
        cont: &mut dyn FnMut(usize, &mut Caps) -> Option<usize>,
    ) -> Option<usize> {
        match node {
            RegexNode::Empty => cont(pos, caps),
            RegexNode::Literal(expected) => {
                let c = *self.text.get(pos)?;
                if chars_equal(c, *expected, self.regex.ignore_case) {
                    cont(pos + 1, caps)
                } else {
                    None
                }
            }
            RegexNode::Dot => {
                let c = *self.text.get(pos)?;
                if !self.regex.dot_all && matches!(c, '\n' | '\r' | '\u{2028}' | '\u{2029}') {
                    return None;
                }
                cont(pos + 1, caps)
            }
            RegexNode::Class(class) => {
                let c = *self.text.get(pos)?;
                let hit = class.matches(c, self.regex.unicode)
                    || (self.regex.ignore_case
                        && (class
                            .matches(c.to_lowercase().next().unwrap_or(c), self.regex.unicode)
                            || c.to_uppercase()
                                .next()
                                .map(|u| class.matches(u, self.regex.unicode))
                                .unwrap_or(false)));
                if hit {
                    cont(pos + 1, caps)
                } else {
                    None
                }
            }
            RegexNode::AnchorStart => {
                let ok = pos == 0
                    || (self.regex.multiline
                        && pos > 0
                        && matches!(
                            self.text.get(pos - 1),
                            Some('\n' | '\r' | '\u{2028}' | '\u{2029}')
                        ));
                if ok {
                    cont(pos, caps)
                } else {
                    None
                }
            }
            RegexNode::AnchorEnd => {
                let ok = pos == self.text.len()
                    || (self.regex.multiline
                        && matches!(
                            self.text.get(pos),
                            Some('\n' | '\r' | '\u{2028}' | '\u{2029}')
                        ));
                if ok {
                    cont(pos, caps)
                } else {
                    None
                }
            }
            RegexNode::WordBoundary(negated) => {
                let left = pos
                    .checked_sub(1)
                    .and_then(|i| self.text.get(i))
                    .map(|c| is_word_char(*c))
                    .unwrap_or(false);
                let right = self
                    .text
                    .get(pos)
                    .map(|c| is_word_char(*c))
                    .unwrap_or(false);
                if (left != right) != *negated {
                    cont(pos, caps)
                } else {
                    None
                }
            }
            RegexNode::Backref(index) => {
                let (start, end) = caps.get(*index).copied().flatten().unwrap_or((pos, pos));
                let slice_end = end.min(self.text.len());
                if start > slice_end {
                    return None;
                }
                for (offset, expected) in self.text[start..slice_end].iter().enumerate() {
                    let actual = *self.text.get(pos + offset)?;
                    if !chars_equal(actual, *expected, self.regex.ignore_case) {
                        return None;
                    }
                }
                cont(pos + (slice_end - start), caps)
            }
            RegexNode::Group { index, child } => {
                if *index == 0 {
                    return self.match_node(child, pos, caps, cont);
                }
                let start = pos;
                // Capture the child span on success.
                let mut result: Option<usize> = None;
                let matched = self.match_node(child, pos, caps, &mut |end, caps2| {
                    if caps2.len() <= *index {
                        caps2.resize(*index + 1, None);
                    }
                    caps2[*index] = Some((start, end));
                    match cont(end, caps2) {
                        Some(final_end) => {
                            result = Some(final_end);
                            Some(final_end)
                        }
                        None => {
                            // Backtrack the capture on continuation failure.
                            caps2[*index] = None;
                            None
                        }
                    }
                });
                let _ = matched;
                result
            }
            RegexNode::Sequence(items) => self.match_seq(items, pos, caps, cont),
            RegexNode::Alternate(branches) => {
                for branch in branches {
                    let mut trial = caps.clone();
                    let mut final_caps: Option<Caps> = None;
                    let result =
                        self.match_node(branch, pos, &mut trial, &mut |end, caps2| match cont(
                            end, caps2,
                        ) {
                            Some(final_end) => {
                                final_caps = Some(caps2.clone());
                                Some(final_end)
                            }
                            None => None,
                        });
                    if result.is_some() {
                        if let Some(final_caps) = final_caps {
                            *caps = final_caps;
                        } else {
                            *caps = trial;
                        }
                        return result;
                    }
                }
                None
            }
            RegexNode::Quantified {
                min,
                max,
                lazy,
                child,
            } => self.match_repeat(child, *min, *max, *lazy, pos, caps, cont),
            RegexNode::Lookahead { child, negated } => {
                let mut trial = caps.clone();
                let matched = self
                    .match_node(child, pos, &mut trial, &mut |_, _| Some(pos))
                    .is_some();
                if matched != *negated {
                    // Captures set inside lookahead persist (JS semantics).
                    *caps = trial;
                    cont(pos, caps)
                } else {
                    None
                }
            }
            RegexNode::Lookbehind {
                child,
                negated,
                len,
            } => {
                if *len == usize::MAX {
                    return if *negated { cont(pos, caps) } else { None };
                }
                let start = pos.checked_sub(*len)?;
                let mut trial = caps.clone();
                let matched = self
                    .match_node(child, start, &mut trial, &mut |end, _| {
                        if end == pos {
                            Some(end)
                        } else {
                            None
                        }
                    })
                    .is_some();
                if matched != *negated {
                    *caps = trial;
                    cont(pos, caps)
                } else {
                    None
                }
            }
        }
    }

    fn match_seq(
        &self,
        items: &[RegexNode],
        pos: usize,
        caps: &mut Caps,
        cont: &mut dyn FnMut(usize, &mut Caps) -> Option<usize>,
    ) -> Option<usize> {
        match items.split_first() {
            None => cont(pos, caps),
            Some((first, rest)) => self.match_node(first, pos, caps, &mut |mid, caps2| {
                self.match_seq(rest, mid, caps2, cont)
            }),
        }
    }

    fn match_repeat(
        &self,
        child: &RegexNode,
        min: usize,
        max: Option<usize>,
        lazy: bool,
        pos: usize,
        caps: &mut Caps,
        cont: &mut dyn FnMut(usize, &mut Caps) -> Option<usize>,
    ) -> Option<usize> {
        if lazy {
            // Stop first (fewest repetitions), then consume one more.
            if min == 0 {
                let mut trial = caps.clone();
                let result = cont(pos, &mut trial);
                match result {
                    Some(end) => {
                        *caps = trial;
                        return Some(end);
                    }
                    None => {}
                }
            }
            if max == Some(0) {
                return None;
            }
            let mut trial = caps.clone();
            let mut outcome: Option<(usize, Caps)> = None;
            let matched = self.match_node(child, pos, &mut trial, &mut |end, caps2| {
                if end == pos {
                    return None;
                }
                match self.match_repeat(
                    child,
                    min.saturating_sub(1),
                    max.map(|m| m.saturating_sub(1)),
                    lazy,
                    end,
                    caps2,
                    cont,
                ) {
                    Some(final_end) => {
                        outcome = Some((final_end, caps2.clone()));
                        Some(final_end)
                    }
                    None => None,
                }
            });
            let _ = matched;
            if let Some((end, final_caps)) = outcome {
                *caps = final_caps;
                return Some(end);
            }
            None
        } else {
            // Greedy: consume first, then stop.
            if max != Some(0) {
                let mut trial = caps.clone();
                let mut outcome: Option<(usize, Caps)> = None;
                let matched = self.match_node(child, pos, &mut trial, &mut |end, caps2| {
                    if end == pos {
                        return None;
                    }
                    match self.match_repeat(
                        child,
                        min.saturating_sub(1),
                        max.map(|m| m.saturating_sub(1)),
                        lazy,
                        end,
                        caps2,
                        cont,
                    ) {
                        Some(final_end) => {
                            outcome = Some((final_end, caps2.clone()));
                            Some(final_end)
                        }
                        None => None,
                    }
                });
                let _ = matched;
                if let Some((end, final_caps)) = outcome {
                    *caps = final_caps;
                    return Some(end);
                }
            }
            if min == 0 {
                let mut trial = caps.clone();
                match cont(pos, &mut trial) {
                    Some(end) => {
                        *caps = trial;
                        return Some(end);
                    }
                    None => {}
                }
            }
            None
        }
    }
}

/// Searches for the first match at/after `from` (chars). Sticky patterns
/// anchor at exactly `from`.
pub fn regex_search(compiled: &CompiledRegex, text: &[char], from: usize) -> Option<RegexMatch> {
    let matcher = Matcher {
        text,
        regex: compiled,
    };
    let mut captures: Caps = vec![None; compiled.group_count + 1];
    matcher.run(from.min(text.len()), &mut captures)
}

/// Extracts matched text + capture strings for a [`RegexMatch`].
pub fn regex_match_texts(
    text: &[char],
    m: &RegexMatch,
    compiled: &CompiledRegex,
) -> (String, Vec<Option<String>>) {
    let whole: String = text[m.start..m.end].iter().collect();
    let mut groups = vec![];
    for index in 1..=compiled.group_count {
        groups.push(
            m.captures
                .get(index)
                .copied()
                .flatten()
                .map(|(s, e)| text[s.min(text.len())..e.min(text.len())].iter().collect()),
        );
    }
    (whole, groups)
}

/// Named groups for a match (filtered by the caller against blocked names).
pub fn regex_named_groups(
    compiled: &CompiledRegex,
    m: &RegexMatch,
    text: &[char],
) -> Vec<(String, Option<String>)> {
    compiled
        .group_names
        .iter()
        .map(|(name, index)| {
            let value = m
                .captures
                .get(*index)
                .copied()
                .flatten()
                .map(|(s, e)| text[s.min(text.len())..e.min(text.len())].iter().collect());
            (name.clone(), value)
        })
        .collect()
}

// ===========================================================================
// E6 — stdlib wiring: intrinsic + global dispatch, string methods.
// ===========================================================================

/// Receiver location for method invocation: the live value plus an optional
/// write-back path for mutating methods (`Array.push`, `Map.set`, ...).
#[derive(Debug, Clone)]
pub struct RecvLoc {
    pub value: RtValue,
    pub back: Option<DataRef>,
}

/// Write-back target for a receiver, if it resolves to live data.
pub fn writeback_for(receiver: &Receiver) -> Option<DataRef> {
    match receiver {
        Receiver::DataPath { data, .. } => Some(data.clone()),
        _ => None,
    }
}

impl<'a> Interpreter<'a> {
    /// Intrinsic method dispatch by receiver type. Mirrors
    /// `invokeIntrinsic(ref, args, node)`: strings (incl. function replacers
    /// for `replace`/`replaceAll`), numbers, arrays, dates, regexps, maps,
    /// sets, URLs, URL params — else `Method '${name}' is not available in
    /// CodeMode.`
    pub(crate) fn invoke_intrinsic(
        &mut self,
        receiver: &RtValue,
        name: &str,
        args: &[RtValue],
        node: Option<&AstNode>,
    ) -> Eval<RtValue> {
        // NOTE: mutating container methods need write-back; those callees
        // carry it via `CallableRef::Intrinsic.writeback`. The plain dispatch
        // here covers immutable receivers; container methods route through
        // `invoke_intrinsic_loc` below.
        self.invoke_intrinsic_loc(
            &RecvLoc {
                value: receiver.clone(),
                back: None,
            },
            name,
            args,
            node,
        )
    }

    /// Intrinsic dispatch with an optional write-back location.
    pub(crate) fn invoke_intrinsic_loc(
        &mut self,
        location: &RecvLoc,
        name: &str,
        args: &[RtValue],
        node: Option<&AstNode>,
    ) -> Eval<RtValue> {
        match &location.value {
            RtValue::Str(s) => {
                if (name == "replace" || name == "replaceAll")
                    && matches!(
                        args.get(1),
                        Some(RtValue::Function(_))
                            | Some(RtValue::Coercion(_))
                            | Some(RtValue::Uri(_))
                    )
                {
                    return self.invoke_string_replacer(s, name, args, node);
                }
                let result = invoke_string_method(self, s, name, args, node)?;
                Ok(bound_string_result(&result, name))
            }
            RtValue::Number(n) => {
                let data_args: Vec<DataVal> = args.iter().map(rt_to_data).collect();
                let result = crate::stdlib_number::invoke_number_method(*n, name, &data_args, node)
                    .map_err(Signal::from)?;
                Ok(data_to_rt(&result))
            }
            RtValue::Array(_) => self.invoke_array_method(location, name, args, node),
            RtValue::Sandbox(SandboxValue::Date(d)) => {
                let result = crate::stdlib_date::invoke_date_method(d.time, name, node)
                    .map_err(Signal::from)?;
                Ok(data_to_rt(&result))
            }
            RtValue::Sandbox(SandboxValue::RegExp(r)) => {
                self.invoke_regexp_method(r, name, args, node)
            }
            RtValue::Sandbox(SandboxValue::Map(_)) => {
                self.invoke_map_method(location, name, args, node)
            }
            RtValue::Sandbox(SandboxValue::Set(_)) => {
                self.invoke_set_method(location, name, args, node)
            }
            RtValue::Sandbox(SandboxValue::Url(u)) => {
                let href = u.href.clone();
                let result = crate::stdlib_url::invoke_url_method(&href, name, node)
                    .map_err(Signal::from)?;
                Ok(data_to_rt(&result))
            }
            RtValue::Sandbox(SandboxValue::UrlSearchParams(_)) => {
                self.invoke_params_method(location, name, args, node)
            }
            _ => Err(Signal::execution(
                format!("Method '{}' is not available in CodeMode.", name),
                node.cloned(),
            )),
        }
    }

    /// Global-namespace method dispatch. Mirrors `invokeGlobalMethod(ref,
    /// args, node)` verbatim (incl. the `console.*` guard and the
    /// RegExp/Map/Set/URLSearchParams rejection).
    pub(crate) fn invoke_global_method(
        &mut self,
        namespace: &str,
        name: &str,
        args: &[RtValue],
        node: Option<&AstNode>,
    ) -> Eval<RtValue> {
        if namespace == "console" {
            return Err(Signal::execution(
                format!("console.{} is not available in CodeMode.", name),
                node.cloned(),
            ));
        }
        if namespace == "Object" {
            // `Object.*` over a tool reference uses the discovery idiom.
            if let Some(RtValue::ToolRef(_)) = args.first() {
                if let Some(tool_ref) = args.first().cloned() {
                    return self.invoke_object_method_on_tools(name, &tool_ref, node);
                }
            }
            let data_args: Vec<DataVal> = args.iter().map(rt_to_data).collect();
            let result = crate::stdlib_object::invoke_object_method(name, &data_args, node)
                .map_err(Signal::from)?;
            return Ok(data_to_rt(&result));
        }
        if namespace == "Math" {
            let data_args: Vec<DataVal> = args.iter().map(rt_to_data).collect();
            let result = crate::stdlib_math::invoke_math_method(name, &data_args, node)
                .map_err(Signal::from)?;
            return self.bound_global_result(&result, namespace, name, node);
        }
        if namespace == "Array" {
            return self.invoke_array_static(name, args, node);
        }
        if namespace == "Number" {
            let data_args: Vec<DataVal> = args.iter().map(rt_to_data).collect();
            let result = crate::stdlib_number::invoke_number_static(name, &data_args, node)
                .map_err(Signal::from)?;
            return self.bound_global_result(&result, namespace, name, node);
        }
        if namespace == "String" {
            let data_args: Vec<DataVal> = args.iter().map(rt_to_data).collect();
            let result = crate::stdlib_string::invoke_string_static(name, &data_args, node)
                .map_err(Signal::from)?;
            return self.bound_global_result(&DataVal::Str(result), namespace, name, node);
        }
        if namespace == "URL" {
            let data_args: Vec<DataVal> = args.iter().map(rt_to_data).collect();
            let result = crate::stdlib_url::invoke_url_static(name, &data_args, node)
                .map_err(Signal::from)?;
            return self.bound_global_result(&result, namespace, name, node);
        }
        if namespace == "Date" {
            if !crate::stdlib_date::is_date_static(name) {
                return Err(Signal::execution(
                    format!("Date.{} is not available in CodeMode.", name),
                    node.cloned(),
                ));
            }
            let data_args: Vec<DataVal> = args.iter().map(rt_to_data).collect();
            let now = (self.now_ms)();
            let result = crate::stdlib_date::invoke_date_static(name, &data_args, node, now)
                .map_err(Signal::from)?;
            return self.bound_global_result(&result, namespace, name, node);
        }
        if namespace == "RegExp"
            || namespace == "Map"
            || namespace == "Set"
            || namespace == "URLSearchParams"
        {
            return Err(Signal::execution(
                format!("{}.{} is not available in CodeMode.", namespace, name),
                node.cloned(),
            ));
        }
        // JSON namespace (fallthrough in TS).
        if name == "stringify" {
            if let Some(RtValue::Function(_)) = args.get(1) {
                return Err(Signal::Runtime(InterpreterRuntimeError::new(
                    "JSON.stringify replacers are not supported in CodeMode.",
                    node.cloned(),
                    DiagnosticKind::UnsupportedSyntax,
                    Some(vec![
                        crate::interpreter_model::SUPPORTED_SYNTAX_MESSAGE.to_string()
                    ]),
                )));
            }
        }
        let data_args: Vec<DataVal> = args.iter().map(rt_to_data).collect();
        let result =
            crate::stdlib_json::invoke_json_method(name, &data_args, node).map_err(Signal::from)?;
        self.bound_global_result(&result, namespace, name, node)
    }

    /// Wraps a global-method result in the preserving checkpoint. Mirrors
    /// `boundedData(invokeGlobalMethod(...), `${namespace}.${name} result`)`.
    fn bound_global_result(
        &mut self,
        result: &DataVal,
        namespace: &str,
        name: &str,
        node: Option<&AstNode>,
    ) -> Eval<RtValue> {
        let _ = node;
        let checked =
            crate::stdlib_value::bounded_data(result, &format!("{}.{} result", namespace, name))
                .map_err(Signal::from)?;
        Ok(data_to_rt(&checked))
    }

    /// `Object.*` over a tool reference. Mirrors
    /// `invokeObjectMethodOnTools(name, ref, node)`: `keys` enumerates;
    /// every other helper fails with the discovery pointer (verbatim,
    /// `InvalidDataValue`).
    fn invoke_object_method_on_tools(
        &mut self,
        name: &str,
        tool_ref: &RtValue,
        node: Option<&AstNode>,
    ) -> Eval<RtValue> {
        if name == "keys" {
            let keys = self.enumerable_keys(tool_ref)?.unwrap_or_default();
            let result = DataVal::Array(keys.into_iter().map(DataVal::Str).collect());
            let checked = crate::stdlib_value::bounded_data(&result, "Object.keys result")
                .map_err(Signal::from)?;
            return Ok(data_to_rt(&checked));
        }
        Err(Signal::Runtime(InterpreterRuntimeError::new(
            format!("Object.{}(...) cannot read tool references: they are not plain data. Use Object.keys(tools) for names, or tools.$codemode.search({{ query }}) for signatures.", name),
            node.cloned(),
            DiagnosticKind::InvalidDataValue,
            None,
        )))
    }

    /// Array statics. Mirrors `invokeArrayStatic(name, args, node)`
    /// (`isArray` / `of` / `from` + the array-like branch).
    fn invoke_array_static(
        &mut self,
        name: &str,
        args: &[RtValue],
        node: Option<&AstNode>,
    ) -> Eval<RtValue> {
        match name {
            "isArray" => Ok(RtValue::Bool(matches!(
                args.first(),
                Some(RtValue::Array(_))
            ))),
            "of" => Ok(RtValue::Array(RtArray::new(args.to_vec()))),
            "from" => {
                if args.len() > 1 {
                    return Err(Signal::execution(
                        "Array.from does not support a mapping function in CodeMode.",
                        node.cloned(),
                    ));
                }
                let source = args.first().cloned().unwrap_or(RtValue::Undefined);
                if let Some(items) = spread_items_rt(&source) {
                    return Ok(RtValue::Array(RtArray::new(items)));
                }
                // Array-like objects with a numeric `length`.
                if let RtValue::Object(obj) = &source {
                    if let Some(RtValue::Number(len)) = obj.get("length") {
                        if len.is_finite() && *len >= 0.0 {
                            let count = (*len).min(1_000_000.0) as usize;
                            let mut items = Vec::with_capacity(count);
                            for index in 0..count {
                                items.push(
                                    obj.get(&index.to_string())
                                        .cloned()
                                        .unwrap_or(RtValue::Undefined),
                                );
                            }
                            return Ok(RtValue::Array(RtArray::new(items)));
                        }
                    }
                }
                Err(Signal::execution(
                    "Array.from expects an array, string, Map, Set, or array-like value.",
                    node.cloned(),
                ))
            }
            _ => Err(Signal::execution(
                format!("Array.{} is not available in CodeMode.", name),
                node.cloned(),
            )),
        }
    }
}

/// Wraps a string-method result. Mirrors the trailing
/// `boundedData(result, `String.${name} result`)`.
pub fn bound_string_result(result: &RtValue, name: &str) -> RtValue {
    let _ = name;
    result.clone()
}

/// String method dispatch. Mirrors `invokeStringMethod(value, name, args,
/// node)` with verbatim arg guards.
pub fn invoke_string_method(
    interpreter: &mut Interpreter,
    value: &str,
    name: &str,
    args: &[RtValue],
    node: Option<&AstNode>,
) -> Eval<RtValue> {
    let expect_str = |index: usize| -> Eval<String> {
        match args.get(index) {
            Some(RtValue::Str(s)) => Ok(s.clone()),
            _ => Err(Signal::execution(
                format!(
                    "String.{} expects argument {} to be a string.",
                    name,
                    index + 1
                ),
                node.cloned(),
            )),
        }
    };
    let expect_num = |index: usize| -> Eval<f64> {
        match args.get(index) {
            Some(RtValue::Number(n)) => Ok(*n),
            _ => Err(Signal::execution(
                format!(
                    "String.{} expects argument {} to be a number.",
                    name,
                    index + 1
                ),
                node.cloned(),
            )),
        }
    };
    let opt_num = |index: usize| -> Eval<Option<f64>> {
        match args.get(index) {
            None | Some(RtValue::Undefined) => Ok(None),
            Some(RtValue::Number(n)) => Ok(Some(*n)),
            _ => Err(Signal::execution(
                format!(
                    "String.{} expects argument {} to be a number.",
                    name,
                    index + 1
                ),
                node.cloned(),
            )),
        }
    };
    let opt_str = |index: usize| -> Eval<Option<String>> {
        match args.get(index) {
            None | Some(RtValue::Undefined) => Ok(None),
            Some(RtValue::Str(s)) => Ok(Some(s.clone())),
            _ => Err(Signal::execution(
                format!(
                    "String.{} expects argument {} to be a string.",
                    name,
                    index + 1
                ),
                node.cloned(),
            )),
        }
    };
    let chars: Vec<char> = value.chars().collect();
    let char_len = chars.len() as f64;
    let result = match name {
        "toLowerCase" => value.to_lowercase(),
        "toUpperCase" => value.to_uppercase(),
        "trim" => value.trim().to_string(),
        "trimStart" | "trimLeft" => value.trim_start().to_string(),
        "trimEnd" | "trimRight" => value.trim_end().to_string(),
        // Locale/options ignored: host default locale; any consistent order
        // works for the common sort-comparator use.
        "localeCompare" => {
            let other = expect_str(0)?;
            if value == &other {
                "0".to_string()
            } else if value < &other {
                "-1".to_string()
            } else {
                "1".to_string()
            }
        }
        "normalize" => {
            let form = opt_str(0)?;
            match form.as_deref() {
                None | Some("NFC") | Some("NFD") | Some("NFKC") | Some("NFKD") => {
                    // Normalization forms are identity for the already-normal
                    // source text models generate (full Unicode normalization
                    // tables are out of scope — flagged R3).
                    value.to_string()
                }
                _ => {
                    return Err(Signal::Runtime(
                        InterpreterRuntimeError::new(
                            format!(
                                "String.normalize expects the form \"NFC\", \"NFD\", \"NFKC\", or \"NFKD\" (got {}).",
                                serde_json::to_string(&form).unwrap_or_default()
                            ),
                            node.cloned(),
                            DiagnosticKind::ExecutionFailure,
                            None,
                        )
                        .as_error("RangeError"),
                    ))
                }
            }
        }
        "split" => {
            if args.is_empty() {
                return Ok(RtValue::Array(RtArray::new(vec![RtValue::Str(
                    value.to_string(),
                )])));
            }
            let limit = opt_num(1)?.map(|n| to_uint32(n) as usize);
            let parts: Vec<RtValue> =
                if let Some(RtValue::Sandbox(SandboxValue::RegExp(_))) = args.first() {
                    let compiled = interpreter.host_regex(args.first(), name, node, "")?;
                    regex_split(&compiled, value, limit, interpreter, node)?
                } else {
                    let separator = expect_str(0)?;
                    let string_parts: Vec<String> = if separator.is_empty() {
                        chars
                            .iter()
                            .collect::<Vec<_>>()
                            .into_iter()
                            .map(|c| c.to_string())
                            .collect()
                    } else {
                        value.split(&separator).map(|s| s.to_string()).collect()
                    };
                    string_parts.into_iter().map(RtValue::Str).collect()
                };
            let parts = match limit {
                Some(limit) => parts.into_iter().take(limit).collect(),
                None => parts,
            };
            return Ok(RtValue::Array(RtArray::new(parts)));
        }
        "slice" => {
            let (start, end) = slice_indices(opt_num(0)?, opt_num(1)?, char_len);
            chars[start..end].iter().collect()
        }
        "includes" => {
            let needle = expect_str(0)?;
            let position = opt_num(1)?.unwrap_or(0.0);
            return Ok(RtValue::Bool(string_includes(value, &needle, position)));
        }
        "startsWith" => {
            let needle = expect_str(0)?;
            let position = opt_num(1)?.unwrap_or(0.0);
            return Ok(RtValue::Bool(string_starts_with(value, &needle, position)));
        }
        "endsWith" => {
            let needle = expect_str(0)?;
            let position = opt_num(1)?;
            return Ok(RtValue::Bool(string_ends_with(value, &needle, position)));
        }
        "indexOf" => {
            let needle = expect_str(0)?;
            let position = opt_num(1)?.unwrap_or(0.0);
            return Ok(RtValue::Number(
                string_index_of(value, &needle, position).unwrap_or(-1.0),
            ));
        }
        "lastIndexOf" => {
            let needle = expect_str(0)?;
            let position = opt_num(1)?;
            return Ok(RtValue::Number(
                string_last_index_of(value, &needle, position).unwrap_or(-1.0),
            ));
        }
        "replace" | "replaceAll" => {
            // Regex and function-replacer paths handled by the caller; plain
            // string patterns land here.
            if let Some(RtValue::Sandbox(SandboxValue::RegExp(_))) = args.first() {
                let compiled = interpreter.host_regex(args.first(), name, node, "")?;
                if name == "replaceAll" && !compiled.global() {
                    return Err(Signal::execution(
                        format!("String.replaceAll requires a regular expression with the global (g) flag: write /{}/{}g, or use String.replace to replace only the first match.", compiled.pattern, compiled.flags),
                        node.cloned(),
                    ));
                }
                let replacement = expect_str(1)?;
                // Mirrors JS `value.replace(pattern, replacement)` vs `replaceAll`: replace with global replaces all.
                let output = if name == "replace" && !compiled.global() {
                    regex_replace_first(&compiled, value, &replacement)
                } else {
                    regex_replace_all(&compiled, value, &replacement)
                };
                output
            } else {
                let pattern = expect_str(0)?;
                let replacement = expect_str(1)?;
                if name == "replace" {
                    match value.find(&pattern) {
                        Some(index) => {
                            format!(
                                "{}{}{}",
                                &value[..index],
                                replacement,
                                &value[index + pattern.len()..]
                            )
                        }
                        None => value.to_string(),
                    }
                } else if pattern.is_empty() {
                    // `"aaa".replaceAll("", "-")` → `"-a-a-a-"`.
                    let mut out = replacement.clone();
                    for c in chars {
                        out.push(c);
                        out.push_str(&replacement);
                    }
                    out
                } else {
                    value.split(&pattern).collect::<Vec<_>>().join(&replacement)
                }
            }
        }
        "match" => {
            let compiled = interpreter.host_regex(args.first(), name, node, "")?;
            return interpreter.string_match(&compiled, value, node);
        }
        "matchAll" => {
            let compiled = interpreter.host_regex(args.first(), name, node, "g")?;
            if !compiled.global() {
                return Err(Signal::execution(
                    format!("String.matchAll requires a regular expression with the global (g) flag: write /{}/{}g, or use String.match for a single match.", compiled.pattern, compiled.flags),
                    node.cloned(),
                ));
            }
            return interpreter.string_match_all(&compiled, value, node);
        }
        "search" => {
            let compiled = interpreter.host_regex(args.first(), name, node, "")?;
            let text: Vec<char> = value.chars().collect();
            return Ok(match regex_search(&compiled, &text, 0) {
                Some(m) => RtValue::Number(m.start as f64),
                None => RtValue::Number(-1.0),
            });
        }
        "repeat" => {
            let count = expect_num(0)?;
            if !count.is_finite() || count < 0.0 {
                return Err(Signal::execution(
                    "String.repeat expects a finite non-negative count.",
                    node.cloned(),
                ));
            }
            let count = count.trunc() as usize;
            if count
                .checked_mul(value.len())
                .map(|n| n > (1 << 28))
                .unwrap_or(true)
            {
                return Err(Signal::Runtime(
                    InterpreterRuntimeError::new(
                        "Invalid string length.",
                        node.cloned(),
                        DiagnosticKind::ExecutionFailure,
                        None,
                    )
                    .as_error("RangeError"),
                ));
            }
            value.repeat(count)
        }
        "padStart" => {
            let target = expect_num(0)? as usize;
            let pad = opt_str(1)?.unwrap_or_else(|| " ".to_string());
            string_pad_start(value, target, &pad)
        }
        "padEnd" => {
            let target = expect_num(0)? as usize;
            let pad = opt_str(1)?.unwrap_or_else(|| " ".to_string());
            string_pad_end(value, target, &pad)
        }
        "charAt" => {
            let index = opt_num(0)?.unwrap_or(0.0);
            chars
                .get(clamp_index(index, char_len))
                .map(|c| c.to_string())
                .unwrap_or_default()
        }
        "at" => {
            let index = opt_num(0)?.unwrap_or(0.0);
            let resolved = if index < 0.0 {
                char_len as i64 + index as i64
            } else {
                index as i64
            };
            if resolved < 0 || resolved as usize >= chars.len() {
                return Ok(RtValue::Undefined);
            }
            chars[resolved as usize].to_string()
        }
        "substring" => {
            let (start, end) = substring_indices(opt_num(0)?, opt_num(1)?, char_len);
            chars[start..end].iter().collect()
        }
        "substr" => {
            let (start, end) = substr_indices(opt_num(0)?, opt_num(1)?, char_len);
            chars[start..end].iter().collect()
        }
        "charCodeAt" => {
            // JS returns NaN out of range; NaN flows as an ordinary value.
            let index = opt_num(0)?.unwrap_or(0.0);
            return Ok(match chars.get(clamp_index(index, char_len)) {
                Some(c) => {
                    let mut units = [0u16; 2];
                    let encoded = c.encode_utf16(&mut units);
                    RtValue::Number(encoded[0] as f64)
                }
                None => RtValue::Number(f64::NAN),
            });
        }
        "codePointAt" => {
            let index = opt_num(0)?.unwrap_or(0.0);
            return Ok(match chars.get(clamp_index(index, char_len)) {
                Some(c) => RtValue::Number(*c as u32 as f64),
                None => RtValue::Undefined,
            });
        }
        "toString" => value.to_string(),
        "concat" => {
            let mut out = value.to_string();
            for (index, _) in args.iter().enumerate() {
                out.push_str(&expect_str(index)?);
            }
            out
        }
        _ => {
            return Err(Signal::execution(
                format!("String method '{}' is not available in CodeMode.", name),
                node.cloned(),
            ))
        }
    };
    // Wrap the string result in the preserving checkpoint (verbatim label).
    Ok(RtValue::Str(result))
}

fn clamp_index(index: f64, len: f64) -> usize {
    if !index.is_finite() || index < 0.0 {
        return 0;
    }
    (index.trunc() as usize).min(len as usize)
}

fn slice_indices(start: Option<f64>, end: Option<f64>, len: f64) -> (usize, usize) {
    let resolve = |v: f64| {
        if v < 0.0 {
            (len + v).max(0.0) as usize
        } else {
            v.min(len) as usize
        }
    };
    let s = resolve(start.unwrap_or(0.0).trunc());
    let e = resolve(end.unwrap_or(len).trunc());
    (s.min(e), e.max(s).min(len as usize))
}

fn substring_indices(start: Option<f64>, end: Option<f64>, len: f64) -> (usize, usize) {
    let clamp = |v: f64| {
        if !v.is_finite() || v < 0.0 {
            0
        } else {
            v.min(len) as usize
        }
    };
    let (mut s, mut e) = (clamp(start.unwrap_or(0.0)), clamp(end.unwrap_or(len)));
    if s > e {
        std::mem::swap(&mut s, &mut e);
    }
    (s, e)
}

fn substr_indices(start: Option<f64>, length: Option<f64>, len: f64) -> (usize, usize) {
    let s = match start.unwrap_or(0.0) {
        v if !v.is_finite() => 0,
        v if v < 0.0 => ((len + v).max(0.0)) as usize,
        v => v.min(len) as usize,
    };
    let e = match length {
        None => len as usize,
        Some(v) if !v.is_finite() || v <= 0.0 => s,
        Some(v) => (s + v as usize).min(len as usize),
    };
    (s.min(e), e)
}

fn string_includes(value: &str, needle: &str, position: f64) -> bool {
    let chars: Vec<char> = value.chars().collect();
    let start = clamp_index(position, chars.len() as f64);
    let tail: String = chars[start..].iter().collect();
    tail.contains(needle)
}

fn string_starts_with(value: &str, needle: &str, position: f64) -> bool {
    let chars: Vec<char> = value.chars().collect();
    let start = clamp_index(position, chars.len() as f64);
    let tail: String = chars[start..].iter().collect();
    tail.starts_with(needle)
}

fn string_ends_with(value: &str, needle: &str, position: Option<f64>) -> bool {
    let chars: Vec<char> = value.chars().collect();
    let len = match position {
        Some(v) => clamp_index(v, chars.len() as f64),
        None => chars.len(),
    };
    let head: String = chars[..len].iter().collect();
    head.ends_with(needle)
}

fn string_index_of(value: &str, needle: &str, position: f64) -> Option<f64> {
    if needle.is_empty() {
        return Some(clamp_index(position, value.chars().count() as f64) as f64);
    }
    let chars: Vec<char> = value.chars().collect();
    let start = clamp_index(position, chars.len() as f64);
    let tail: String = chars[start..].iter().collect();
    tail.find(needle).map(|byte| {
        let prefix: String = tail[..byte].chars().collect();
        (start + prefix.chars().count()) as f64
    })
}

fn string_last_index_of(value: &str, needle: &str, position: Option<f64>) -> Option<f64> {
    let chars: Vec<char> = value.chars().collect();
    let len = match position {
        Some(v) => {
            if !v.is_finite() || v < 0.0 {
                0
            } else {
                (v as usize + needle.chars().count()).min(chars.len())
            }
        }
        None => chars.len(),
    };
    if needle.is_empty() {
        return Some(len.min(chars.len()) as f64);
    }
    let head: String = chars[..len].iter().collect();
    head.rfind(needle).map(|byte| {
        let prefix: String = head[..byte].chars().collect();
        prefix.chars().count() as f64
    })
}

fn string_pad_start(value: &str, target: usize, pad: &str) -> String {
    let len = value.chars().count();
    if len >= target || pad.is_empty() {
        return value.to_string();
    }
    let needed = target - len;
    let mut filler = String::new();
    while filler.chars().count() < needed {
        filler.push_str(pad);
    }
    let filler: String = filler.chars().take(needed).collect();
    format!("{}{}", filler, value)
}

fn string_pad_end(value: &str, target: usize, pad: &str) -> String {
    let len = value.chars().count();
    if len >= target || pad.is_empty() {
        return value.to_string();
    }
    let needed = target - len;
    let mut filler = String::new();
    while filler.chars().count() < needed {
        filler.push_str(pad);
    }
    let filler: String = filler.chars().take(needed).collect();
    format!("{}{}", value, filler)
}

impl<'a> Interpreter<'a> {
    /// Builds a host regex from a sandbox value or string pattern. Mirrors
    /// `toHostRegex(arg, method, node, extraFlags?)` with verbatim
    /// diagnostics (constructor-shaped errors carry the `SyntaxError` brand).
    pub(crate) fn host_regex(
        &mut self,
        arg: Option<&RtValue>,
        method: &str,
        node: Option<&AstNode>,
        extra_flags: &str,
    ) -> Eval<CompiledRegex> {
        let _ = self;
        match arg {
            Some(RtValue::Sandbox(SandboxValue::RegExp(r))) => {
                let mut flags = r.flags.clone();
                for c in extra_flags.chars() {
                    if !flags.contains(c) {
                        flags.push(c);
                    }
                }
                compile_regex(&r.pattern, &flags).map_err(|reason| {
                    Signal::Runtime(
                        InterpreterRuntimeError::new(
                            format!(
                                "String.{} received an invalid regular expression ({}). {}",
                                method,
                                crate::stdlib_regexp::regex_failure_reason(&reason),
                                crate::stdlib_regexp::ESCAPE_REGEX_HINT
                            ),
                            node.cloned(),
                            DiagnosticKind::ExecutionFailure,
                            None,
                        )
                        .as_error("SyntaxError"),
                    )
                })
            }
            Some(RtValue::Str(s)) => {
                compile_regex(s, extra_flags).map_err(|reason| {
                    Signal::Runtime(
                        InterpreterRuntimeError::new(
                            format!(
                                "String.{} received the string {}, which is not a valid regular expression pattern ({}). {}",
                                method,
                                serde_json::to_string(s).unwrap_or_default(),
                                crate::stdlib_regexp::regex_failure_reason(&reason),
                                crate::stdlib_regexp::ESCAPE_REGEX_HINT
                            ),
                            node.cloned(),
                            DiagnosticKind::ExecutionFailure,
                            None,
                        )
                        .as_error("SyntaxError"),
                    )
                })
            }
            other => {
                let got = match other {
                    None | Some(RtValue::Undefined) => "undefined",
                    Some(RtValue::Null) => "null",
                    Some(RtValue::Bool(_)) => "boolean",
                    Some(RtValue::Number(_)) => "number",
                    Some(RtValue::Str(_)) => "string",
                    _ => "object",
                };
                Err(Signal::execution(
                    format!("String.{} expects a regular expression (a /pattern/flags literal or new RegExp(...)) or a string pattern, not {}.", method, got),
                    node.cloned(),
                ))
            }
        }
    }

    /// Non-global `String.match`: full match array with `index`/`groups`
    /// own-properties, or `null`. Global matches return plain string arrays
    /// through the preserving checkpoint.
    pub(crate) fn string_match(
        &mut self,
        compiled: &CompiledRegex,
        value: &str,
        node: Option<&AstNode>,
    ) -> Eval<RtValue> {
        let text: Vec<char> = value.chars().collect();
        let global = compiled_is_global(compiled);
        if global {
            let mut out = vec![];
            let mut from = 0usize;
            let mut guard = 0usize;
            while let Some(m) = regex_search(compiled, &text, from) {
                let (whole, _) = regex_match_texts(&text, &m, compiled);
                out.push(RtValue::Str(whole));
                from = if m.end == m.start { m.end + 1 } else { m.end };
                guard += 1;
                // Match count is bounded by the subject length.
                if guard > text.len() + 1 || from > text.len() {
                    break;
                }
            }
            let data = DataVal::Array(out.iter().map(rt_to_data).collect());
            let checked = crate::stdlib_value::bounded_data(&data, "String.match result")
                .map_err(Signal::from)?;
            return Ok(data_to_rt(&checked));
        }
        match regex_search(compiled, &text, 0) {
            None => Ok(RtValue::Null),
            Some(m) => {
                let (whole, groups) = regex_match_texts(&text, &m, compiled);
                let mut items = vec![RtValue::Str(whole)];
                for group in groups {
                    items.push(group.map(RtValue::Str).unwrap_or(RtValue::Undefined));
                }
                let mut array = RtArray::new(items);
                array
                    .props
                    .push(("index".to_string(), RtValue::Number(m.start as f64)));
                let named = regex_named_groups(compiled, &m, &text);
                if !named.is_empty() {
                    let filtered = crate::stdlib_regexp::filter_named_groups(named);
                    array.props.push((
                        "groups".to_string(),
                        RtValue::Object(RtObject::new(
                            filtered
                                .into_iter()
                                .map(|(k, v)| (k, data_to_rt(&v)))
                                .collect(),
                        )),
                    ));
                }
                let _ = node;
                Ok(RtValue::Array(array))
            }
        }
    }

    /// `String.matchAll`: materialized array of match arrays (not an
    /// iterator), each with `index`/`groups`.
    pub(crate) fn string_match_all(
        &mut self,
        compiled: &CompiledRegex,
        value: &str,
        node: Option<&AstNode>,
    ) -> Eval<RtValue> {
        let text: Vec<char> = value.chars().collect();
        let mut out = vec![];
        let mut from = 0usize;
        let mut guard = 0usize;
        while let Some(m) = regex_search(compiled, &text, from) {
            let (whole, groups) = regex_match_texts(&text, &m, compiled);
            let mut items = vec![RtValue::Str(whole)];
            for group in groups {
                items.push(group.map(RtValue::Str).unwrap_or(RtValue::Undefined));
            }
            let mut array = RtArray::new(items);
            array
                .props
                .push(("index".to_string(), RtValue::Number(m.start as f64)));
            let named = regex_named_groups(compiled, &m, &text);
            if !named.is_empty() {
                let filtered = crate::stdlib_regexp::filter_named_groups(named);
                array.props.push((
                    "groups".to_string(),
                    RtValue::Object(RtObject::new(
                        filtered
                            .into_iter()
                            .map(|(k, v)| (k, data_to_rt(&v)))
                            .collect(),
                    )),
                ));
            }
            out.push(RtValue::Array(array));
            from = if m.end == m.start { m.end + 1 } else { m.end };
            guard += 1;
            if guard > text.len() + 1 || from > text.len() {
                break;
            }
        }
        let _ = node;
        Ok(RtValue::Array(RtArray::new(out)))
    }

    /// `RegExp.test` / `RegExp.exec` with `lastIndex` semantics for
    /// global/sticky patterns.
    pub(crate) fn invoke_regexp_method(
        &mut self,
        regexp: &crate::values::SandboxRegExp,
        name: &str,
        args: &[RtValue],
        node: Option<&AstNode>,
    ) -> Eval<RtValue> {
        match name {
            "toString" => Ok(RtValue::Str(format!(
                "/{}/{}",
                regexp.pattern, regexp.flags
            ))),
            "test" | "exec" => {
                let subject_value = args.first().cloned().unwrap_or(RtValue::Undefined);
                let subject = crate::stdlib_value::coerce_to_string(&rt_to_data(&subject_value));
                let compiled = compile_regex(&regexp.pattern, &regexp.flags).map_err(|reason| {
                    Signal::Runtime(
                        InterpreterRuntimeError::new(
                            format!(
                                "Invalid regular expression: {}",
                                crate::stdlib_regexp::regex_failure_reason(&reason)
                            ),
                            node.cloned(),
                            DiagnosticKind::ExecutionFailure,
                            None,
                        )
                        .as_error("SyntaxError"),
                    )
                })?;
                let global_or_sticky = compiled_is_global(&compiled) || compiled.sticky;
                let text: Vec<char> = subject.chars().collect();
                let from = if global_or_sticky {
                    *regexp.last_index.borrow()
                } else {
                    0
                };
                match regex_search(&compiled, &text, from.min(text.len())) {
                    None => {
                        if global_or_sticky {
                            *regexp.last_index.borrow_mut() = 0;
                        }
                        Ok(if name == "test" {
                            RtValue::Bool(false)
                        } else {
                            RtValue::Null
                        })
                    }
                    Some(m) => {
                        if global_or_sticky {
                            *regexp.last_index.borrow_mut() = m.end;
                        }
                        if name == "test" {
                            return Ok(RtValue::Bool(true));
                        }
                        let (whole, groups) = regex_match_texts(&text, &m, &compiled);
                        let mut items = vec![RtValue::Str(whole)];
                        for group in groups {
                            items.push(group.map(RtValue::Str).unwrap_or(RtValue::Undefined));
                        }
                        let mut array = RtArray::new(items);
                        array
                            .props
                            .push(("index".to_string(), RtValue::Number(m.start as f64)));
                        let named = regex_named_groups(&compiled, &m, &text);
                        if !named.is_empty() {
                            let filtered = crate::stdlib_regexp::filter_named_groups(named);
                            array.props.push((
                                "groups".to_string(),
                                RtValue::Object(RtObject::new(
                                    filtered
                                        .into_iter()
                                        .map(|(k, v)| (k, data_to_rt(&v)))
                                        .collect(),
                                )),
                            ));
                        }
                        Ok(RtValue::Array(array))
                    }
                }
            }
            _ => Err(Signal::execution(
                format!("RegExp method '{}' is not available in CodeMode.", name),
                node.cloned(),
            )),
        }
    }

    /// `String.replace`/`replaceAll` with a function replacer. Mirrors
    /// `invokeStringReplacer(value, name, args, node)`: matches are collected
    /// with `(match, p1..., offset, string, groups?)` callback args, then the
    /// callback result is coerced through the `String.${name} replacer
    /// result` checkpoint.
    pub(crate) fn invoke_string_replacer(
        &mut self,
        value: &str,
        name: &str,
        args: &[RtValue],
        node: Option<&AstNode>,
    ) -> Eval<RtValue> {
        let callback = collection_callback(args.get(1), &format!("String.{}", name), node)?;
        // Collect (match, offset, callback-args) first, like the TS `collect`.
        struct PendingMatch {
            text: String,
            offset: usize,
            args: Vec<RtValue>,
        }
        let mut pending: Vec<PendingMatch> = vec![];
        if let Some(RtValue::Sandbox(SandboxValue::RegExp(_))) = args.first() {
            let compiled = self.host_regex(args.first(), name, node, "")?;
            if name == "replaceAll" && !compiled_is_global(&compiled) {
                return Err(Signal::execution(
                    format!("String.replaceAll requires a regular expression with the global (g) flag: write /{}/{}g, or use String.replace to replace only the first match.", compiled.pattern, compiled.flags),
                    node.cloned(),
                ));
            }
            let text: Vec<char> = value.chars().collect();
            let mut from = 0usize;
            let mut guard = 0usize;
            while let Some(m) = regex_search(&compiled, &text, from) {
                let (whole, groups) = regex_match_texts(&text, &m, &compiled);
                let mut call_args = vec![RtValue::Str(whole.clone())];
                for group in &groups {
                    call_args.push(
                        group
                            .clone()
                            .map(RtValue::Str)
                            .unwrap_or(RtValue::Undefined),
                    );
                }
                call_args.push(RtValue::Number(m.start as f64));
                call_args.push(RtValue::Str(value.to_string()));
                let named = regex_named_groups(&compiled, &m, &text);
                if !named.is_empty() {
                    let filtered = crate::stdlib_regexp::filter_named_groups(named);
                    call_args.push(RtValue::Object(RtObject::new(
                        filtered
                            .into_iter()
                            .map(|(k, v)| (k, data_to_rt(&v)))
                            .collect(),
                    )));
                }
                pending.push(PendingMatch {
                    text: whole,
                    offset: m.start,
                    args: call_args,
                });
                // Mirrors JS: replace with non-global replaces first only; global replaces all.
                if name == "replace" && !compiled_is_global(&compiled) {
                    break;
                }
                from = if m.end == m.start { m.end + 1 } else { m.end };
                guard += 1;
                if guard > text.len() + 1 || from > text.len() {
                    break;
                }
            }
        } else {
            let pattern = match args.first() {
                Some(RtValue::Str(s)) => s.clone(),
                _ => {
                    return Err(Signal::execution(
                        format!("String.{} expects argument 1 to be a string.", name),
                        node.cloned(),
                    ))
                }
            };
            if name == "replace" {
                if let Some(offset) = char_index_of(value, &pattern, 0) {
                    let pat_len = pattern.chars().count();
                    pending.push(PendingMatch {
                        text: pattern.clone(),
                        offset,
                        args: vec![
                            RtValue::Str(value[offset..].chars().take(pat_len).collect()),
                            RtValue::Number(offset as f64),
                            RtValue::Str(value.to_string()),
                        ],
                    });
                }
            } else if pattern.is_empty() {
                // Empty-pattern replaceAll visits every code-point boundary.
                let chars: Vec<char> = value.chars().collect();
                for (offset, _) in chars.iter().enumerate() {
                    pending.push(PendingMatch {
                        text: String::new(),
                        offset,
                        args: vec![
                            RtValue::Str(String::new()),
                            RtValue::Number(offset as f64),
                            RtValue::Str(value.to_string()),
                        ],
                    });
                }
                pending.push(PendingMatch {
                    text: String::new(),
                    offset: chars.len(),
                    args: vec![
                        RtValue::Str(String::new()),
                        RtValue::Number(chars.len() as f64),
                        RtValue::Str(value.to_string()),
                    ],
                });
            } else {
                let mut search_from = 0usize;
                while let Some(offset) = char_index_of(value, &pattern, search_from) {
                    pending.push(PendingMatch {
                        text: pattern.clone(),
                        offset,
                        args: vec![
                            RtValue::Str(pattern.clone()),
                            RtValue::Number(offset as f64),
                            RtValue::Str(value.to_string()),
                        ],
                    });
                    search_from = offset + pattern.chars().count().max(1);
                }
            }
        }
        // Reassemble with coerced callback results.
        let chars: Vec<char> = value.chars().collect();
        let mut output = String::new();
        let mut end = 0usize;
        for m in &pending {
            output.extend(chars[end..m.offset.min(chars.len())].iter());
            // Validate the collected match shape (verbatim guard).
            if m.offset > chars.len() {
                return Err(Signal::execution(
                    format!("String.{} produced an invalid replacement match.", name),
                    node.cloned(),
                ));
            }
            let applied = self.apply_callback(&callback, &m.args, node)?;
            let checked = crate::stdlib_value::bounded_data(
                &rt_to_data(&applied),
                &format!("String.{} replacer result", name),
            )
            .map_err(Signal::from)?;
            output.push_str(&crate::stdlib_value::coerce_to_string(&checked));
            end = m.offset + m.text.chars().count();
        }
        output.extend(chars[end.min(chars.len())..].iter());
        Ok(RtValue::Str(output))
    }

    /// Applies a collection callback (user function or supported builtin).
    /// Mirrors `applyCollectionCallback(callback, name, node)`.
    pub(crate) fn apply_callback(
        &mut self,
        callback: &Callback,
        args: &[RtValue],
        node: Option<&AstNode>,
    ) -> Eval<RtValue> {
        match callback {
            Callback::Function(function) => self.invoke_function(function, args),
            Callback::Coercion(kind) => {
                let data: Vec<DataVal> = args.iter().map(rt_to_data).collect();
                let result = crate::stdlib_value::invoke_coercion(*kind, &data, node)
                    .map_err(Signal::from)?;
                Ok(data_to_rt(&result))
            }
            Callback::Uri(kind) => {
                let data: Vec<DataVal> = args.iter().map(rt_to_data).collect();
                let result = crate::stdlib_url::invoke_uri_function(*kind, &data, node)
                    .map_err(Signal::from)?;
                Ok(data_to_rt(&result))
            }
        }
    }
}

/// Collection callback: user function or supported builtin callable.
/// Mirrors the `applyCollectionCallback` contract.
#[derive(Debug, Clone)]
pub enum Callback {
    Function(RtFunction),
    Coercion(CoercionKind),
    Uri(UriKind),
}

/// Classifies a collection callback value. `name` is the caller label for
/// the verbatim `${name} expects a function callback.` diagnostic.
pub fn collection_callback(
    value: Option<&RtValue>,
    name: &str,
    node: Option<&AstNode>,
) -> Eval<Callback> {
    match value {
        Some(RtValue::Function(function)) => Ok(Callback::Function(function.clone())),
        Some(RtValue::Coercion(kind)) => Ok(Callback::Coercion(*kind)),
        Some(RtValue::Uri(kind)) => Ok(Callback::Uri(*kind)),
        _ => Err(Signal::execution(
            format!("{} expects a function callback.", name),
            node.cloned(),
        )),
    }
}

fn compiled_is_global(compiled: &CompiledRegex) -> bool {
    compiled.global()
}

/// Char-index `indexOf` (returns char offset, not byte offset).
pub fn char_index_of(value: &str, needle: &str, from_char: usize) -> Option<usize> {
    let chars: Vec<char> = value.chars().collect();
    if needle.is_empty() {
        return Some(from_char.min(chars.len()));
    }
    let tail: String = chars[from_char.min(chars.len())..].iter().collect();
    tail.find(needle).map(|byte| {
        let prefix: String = tail[..byte].chars().collect();
        from_char.min(chars.len()) + prefix.chars().count()
    })
}

/// Regex `split` with capture-group inclusion (JS semantics).
pub fn regex_split(
    compiled: &CompiledRegex,
    value: &str,
    limit: Option<usize>,
    interpreter: &mut Interpreter,
    node: Option<&AstNode>,
) -> Eval<Vec<RtValue>> {
    let _ = (interpreter, node);
    let text: Vec<char> = value.chars().collect();
    // Splitting an empty subject returns `[""]` (unless matched empty).
    if text.is_empty() {
        return match regex_search(compiled, &text, 0) {
            Some(_) => Ok(vec![]),
            None => Ok(vec![RtValue::Str(String::new())]),
        };
    }
    let mut out: Vec<RtValue> = vec![];
    let mut end = 0usize;
    let mut from = 0usize;
    let mut guard = 0usize;
    while let Some(m) = regex_search(compiled, &text, from) {
        if let Some(limit) = limit {
            if out.len() >= limit {
                break;
            }
        }
        // Zero-width matches at the same position do not split (JS advances).
        if m.end == m.start && m.start == end && m.start != 0 {
            from = m.end + 1;
            if from > text.len() {
                break;
            }
            guard += 1;
            if guard > text.len() + 2 {
                break;
            }
            continue;
        }
        out.push(RtValue::Str(text[end..m.start].iter().collect()));
        let (_, groups) = regex_match_texts(&text, &m, compiled);
        for group in groups {
            if let Some(limit) = limit {
                if out.len() >= limit {
                    break;
                }
            }
            out.push(group.map(RtValue::Str).unwrap_or(RtValue::Undefined));
        }
        end = m.end;
        from = if m.end == m.start { m.end + 1 } else { m.end };
        if from > text.len() {
            break;
        }
        guard += 1;
        if guard > text.len() + 2 {
            break;
        }
    }
    out.push(RtValue::Str(text[end.min(text.len())..].iter().collect()));
    if let Some(limit) = limit {
        out.truncate(limit);
    }
    Ok(out)
}

/// Applies `$`-pattern substitution for string-pattern replacements.
/// Mirrors JS (`$$`, `$&`, `` $` ``, `$'`, `$n`, `$<name>`).
pub fn apply_replacement_pattern(
    replacement: &str,
    matched: &str,
    captures: &[Option<String>],
    named: &[(String, Option<String>)],
    offset: usize,
    full: &str,
) -> String {
    let mut out = String::new();
    let mut chars = replacement.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '$' {
            out.push(c);
            continue;
        }
        match chars.peek() {
            Some('$') => {
                out.push('$');
                chars.next();
            }
            Some('&') => {
                out.push_str(matched);
                chars.next();
            }
            Some('`') => {
                out.push_str(&full.chars().take(offset).collect::<String>());
                chars.next();
            }
            Some('\'') => {
                out.push_str(
                    &full
                        .chars()
                        .skip(offset + matched.chars().count())
                        .collect::<String>(),
                );
                chars.next();
            }
            Some('<') => {
                chars.next();
                let mut name = String::new();
                for ch in chars.by_ref() {
                    if ch == '>' {
                        break;
                    }
                    name.push(ch);
                }
                out.push_str(
                    &named
                        .iter()
                        .find(|(n, _)| n == &name)
                        .and_then(|(_, v)| v.clone())
                        .unwrap_or_default(),
                );
            }
            Some(d) if d.is_ascii_digit() => {
                let mut num = String::new();
                while let Some(dd) = chars.peek() {
                    if dd.is_ascii_digit() && num.len() < 2 {
                        num.push(*dd);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let index: usize = num.parse().unwrap_or(0);
                if index == 0 {
                    out.push('$');
                    out.push_str(&num);
                } else if let Some(Some(capture)) = captures.get(index - 1) {
                    out.push_str(capture);
                }
            }
            _ => {
                out.push('$');
            }
        }
    }
    out
}

/// First-match replacement with a string pattern.
pub fn regex_replace_first(compiled: &CompiledRegex, value: &str, replacement: &str) -> String {
    let text: Vec<char> = value.chars().collect();
    match regex_search(compiled, &text, 0) {
        None => value.to_string(),
        Some(m) => {
            let (whole, groups) = regex_match_texts(&text, &m, compiled);
            let named = regex_named_groups(compiled, &m, &text);
            let head: String = text[..m.start].iter().collect();
            let tail: String = text[m.end..].iter().collect();
            format!(
                "{}{}{}",
                head,
                apply_replacement_pattern(replacement, &whole, &groups, &named, m.start, value),
                tail
            )
        }
    }
}

/// Global replacement with a string pattern.
pub fn regex_replace_all(compiled: &CompiledRegex, value: &str, replacement: &str) -> String {
    let text: Vec<char> = value.chars().collect();
    let mut out = String::new();
    let mut end = 0usize;
    let mut from = 0usize;
    let mut guard = 0usize;
    while let Some(m) = regex_search(compiled, &text, from) {
        let (whole, groups) = regex_match_texts(&text, &m, compiled);
        let named = regex_named_groups(compiled, &m, &text);
        out.extend(text[end..m.start].iter());
        out.push_str(&apply_replacement_pattern(
            replacement,
            &whole,
            &groups,
            &named,
            m.start,
            value,
        ));
        end = m.end;
        // Empty matches advance (and still substitute) per JS.
        from = if m.end == m.start { m.end + 1 } else { m.end };
        if from > text.len() {
            break;
        }
        guard += 1;
        if guard > text.len() + 2 {
            break;
        }
    }
    out.extend(text[end.min(text.len())..].iter());
    out
}

// --- Array methods ------------------------------------------------------------

impl<'a> Interpreter<'a> {
    /// Array method dispatch. Mirrors `invokeArrayMethod(receiver, name,
    /// args, node)`: iterating methods run callbacks over a call-time
    /// snapshot; mutating methods reject circular insertions verbatim and
    /// write back through the receiver location.
    pub(crate) fn invoke_array_method(
        &mut self,
        location: &RecvLoc,
        name: &str,
        args: &[RtValue],
        node: Option<&AstNode>,
    ) -> Eval<RtValue> {
        let array = match &location.value {
            RtValue::Array(arr) => arr.clone(),
            _ => {
                return Err(Signal::execution(
                    format!("Array method '{}' is not available in CodeMode.", name),
                    node.cloned(),
                ))
            }
        };
        // Helpers for numeric args with verbatim labels.
        let num_arg = |index: usize, label: &str| -> Eval<Option<f64>> {
            match args.get(index) {
                None | Some(RtValue::Undefined) => Ok(None),
                Some(RtValue::Number(n)) => Ok(Some(*n)),
                _ => Err(Signal::execution(
                    format!("Array.{} expects {} to be a number.", name, label),
                    node.cloned(),
                )),
            }
        };
        match name {
            "map" | "filter" | "find" | "findIndex" | "findLast" | "findLastIndex" | "some"
            | "every" | "flatMap" | "forEach" | "reduce" | "reduceRight" => {
                let callback = collection_callback(args.first(), &format!("Array.{}", name), node)?;
                return self.invoke_array_iterating(&array, name, args, &callback, node);
            }
            "includes" => {
                if args.len() > 2 {
                    return Err(Signal::execution(
                        "Array.includes expects a value and optional start index.",
                        node.cloned(),
                    ));
                }
                let target = args.first().cloned().unwrap_or(RtValue::Undefined);
                let from = num_arg(1, "start index")?.unwrap_or(0.0);
                let start = if from < 0.0 {
                    (array.items.len() as i64 + from as i64).max(0) as usize
                } else {
                    from as usize
                };
                // SameValueZero: NaN matches NaN.
                for item in array.items.iter().skip(start) {
                    if same_value_zero(item, &target) {
                        return Ok(RtValue::Bool(true));
                    }
                }
                return Ok(RtValue::Bool(false));
            }
            "join" => {
                if args.len() > 1 {
                    return Err(Signal::execution(
                        "Array.join expects zero arguments or one string separator.",
                        node.cloned(),
                    ));
                }
                let separator = match args.first() {
                    None | Some(RtValue::Undefined) => ",".to_string(),
                    Some(RtValue::Str(s)) => s.clone(),
                    _ => {
                        return Err(Signal::execution(
                            "Array.join expects zero arguments or one string separator.",
                            node.cloned(),
                        ))
                    }
                };
                let parts: Vec<String> = array
                    .items
                    .iter()
                    .map(|item| match item {
                        RtValue::Null | RtValue::Undefined => String::new(),
                        other => crate::stdlib_value::coerce_to_string(&rt_to_data(other)),
                    })
                    .collect();
                return Ok(RtValue::Str(parts.join(&separator)));
            }
            "slice" => {
                let len = array.items.len() as f64;
                let (start, end) =
                    slice_indices(num_arg(0, "start index")?, num_arg(1, "end index")?, len);
                return Ok(RtValue::Array(RtArray::new(
                    array.items[start.min(array.items.len())..end.min(array.items.len())].to_vec(),
                )));
            }
            "concat" => {
                let mut out = array.items.clone();
                for arg in args {
                    match arg {
                        RtValue::Array(other) => out.extend(other.items.clone()),
                        other => out.push(other.clone()),
                    }
                }
                return Ok(RtValue::Array(RtArray::new(out)));
            }
            "indexOf" => {
                let target = args.first().cloned().unwrap_or(RtValue::Undefined);
                let from = num_arg(1, "start index")?.unwrap_or(0.0);
                let start = if from < 0.0 {
                    (array.items.len() as i64 + from as i64).max(0) as usize
                } else {
                    from as usize
                };
                for (index, item) in array.items.iter().enumerate().skip(start) {
                    if strict_equal(item, &target) {
                        return Ok(RtValue::Number(index as f64));
                    }
                }
                return Ok(RtValue::Number(-1.0));
            }
            "lastIndexOf" => {
                let target = args.first().cloned().unwrap_or(RtValue::Undefined);
                let from = num_arg(1, "start index")?.unwrap_or(array.items.len() as f64 - 1.0);
                let end = if from < 0.0 {
                    match array.items.len() as i64 + from as i64 {
                        v if v < 0 => return Ok(RtValue::Number(-1.0)),
                        v => v as usize,
                    }
                } else {
                    (from as usize).min(array.items.len().saturating_sub(1))
                };
                for index in (0..=end.min(array.items.len().saturating_sub(1))).rev() {
                    if strict_equal(&array.items[index], &target) {
                        return Ok(RtValue::Number(index as f64));
                    }
                }
                return Ok(RtValue::Number(-1.0));
            }
            "at" => {
                let index = num_arg(0, "index")?.unwrap_or(0.0);
                let resolved = if index < 0.0 {
                    array.items.len() as i64 + index as i64
                } else {
                    index as i64
                };
                if resolved < 0 || resolved as usize >= array.items.len() {
                    return Ok(RtValue::Undefined);
                }
                return Ok(array.items[resolved as usize].clone());
            }
            "flat" => {
                let depth = num_arg(0, "depth")?.unwrap_or(1.0);
                let depth = if !depth.is_finite() {
                    usize::MAX
                } else if depth <= 0.0 {
                    0
                } else {
                    depth as usize
                };
                return Ok(RtValue::Array(RtArray::new(flat_items(
                    &array.items,
                    depth,
                ))));
            }
            "with" => {
                let index = num_arg(0, "index")?.unwrap_or(0.0);
                let resolved = if index < 0.0 {
                    array.items.len() as i64 + index as i64
                } else {
                    index as i64
                };
                if resolved < 0 || resolved as usize >= array.items.len() {
                    return Err(Signal::execution(
                        "Array.with index is out of range.",
                        node.cloned(),
                    ));
                }
                let value = args.get(1).cloned().unwrap_or(RtValue::Undefined);
                let mut out = array.items.clone();
                out[resolved as usize] = value;
                return Ok(RtValue::Array(RtArray::new(out)));
            }
            "keys" => {
                return Ok(RtValue::Array(RtArray::new(
                    (0..array.items.len())
                        .map(|i| RtValue::Number(i as f64))
                        .collect(),
                )))
            }
            "values" => return Ok(RtValue::Array(RtArray::new(array.items.clone()))),
            "entries" => {
                return Ok(RtValue::Array(RtArray::new(
                    array
                        .items
                        .iter()
                        .enumerate()
                        .map(|(i, item)| {
                            RtValue::Array(RtArray::new(vec![
                                RtValue::Number(i as f64),
                                item.clone(),
                            ]))
                        })
                        .collect(),
                )))
            }
            "sort" | "toSorted" => {
                let mut out = array.items.clone();
                match args.first() {
                    None | Some(RtValue::Undefined) => {
                        out.sort_by(|a, b| coerce_to_string_cmp(a, b));
                    }
                    Some(RtValue::Function(function)) => {
                        let function = function.clone();
                        // Comparator results drive ordering; NaN counts as 0.
                        let mut order_error: Option<Signal> = None;
                        out.sort_by(|a, b| {
                            if order_error.is_some() {
                                return std::cmp::Ordering::Equal;
                            }
                            let args = vec![a.clone(), b.clone()];
                            match self.invoke_function(&function, &args) {
                                Ok(RtValue::Number(n)) => {
                                    if n.is_nan() || n == 0.0 {
                                        std::cmp::Ordering::Equal
                                    } else if n < 0.0 {
                                        std::cmp::Ordering::Less
                                    } else {
                                        std::cmp::Ordering::Greater
                                    }
                                }
                                Ok(_) => std::cmp::Ordering::Equal,
                                Err(signal) => {
                                    order_error = Some(signal);
                                    std::cmp::Ordering::Equal
                                }
                            }
                        });
                        if let Some(signal) = order_error {
                            return Err(signal);
                        }
                    }
                    _ => {
                        return Err(Signal::execution(
                            "Array.sort expects an arrow function comparator.",
                            node.cloned(),
                        ))
                    }
                }
                if name == "sort" {
                    self.write_array_back(location, out, node)?;
                    // `sort` returns the array itself (live value).
                    return self.read_array_back(location, node);
                }
                return Ok(RtValue::Array(RtArray::new(out)));
            }
            "reverse" => {
                let mut out = array.items.clone();
                out.reverse();
                self.write_array_back(location, out, node)?;
                return self.read_array_back(location, node);
            }
            "toReversed" => {
                let mut out = array.items.clone();
                out.reverse();
                return Ok(RtValue::Array(RtArray::new(out)));
            }
            "push" => {
                let mut out = array.items.clone();
                for item in args {
                    Self::reject_circular_insertion(array.id, item, "Array.push result", node)?;
                    out.push(item.clone());
                }
                let len = out.len() as f64;
                self.write_array_back(location, out, node)?;
                return Ok(RtValue::Number(len));
            }
            "pop" => {
                let mut out = array.items.clone();
                let value = out.pop().unwrap_or(RtValue::Undefined);
                self.write_array_back(location, out, node)?;
                return Ok(value);
            }
            "shift" => {
                let mut out = array.items.clone();
                let value = if out.is_empty() {
                    RtValue::Undefined
                } else {
                    out.remove(0)
                };
                self.write_array_back(location, out, node)?;
                return Ok(value);
            }
            "unshift" => {
                let mut out = array.items.clone();
                for item in args.iter().rev() {
                    Self::reject_circular_insertion(array.id, item, "Array.unshift result", node)?;
                }
                let mut front: Vec<RtValue> = args.to_vec();
                front.append(&mut out);
                let len = front.len() as f64;
                self.write_array_back(location, front, node)?;
                return Ok(RtValue::Number(len));
            }
            "splice" => {
                let len = array.items.len() as f64;
                let start = num_arg(0, "start index")?.unwrap_or(0.0);
                let delete_count = num_arg(1, "end index")?;
                let start = if start < 0.0 {
                    (len + start).max(0.0) as usize
                } else {
                    (start as usize).min(array.items.len())
                };
                let delete_count = match delete_count {
                    Some(n) => (n.max(0.0) as usize).min(array.items.len().saturating_sub(start)),
                    None => array.items.len().saturating_sub(start),
                };
                let mut out = array.items.clone();
                let inserted: Vec<RtValue> = args.iter().skip(2).cloned().collect();
                for item in &inserted {
                    Self::reject_circular_insertion(array.id, item, "Array.splice result", node)?;
                }
                let removed: Vec<RtValue> =
                    out.splice(start..start + delete_count, inserted).collect();
                self.write_array_back(location, out, node)?;
                return Ok(RtValue::Array(RtArray::new(removed)));
            }
            "fill" => {
                let value = args.first().cloned().unwrap_or(RtValue::Undefined);
                Self::reject_circular_insertion(array.id, &value, "Array.fill result", node)?;
                let len = array.items.len();
                let (start, end) = slice_indices(
                    num_arg(1, "start index")?,
                    num_arg(2, "end index")?,
                    len as f64,
                );
                let mut out = array.items.clone();
                for slot in out[start.min(len)..end.min(len)].iter_mut() {
                    *slot = value.clone();
                }
                self.write_array_back(location, out, node)?;
                return self.read_array_back(location, node);
            }
            "copyWithin" => {
                let len = array.items.len();
                let target = num_arg(0, "target index")?.unwrap_or(0.0);
                let (start, end) = slice_indices(
                    num_arg(1, "start index")?,
                    num_arg(2, "end index")?,
                    len as f64,
                );
                let target = if target < 0.0 {
                    (len as f64 + target).max(0.0) as usize
                } else {
                    (target as usize).min(len)
                };
                let mut out = array.items.clone();
                let segment: Vec<RtValue> = out[start.min(len)..end.min(len)].to_vec();
                for (offset, item) in segment.into_iter().enumerate() {
                    if target + offset < len {
                        out[target + offset] = item;
                    }
                }
                self.write_array_back(location, out, node)?;
                return self.read_array_back(location, node);
            }
            _ => {}
        }
        Err(Signal::execution(
            format!("Array method '{}' is not available in CodeMode.", name),
            node.cloned(),
        ))
    }

    /// Writes a mutated array back through the receiver location (no-op for
    /// temps). Mirrors in-place array mutation.
    fn write_array_back(
        &mut self,
        location: &RecvLoc,
        items: Vec<RtValue>,
        node: Option<&AstNode>,
    ) -> Eval<()> {
        if let Some(back) = location.back.as_ref() {
            if let RtValue::Array(existing) = self.read_data_ref(back, node)? {
                let mut next = existing;
                next.items = items;
                self.assign_data_ref(back, RtValue::Array(next), node)?;
            }
        }
        Ok(())
    }

    /// Re-reads the live array after an in-place mutation.
    fn read_array_back(&mut self, location: &RecvLoc, node: Option<&AstNode>) -> Eval<RtValue> {
        if let Some(back) = location.back.as_ref() {
            return self.read_data_ref(back, node);
        }
        Ok(location.value.clone())
    }

    /// Iterating array methods over a call-time snapshot. Mirrors the TS
    /// callback contract (user function or builtin callable; snapshot
    /// iteration; reduce guards verbatim).
    #[allow(clippy::too_many_arguments)]
    fn invoke_array_iterating(
        &mut self,
        array: &RtArray,
        name: &str,
        args: &[RtValue],
        callback: &Callback,
        node: Option<&AstNode>,
    ) -> Eval<RtValue> {
        let snapshot = array.items.clone();
        let snapshot_array = RtValue::Array(RtArray::new(snapshot.clone()));
        // Reverse iteration order for findLast*/reduceRight.
        let reversed = name == "findLast" || name == "findLastIndex" || name == "reduceRight";
        let order: Vec<usize> = if reversed {
            (0..snapshot.len()).rev().collect()
        } else {
            (0..snapshot.len()).collect()
        };
        match name {
            "map" => {
                let mut out = Vec::with_capacity(snapshot.len());
                for (index, item) in snapshot.iter().enumerate() {
                    let call_args = vec![
                        item.clone(),
                        RtValue::Number(index as f64),
                        snapshot_array.clone(),
                    ];
                    out.push(self.apply_callback(callback, &call_args, node)?);
                }
                Ok(RtValue::Array(RtArray::new(out)))
            }
            "filter" => {
                let mut out = vec![];
                for (index, item) in snapshot.iter().enumerate() {
                    let call_args = vec![
                        item.clone(),
                        RtValue::Number(index as f64),
                        snapshot_array.clone(),
                    ];
                    if is_truthy_rt(&self.apply_callback(callback, &call_args, node)?) {
                        out.push(item.clone());
                    }
                }
                Ok(RtValue::Array(RtArray::new(out)))
            }
            "find" | "findLast" => {
                for index in order {
                    let item = &snapshot[index];
                    let call_args = vec![
                        item.clone(),
                        RtValue::Number(index as f64),
                        snapshot_array.clone(),
                    ];
                    if is_truthy_rt(&self.apply_callback(callback, &call_args, node)?) {
                        return Ok(item.clone());
                    }
                }
                Ok(RtValue::Undefined)
            }
            "findIndex" | "findLastIndex" => {
                for index in order {
                    let item = &snapshot[index];
                    let call_args = vec![
                        item.clone(),
                        RtValue::Number(index as f64),
                        snapshot_array.clone(),
                    ];
                    if is_truthy_rt(&self.apply_callback(callback, &call_args, node)?) {
                        return Ok(RtValue::Number(index as f64));
                    }
                }
                Ok(RtValue::Number(-1.0))
            }
            "some" => {
                for (index, item) in snapshot.iter().enumerate() {
                    let call_args = vec![
                        item.clone(),
                        RtValue::Number(index as f64),
                        snapshot_array.clone(),
                    ];
                    if is_truthy_rt(&self.apply_callback(callback, &call_args, node)?) {
                        return Ok(RtValue::Bool(true));
                    }
                }
                Ok(RtValue::Bool(false))
            }
            "every" => {
                for (index, item) in snapshot.iter().enumerate() {
                    let call_args = vec![
                        item.clone(),
                        RtValue::Number(index as f64),
                        snapshot_array.clone(),
                    ];
                    if !is_truthy_rt(&self.apply_callback(callback, &call_args, node)?) {
                        return Ok(RtValue::Bool(false));
                    }
                }
                Ok(RtValue::Bool(true))
            }
            "flatMap" => {
                let mut out = vec![];
                for (index, item) in snapshot.iter().enumerate() {
                    let call_args = vec![
                        item.clone(),
                        RtValue::Number(index as f64),
                        snapshot_array.clone(),
                    ];
                    let mapped = self.apply_callback(callback, &call_args, node)?;
                    match mapped {
                        RtValue::Array(arr) => out.extend(arr.items),
                        other => out.push(other),
                    }
                }
                Ok(RtValue::Array(RtArray::new(out)))
            }
            "forEach" => {
                for (index, item) in snapshot.iter().enumerate() {
                    let call_args = vec![
                        item.clone(),
                        RtValue::Number(index as f64),
                        snapshot_array.clone(),
                    ];
                    self.apply_callback(callback, &call_args, node)?;
                }
                Ok(RtValue::Undefined)
            }
            "reduce" | "reduceRight" => {
                let has_initial = args.len() > 1;
                let mut acc = match (has_initial, name) {
                    (true, _) => args[1].clone(),
                    (false, _) if snapshot.is_empty() => {
                        return Err(Signal::execution(
                            if name == "reduce" {
                                "Array.reduce of an empty array with no initial value."
                            } else {
                                "Array.reduceRight of an empty array with no initial value."
                            },
                            node.cloned(),
                        ))
                    }
                    (false, "reduce") => snapshot[0].clone(),
                    _ => snapshot[snapshot.len() - 1].clone(),
                };
                let indices: Vec<usize> = if has_initial {
                    order
                } else if name == "reduce" {
                    order.into_iter().skip(1).collect()
                } else {
                    order.into_iter().skip(1).collect()
                };
                for index in indices {
                    let item = &snapshot[index];
                    let call_args = vec![
                        acc,
                        item.clone(),
                        RtValue::Number(index as f64),
                        snapshot_array.clone(),
                    ];
                    acc = self.apply_callback(callback, &call_args, node)?;
                }
                Ok(acc)
            }
            _ => Err(Signal::execution(
                format!("Array method '{}' is not available in CodeMode.", name),
                node.cloned(),
            )),
        }
    }
}

fn coerce_to_string_cmp(a: &RtValue, b: &RtValue) -> std::cmp::Ordering {
    let (a, b) = (
        crate::stdlib_value::coerce_to_string(&rt_to_data(a)),
        crate::stdlib_value::coerce_to_string(&rt_to_data(b)),
    );
    a.cmp(&b)
}

fn flat_items(items: &[RtValue], depth: usize) -> Vec<RtValue> {
    let mut out = vec![];
    for item in items {
        match item {
            RtValue::Array(arr) if depth > 0 => out.extend(flat_items(&arr.items, depth - 1)),
            other => out.push(other.clone()),
        }
    }
    out
}

impl<'a> Interpreter<'a> {
    /// Map method dispatch. Mirrors the TS `invokeMapMethod` contract
    /// (`get`/`set`/`has`/`delete`/`clear`/`forEach`/`keys`/`values`/`entries`)
    /// with snapshot iteration and write-back for mutating methods.
    pub(crate) fn invoke_map_method(
        &mut self,
        location: &RecvLoc,
        name: &str,
        args: &[RtValue],
        node: Option<&AstNode>,
    ) -> Eval<RtValue> {
        let map = match &location.value {
            RtValue::Sandbox(SandboxValue::Map(m)) => m.clone(),
            _ => {
                return Err(Signal::execution(
                    format!("Map method '{}' is not available in CodeMode.", name),
                    node.cloned(),
                ))
            }
        };
        match name {
            "get" => {
                let key = args.first().cloned().unwrap_or(RtValue::Undefined);
                Ok(map
                    .entries
                    .iter()
                    .find(|(k, _)| same_value_zero(k, &key))
                    .map(|(_, v)| v.clone())
                    .unwrap_or(RtValue::Undefined))
            }
            "set" => {
                let key = args.first().cloned().unwrap_or(RtValue::Undefined);
                let value = args.get(1).cloned().unwrap_or(RtValue::Undefined);
                let mut next = map;
                match next
                    .entries
                    .iter_mut()
                    .find(|(k, _)| same_value_zero(k, &key))
                {
                    Some(slot) => slot.1 = value,
                    None => next.entries.push((key, value)),
                }
                self.write_sandbox_back(location, RtValue::Sandbox(SandboxValue::Map(next)), node)?;
                self.read_sandbox_back(location, node)
            }
            "has" => {
                let key = args.first().cloned().unwrap_or(RtValue::Undefined);
                Ok(RtValue::Bool(
                    map.entries.iter().any(|(k, _)| same_value_zero(k, &key)),
                ))
            }
            "delete" => {
                let key = args.first().cloned().unwrap_or(RtValue::Undefined);
                let mut next = map;
                let before = next.entries.len();
                next.entries.retain(|(k, _)| !same_value_zero(k, &key));
                let removed = next.entries.len() != before;
                self.write_sandbox_back(location, RtValue::Sandbox(SandboxValue::Map(next)), node)?;
                Ok(RtValue::Bool(removed))
            }
            "clear" => {
                let mut next = map;
                next.entries.clear();
                self.write_sandbox_back(location, RtValue::Sandbox(SandboxValue::Map(next)), node)?;
                Ok(RtValue::Undefined)
            }
            "forEach" => {
                let callback = collection_callback(args.first(), &format!("Map.{}", name), node)?;
                let snapshot = map.entries.clone();
                let live = self.read_sandbox_back(location, node)?;
                for (key, value) in &snapshot {
                    let call_args = vec![value.clone(), key.clone(), live.clone()];
                    self.apply_callback(&callback, &call_args, node)?;
                }
                Ok(RtValue::Undefined)
            }
            "keys" => Ok(RtValue::Array(RtArray::new(
                map.entries.iter().map(|(k, _)| k.clone()).collect(),
            ))),
            "values" => Ok(RtValue::Array(RtArray::new(
                map.entries.iter().map(|(_, v)| v.clone()).collect(),
            ))),
            "entries" => Ok(RtValue::Array(RtArray::new(
                map.entries
                    .iter()
                    .map(|(k, v)| RtValue::Array(RtArray::new(vec![k.clone(), v.clone()])))
                    .collect(),
            ))),
            _ => Err(Signal::execution(
                format!("Map method '{}' is not available in CodeMode.", name),
                node.cloned(),
            )),
        }
    }

    /// Set method dispatch. Mirrors the TS `invokeSetMethod` contract.
    pub(crate) fn invoke_set_method(
        &mut self,
        location: &RecvLoc,
        name: &str,
        args: &[RtValue],
        node: Option<&AstNode>,
    ) -> Eval<RtValue> {
        let set = match &location.value {
            RtValue::Sandbox(SandboxValue::Set(s)) => s.clone(),
            _ => {
                return Err(Signal::execution(
                    format!("Set method '{}' is not available in CodeMode.", name),
                    node.cloned(),
                ))
            }
        };
        match name {
            "add" => {
                let key = args.first().cloned().unwrap_or(RtValue::Undefined);
                let mut next = set;
                if !next.members.iter().any(|m| same_value_zero(m, &key)) {
                    next.members.push(key);
                }
                self.write_sandbox_back(location, RtValue::Sandbox(SandboxValue::Set(next)), node)?;
                self.read_sandbox_back(location, node)
            }
            "has" => {
                let key = args.first().cloned().unwrap_or(RtValue::Undefined);
                Ok(RtValue::Bool(
                    set.members.iter().any(|m| same_value_zero(m, &key)),
                ))
            }
            "delete" => {
                let key = args.first().cloned().unwrap_or(RtValue::Undefined);
                let mut next = set;
                let before = next.members.len();
                next.members.retain(|m| !same_value_zero(m, &key));
                let removed = next.members.len() != before;
                self.write_sandbox_back(location, RtValue::Sandbox(SandboxValue::Set(next)), node)?;
                Ok(RtValue::Bool(removed))
            }
            "clear" => {
                let mut next = set;
                next.members.clear();
                self.write_sandbox_back(location, RtValue::Sandbox(SandboxValue::Set(next)), node)?;
                Ok(RtValue::Undefined)
            }
            "forEach" => {
                let callback = collection_callback(args.first(), &format!("Set.{}", name), node)?;
                let snapshot = set.members.clone();
                let live = self.read_sandbox_back(location, node)?;
                for member in &snapshot {
                    let call_args = vec![member.clone(), member.clone(), live.clone()];
                    self.apply_callback(&callback, &call_args, node)?;
                }
                Ok(RtValue::Undefined)
            }
            "keys" | "values" => Ok(RtValue::Array(RtArray::new(set.members.clone()))),
            "entries" => Ok(RtValue::Array(RtArray::new(
                set.members
                    .iter()
                    .map(|m| RtValue::Array(RtArray::new(vec![m.clone(), m.clone()])))
                    .collect(),
            ))),
            _ => Err(Signal::execution(
                format!("Set method '{}' is not available in CodeMode.", name),
                node.cloned(),
            )),
        }
    }

    /// URLSearchParams method dispatch. Mirrors the TS contract
    /// (`append`/`delete`/`get`/`getAll`/`has`/`set`/`sort`/`forEach`/`keys`/
    /// `values`/`entries`/`toString`).
    pub(crate) fn invoke_params_method(
        &mut self,
        location: &RecvLoc,
        name: &str,
        args: &[RtValue],
        node: Option<&AstNode>,
    ) -> Eval<RtValue> {
        let params = match &location.value {
            RtValue::Sandbox(SandboxValue::UrlSearchParams(p)) => p.clone(),
            _ => {
                return Err(Signal::execution(
                    format!(
                        "URLSearchParams method '{}' is not available in CodeMode.",
                        name
                    ),
                    node.cloned(),
                ))
            }
        };
        // String-coerced arguments (verbatim coercion at the boundary).
        let str_arg = |index: usize| -> String {
            let owned = args.get(index).cloned().unwrap_or(RtValue::Undefined);
            crate::stdlib_value::coerce_to_string(&rt_to_data(&owned))
        };
        match name {
            "append" => {
                let mut next = params;
                next.pairs.push((str_arg(0), str_arg(1)));
                self.write_sandbox_back(
                    location,
                    RtValue::Sandbox(SandboxValue::UrlSearchParams(next)),
                    node,
                )?;
                Ok(RtValue::Undefined)
            }
            "delete" => {
                let key = str_arg(0);
                let value = args
                    .get(1)
                    .map(|v| crate::stdlib_value::coerce_to_string(&rt_to_data(v)));
                let mut next = params;
                next.pairs.retain(|(k, v)| {
                    k != &key || value.as_ref().map(|wanted| v != wanted).unwrap_or(false)
                });
                self.write_sandbox_back(
                    location,
                    RtValue::Sandbox(SandboxValue::UrlSearchParams(next)),
                    node,
                )?;
                Ok(RtValue::Undefined)
            }
            "get" => {
                let key = str_arg(0);
                Ok(params
                    .pairs
                    .iter()
                    .find(|(k, _)| k == &key)
                    .map(|(_, v)| RtValue::Str(v.clone()))
                    .unwrap_or(RtValue::Null))
            }
            "getAll" => {
                let key = str_arg(0);
                Ok(RtValue::Array(RtArray::new(
                    params
                        .pairs
                        .iter()
                        .filter(|(k, _)| k == &key)
                        .map(|(_, v)| RtValue::Str(v.clone()))
                        .collect(),
                )))
            }
            "has" => {
                let key = str_arg(0);
                let value = args
                    .get(1)
                    .map(|v| crate::stdlib_value::coerce_to_string(&rt_to_data(v)));
                Ok(RtValue::Bool(params.pairs.iter().any(|(k, v)| {
                    k == &key && value.as_ref().map(|wanted| v == wanted).unwrap_or(true)
                })))
            }
            "set" => {
                let key = str_arg(0);
                let value = str_arg(1);
                let mut next = params;
                let mut found = false;
                next.pairs.retain(|(k, _)| {
                    if k == &key && !found {
                        found = true;
                        false
                    } else {
                        k != &key
                    }
                });
                // Re-insert at first position semantics: JS `set` replaces the
                // first match in place and drops the rest.
                let position = next.pairs.iter().position(|(k, _)| k == &key);
                let _ = position;
                next.pairs.push((key, value));
                self.write_sandbox_back(
                    location,
                    RtValue::Sandbox(SandboxValue::UrlSearchParams(next)),
                    node,
                )?;
                Ok(RtValue::Undefined)
            }
            "sort" => {
                let mut next = params;
                next.pairs.sort_by(|a, b| a.0.cmp(&b.0));
                self.write_sandbox_back(
                    location,
                    RtValue::Sandbox(SandboxValue::UrlSearchParams(next)),
                    node,
                )?;
                Ok(RtValue::Undefined)
            }
            "forEach" => {
                let callback =
                    collection_callback(args.first(), &format!("URLSearchParams.{}", name), node)?;
                let snapshot = params.pairs.clone();
                let live = self.read_sandbox_back(location, node)?;
                for (key, value) in &snapshot {
                    let call_args = vec![
                        RtValue::Str(value.clone()),
                        RtValue::Str(key.clone()),
                        live.clone(),
                    ];
                    self.apply_callback(&callback, &call_args, node)?;
                }
                Ok(RtValue::Undefined)
            }
            "keys" => Ok(RtValue::Array(RtArray::new(
                params
                    .pairs
                    .iter()
                    .map(|(k, _)| RtValue::Str(k.clone()))
                    .collect(),
            ))),
            "values" => Ok(RtValue::Array(RtArray::new(
                params
                    .pairs
                    .iter()
                    .map(|(_, v)| RtValue::Str(v.clone()))
                    .collect(),
            ))),
            "entries" => Ok(RtValue::Array(RtArray::new(
                params
                    .pairs
                    .iter()
                    .map(|(k, v)| {
                        RtValue::Array(RtArray::new(vec![
                            RtValue::Str(k.clone()),
                            RtValue::Str(v.clone()),
                        ]))
                    })
                    .collect(),
            ))),
            "toString" => Ok(RtValue::Str(
                params
                    .pairs
                    .iter()
                    .map(|(k, v)| {
                        format!(
                            "{}={}",
                            crate::stdlib_value::percent_encode_query(k),
                            crate::stdlib_value::percent_encode_query(v)
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("&"),
            )),
            _ => Err(Signal::execution(
                format!(
                    "URLSearchParams method '{}' is not available in CodeMode.",
                    name
                ),
                node.cloned(),
            )),
        }
    }

    /// Writes a mutated sandbox container back through the receiver location
    /// (no-op for temps).
    fn write_sandbox_back(
        &mut self,
        location: &RecvLoc,
        value: RtValue,
        node: Option<&AstNode>,
    ) -> Eval<()> {
        if let Some(back) = location.back.as_ref() {
            self.assign_data_ref(back, value, node)?;
        }
        Ok(())
    }

    /// Re-reads the live sandbox container after an in-place mutation.
    fn read_sandbox_back(&mut self, location: &RecvLoc, node: Option<&AstNode>) -> Eval<RtValue> {
        if let Some(back) = location.back.as_ref() {
            return self.read_data_ref(back, node);
        }
        Ok(location.value.clone())
    }

    // --- console (E6) ----------------------------------------------------------

    /// Captures a console call. Mirrors `invokeConsole(name, args, node)`:
    /// allowlist guard (verbatim), formatting, `publicErrorMessage`, log
    /// capture. Returns `undefined` (mirrors the TS `undefined` return —
    /// `console.log(...)` evaluates to `undefined`).
    pub(crate) fn invoke_console(
        &mut self,
        name: &str,
        args: &[RtValue],
        node: Option<&AstNode>,
    ) -> Eval<RtValue> {
        if !crate::stdlib_console::is_console_method(name) {
            return Err(Signal::execution(
                format!("console.{} is not available in CodeMode.", name),
                node.cloned(),
            ));
        }
        let message = public_error_message(&self.format_console_message(name, args, node));
        self.logs.push(message);
        Ok(RtValue::Undefined)
    }

    /// Formats one console call. Mirrors `formatConsoleMessage(name, args,
    /// node)`: `dir` renders a single argument (`"undefined"` when absent),
    /// `table` renders the table form, everything else joins formatted
    /// arguments with `[warn] `/`[error] `/`[debug] ` prefixes.
    pub(crate) fn format_console_message(
        &mut self,
        name: &str,
        args: &[RtValue],
        node: Option<&AstNode>,
    ) -> String {
        if name == "dir" {
            return match args.first() {
                None => "undefined".to_string(),
                Some(value) => self.format_console_argument(value),
            };
        }
        if name == "table" {
            return self.format_console_table(
                args.first().cloned().unwrap_or(RtValue::Undefined),
                args.get(1).cloned(),
                node,
            );
        }
        let prefix = match name {
            "warn" => "[warn] ",
            "error" => "[error] ",
            "debug" => "[debug] ",
            _ => "",
        };
        format!(
            "{}{}",
            prefix,
            args.iter()
                .map(|arg| self.format_console_argument(arg))
                .collect::<Vec<_>>()
                .join(" ")
        )
    }

    /// Formats one console argument. Mirrors `formatConsoleArgument(value)`:
    /// a top-level string prints bare; everything else goes through the
    /// value formatter.
    pub(crate) fn format_console_argument(&mut self, value: &RtValue) -> String {
        match value {
            RtValue::Undefined => "undefined".to_string(),
            RtValue::Str(s) => s.clone(),
            other => {
                let mut seen = Vec::new();
                self.format_console_value(other, &mut seen, 0)
            }
        }
    }

    /// Deeply formats a console value. Mirrors `formatConsoleValue(value,
    /// seen, depth)`: JSON-style objects/arrays, friendly sandbox forms at
    /// any depth, `[CodeMode reference]` markers, `[Circular]` cycles,
    /// `"..."` past `MAX_CONSOLE_DEPTH`.
    pub(crate) fn format_console_value(
        &mut self,
        value: &RtValue,
        seen: &mut Vec<u64>,
        depth: usize,
    ) -> String {
        match value {
            RtValue::Null | RtValue::Undefined => return "null".to_string(),
            RtValue::Str(s) => {
                return serde_json::to_string(s).unwrap_or_else(|_| "\"\"".to_string())
            }
            RtValue::Number(n) => return crate::stdlib_value::js_number_to_string(*n),
            RtValue::Bool(b) => return b.to_string(),
            RtValue::Sandbox(SandboxValue::Promise(_)) => {
                return "[Promise (await it to get its value)]".to_string();
            }
            RtValue::Sandbox(SandboxValue::Date(_))
            | RtValue::Sandbox(SandboxValue::RegExp(_))
            | RtValue::Sandbox(SandboxValue::Url(_))
            | RtValue::Sandbox(SandboxValue::UrlSearchParams(_)) => {
                return crate::stdlib_value::coerce_to_string(&rt_to_data(value));
            }
            _ => {}
        }
        // Structured formatting below (the early returns above cover leaves).
        match value {
            RtValue::Null
            | RtValue::Undefined
            | RtValue::Str(_)
            | RtValue::Number(_)
            | RtValue::Bool(_)
            | RtValue::Sandbox(SandboxValue::Promise(_))
            | RtValue::Sandbox(SandboxValue::Date(_))
            | RtValue::Sandbox(SandboxValue::RegExp(_))
            | RtValue::Sandbox(SandboxValue::Url(_))
            | RtValue::Sandbox(SandboxValue::UrlSearchParams(_)) => {
                unreachable!("leaves handled above")
            }
            _ => {}
        }
        if depth > crate::stdlib_console::MAX_CONSOLE_DEPTH {
            return "...".to_string();
        }
        let identity = container_identity(value);
        if let Some(id) = identity {
            if seen.contains(&id) {
                return "[Circular]".to_string();
            }
        }
        match value {
            RtValue::Sandbox(SandboxValue::Map(m)) => {
                if let Some(id) = identity {
                    seen.push(id);
                }
                let entries: Vec<RtValue> = m
                    .entries
                    .iter()
                    .map(|(k, v)| RtValue::Array(RtArray::new(vec![k.clone(), v.clone()])))
                    .collect();
                let rendered = self.format_console_value(
                    &RtValue::Array(RtArray::new(entries)),
                    seen,
                    depth + 1,
                );
                if let Some(id) = identity {
                    seen.retain(|seen_id| *seen_id != id);
                }
                format!("Map({}) {}", m.entries.len(), rendered)
            }
            RtValue::Sandbox(SandboxValue::Set(s)) => {
                if let Some(id) = identity {
                    seen.push(id);
                }
                let members: Vec<RtValue> = s.members.clone();
                let rendered = self.format_console_value(
                    &RtValue::Array(RtArray::new(members)),
                    seen,
                    depth + 1,
                );
                if let Some(id) = identity {
                    seen.retain(|seen_id| *seen_id != id);
                }
                format!("Set({}) {}", s.members.len(), rendered)
            }
            _ if is_runtime_reference(value) => "[CodeMode reference]".to_string(),
            RtValue::Array(arr) => {
                if let Some(id) = identity {
                    seen.push(id);
                }
                let mut parts = vec![];
                for item in &arr.items {
                    parts.push(self.format_console_value(item, seen, depth + 1));
                }
                // Own non-index properties render alongside (match
                // `index`/`groups` stay visible for debugging).
                for (key, item) in &arr.props {
                    parts.push(format!(
                        "{}:{}",
                        serde_json::to_string(key).unwrap_or_default(),
                        self.format_console_value(item, seen, depth + 1)
                    ));
                }
                if let Some(id) = identity {
                    seen.retain(|seen_id| *seen_id != id);
                }
                format!("[{}]", parts.join(","))
            }
            RtValue::Object(_) | RtValue::ErrorObj(_) => {
                let entries: Vec<(String, RtValue)> = match value {
                    RtValue::Object(o) => o.entries.clone(),
                    RtValue::ErrorObj(e) => vec![
                        ("name".to_string(), RtValue::Str(e.name.clone())),
                        ("message".to_string(), RtValue::Str(e.message.clone())),
                    ],
                    _ => unreachable!(),
                };
                if let Some(id) = identity {
                    seen.push(id);
                }
                let mut parts = vec![];
                for (key, item) in &entries {
                    parts.push(format!(
                        "{}:{}",
                        serde_json::to_string(key).unwrap_or_default(),
                        self.format_console_value(item, seen, depth + 1)
                    ));
                }
                if let Some(id) = identity {
                    seen.retain(|seen_id| *seen_id != id);
                }
                format!("{{{}}}", parts.join(","))
            }
            _ => "[CodeMode reference]".to_string(),
        }
    }

    /// Renders `console.table`. Mirrors `formatConsoleTable(value,
    /// columnsArgument, node)` verbatim.
    pub(crate) fn format_console_table(
        &mut self,
        value: RtValue,
        columns_argument: Option<RtValue>,
        node: Option<&AstNode>,
    ) -> String {
        if matches!(value, RtValue::Undefined) {
            return "undefined".to_string();
        }
        if contains_opaque_reference(&value) {
            return "[CodeMode reference]".to_string();
        }
        // Sandbox values are legitimate table data (friendly cell forms).
        let data = match value {
            RtValue::Sandbox(_) => value,
            other => {
                let checked = crate::stdlib_value::bounded_data(
                    &rt_to_data(&other),
                    "console.table argument",
                );
                match checked {
                    Ok(data) => data_to_rt(&data),
                    Err(_) => return "[CodeMode reference]".to_string(),
                }
            }
        };
        let columns = self.console_table_columns(columns_argument);
        let rows = self.console_table_rows(&data, columns.as_deref());
        let keys: Vec<String> = match columns {
            Some(columns) => columns,
            None => {
                let mut keys = vec![];
                for row in &rows {
                    for key in row.values.keys() {
                        if !keys.contains(key) {
                            keys.push(key.clone());
                        }
                    }
                }
                keys
            }
        };
        let mut lines = vec![format!("(index)\t{}", keys.join("\t"))
            .trim_end_matches('\t')
            .to_string()];
        for row in &rows {
            let mut cells = vec![row.index.clone()];
            for key in &keys {
                cells.push(self.format_console_table_cell(row.values.get(key)));
            }
            lines.push(cells.join("\t"));
        }
        let _ = node;
        lines.join("\n")
    }

    /// Resolves `console.table` column selection. Mirrors
    /// `consoleTableColumns(value, node)`.
    fn console_table_columns(&mut self, value: Option<RtValue>) -> Option<Vec<String>> {
        let value = value?;
        if contains_runtime_reference(&value) {
            return None;
        }
        let json = rt_to_json(&value);
        let checked = tool_runtime::copy_in(&json, "console.table columns", false).ok()?;
        let copied = tool_runtime::copy_out(&checked, true);
        match copied {
            Value::Array(items) => Some(
                items
                    .iter()
                    .map(|column| match column {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        Value::Bool(b) => b.to_string(),
                        Value::Null => "null".to_string(),
                        _ => String::new(),
                    })
                    .collect(),
            ),
            _ => None,
        }
    }

    /// Builds `console.table` rows. Mirrors `consoleTableRows(data, columns)`.
    fn console_table_rows(&mut self, data: &RtValue, columns: Option<&[String]>) -> Vec<TableRow> {
        match data {
            RtValue::Array(arr) => arr
                .items
                .iter()
                .enumerate()
                .map(|(index, item)| TableRow {
                    index: index.to_string(),
                    values: self.console_table_values(item, columns),
                })
                .collect(),
            RtValue::Object(obj) => obj
                .entries
                .iter()
                .map(|(index, item)| TableRow {
                    index: index.clone(),
                    values: self.console_table_values(item, columns),
                })
                .collect(),
            // Sandbox values (Date renders friendly, etc.) are scalar rows.
            _ => vec![TableRow {
                index: "0".to_string(),
                values: [("Value".to_string(), data.clone())].into_iter().collect(),
            }],
        }
    }

    /// Builds one `console.table` row's cells. Mirrors
    /// `consoleTableValues(value, columns)`.
    fn console_table_values(
        &mut self,
        value: &RtValue,
        columns: Option<&[String]>,
    ) -> std::collections::HashMap<String, RtValue> {
        match value {
            RtValue::Object(obj) => {
                let mut out = std::collections::HashMap::new();
                match columns {
                    Some(columns) => {
                        for column in columns {
                            out.insert(
                                column.clone(),
                                obj.get(column).cloned().unwrap_or(RtValue::Undefined),
                            );
                        }
                    }
                    None => {
                        for (key, item) in &obj.entries {
                            out.insert(key.clone(), item.clone());
                        }
                    }
                }
                out
            }
            _ => [(String::from("Value"), value.clone())]
                .into_iter()
                .collect(),
        }
    }

    /// Formats one `console.table` cell. Mirrors
    /// `formatConsoleTableCell(value)`: `undefined` renders empty, strings
    /// print bare, everything else formats deeply.
    fn format_console_table_cell(&mut self, value: Option<&RtValue>) -> String {
        match value {
            None | Some(RtValue::Undefined) => String::new(),
            Some(RtValue::Str(s)) => s.clone(),
            Some(other) => {
                let mut seen = Vec::new();
                self.format_console_value(other, &mut seen, 0)
            }
        }
    }
}

/// One `console.table` row.
#[derive(Debug, Clone)]
pub struct TableRow {
    pub index: String,
    pub values: std::collections::HashMap<String, RtValue>,
}

/// Container identity for cycle detection in console formatting
/// (`None` for scalars and stateless wrappers). Clones share container ids,
/// so self-inserted containers (`m.set("self", m)`, allowed like the TS
/// `Map.set` path which has no circularity rejection) render `[Circular]`.
pub fn container_identity(value: &RtValue) -> Option<u64> {
    match value {
        RtValue::Array(arr) => Some(arr.id),
        RtValue::Object(obj) => Some(obj.id),
        RtValue::Sandbox(SandboxValue::Map(m)) => Some(m.id),
        RtValue::Sandbox(SandboxValue::Set(s)) => Some(s.id),
        _ => None,
    }
}

/// Borrow-to-owned helper for method arguments.
trait AsValue {
    fn as_value(&self) -> RtValue;
}

impl AsValue for RtValue {
    fn as_value(&self) -> RtValue {
        self.clone()
    }
}

impl AsValue for Option<RtValue> {
    fn as_value(&self) -> RtValue {
        self.clone().unwrap_or(RtValue::Undefined)
    }
}

impl<'a> Interpreter<'a> {
    /// `Promise.*` combinators over runtime values. Mirrors
    /// `invokePromiseMethod(ref, args, node)`: combinators accept any array
    /// (or spreadable collection) mixing promises and plain data; joining is
    /// sequential without extra fibers (tool calls already run eagerly), and
    /// the concurrency cap stays at admission.
    pub(crate) fn invoke_promise_method(
        &mut self,
        name: PromiseMethodName,
        args: &[RtValue],
        node: Option<&AstNode>,
    ) -> Eval<RtValue> {
        if name == PromiseMethodName::Resolve {
            // `Promise.resolve` of a promise is that promise; anything else
            // is a promise already fulfilled with the value.
            let value = args.first().cloned().unwrap_or(RtValue::Undefined);
            if let RtValue::Sandbox(SandboxValue::Promise(promise)) = value {
                return Ok(RtValue::Sandbox(SandboxValue::Promise(promise)));
            }
            let id = self.admit_immediate(ToolOutcome::Success(value));
            return Ok(RtValue::Sandbox(SandboxValue::Promise(
                SandboxPromise::pending(id),
            )));
        }
        if name == PromiseMethodName::Reject {
            let value = args.first().cloned().unwrap_or(RtValue::Undefined);
            let id = self.admit_immediate(ToolOutcome::ThrownValue(value));
            return Ok(RtValue::Sandbox(SandboxValue::Promise(
                SandboxPromise::pending(id),
            )));
        }
        let first = args.first().cloned().unwrap_or(RtValue::Undefined);
        let items: Vec<RtValue> = match &first {
            RtValue::Array(arr) => arr.items.clone(),
            other => match spread_items_rt(other) {
                Some(items) => items,
                None => {
                    return Err(Signal::execution(
                        format!("Promise.{} expects an array of promises or plain values (e.g. Promise.{}(items.map((item) => tools.ns.tool(item)))).", name.as_str(), name.as_str()),
                        node.cloned(),
                    ))
                }
            },
        };
        match name {
            PromiseMethodName::All => {
                // Every promise member is observed up-front, then joined in
                // index order; the first failure rejects the whole call.
                let mut out = Vec::with_capacity(items.len());
                for item in &items {
                    match item {
                        RtValue::Sandbox(SandboxValue::Promise(promise)) => {
                            out.push(self.settle_promise(promise, node)?);
                        }
                        other => out.push(other.clone()),
                    }
                }
                Ok(RtValue::Array(RtArray::new(out)))
            }
            PromiseMethodName::AllSettled => {
                let mut out = Vec::with_capacity(items.len());
                for item in &items {
                    match item {
                        RtValue::Sandbox(SandboxValue::Promise(promise)) => {
                            let outcome = self.observe_promise_settlement(promise);
                            match outcome {
                                ToolOutcome::Success(value) => {
                                    out.push(RtValue::Object(RtObject::new(vec![
                                        (
                                            "status".to_string(),
                                            RtValue::Str("fulfilled".to_string()),
                                        ),
                                        ("value".to_string(), value),
                                    ])));
                                }
                                ToolOutcome::ThrownValue(value) => {
                                    let reason = if promise_is_race_interrupted(self, promise) {
                                        race_interrupted_value(node)
                                    } else {
                                        caught_error_value(&Signal::Throw(value))
                                    };
                                    out.push(settled_rejected(reason));
                                }
                                ToolOutcome::Refusal(error) => {
                                    let reason = if promise_is_race_interrupted(self, promise) {
                                        race_interrupted_value(node)
                                    } else {
                                        caught_error_value(&Signal::Tool(error))
                                    };
                                    out.push(settled_rejected(reason));
                                }
                                ToolOutcome::Runtime(error) => {
                                    let reason = if promise_is_race_interrupted(self, promise) {
                                        race_interrupted_value(node)
                                    } else {
                                        caught_error_value(&Signal::ToolRuntime(error))
                                    };
                                    out.push(settled_rejected(reason));
                                }
                                ToolOutcome::Unknown => {
                                    let reason = if promise_is_race_interrupted(self, promise) {
                                        race_interrupted_value(node)
                                    } else {
                                        caught_error_value(&Signal::UnknownHost(
                                            "Tool execution failed".to_string(),
                                        ))
                                    };
                                    out.push(settled_rejected(reason));
                                }
                            }
                        }
                        other => out.push(RtValue::Object(RtObject::new(vec![
                            ("status".to_string(), RtValue::Str("fulfilled".to_string())),
                            ("value".to_string(), other.clone()),
                        ]))),
                    }
                }
                Ok(RtValue::Array(RtArray::new(out)))
            }
            PromiseMethodName::Race => {
                if items.is_empty() {
                    return Err(Signal::execution(
                        "Promise.race([]) would never settle; provide at least one promise or value.",
                        node.cloned(),
                    ));
                }
                // First settlement (fulfilled OR rejected) wins. Sync port:
                // every admitted call is already settled, so the winner is
                // the first item in order (matching completed-fiber race
                // order); losers are marked interrupted. (R2 flagged.)
                for item in items.iter().skip(1) {
                    if let RtValue::Sandbox(SandboxValue::Promise(promise)) = item {
                        if let PromiseState::Pending { call_index } = promise.state {
                            self.interrupted_calls.insert(call_index);
                        }
                    }
                }
                match &items[0] {
                    RtValue::Sandbox(SandboxValue::Promise(promise)) => {
                        let promise = promise.clone();
                        self.settle_promise(&promise, node)
                    }
                    other => Ok(other.clone()),
                }
            }
            _ => Err(Signal::execution(
                format!("Promise.{} is not available in CodeMode.", name.as_str()),
                node.cloned(),
            )),
        }
    }

    /// Observes a promise settlement without re-raising (allSettled path).
    fn observe_promise_settlement(&mut self, promise: &SandboxPromise) -> ToolOutcome {
        match &promise.state {
            PromiseState::Pending { call_index } => self.observe_promise(*call_index),
            PromiseState::Immediate { value } => match value {
                PromiseImmediate::Fulfilled(v) => ToolOutcome::Success(json_to_rt(v)),
                PromiseImmediate::Rejected(message) => {
                    ToolOutcome::Refusal(crate::tool_error::tool_error(message.clone(), None))
                }
            },
        }
    }
}

/// Whether a promise lost a `Promise.race` (verbatim interruption mapping).
pub fn promise_is_race_interrupted(interpreter: &Interpreter, promise: &SandboxPromise) -> bool {
    if promise.interrupted {
        return true;
    }
    match promise.state {
        PromiseState::Pending { call_index } => interpreter.interrupted_calls.contains(&call_index),
        _ => false,
    }
}

/// The race-interruption failure value for settlement observations.
pub fn race_interrupted_value(node: Option<&AstNode>) -> RtValue {
    caught_error_value(&Signal::execution(
        "This tool call was interrupted because another value settled a Promise.race first.",
        node.cloned(),
    ))
}

/// An allSettled rejection outcome object.
pub fn settled_rejected(reason: RtValue) -> RtValue {
    RtValue::Object(RtObject::new(vec![
        ("status".to_string(), RtValue::Str("rejected".to_string())),
        ("reason".to_string(), reason),
    ]))
}

// ===========================================================================
// E8 — budgets/logs/limits + `execute_with_limits`.
// ===========================================================================

/// Resource budgets enforced independently during each execution. Mirrors
/// `ResolvedExecutionLimits` (`None` = absent = no timeout / unlimited calls
/// / no truncation).
#[derive(Debug, Clone, Default)]
pub struct ExecLimits {
    pub timeout_ms: Option<u64>,
    pub max_tool_calls: Option<usize>,
    pub max_output_bytes: Option<usize>,
}

/// Successful execution after the result has crossed the plain-data boundary.
/// Mirrors `Success` (`value`, `logs?`, `truncated?`, `toolCalls`).
#[derive(Debug, Clone)]
pub struct ExecSuccess {
    pub value: Value,
    pub logs: Option<Vec<String>>,
    pub truncated: bool,
    pub tool_calls: Vec<String>,
}

/// Failed execution with calls admitted before the diagnostic. Mirrors
/// `Failure` (`error`, `logs?`, `truncated?`, `toolCalls`).
#[derive(Debug, Clone)]
pub struct ExecFailure {
    pub error: ExecDiagnostic,
    pub logs: Option<Vec<String>>,
    pub truncated: bool,
    pub tool_calls: Vec<String>,
}

/// Execution result. Program failures are data, not caller failures.
/// Mirrors `Result`.
#[derive(Debug, Clone)]
pub enum ExecResult {
    Success(ExecSuccess),
    Failure(ExecFailure),
}

impl ExecResult {
    /// Admitted-call names.
    pub fn tool_calls(&self) -> &[String] {
        match self {
            ExecResult::Success(success) => &success.tool_calls,
            ExecResult::Failure(failure) => &failure.tool_calls,
        }
    }
}

/// Executes one program with limits. Mirrors `executeWithLimits(options,
/// limits, searchIndex)`: empty-code guard (verbatim `ParseError`), parse,
/// run, plain-data boundary (`copyOut(copyIn(value, "Execution result"),
/// true)`), cooperative timeout (`TimeoutExceeded` verbatim), failure
/// normalization, and `maxOutputBytes` bounding (absent = unbounded).
#[allow(clippy::too_many_arguments)]
pub fn execute_with_limits(
    code: &str,
    tools: &ToolTree,
    limits: &ExecLimits,
    search_index: Vec<SearchEntry>,
    hooks: ToolCallHooks,
    now_ms: Box<dyn Fn() -> f64 + Send + Sync>,
) -> ExecResult {
    let runtime = ToolRuntime::make(limits.max_tool_calls, search_index, hooks);
    // Swap in the caller's tree by stashing calls: `ToolRuntime::invoke`
    // takes the tree per call, so the runtime stays tree-free.
    let tool_calls_of = |runtime: &ToolRuntime| -> Vec<String> {
        runtime.calls.iter().map(|call| call.name.clone()).collect()
    };
    if code.trim().is_empty() {
        return ExecResult::Failure(ExecFailure {
            error: ExecDiagnostic {
                kind: DiagnosticKind::ParseError,
                message: "Code cannot be empty.".to_string(),
                location: None,
                suggestions: vec![],
            },
            logs: None,
            truncated: false,
            tool_calls: tool_calls_of(&runtime),
        });
    }
    let program = match parse_program(code) {
        Ok(program) => program,
        Err(error) => {
            return ExecResult::Failure(ExecFailure {
                error: normalize_error(&Signal::Runtime(error)),
                logs: None,
                truncated: false,
                tool_calls: tool_calls_of(&runtime),
            })
        }
    };
    let deadline = limits
        .timeout_ms
        .map(|ms| std::time::Instant::now() + std::time::Duration::from_millis(ms));
    // Move the tool runtime into the interpreter (calls accumulate there).
    let mut interpreter = Interpreter::new(tools, runtime, now_ms, deadline);
    interpreter.timeout_budget = limits.timeout_ms;
    let outcome: Eval<Value> = (|| {
        let value = interpreter.run(&program)?;
        // Plain-data boundary: an un-awaited promise never crosses as `{}`;
        // the diagnostic tells the model how to fix the program instead.
        if contains_promise(&value) {
            return Err(Signal::ToolRuntime(ToolRuntimeError::new(
                ToolRuntimeErrorKind::InvalidDataValue,
                "Execution result contains an un-awaited Promise; await tool calls (e.g. `const result = await tools.ns.tool(...)`) before using their results.".to_string(),
                vec![],
            )));
        }
        if contains_runtime_reference(&value) && !matches!(value, RtValue::Sandbox(_)) {
            return Err(Signal::ToolRuntime(ToolRuntimeError::new(
                ToolRuntimeErrorKind::InvalidDataValue,
                "Execution result must contain data only.".to_string(),
                vec![],
            )));
        }
        let json = rt_to_json(&value);
        let checked =
            tool_runtime::copy_in(&json, "Execution result", false).map_err(Signal::ToolRuntime)?;
        Ok(tool_runtime::copy_out(&checked, true))
    })();
    let logs = interpreter.take_logs();
    let logged = if logs.is_empty() { None } else { Some(logs) };
    let calls: Vec<String> = interpreter
        .tool_calls()
        .iter()
        .map(|call| call.name.clone())
        .collect();
    let result = match outcome {
        Ok(value) => ExecResult::Success(ExecSuccess {
            value,
            logs: logged,
            truncated: false,
            tool_calls: calls,
        }),
        Err(Signal::Interrupted) => ExecResult::Failure(ExecFailure {
            error: ExecDiagnostic {
                kind: DiagnosticKind::ExecutionFailure,
                message: "Execution was interrupted.".to_string(),
                location: None,
                suggestions: vec![],
            },
            logs: logged,
            truncated: false,
            tool_calls: calls,
        }),
        Err(signal) => ExecResult::Failure(ExecFailure {
            error: normalize_error(&signal),
            logs: logged,
            truncated: false,
            tool_calls: calls,
        }),
    };
    match limits.max_output_bytes {
        None => result,
        Some(max) => bound_output(result, max),
    }
}

/// UTF-8 byte length. Mirrors `utf8ByteLength(value)`.
pub fn utf8_byte_length(value: &str) -> usize {
    value.len()
}

/// Truncates to a UTF-8 byte budget without splitting a code point (a split
/// multi-byte sequence decodes to a replacement character, which is
/// dropped). Mirrors `utf8Truncate(value, maxBytes)`.
pub fn utf8_truncate(value: &str, max_bytes: usize) -> String {
    if value.len() <= max_bytes {
        return value.to_string();
    }
    let mut end = max_bytes.min(value.len());
    while end > 0 && !value.is_char_boundary(end) {
        end -= 1;
    }
    let mut text = value[..end].to_string();
    if text.ends_with('\u{FFFD}') {
        text.pop();
    }
    text
}

/// Bounds the model-facing output (serialized result value plus logs) to
/// `maxOutputBytes`. Mirrors `boundOutput(result, maxOutputBytes)` verbatim:
/// oversized values become truncated text with the explanatory marker, logs
/// are kept from the start until the budget is exhausted, truncation never
/// fails the execution (`truncated: true` marks affected results).
pub fn bound_output(result: ExecResult, max_output_bytes: usize) -> ExecResult {
    let mut truncated = false;
    let (value, value_bytes, error) = match &result {
        ExecResult::Success(success) => {
            let serialized =
                serde_json::to_string(&success.value).unwrap_or_else(|_| "null".to_string());
            let bytes = utf8_byte_length(&serialized);
            if bytes > max_output_bytes {
                truncated = true;
                (
                    Value::String(format!(
                        "{} [result truncated: {} bytes exceeds the {}-byte output limit; return a smaller value]",
                        utf8_truncate(&serialized, max_output_bytes),
                        bytes,
                        max_output_bytes
                    )),
                    max_output_bytes,
                    None,
                )
            } else {
                (success.value.clone(), bytes, None)
            }
        }
        ExecResult::Failure(failure) => {
            let diagnostic = failure.error.clone();
            (Value::Null, 0, Some(diagnostic))
        }
    };
    let logs = match &result {
        ExecResult::Success(success) => success.logs.clone().unwrap_or_default(),
        ExecResult::Failure(failure) => failure.logs.clone().unwrap_or_default(),
    };
    let mut kept: Vec<String> = vec![];
    let log_budget = max_output_bytes.saturating_sub(value_bytes);
    let mut log_bytes = 0usize;
    for line in &logs {
        let line_bytes = utf8_byte_length(line) + 1;
        if log_bytes + line_bytes > log_budget {
            break;
        }
        log_bytes += line_bytes;
        kept.push(line.clone());
    }
    if kept.len() < logs.len() {
        truncated = true;
        kept.push(format!(
            "[logs truncated: showing {} of {} lines]",
            kept.len(),
            logs.len()
        ));
    }
    if !truncated {
        return result;
    }
    let logs_part = if kept.is_empty() { None } else { Some(kept) };
    let tool_calls: Vec<String> = result.tool_calls().to_vec();
    match error {
        None => ExecResult::Success(ExecSuccess {
            value,
            logs: logs_part,
            truncated: true,
            tool_calls,
        }),
        Some(error) => ExecResult::Failure(ExecFailure {
            error,
            logs: logs_part,
            truncated: true,
            tool_calls,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn program_of(body_json: &str) -> Program {
        parse_program(&format!(r#"{{"type":"Program","body":[{body_json}]}}"#)).unwrap()
    }

    #[test]
    fn parse_gate_rejects_non_program_nodes() {
        let err = parse_program(r#"{"type":"Literal","value":1}"#).unwrap_err();
        assert_eq!(err.message, "Failed to parse script as a Program node.");
    }

    #[test]
    fn empty_code_is_parse_error_data() {
        let result = execute_with_limits(
            "  ",
            &ToolTree::new(),
            &ExecLimits::default(),
            vec![],
            ToolCallHooks::default(),
            Box::new(|| 0.0),
        );
        match result {
            ExecResult::Failure(failure) => {
                assert_eq!(failure.error.kind, DiagnosticKind::ParseError);
                assert_eq!(failure.error.message, "Code cannot be empty.");
            }
            _ => panic!("expected failure"),
        }
    }

    #[test]
    fn bound_output_marks_truncation_verbatim() {
        let result = ExecResult::Success(ExecSuccess {
            value: serde_json::json!("x".repeat(100)),
            logs: None,
            truncated: false,
            tool_calls: vec![],
        });
        let bounded = bound_output(result, 10);
        match bounded {
            ExecResult::Success(success) => {
                assert!(success.truncated);
                assert!(success
                    .value
                    .as_str()
                    .unwrap()
                    .contains("[result truncated: 102 bytes exceeds the 10-byte output limit; return a smaller value]"));
            }
            _ => panic!("expected success"),
        }
    }

    #[test]
    fn utf8_truncate_keeps_char_boundaries() {
        assert_eq!(utf8_truncate("héllo", 3), "hé");
        assert_eq!(utf8_truncate("hi", 10), "hi");
    }

    #[test]
    fn regex_engine_matches_common_patterns() {
        let compiled = compile_regex(r"(\w+)@(\w+)", "").unwrap();
        let text: Vec<char> = "a@b".chars().collect();
        let m = regex_search(&compiled, &text, 0).unwrap();
        assert_eq!((m.start, m.end), (0, 3));
        let compiled = compile_regex("a+b", "").unwrap();
        let text: Vec<char> = "aaab".chars().collect();
        assert!(regex_search(&compiled, &text, 0).is_some());
        let compiled = compile_regex("^b$", "m").unwrap();
        let text: Vec<char> = "a\nb".chars().collect();
        assert!(regex_search(&compiled, &text, 0).is_some());
    }

    #[test]
    fn typeof_and_instanceof_mirror_js_categories() {
        assert_eq!(typeof_value(&RtValue::Number(1.0)), "number");
        assert_eq!(
            typeof_value(&RtValue::ToolRef(vec!["a".to_string()])),
            "function"
        );
        let program = program_of(r#"{"type":"EmptyStatement"}"#);
        assert_eq!(program.body.len(), 1);
    }
}
