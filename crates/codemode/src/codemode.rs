//! Port of `src/codemode.ts`.
//!
//! Public envelope: `Input`/`DiagnosticKind`/`Diagnostic`/`Success`/
//! `Failure`/`Result`, `ExecuteOptions`/`Options`, `execute`/`make`, plus
//! limit validation. Execution itself lives in `interpreter_runtime`
//! (`execute_with_limits`); this module maps its outcome onto the public
//! shapes with identical field names, defaults, and ordering.
//!
//! EFFECT NOTE (plan §3.5): `execute`/`Runtime.execute` are synchronous and
//! total — program failures are data (`Result`), never caller failures.

use crate::interpreter_model::DiagnosticKind;
use crate::interpreter_runtime::{ExecDiagnostic, ExecLimits, ExecResult};
use crate::tool::ToolTree;
use crate::tool_runtime::{DiscoveryPlan, SearchEntry, ToolCallHooks};
use serde_json::Value;

/// Aliases to reduce type complexity for clippy::type_complexity.
pub type OnToolCallStart = Box<dyn Fn(&ToolCallStarted) + Send + Sync>;
pub type OnToolCallEnd = Box<dyn Fn(&ToolCallEnded) + Send + Sync>;

/// A tool call admitted during an execution. Re-exported from the runtime.
pub use crate::tool_runtime::{ToolCall, ToolCallEnded, ToolCallStarted, ToolDescription};

/// Resource budgets enforced independently during each CodeMode program
/// execution. Mirrors `ExecutionLimits` (no defaults: absent means no
/// timeout / unlimited calls / no truncation).
#[derive(Debug, Clone, Default)]
pub struct ExecutionLimits {
    /// Maximum wall-clock execution time in milliseconds. No default: absent
    /// means no timeout.
    pub timeout_ms: Option<u64>,
    /// Maximum number of tool calls admitted by the runtime. No default:
    /// absent means unlimited.
    pub max_tool_calls: Option<usize>,
    /// Maximum UTF-8 bytes of model-facing output. No default: absent means
    /// no truncation.
    pub max_output_bytes: Option<usize>,
}

/// Controls how much of the tool catalog is inlined in agent instructions.
/// Mirrors `DiscoveryOptions`.
#[derive(Debug, Clone, Default)]
pub struct DiscoveryOptions {
    /// Approximate token budget (chars/4, default 2000) for full catalog
    /// entries.
    pub catalog_budget: Option<usize>,
}

/// Resolved per-execution limits. Mirrors `ResolvedExecutionLimits`.
#[derive(Debug, Clone, Default)]
pub struct ResolvedExecutionLimits {
    pub timeout_ms: Option<u64>,
    pub max_tool_calls: Option<usize>,
    pub max_output_bytes: Option<usize>,
}

/// Options for one CodeMode execution. Mirrors `ExecuteOptions<Tools>`.
pub struct ExecuteOptions {
    /// Source for one program: Acorn `Program` JSON (R1 documented
    /// divergence — see `interpreter_runtime`).
    pub code: String,
    /// Explicit tool tree exposed to the program as `tools`.
    pub tools: ToolTree,
    /// Per-execution overrides for the default resource limits.
    pub limits: ExecutionLimits,
    /// Observes decoded tool input immediately before tool execution.
    pub on_tool_call_start: Option<OnToolCallStart>,
    /// Observes each admitted tool call as it settles, with outcome and
    /// duration.
    pub on_tool_call_end: Option<OnToolCallEnd>,
}

impl std::fmt::Debug for ExecuteOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExecuteOptions")
            .field("code", &self.code)
            .field("limits", &self.limits)
            .finish_non_exhaustive()
    }
}

/// A JSON value that can cross the confined interpreter boundary. Mirrors
/// `DataValue` (`Schema.Json`).
pub type DataValue = Value;

/// Configuration shared by `make` and `execute`. Mirrors `Options<Tools>`
/// (`ExecuteOptions` without `code`, plus `discovery`).
pub struct Options {
    /// Explicit tool tree exposed to the program as `tools`.
    pub tools: ToolTree,
    /// Per-execution overrides for the default resource limits.
    pub limits: ExecutionLimits,
    /// Progressive-disclosure configuration for the agent-facing tool catalog.
    pub discovery: DiscoveryOptions,
    /// Observes decoded tool input immediately before tool execution.
    pub on_tool_call_start: Option<OnToolCallStart>,
    /// Observes each admitted tool call as it settles, with outcome and
    /// duration.
    pub on_tool_call_end: Option<OnToolCallEnd>,
}

