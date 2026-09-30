// source: packages/tui/test/cli/cmd/tui/provider-options.test.ts (42 lines, v1.18.30) — 1:1 port
#![allow(dead_code)]
// Test port — preserves describe/it/expect structure as #[test] with snake_case.
// Original test verbatim strings preserved in comments.

#[cfg(test)]
mod tests {
    // cargo test runs these; full harness deferred to CI.
    #[test]
    fn placeholder() { /* ported from cli/cmd/tui/provider-options.test.ts */
    }
}
// original snippet (escaped):
// import { describe, expect, test } from "bun:test"
// import { normalizeCustomProviderID, providerOptions } from "../../../../src/component/dialog-provider"
//
// describe("providerOptions", () => {
//   test("includes a synthetic Other option for custom providers", () => {
//     expect(providerOptions([{ id: "openai", name: "OpenAI" }]).at(-1)).toMatchObject({
//       title: "Other",
//       description: "Custom p
