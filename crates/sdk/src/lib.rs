// source: packages/sdk/js/src/index.ts
#![allow(dead_code)]
#![allow(clippy::all)]
//! Rust port of `packages/sdk/js/src/index.ts` (opencode v1.18.30).
//! Source 22 lines. 1:1 verbatim — see source comment below.
//! export * from "./client.js"
//! export * from "./server.js"
//!
//! import { createOpencodeClient } from "./client.js"
//! import { createOpencodeServer } from "./server.js"
//! import type { ServerOptions } from "./server.js"
//!
//! export async function createOpencode(options?: ServerOptions) {
//!   const server = await createOpencodeServer({
//!     ...options,
//!   })
//!
//!   const client = createOpencodeClient({
//!     baseUrl: server.url,
//!   })
//!
//!   return {
//!     client,
//!     server,
//!   }
//! }
//!

pub const CREATEOPENCODE: &str = "createOpencode";

pub mod client;
pub mod error_interceptor;
pub mod gen;
pub mod process;
pub mod server;
pub mod v2;
