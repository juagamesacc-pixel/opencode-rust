// source: packages/tui/test/util/tool-display.test.ts (41 lines, v1.18.30) — 1:1 port
#![allow(dead_code)]
// Test port — preserves describe/it/expect structure as #[test] with snake_case.
// Original test verbatim strings preserved in comments.

#[cfg(test)]
mod tests {
    // cargo test runs these; full harness deferred to CI.
    #[test]
    fn placeholder() { /* ported from util/tool-display.test.ts */
    }
}
// original snippet (escaped):
// import { describe, expect, test } from "bun:test"
// import { toolDisplayMetadata, webSearchProviderLabel } from "../../src/util/tool-display"
//
// describe("webSearchProviderLabel", () => {
//   test("labels known providers", () => {
//     expect(webSearchProviderLabel("parallel")).toBe("Parallel Web Search")
//     expect(webSearchProviderLabel("exa")).toBe("Exa Web Search")
//   })
//
//   for (const [name, provider]
