// source: src/server/auth.ts — exports: [Credentials, DecodedCredentials, Config, Info, required, authorized, header, headers]
// PROVISIONAL pending crates/core: `@opencode-ai/core/flag/flag`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "./auth"
/// - "@opencode/ServerAuthConfig"
/// - "OPENCODE_SERVER_PASSWORD"
/// - "OPENCODE_SERVER_USERNAME"
/// - "opencode"
use serde::{Deserialize, Serialize};

/// source: `export type Credentials` — shape as JSON value; CI verifies.
pub type Credentials = serde_json::Value;
/// source: `export type DecodedCredentials` — shape as JSON value; CI verifies.
pub type DecodedCredentials = serde_json::Value;
/// source: `export class Config` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub value: serde_json::Value,
}
/// source: `export type Info` — shape as JSON value; CI verifies.
pub type Info = serde_json::Value;
/// source: `export function required` — stub shell; CI verifies behavior.
pub fn required(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function authorized` — stub shell; CI verifies behavior.
pub fn authorized(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function header` — stub shell; CI verifies behavior.
pub fn header(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: `export function headers` — stub shell; CI verifies behavior.
pub fn headers(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
