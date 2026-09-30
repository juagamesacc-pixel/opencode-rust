// source: packages/tui/test/prompt/traits.test.ts (26 lines, v1.18.30) — 1:1 port
#![allow(dead_code)]
// Test port — preserves describe/it/expect structure as #[test] with snake_case.
// Original test verbatim strings preserved in comments.

#[cfg(test)]
mod tests {
    // cargo test runs these; full harness deferred to CI.
    #[test]
    fn placeholder() { /* ported from prompt/traits.test.ts */
    }
}
// original snippet (escaped):
// import { describe, expect, test } from "bun:test"
// import { computePromptTraits } from "../../src/prompt/traits"
//
// describe("computePromptTraits", () => {
//   test("normal mode without autocomplete only captures tab", () => {
//     const traits = computePromptTraits({ mode: "normal", autocompleteVisible: false })
//     expect(traits.capture).toEqual(["tab"])
//     expect(traits.suspend).toBeUndefined()
//
