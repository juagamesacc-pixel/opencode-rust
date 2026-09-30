// source: src/cli/tui/worker.ts — exports: [rpc]
// PROVISIONAL pending external `node:v8` (host-provided; no new dep)
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@/server/server"
/// - "unhandledRejection"
/// - "uncaughtException"
/// - "global.event"
/// - "authorization"
/// - "Authorization"
/// - "server.heapsnapshot"
/// source: `export const rpc` — shape as JSON value; CI verifies.
pub type rpc = serde_json::Value;
