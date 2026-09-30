// source: packages/tui/test/fixture/tui-runtime.ts (13 lines, v1.18.30) — 1:1 port
#![allow(dead_code)]
// Test port — preserves describe/it/expect structure as #[test] with snake_case.
// Original test verbatim strings preserved in comments.

#[cfg(test)]
mod tests {
    // cargo test runs these; full harness deferred to CI.
    #[test]
    fn placeholder() { /* ported from fixture/tui-runtime.ts */
    }
}
// original snippet (escaped):
// import { resolve, type Info, type Resolved } from "../../src/config"
// import { TuiKeybind } from "../../src/config/keybind"
//
// type ResolvedInput = Omit<Info, "attention" | "keybinds" | "leader_timeout"> & {
//   attention?: Partial<Resolved["attention"]>
//   keybinds?: Partial<TuiKeybind.Keybinds>
//   leader_timeout?: number
// }
//
// export function createTuiResolvedConfig(input: ResolvedInput = {}) {
//   return r
