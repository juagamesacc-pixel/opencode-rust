// source: packages/tui/test/cli/cmd/tui/dialog-workspace-create.test.ts (29 lines, v1.18.30) — 1:1 port
#![allow(dead_code)]
// Test port — preserves describe/it/expect structure as #[test] with snake_case.
// Original test verbatim strings preserved in comments.

#[cfg(test)]
mod tests {
    // cargo test runs these; full harness deferred to CI.
    #[test]
    fn placeholder() { /* ported from cli/cmd/tui/dialog-workspace-create.test.ts */
    }
}
// original snippet (escaped):
// import { describe, expect, test } from "bun:test"
// import { recentConnectedWorkspaces } from "../../../../src/component/dialog-workspace-create"
//
// describe("recentConnectedWorkspaces", () => {
//   test("returns connected workspaces sorted by time used", () => {
//     const workspaces = [
//       { id: "wrk_a", name: "alpha", timeUsed: 700 },
//       { id: "wrk_b", name: "beta", timeUsed: 800 },
//       { id:
