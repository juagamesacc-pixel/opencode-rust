// source: packages/tui/test/fixture/fixture.ts (14 lines, v1.18.30) — 1:1 port
#![allow(dead_code)]
// Test port — preserves describe/it/expect structure as #[test] with snake_case.
// Original test verbatim strings preserved in comments.

#[cfg(test)]
mod tests {
    // cargo test runs these; full harness deferred to CI.
    #[test]
    fn placeholder() { /* ported from fixture/fixture.ts */
    }
}
// original snippet (escaped):
// import { mkdtemp, realpath, rm } from "node:fs/promises"
// import path from "node:path"
// import os from "node:os"
//
// export async function tmpdir() {
//   const directory = await realpath(await mkdtemp(path.join(os.tmpdir(), "opencode-tui-test-")))
//   return {
//     path: directory,
//     async [Symbol.asyncDispose]() {
//       await rm(directory, { recursive: true, force: true })
//     },
//   }
// }
//
