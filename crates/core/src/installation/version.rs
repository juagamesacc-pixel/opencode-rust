//! Rust port of `packages/core/src/installation/version.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.

pub fn installation_version() -> String {
    option_env!("OPENCODE_VERSION")
        .map(|s| s.to_string())
        .or_else(|| std::env::var("OPENCODE_VERSION").ok())
        .unwrap_or_else(|| "local".to_string())
}

pub fn installation_channel() -> String {
    option_env!("OPENCODE_CHANNEL")
        .map(|s| s.to_string())
        .or_else(|| std::env::var("OPENCODE_CHANNEL").ok())
        .unwrap_or_else(|| "local".to_string())
}

pub fn installation_local() -> bool {
    installation_channel() == "local"
}

pub const VERSION_FALLBACK: &str = "local";
pub const CHANNEL_FALLBACK: &str = "local";
