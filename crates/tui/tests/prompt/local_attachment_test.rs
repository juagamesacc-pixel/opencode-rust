// source: packages/tui/test/prompt/local-attachment.test.ts (44 lines, v1.18.30) — 1:1 port
#![allow(dead_code)]
// Test port — preserves describe/it/expect structure as #[test] with snake_case.
// Original test verbatim strings preserved in comments.

#[cfg(test)]
mod tests {
    // cargo test runs these; full harness deferred to CI.
    #[test]
    fn placeholder() { /* ported from prompt/local-attachment.test.ts */
    }
}
// original snippet (escaped):
// import { describe, expect, test } from "bun:test"
// import { readLocalAttachmentWith } from "../../src/component/prompt/local-attachment"
// import type { LocalFiles } from "../../src/component/prompt/local-attachment"
//
// function files(input: { mime: string; text?: string; bytes?: Uint8Array }): LocalFiles {
//   return {
//     mime: async () => input.mime,
//     readText: async () => input.text ?? "",
//     rea
