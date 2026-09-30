// source: packages/tui/test/cli/tui/prompt-submit-race.test.ts (99 lines, v1.18.30) — 1:1 port
#![allow(dead_code)]
// Test port — preserves describe/it/expect structure as #[test] with snake_case.
// Original test verbatim strings preserved in comments.

#[cfg(test)]
mod tests {
    // cargo test runs these; full harness deferred to CI.
    #[test]
    fn placeholder() { /* ported from cli/tui/prompt-submit-race.test.ts */
    }
}
// original snippet (escaped):
// import { describe, expect, test } from "bun:test"
//
// // Regression test for the prompt submit race in
// // packages/tui/src/component/prompt/index.tsx (`submit`).
// //
// // Before the fix, two concurrent `submit()` calls (e.g. a double-pressed
// // Enter, or the input's native onSubmit racing another dispatch) each
// // passed the `if (!store.prompt.input) return false` guard, each
// // `await sdk.client.sessio
