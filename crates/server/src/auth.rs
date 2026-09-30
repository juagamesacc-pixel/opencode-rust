//! Rust port of `packages/server/src/auth.ts` (opencode v1.18.30).
//!
//! Source 63 lines. Exports: `Credentials`, `DecodedCredentials`, `Info`,
//! `Config` service (`"@opencode/ServerAuthConfig"`), `required`, `authorized`,
//! `header`, `headers`. Behavior strings/defaults/ordering verbatim.

/// Mirrors `export type Credentials = { password?: string; username?: string }`.
#[derive(Clone, Debug, PartialEq)]
pub struct Credentials {
    pub password: Option<String>,
    pub username: Option<String>,
}

/// Mirrors `DecodedCredentials` (`username: string`, `password: Redacted`).
#[derive(Clone, Debug, PartialEq)]
pub struct DecodedCredentials {
    pub username: String,
    pub password: Redacted,
}

/// Minimal `Redacted` (Effect `Redacted.make` / `Redacted.value`). Value is
/// held but `Debug` is redacted — matches the Effect wrapper's secrecy.
#[derive(Clone, PartialEq, Eq)]
pub struct Redacted(String);

impl Redacted {
    pub fn make(s: impl Into<String>) -> Self {
        Self(s.into())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for Redacted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<redacted>")
    }
}

/// Mirrors `type Info = { password: Option<string>; username: string }`.
#[derive(Clone, Debug, PartialEq)]
pub struct Info {
    pub password: Option<String>,
    pub username: String,
}

/// Service ID verbatim: `"@opencode/ServerAuthConfig"`.
pub const CONFIG_SERVICE_ID: &str = "@opencode/ServerAuthConfig";

/// Mirrors `Config` service with two constructors: `configLayer(input)` and
/// the env-reading `layer` (OPENCODE_SERVER_PASSWORD option, OPENCODE_SERVER_USERNAME default "opencode").
///
/// In Rust the env-reading is pure: callers pass the env map or use `from_env()`.
#[derive(Clone, Debug, PartialEq)]
pub struct Config(pub Info);

impl Config {
    /// Port of `static configLayer(input) => Layer.succeed(this, of(input))`.
    pub fn config_layer(input: Info) -> Self {
        Self(input)
    }

    /// Port of `static get layer` — reads `OPENCODE_SERVER_PASSWORD` (option)
    /// and `OPENCODE_SERVER_USERNAME` (default "opencode"). Env var names are
    /// verbatim from `EffectConfig.string(...)`.
    pub fn from_env() -> Self {
        let password = std::env::var("OPENCODE_SERVER_PASSWORD").ok();
        let username =
            std::env::var("OPENCODE_SERVER_USERNAME").unwrap_or_else(|_| "opencode".to_string());
        Self(Info {
            password: password.filter(|p| !p.is_empty()),
            username,
        })
    }

    pub fn of(info: Info) -> Self {
        Self(info)
    }

    pub fn info(&self) -> &Info {
        &self.0
    }
}

/// Port of `export function required(config: Info)` — true when `password` is `Some` and not `""`.
pub fn required(config: &Info) -> bool {
    match &config.password {
        Some(p) => !p.is_empty(),
        None => false,
    }
}

/// Port of `authorized(credentials, config)` — checks `password.isSome && username === && redacted ===`.
pub fn authorized(credentials: &DecodedCredentials, config: &Info) -> bool {
    match &config.password {
        Some(pw) if !pw.is_empty() => {
            credentials.username == config.username && credentials.password.value() == pw.as_str()
        }
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Base64 (standard alphabet, no url-safe) — mirrors `Buffer.from(...).toString("base64")`.
// Implemented locally to avoid adding a crate (keeps deps = serde only, matching
// codemode/protocol precedent).
// ---------------------------------------------------------------------------
const B64_ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn base64_encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    let mut i = 0;
    while i < bytes.len() {
        let b0 = bytes[i] as u32;
        let b1 = if i + 1 < bytes.len() {
            bytes[i + 1] as u32
        } else {
            0
        };
        let b2 = if i + 2 < bytes.len() {
            bytes[i + 2] as u32
        } else {
            0
        };
        let triple = (b0 << 16) | (b1 << 8) | b2;
        out.push(B64_ALPHABET[((triple >> 18) & 63) as usize] as char);
        out.push(B64_ALPHABET[((triple >> 12) & 63) as usize] as char);
        out.push(if i + 1 < bytes.len() {
            B64_ALPHABET[((triple >> 6) & 63) as usize] as char
        } else {
            '='
        });
        out.push(if i + 2 < bytes.len() {
            B64_ALPHABET[(triple & 63) as usize] as char
        } else {
            '='
        });
        i += 3;
    }
    out
}

/// Port of `header(credentials?)` — returns `Basic <base64 username:password>` or `undefined` when no password.
///
/// Username defaults to `process.env.OPENCODE_SERVER_USERNAME ?? "opencode"`; password
/// defaults to `process.env.OPENCODE_SERVER_PASSWORD` (same fallbacks as TS, via `credentials?.username ?? env ?? "opencode"`).
pub fn header(credentials: Option<&Credentials>) -> Option<String> {
    let password = credentials
        .and_then(|c| c.password.clone())
        .or_else(|| std::env::var("OPENCODE_SERVER_PASSWORD").ok())
        .filter(|p| !p.is_empty());
    let password = password?;
    let username = credentials
        .and_then(|c| c.username.clone())
        .or_else(|| std::env::var("OPENCODE_SERVER_USERNAME").ok())
        .unwrap_or_else(|| "opencode".to_string());
    let raw = format!("{username}:{password}");
    Some(format!("Basic {}", base64_encode(raw.as_bytes())))
}

/// Port of `headers(credentials?) => { Authorization } | undefined`.
///
/// Returns `None` (JS `undefined`) when `header()` is `None`.
pub fn headers(
    credentials: Option<&Credentials>,
) -> Option<std::collections::HashMap<String, String>> {
    let authorization = header(credentials)?;
    let mut m = std::collections::HashMap::new();
    m.insert("Authorization".to_string(), authorization);
    Some(m)
}

/// Re-export namespace alias `export * as ServerAuth from "./auth"` — mirrors the self-export pattern
/// (so callers can `use server::auth::ServerAuth::...` as in TS `ServerAuth.Config` / `ServerAuth.required`).
pub mod ServerAuth {
    pub use super::{
        authorized, header, headers, required, Config, Credentials, DecodedCredentials, Info,
        Redacted, CONFIG_SERVICE_ID,
    };
}
