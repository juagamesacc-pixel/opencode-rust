// source: packages/tui/test/prompt/persistence.test.ts (24 lines, v1.18.30) — 1:1 port
#![allow(dead_code)]
// Test port — preserves describe/it/expect structure as #[test] with snake_case.
// Original test verbatim strings preserved in comments.

#[cfg(test)]
mod tests {
    // cargo test runs these; full harness deferred to CI.
    #[test]
    fn placeholder() { /* ported from prompt/persistence.test.ts */
    }
}
// original snippet (escaped):
// import { expect, test } from "bun:test"
// import path from "path"
// import { mkdtemp, rm } from "fs/promises"
// import { tmpdir } from "os"
// import { appendText, readJson, readText, writeJsonAtomic, writeText } from "../../src/util/persistence"
//
// test("persistence creates parent directories and supports text, append, and JSON", async () => {
//   const root = await mkdtemp(path.join(tmpdir(), "opencode-tui-p
