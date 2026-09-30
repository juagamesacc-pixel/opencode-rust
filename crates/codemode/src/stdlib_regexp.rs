//! Port of `src/stdlib/regexp.ts`.

use crate::interpreter_model::{AstNode, DiagnosticKind, InterpreterRuntimeError};
use crate::stdlib_value::{coerce_to_string, DataVal};
use crate::values::SandboxRegExp;

/// RegExp methods, verbatim. Mirrors `regexpMethods`.
pub const REGEXP_METHODS: &[&str] = &["test", "exec", "toString"];

/// RegExp properties, verbatim. Mirrors `regexpProperties`.
pub const REGEXP_PROPERTIES: &[&str] = &[
    "source",
    "flags",
    "lastIndex",
    "global",
    "ignoreCase",
    "multiline",
    "sticky",
    "unicode",
    "dotAll",
];

/// Mirrors `regexpMethods.has(name)`.
pub fn is_regexp_method(name: &str) -> bool {
    REGEXP_METHODS.contains(&name)
}

/// Mirrors `regexpProperties.has(name)`.
pub fn is_regexp_property(name: &str) -> bool {
    REGEXP_PROPERTIES.contains(&name)
}

/// Strips the `Invalid regular expression: ` prefix. Mirrors
/// `regexFailureReason(error)`.
pub fn regex_failure_reason(message: &str) -> String {
    let lower = message.to_lowercase();
    if let Some(pos) = lower.find("invalid regular expression:") {
        message[pos + "invalid regular expression:".len()..]
            .trim_start()
            .to_string()
    } else {
        message.to_string()
    }
}

/// Verbatim escape hint. Mirrors `escapeRegexHint`.
pub const ESCAPE_REGEX_HINT: &str = "To match special characters like ( ) [ ] { } + * ? . literally, escape them with a backslash (e.g. \"\\\\(\") or test for them with String.includes instead.";

/// Validates a regex pattern/flags pair without a regex engine dependency.
/// Returns the failure reason, if invalid. The interpreter reports invalid
/// literals at construction with the verbatim `Syntax '<kind>' ...` triage;
/// this helper backs string-pattern methods (`String.match` etc.).
pub fn validate_regex(pattern: &str, flags: &str) -> Result<(), String> {
    // Flag validation (verbatim JS flag set).
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
    // Balanced-group scan (best-effort structural check; full equivalence
    // with the host engine is R1-adjacent and flagged in the interpreter).
    let mut depth = 0i32;
    let mut chars = pattern.chars().peekable();
    let mut in_class = false;
    while let Some(c) = chars.next() {
        if c == '\\' {
            chars.next();
            continue;
        }
        if in_class {
            if c == ']' {
                in_class = false;
            }
            continue;
        }
        match c {
            '[' => in_class = true,
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth < 0 {
                    return Err("Unmatched ')'".to_string());
                }
            }
            _ => {}
        }
    }
    if in_class {
        return Err("Unterminated character class".to_string());
    }
    if depth != 0 {
        return Err("Unterminated group".to_string());
    }
    Ok(())
}

/// Builds a host regex descriptor from a sandbox value or string pattern.
/// Mirrors `toHostRegex(arg, method, node, extraFlags?)`.
pub fn to_host_regex(
    arg: &DataVal,
    method: &str,
    node: Option<&AstNode>,
    extra_flags: &str,
) -> Result<HostRegex, InterpreterRuntimeError> {
    if let DataVal::Sandbox(crate::values::SandboxValue::RegExp(r)) = arg {
        let mut flags = r.flags.clone();
        for c in extra_flags.chars() {
            if !flags.contains(c) {
                flags.push(c);
            }
        }
        if let Err(reason) = validate_regex(&r.pattern, &flags) {
            return Err(InterpreterRuntimeError::new(
                format!(
                    "String.{} received an invalid regular expression ({}). {}",
                    method,
                    regex_failure_reason(&reason),
                    ESCAPE_REGEX_HINT
                ),
                node.cloned(),
                DiagnosticKind::ExecutionFailure,
                None,
            )
            .as_error("SyntaxError"));
        }
        return Ok(HostRegex {
            pattern: r.pattern.clone(),
            flags,
        });
    }
    if let DataVal::Str(s) = arg {
        if let Err(reason) = validate_regex(s, extra_flags) {
            return Err(InterpreterRuntimeError::new(
                format!(
                    "String.{} received the string {}, which is not a valid regular expression pattern ({}). {}",
                    method,
                    serde_json::to_string(s).unwrap_or_default(),
                    regex_failure_reason(&reason),
                    ESCAPE_REGEX_HINT
                ),
                node.cloned(),
                DiagnosticKind::ExecutionFailure,
                None,
            )
            .as_error("SyntaxError"));
        }
        return Ok(HostRegex {
            pattern: s.clone(),
            flags: extra_flags.to_string(),
        });
    }
    let got = match arg {
        DataVal::Null => "null".to_string(),
        DataVal::Undefined => "undefined".to_string(),
        DataVal::Bool(_) => "boolean".to_string(),
        DataVal::Number(_) => "number".to_string(),
        DataVal::Str(_) => "string".to_string(),
        DataVal::Array(_) => "object".to_string(),
        DataVal::Object(_) => "object".to_string(),
        DataVal::Sandbox(_) => "object".to_string(),
    };
    Err(InterpreterRuntimeError::new(
        format!(
            "String.{} expects a regular expression (a /pattern/flags literal or new RegExp(...)) or a string pattern, not {}.",
            method, got
        ),
        node.cloned(),
        DiagnosticKind::ExecutionFailure,
        None,
    ))
}

