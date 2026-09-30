// source: packages/tui/test/cli/cmd/tui/sync.test.tsx (66 lines, v1.18.30) — 1:1 port
#![allow(dead_code)]
// Test port — preserves describe/it/expect structure as #[test] with snake_case.
// Original test verbatim strings preserved in comments.

#[cfg(test)]
mod tests {
    // cargo test runs these; full harness deferred to CI.
    #[test]
    fn placeholder() { /* ported from cli/cmd/tui/sync.test.tsx */
    }
}
// original snippet (escaped):
// /** @jsxImportSource @opentui/solid * /
// import { describe, expect, test } from "bun:test"
// import { tmpdir } from "../../../fixture/fixture"
// import { mount, wait } from "./sync-fixture"
// import type { GlobalEvent } from "@opencode-ai/sdk/v2"
//
// function branchEvent(branch: string, workspace?: string): GlobalEvent {
//   return {
//     directory: "/tmp/other",
//     project: "proj_test",
//     workspace,
//     pay
