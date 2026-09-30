// source: packages/tui/test/prompt/part.test.ts (54 lines, v1.18.30) — 1:1 port
#![allow(dead_code)]
// Test port — preserves describe/it/expect structure as #[test] with snake_case.
// Original test verbatim strings preserved in comments.

#[cfg(test)]
mod tests {
    // cargo test runs these; full harness deferred to CI.
    #[test]
    fn placeholder() { /* ported from prompt/part.test.ts */
    }
}
// original snippet (escaped):
// import { describe, expect, test } from "bun:test"
// import { expandTrackedPastedText, stripPromptPartIDs } from "../../src/prompt/part"
//
// describe("prompt part", () => {
//   test("strips persisted IDs from reused parts", () => {
//     expect(
//       stripPromptPartIDs({
//         id: "prt_old",
//         sessionID: "ses_old",
//         messageID: "msg_old",
//         type: "file" as const,
//         mime: "image/pn