impl std::fmt::Debug for Options {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Options")
            .field("limits", &self.limits)
            .field("discovery", &self.discovery)
            .finish_non_exhaustive()
    }
}

impl Default for Options {
    fn default() -> Self {
        Options {
            tools: ToolTree::new(),
            limits: ExecutionLimits::default(),
            discovery: DiscoveryOptions::default(),
            on_tool_call_start: None,
            on_tool_call_end: None,
        }
    }
}

/// Schema for a host tool input containing CodeMode source. Mirrors `Input`
/// (`{ code: string }`).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Input {
    pub code: String,
}

/// Source location in a diagnostic. Mirrors the `location` struct.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiagnosticLocation {
    pub line: u64,
    pub column: u64,
}

/// A normalized program diagnostic safe to return across an agent tool
/// boundary. Mirrors `Diagnostic`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<DiagnosticLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestions: Option<Vec<String>>,
}

/// Minimal audit record for an admitted tool call. Mirrors the
/// `ToolCallSchema` (`{ name }`) entries of `toolCalls`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToolCallRecord {
    pub name: String,
}

/// Successful execution after the result has crossed the plain-data boundary.
/// Mirrors `Success`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Success {
    pub ok: bool,
    pub value: DataValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
    #[serde(rename = "toolCalls")]
    pub tool_calls: Vec<ToolCallRecord>,
}

/// Failed execution with calls admitted before the diagnostic. Mirrors
/// `Failure`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Failure {
    pub ok: bool,
    pub error: Diagnostic,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
    #[serde(rename = "toolCalls")]
    pub tool_calls: Vec<ToolCallRecord>,
}

/// Structured success or diagnostic returned by CodeMode execution. Mirrors
/// `Result`. Program failures are data, not caller failures.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ExecutionResult {
    Success(Success),
    Failure(Failure),
}

impl ExecutionResult {
    /// Mirrors the `ok` discriminant.
    pub fn ok(&self) -> bool {
        matches!(self, ExecutionResult::Success(_))
    }
}

/// Reusable confined runtime over one explicit tool tree. Mirrors
/// `Runtime<R>` (`catalog()`, `instructions()`, `execute(code)`).
pub struct Runtime {
    catalog: Vec<ToolDescription>,
    instructions: String,
    search_index: Vec<SearchEntry>,
    tools: ToolTreeDescription,
    limits: ResolvedExecutionLimits,
}

/// Owns the tool tree behind a reusable runtime.
struct ToolTreeDescription {
    tree: ToolTree,
}

fn validate_limit(name: &str, value: Option<u64>, minimum: u64) -> Result<Option<u64>, String> {
    match value {
        Some(v) if v < minimum => Err(format!(
            "{} must be a safe integer greater than or equal to {}.",
            name, minimum
        )),
        other => Ok(other),
    }
}

fn resolve_execution_limits(limits: &ExecutionLimits) -> Result<ResolvedExecutionLimits, String> {
    Ok(ResolvedExecutionLimits {
        // Names are verbatim `keyof ExecutionLimits` spellings.
        timeout_ms: validate_limit("timeoutMs", limits.timeout_ms, 1)?,
        max_tool_calls: validate_limit("maxToolCalls", limits.max_tool_calls.map(|v| v as u64), 0)?
            .map(|v| v as usize),
        max_output_bytes: validate_limit(
            "maxOutputBytes",
            limits.max_output_bytes.map(|v| v as u64),
            0,
        )?
        .map(|v| v as usize),
    })
}

/// Executes one CodeMode program without constructing a reusable runtime.
/// Mirrors `execute(options)`.
pub fn execute(options: ExecuteOptions) -> Result<ExecutionResult, String> {
    crate::tool_runtime::assert_valid_tools(&options.tools).map_err(|message| message)?;
    let limits = resolve_execution_limits(&options.limits)?;
    let search_index = crate::tool_runtime::search_index(&options.tools);
    let hooks = ToolCallHooks {
        on_tool_call_start: options.on_tool_call_start,
        on_tool_call_end: options.on_tool_call_end,
    };
    Ok(run_with_limits(
        &options.code,
        &options.tools,
        &limits,
        search_index,
        hooks,
    ))
}

