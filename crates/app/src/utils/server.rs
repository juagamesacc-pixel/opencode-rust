//! Rust port of `packages/app/src/utils/server.ts` (opencode v1.18.30).
//!
//! Source 62 lines: `authTokenFromCredentials`, `authFromToken`,
//! `createSdkForServer`, `createApiForServer`, `ServerApi`.
//!
//! 1:1 notes:
//! - Pure auth helpers are fully ported (tested by `server.test.ts`).
//! - `btoa` is provided by `crate::utils::base64::encode_padded`.
//! - `createSdkForServer`/`createApiForServer` build the `Authorization` header
//!   for real but return PROVISIONAL stubs for the SDK client
//!   (`createOpencodeClient` from `@opencode-ai/sdk/v2/client`,
//!   `OpenCode.make` from `@opencode-ai/client/promise`); `config.headers`
//!   `instanceof Headers` handling is dropped (headers arrive as a JSON object).
//! - `ServerApi` (type `OpenCodeClient`) is an empty placeholder.
//! - Original file: `packages/app/src/utils/server.ts`

#![allow(dead_code)]

use crate::utils::base64::{decode64, encode_padded};

/// Mirrors the `{ username?: string; password: string }` input shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Credentials {
    pub username: Option<String>,
    pub password: String,
}

/// Mirrors `authTokenFromCredentials` → `btoa(\`${username ?? "opencode"}:${password}\`)`.
pub fn auth_token_from_credentials(input: &Credentials) -> String {
    let username = input.username.as_deref().unwrap_or("opencode");
    encode_padded(&format!("{username}:{}", input.password))
}

/// Mirrors the `{ username: string; password: string }` result shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedCredentials {
    pub username: String,
    pub password: String,
}

/// Mirrors `authFromToken(token: string | null)`.
pub fn auth_from_token(token: Option<&str>) -> Option<DecodedCredentials> {
    let decoded = decode64(token)?;
    let separator = decoded.find(':')?;
    let raw_username = &decoded[..separator];
    let username = if raw_username.is_empty() {
        "opencode".to_string()
    } else {
        raw_username.to_string()
    };
    Some(DecodedCredentials {
        username,
        password: decoded[separator + 1..].to_string(),
    })
}

/// PROVISIONAL: `ServerConnection.HttpBase` — `context/server` stub has an
/// empty `HttpBase` struct; this mirrors the `{ url, username?, password? }`
/// shape its consumers use.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpBase {
    pub url: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

/// Mirrors `createSdkForServer`: header assembly is real; the SDK client call
/// is PROVISIONAL (returns the client descriptor as JSON).
pub fn create_sdk_for_server(server: &HttpBase, config: serde_json::Value) -> serde_json::Value {
    let auth = server.password.as_ref().map(|password| {
        format!(
            "Basic {}",
            auth_token_from_credentials(&Credentials {
                username: server.username.clone(),
                password: password.clone(),
            })
        )
    });
    let mut headers = config
        .get("headers")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));
    if let Some(auth) = auth {
        headers["Authorization"] = serde_json::Value::String(auth);
    }
    // PROVISIONAL: `createOpencodeClient({ ...config, headers, baseUrl: server.url })`
    serde_json::json!({
        "baseUrl": server.url,
        "headers": headers,
    })
}

/// Mirrors `createApiForServer`: generates the `OpenCode.make` input shape.
pub fn create_api_for_server(server: &HttpBase) -> serde_json::Value {
    let headers = server.password.as_ref().map(|password| {
        serde_json::json!({
            "Authorization": format!(
                "Basic {}",
                auth_token_from_credentials(&Credentials {
                    username: server.username.clone(),
                    password: password.clone(),
                })
            )
        })
    });
    // PROVISIONAL: `OpenCode.make({ baseUrl, fetch, headers })`
    serde_json::json!({
        "baseUrl": server.url,
        "headers": headers.unwrap_or(serde_json::Value::Null),
    })
}

/// PROVISIONAL: `export type ServerApi = OpenCodeClient`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServerApi;
