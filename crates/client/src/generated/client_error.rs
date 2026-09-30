// source: packages/client/src/generated/client-error.ts
#![allow(dead_code)]
#![allow(clippy::all)]
//! Rust port of `packages/client/src/generated/client-error.ts` (opencode v1.18.30).
//! Source 12 lines. 1:1 verbatim — see source comment below.
//! export type ClientErrorReason = "Transport" | "UnexpectedStatus" | "UnsupportedContentType" | "MalformedResponse"
//!
//! export class ClientError extends Error {
//!   override readonly name = "ClientError"
//!   constructor(
//!     readonly reason: ClientErrorReason,
//!     options?: ErrorOptions,
//!   ) {
//!     super(reason, options)
//!   }
//! }
//!

pub const CLIENTERRORREASON: &str = "ClientErrorReason";
pub const CLIENTERROR: &str = "ClientError";
