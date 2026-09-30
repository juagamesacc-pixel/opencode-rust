// source: packages/http-recorder/src/index.ts
#![allow(dead_code)]
#![allow(clippy::all)]
//! Rust port of `packages/http-recorder/src/index.ts` (opencode v1.18.30).
//! Source 19 lines. 1:1 verbatim — see source comment below.
//! import { http } from "./effect.js"
//! import { socket } from "./socket.js"
//!
//! /** HTTP and WebSocket cassette recording. */
//! export const HttpRecorder = { http, socket } as const
//!
//! export namespace HttpRecorder {
//!   /** Additional JSON metadata stored with a cassette. */
//!   export type CassetteMetadata = import("./types.js").CassetteMetadata
//!   /** Recorder configuration. */
//!   export type RecorderOptions = import("./types.js").RecorderOptions
//!   /** Additive redaction and header-preservation policy. */
//!   export type RedactOptions = import("./types.js").RedactOptions
//!   /** Returns whether an incoming HTTP request matches a recorded request. */
//!   export type RequestMatcher = import("./types.js").RequestMatcher
//!   /** The normalized HTTP request representation used for matching. */
//!   export type RequestSnapshot = import("./types.js").RequestSnapshot
//! }
//!

pub const HTTPRECORDER: &str = "HttpRecorder";
pub const CASSETTEMETADATA: &str = "CassetteMetadata";
pub const RECORDEROPTIONS: &str = "RecorderOptions";
pub const REDACTOPTIONS: &str = "RedactOptions";
pub const REQUESTMATCHER: &str = "RequestMatcher";
pub const REQUESTSNAPSHOT: &str = "RequestSnapshot";

pub mod cassette;
pub mod effect;
pub mod internal;
pub mod internal_effect;
pub mod matching;
pub mod recorder;
pub mod redaction;
pub mod redactor;
pub mod schema;
pub mod socket;
pub mod types;
pub mod websocket;
