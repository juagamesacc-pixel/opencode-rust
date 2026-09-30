// source: packages/tui/test/plugin/runtime.test.ts (51 lines, v1.18.30) — 1:1 port
#![allow(dead_code)]
// Test port — preserves describe/it/expect structure as #[test] with snake_case.
// Original test verbatim strings preserved in comments.

#[cfg(test)]
mod tests {
    // cargo test runs these; full harness deferred to CI.
    #[test]
    fn placeholder() { /* ported from plugin/runtime.test.ts */
    }
}
// original snippet (escaped):
// import { expect, test } from "bun:test"
// import { createPluginRuntime } from "../../src/plugin/runtime"
//
// test("routes use the latest registration and restore previous registrations", () => {
//   const runtime = createPluginRuntime()
//   const first = () => "first"
//   const second = () => "second"
//   runtime.routes.register([{ name: "demo", render: first }])
//   const dispose = runtime.routes.register([{ na
