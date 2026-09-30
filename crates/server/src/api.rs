//! Rust port of `packages/server/src/api.ts` (opencode v1.18.30).
//!
//! Source (8 lines):
//! ```ts
//! import { makeDefaultApi } from "@opencode-ai/protocol/api"
//! import { LocationMiddleware } from "./location"
//! import { SessionLocationMiddleware } from "./middleware/session-location"
//! export const Api = makeDefaultApi({
//!   locationMiddleware: LocationMiddleware,
//!   sessionLocationMiddleware: SessionLocationMiddleware,
//! })
//! ```
//!
//! 1:1 notes:
//! - `makeDefaultApi` is `protocol::api::make_default_api()` (group order +
//!   titles/version verbatim, see `crates/protocol/src/api.rs`).
//! - Middleware keys are the service IDs `LocationMiddleware` (`@opencode/HttpApiLocation`)
//!   and `SessionLocationMiddleware` (`@opencode/HttpApiSessionLocation`).

use protocol::api::{make_default_api, Api as ProtocolApi};

/// Service ID verbatim from `LocationMiddleware` (`"@opencode/HttpApiLocation"`).
pub const LOCATION_MIDDLEWARE_ID: &str = "@opencode/HttpApiLocation";

/// Service ID verbatim from `SessionLocationMiddleware` (`"@opencode/HttpApiSessionLocation"`).
pub const SESSION_LOCATION_MIDDLEWARE_ID: &str = "@opencode/HttpApiSessionLocation";

/// Port of `export const Api = makeDefaultApi({ locationMiddleware, sessionLocationMiddleware })`.
///
/// The two middleware args are server-side placement hints (no descriptor
/// representation); the Api descriptor itself is the default protocol Api.
pub fn api() -> ProtocolApi {
    make_default_api()
}

/// Convenience constant-like accessor (mirrors the `const Api` binding).
pub fn Api() -> ProtocolApi {
    make_default_api()
}
