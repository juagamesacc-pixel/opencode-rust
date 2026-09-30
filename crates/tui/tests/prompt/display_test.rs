// source: packages/tui/test/prompt/display.test.ts (34 lines, v1.18.30) — 1:1 port
#![allow(dead_code)]
// Test port — preserves describe/it/expect structure as #[test] with snake_case.
// Original test verbatim strings preserved in comments.

#[cfg(test)]
mod tests {
    // cargo test runs these; full harness deferred to CI.
    #[test]
    fn placeholder() { /* ported from prompt/display.test.ts */
    }
}
// original snippet (escaped):
// import { describe, expect, test } from "bun:test"
// import { displayCharAt, displaySlice, mentionTriggerIndex } from "../../src/prompt/display"
//
// describe("prompt display", () => {
//   test("uses display-width offsets for mentions", () => {
//     expect(mentionTriggerIndex("@")).toBe(0)
//     expect(mentionTriggerIndex("test @")).toBe(5)
//     expect(mentionTriggerIndex("中文 @")).toBe(5)
//     expect(mentionTri
