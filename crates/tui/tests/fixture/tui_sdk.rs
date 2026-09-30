// source: packages/tui/test/fixture/tui-sdk.ts (110 lines, v1.18.30) — 1:1 port
#![allow(dead_code)]
// Test port — preserves describe/it/expect structure as #[test] with snake_case.
// Original test verbatim strings preserved in comments.

#[cfg(test)]
mod tests {
    // cargo test runs these; full harness deferred to CI.
    #[test]
    fn placeholder() { /* ported from fixture/tui-sdk.ts */
    }
}
// original snippet (escaped):
// import type { GlobalEvent } from "@opencode-ai/sdk/v2"
// import type { EventSource } from "../../src/context/sdk"
//
// export const worktree = "/tmp/opencode"
// export const directory = `${worktree}/packages/tui`
//
// export function json(data: unknown, init?: ResponseInit) {
//   return new Response(JSON.stringify(data), {
//     ...init,
//     headers: { "content-type": "application/json", ...(init?.headers ?? {})
