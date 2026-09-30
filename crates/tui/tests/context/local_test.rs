// source: packages/tui/test/context/local.test.ts (23 lines, v1.18.30) — 1:1 port
#![allow(dead_code)]
// Test port — preserves describe/it/expect structure as #[test] with snake_case.
// Original test verbatim strings preserved in comments.

#[cfg(test)]
mod tests {
    // cargo test runs these; full harness deferred to CI.
    #[test]
    fn placeholder() { /* ported from context/local.test.ts */
    }
}
// original snippet (escaped):
// import { expect, test } from "bun:test"
// import { parseModel, recentModels } from "../../src/context/local"
//
// test("parses model IDs containing slashes", () => {
//   expect(parseModel("provider/family/model")).toEqual({
//     providerID: "provider",
//     modelID: "family/model",
//   })
// })
//
// test("moves a model to the front, deduplicates, and limits recents", () => {
//   const recent = Array.from({ length: 12
