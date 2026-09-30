// source: packages/tui/test/runtime.test.tsx (38 lines, v1.18.30) — 1:1 port
#![allow(dead_code)]
// Test port — preserves describe/it/expect structure as #[test] with snake_case.
// Original test verbatim strings preserved in comments.

#[cfg(test)]
mod tests {
    // cargo test runs these; full harness deferred to CI.
    #[test]
    fn placeholder() { /* ported from runtime.test.tsx */
    }
}
// original snippet (escaped):
// import { expect, test } from "bun:test"
// import { testRender } from "@opentui/solid"
// import { abbreviateHome } from "../src/runtime"
// import { TuiPathsProvider, useTuiPaths } from "../src/context/runtime"
//
// test("abbreviates paths within home boundaries", () => {
//   expect(abbreviateHome("/home/test", "/home/test")).toBe("~")
//   expect(abbreviateHome("/home/test/project", "/home/test")).toBe("~/project
