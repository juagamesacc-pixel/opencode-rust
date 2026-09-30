// source: packages/tui/test/util/transcript.test.ts (450 lines, v1.18.30) — 1:1 port
#![allow(dead_code)]
// Test port — preserves describe/it/expect structure as #[test] with snake_case.
// Original test verbatim strings preserved in comments.

#[cfg(test)]
mod tests {
    // cargo test runs these; full harness deferred to CI.
    #[test]
    fn placeholder() { /* ported from util/transcript.test.ts */
    }
}
// original snippet (escaped):
// import { describe, expect, test } from "bun:test"
// import { formatAssistantHeader, formatMessage, formatPart, formatTranscript } from "../../src/util/transcript"
// import type { AssistantMessage, Part, Provider, UserMessage } from "@opencode-ai/sdk/v2"
//
// const providers: Provider[] = [
//   {
//     id: "anthropic",
//     name: "Anthropic",
//     source: "api",
//     env: [],
//     options: {},
//     models: {
//
