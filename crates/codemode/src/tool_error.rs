//! Port of `src/tool-error.ts`.
//!
//! Safe operational refusal channel: a tool returns `ToolError` (via
//! [`tool_error`]) when it wants the model to see a safe message; the private
//! `cause` stays host-side. Reported as `ToolFailure`.

/// Safe operational refusal from a standard tool pack, reported as `ToolFailure`.
#[derive(Debug, Clone)]
pub struct ToolError {
    /// Model-safe message included in the execution diagnostic.
    pub message: String,
    /// Private host-side cause. Never rendered into model-visible output.
    pub cause: Option<String>,
}

impl ToolError {
    /// Mirrors `new ToolError({ message, cause? })`.
    pub fn new(message: impl Into<String>, cause: Option<String>) -> Self {
        ToolError {
            message: message.into(),
            cause,
        }
    }

    /// Message accessor mirroring the TS `message` field.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for ToolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ToolError: {}", self.message)
    }
}

impl std::error::Error for ToolError {}

/// Creates a tool refusal whose message is safe to include in an execution diagnostic.
///
/// Mirrors `toolError(message, cause?)`.
pub fn tool_error(message: impl Into<String>, cause: Option<String>) -> ToolError {
    ToolError::new(message, cause)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_error_without_cause_has_no_cause() {
        let err = tool_error("Authorized request was refused", None);
        assert_eq!(err.message(), "Authorized request was refused");
        assert!(err.cause.is_none());
    }

    #[test]
    fn tool_error_with_cause_keeps_private_cause() {
        let err = tool_error(
            "Tool execution failed",
            Some("postgres://user:defect-secret@example.invalid".to_string()),
        );
        assert_eq!(err.message(), "Tool execution failed");
        assert!(err.cause.is_some());
    }
}
