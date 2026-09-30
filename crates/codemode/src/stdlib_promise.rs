//! Port of `src/stdlib/promise.ts`.

use crate::interpreter_model::PromiseMethodName;

/// Promise statics, verbatim. Mirrors `promiseStatics`.
pub const PROMISE_STATICS: &[PromiseMethodName] = &[
    PromiseMethodName::All,
    PromiseMethodName::AllSettled,
    PromiseMethodName::Race,
    PromiseMethodName::Resolve,
    PromiseMethodName::Reject,
];

/// Mirrors `promiseStatics.has(name)`.
pub fn is_promise_static(name: &str) -> bool {
    PromiseMethodName::parse(name).is_some()
}

/// Maximum number of eagerly forked tool calls that may run concurrently.
/// Mirrors `TOOL_CALL_CONCURRENCY = 8`.
pub const TOOL_CALL_CONCURRENCY: usize = 8;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn promise_statics_and_concurrency_verbatim() {
        assert!(is_promise_static("allSettled"));
        assert!(!is_promise_static("any"));
        assert_eq!(TOOL_CALL_CONCURRENCY, 8);
    }
}
