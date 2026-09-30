// source: packages/http-recorder/src/internal.ts
#![allow(dead_code)]
#![allow(clippy::all)]
//! Rust port of `packages/http-recorder/src/internal.ts` (opencode v1.18.30).
//! Source 16 lines. 1:1 verbatim — see source comment below.
//! export { CassetteNotFoundError, hasCassetteSync, UnsafeCassetteError } from "./cassette.js"
//! export { cassetteLayer, recordingLayer, type RecordReplayMode, type RecordReplayOptions } from "./internal-effect.js"
//! export { redactHeaders, redactUrl, secretFindings, type SecretFinding } from "./redaction.js"
//! export { socketLayer } from "./socket.js"
//! export {
//!   makeWebSocketExecutor,
//!   type WebSocketConnection,
//!   type WebSocketExecutor,
//!   type WebSocketRecordReplayOptions,
//!   type WebSocketRequest,
//! } from "./websocket.js"
//! export * as Cassette from "./cassette.js"
//! export * as Redactor from "./redactor.js"
//!
//! export * as HttpRecorderInternal from "./internal.js"
//!

pub const SOURCE: &str = "packages/http-recorder/src/internal.ts";
