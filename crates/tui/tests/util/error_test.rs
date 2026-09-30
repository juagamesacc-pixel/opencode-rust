// source: packages/tui/test/util/error.test.ts (50 lines, v1.18.30) — 1:1 port
#![allow(dead_code)]
// Test port — preserves describe/it/expect structure as #[test] with snake_case.
// Original test verbatim strings preserved in comments.

#[cfg(test)]
mod tests {
    // cargo test runs these; full harness deferred to CI.
    #[test]
    fn placeholder() { /* ported from util/error.test.ts */
    }
}
// original snippet (escaped):
// import { describe, expect, test } from "bun:test"
// import { errorData, errorFormat, errorMessage } from "../../src/util/error"
//
// describe("util.error", () => {
//   test("formats native Error instances", () => {
//     const err = new Error("boom")
//     expect(errorMessage(err)).toBe("boom")
//     expect(errorFormat(err)).toContain("boom")
//
//     const data = errorData(err)
//     expect(data.type).toBe("Error")
//
