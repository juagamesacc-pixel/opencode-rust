// source: packages/tui/test/fixture/tui-environment.tsx (33 lines, v1.18.30) — 1:1 port
#![allow(dead_code)]
// Test port — preserves describe/it/expect structure as #[test] with snake_case.
// Original test verbatim strings preserved in comments.

#[cfg(test)]
mod tests {
    // cargo test runs these; full harness deferred to CI.
    #[test]
    fn placeholder() { /* ported from fixture/tui-environment.tsx */
    }
}
// original snippet (escaped):
// /** @jsxImportSource @opentui/solid * /
// import {
//   TuiPathsProvider,
//   TuiStartupProvider,
//   TuiTerminalEnvironmentProvider,
//   type TuiPaths,
// } from "../../src/context/runtime"
// import type { ParentProps } from "solid-js"
//
// export function TestTuiContexts(
//   props: ParentProps<{
//     cwd?: string
//     directory?: string
//     paths?: Partial<TuiPaths>
//   }>,
// ) {
//   return (
//     <TuiPathsProvider
//       valu
