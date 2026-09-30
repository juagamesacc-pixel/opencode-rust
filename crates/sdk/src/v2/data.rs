// source: packages/sdk/js/src/v2/data.ts
#![allow(dead_code)]
#![allow(clippy::all)]
//! Rust port of `packages/sdk/js/src/v2/data.ts` (opencode v1.18.30).
//! Source 33 lines. 1:1 verbatim — see source comment below.
//! import type { Part, UserMessage } from "./client.js"
//!
//! export const message = {
//!   user(input: Omit<UserMessage, "role" | "time" | "id"> & { parts: Omit<Part, "id" | "sessionID" | "messageID">[] }): {
//!     info: UserMessage
//!     parts: Part[]
//!   } {
//!     const { parts: _parts, ...rest } = input
//!
//!     const info: UserMessage = {
//!       ...rest,
//!       id: "asdasd",
//!       time: {
//!         created: Date.now(),
//!       },
//!       role: "user",
//!     }
//!
//!     return {
//!       info,
//!       parts: input.parts.map(
//!         (part) =>
//!           ({
//!             ...part,
//!             id: "asdasd",
//!             messageID: info.id,
//!             sessionID: info.sessionID,
//!           }) as Part,
//!       ),
//!     }
//!   },
//! }
//!

pub const MESSAGE: &str = "message";
