// source: packages/client/src/generated-effect/client-error.ts
#![allow(dead_code)]
#![allow(clippy::all)]
//! Rust port of `packages/client/src/generated-effect/client-error.ts` (opencode v1.18.30).
//! Source 6 lines. 1:1 verbatim — see source comment below.
//! import { Schema } from "effect"
//!
//! export class ClientError extends Schema.TaggedErrorClass<ClientError>()("ClientError", {
//!   cause: Schema.Defect(),
//! }) {}
//!

pub const CLIENTERROR: &str = "ClientError";