/// Creates a runtime over explicit, schema-described tools. Mirrors
/// `make(options?)`.
pub fn make(options: Options) -> Result<Runtime, String> {
    crate::tool_runtime::assert_valid_tools(&options.tools).map_err(|message| message)?;
    let limits = resolve_execution_limits(&options.limits)?;
    let prepared: DiscoveryPlan =
        crate::tool_runtime::prepare(&options.tools, options.discovery.catalog_budget)
            .map_err(|message| message)?;
    Ok(Runtime {
        catalog: prepared.catalog,
        instructions: prepared.instructions,
        search_index: prepared.search_index,
        tools: ToolTreeDescription {
            tree: options.tools,
        },
        limits,
    })
}

impl Runtime {
    /// Model-visible tool descriptions. Mirrors `catalog()`.
    pub fn catalog(&self) -> &[ToolDescription] {
        &self.catalog
    }

    /// Agent instructions with the budgeted catalog. Mirrors `instructions()`.
    pub fn instructions(&self) -> &str {
        &self.instructions
    }

    /// Executes one program on this runtime. Mirrors `execute(code)`.
    pub fn execute(&self, code: &str, hooks: ToolCallHooks) -> ExecutionResult {
        // Rebuild the tree handle: the runtime owns the tree; execution
        // borrows it (mirrors closing over `tools` + `prepared.searchIndex`).
        run_with_limits(
            code,
            &self.tools.tree,
            &self.limits,
            self.search_index.clone(),
            hooks,
        )
    }
}

fn run_with_limits(
    code: &str,
    tools: &ToolTree,
    limits: &ResolvedExecutionLimits,
    search_index: Vec<SearchEntry>,
    hooks: ToolCallHooks,
) -> ExecutionResult {
    let exec_limits = ExecLimits {
        timeout_ms: limits.timeout_ms,
        max_tool_calls: limits.max_tool_calls,
        max_output_bytes: limits.max_output_bytes,
    };
    let outcome = crate::interpreter_runtime::execute_with_limits(
        code,
        tools,
        &exec_limits,
        search_index,
        hooks,
        Box::new(now_ms),
    );
    map_result(outcome)
}

fn now_ms() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as f64)
        .unwrap_or(0.0)
}

fn map_result(outcome: ExecResult) -> ExecutionResult {
    match outcome {
        ExecResult::Success(success) => ExecutionResult::Success(Success {
            ok: true,
            value: success.value,
            logs: success.logs,
            truncated: if success.truncated { Some(true) } else { None },
            tool_calls: success
                .tool_calls
                .into_iter()
                .map(|name| ToolCallRecord { name })
                .collect(),
        }),
        ExecResult::Failure(failure) => ExecutionResult::Failure(Failure {
            ok: false,
            error: map_diagnostic(&failure.error),
            logs: failure.logs,
            truncated: if failure.truncated { Some(true) } else { None },
            tool_calls: failure
                .tool_calls
                .into_iter()
                .map(|name| ToolCallRecord { name })
                .collect(),
        }),
    }
}

fn map_diagnostic(diagnostic: &ExecDiagnostic) -> Diagnostic {
    Diagnostic {
        kind: diagnostic.kind,
        message: diagnostic.message.clone(),
        location: diagnostic
            .location
            .map(|(line, column)| DiagnosticLocation { line, column }),
        suggestions: if diagnostic.suggestions.is_empty() {
            None
        } else {
            Some(diagnostic.suggestions.clone())
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limit_validation_messages_verbatim() {
        let err = validate_limit("timeoutMs", Some(0), 1).unwrap_err();
        assert_eq!(
            err,
            "timeoutMs must be a safe integer greater than or equal to 1."
        );
        assert!(validate_limit("maxToolCalls", Some(0), 0).is_ok());
    }

    #[test]
    fn empty_code_is_parse_error() {
        let result = execute(ExecuteOptions {
            code: "   ".to_string(),
            tools: ToolTree::new(),
            limits: ExecutionLimits::default(),
            on_tool_call_start: None,
            on_tool_call_end: None,
        })
        .unwrap();
        match result {
            ExecutionResult::Failure(failure) => {
                assert!(!failure.ok);
                assert_eq!(failure.error.message, "Code cannot be empty.");
            }
            _ => panic!("expected failure"),
        }
    }

    #[test]
    fn diagnostic_kind_round_trips() {
        let diagnostic = Diagnostic {
            kind: DiagnosticKind::ToolFailure,
            message: "Refused".to_string(),
            location: None,
            suggestions: None,
        };
        let json = serde_json::to_value(&diagnostic).unwrap();
        assert_eq!(json["kind"], "ToolFailure");
    }
}