/// A validated host regex descriptor. Actual matching is executed by the
/// interpreter's masonry matcher over the pattern subset models generate
/// (literals, classes, quantifiers, anchors, groups, alternation); see
/// `interpreter_runtime::regex_match`. This descriptor preserves the exact
/// construction-time triage.
#[derive(Debug, Clone)]
pub struct HostRegex {
    pub pattern: String,
    pub flags: String,
}

impl HostRegex {
    /// Whether the global flag is set.
    pub fn global(&self) -> bool {
        self.flags.contains('g')
    }

    /// Whether matching ignores case.
    pub fn ignore_case(&self) -> bool {
        self.flags.contains('i')
    }

    /// Whether `.` matches line terminators.
    pub fn dot_all(&self) -> bool {
        self.flags.contains('s')
    }

    /// Whether `^`/`$` match line boundaries.
    pub fn multiline(&self) -> bool {
        self.flags.contains('m')
    }
}

/// Converts a sandbox RegExp construction (`new RegExp(pattern, flags)`).
/// Reports invalid patterns with the verbatim constructor diagnostic.
pub fn construct_regex(
    pattern: &DataVal,
    flags: &DataVal,
    node: Option<&AstNode>,
) -> Result<SandboxRegExp, InterpreterRuntimeError> {
    let pattern_str = coerce_to_string(pattern);
    let flags_str = match flags {
        DataVal::Undefined => String::new(),
        other => coerce_to_string(other),
    };
    if let Err(reason) = validate_regex(&pattern_str, &flags_str) {
        return Err(InterpreterRuntimeError::new(
            format!(
                "Invalid regular expression: {}",
                regex_failure_reason(&reason)
            ),
            node.cloned(),
            DiagnosticKind::ExecutionFailure,
            None,
        )
        .as_error("SyntaxError"));
    }
    Ok(SandboxRegExp::new(pattern_str, flags_str))
}

/// Mirrors `invokeRegExpMethod(value, name, args, node)` for `toString`;
/// `test`/`exec` dispatch to the interpreter matcher (needs subject +
/// match-materialization helpers below).
pub fn regexp_to_string(value: &SandboxRegExp) -> String {
    format!("/{}/{}", value.pattern, value.flags)
}

/// Builds a match-result array value from capture groups.
/// Mirrors `matchToValue(match)`: the array holds the groups; the interpreter
/// attaches the `index` and filtered `groups` own-properties (blocked member
/// names dropped, verbatim) to the array's property bag.
pub fn match_to_value(groups: Vec<DataVal>) -> DataVal {
    DataVal::Array(groups)
}

/// Filters named groups for the `groups` own-property, dropping blocked
/// member names. Mirrors the `matchToValue` groups loop verbatim.
pub fn filter_named_groups(named: Vec<(String, Option<String>)>) -> Vec<(String, DataVal)> {
    named
        .into_iter()
        .filter(|(k, _)| !crate::tool_runtime::is_blocked_member(k))
        .map(|(k, v)| (k, v.map(DataVal::Str).unwrap_or(DataVal::Undefined)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regex_tables_verbatim() {
        assert!(is_regexp_method("test"));
        assert!(is_regexp_property("dotAll"));
        assert_eq!(
            ESCAPE_REGEX_HINT,
            "To match special characters like ( ) [ ] { } + * ? . literally, escape them with a backslash (e.g. \"\\\\(\") or test for them with String.includes instead."
        );
    }

    #[test]
    fn validate_rejects_unbalanced() {
        assert!(validate_regex("(a", "").is_err());
        assert!(validate_regex("a", "z").is_err());
        assert!(validate_regex("abc", "gi").is_ok());
    }
}
