//! Rust port of `src/main/constants.ts` (opencode v1.18.30).
//!
//! `CHANNEL` is evaluated from `import.meta.env.OPENCODE_CHANNEL` at module
//! load in the source; here it is read from the `OPENCODE_CHANNEL`
//! environment variable with the same `"dev"` fallback.
//! `UPDATER_ENABLED` additionally needs Electron's `app.isPackaged`, which
//! has no in-workspace binding — the boolean decision is ported as
//! [`updater_enabled`], and the `app.isPackaged` read is PROVISIONAL.
//!
//! Original file: `packages/desktop/src/main/constants.ts`

/// Mirrors `type Channel = "dev" | "beta" | "prod"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    Dev,
    Beta,
    Prod,
}

impl Channel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Channel::Dev => "dev",
            Channel::Beta => "beta",
            Channel::Prod => "prod",
        }
    }
}

/// Mirrors the `CHANNEL` initializer ternary.
pub fn parse_channel(raw: &str) -> Channel {
    match raw {
        "beta" => Channel::Beta,
        "prod" => Channel::Prod,
        _ => Channel::Dev,
    }
}

/// Mirrors `export const CHANNEL`.
pub fn channel() -> Channel {
    parse_channel(&std::env::var("OPENCODE_CHANNEL").unwrap_or_default())
}

/// Mirrors the `UPDATER_ENABLED` boolean decision
/// (`app.isPackaged && CHANNEL !== "dev"`).
pub fn updater_enabled(is_packaged: bool, channel: Channel) -> bool {
    is_packaged && channel != Channel::Dev
}

// PROVISIONAL(packages/desktop/src/main/constants.ts): `app.isPackaged` has
// no in-workspace Electron binding. Call `updater_enabled(is_packaged,
// channel())` once the runtime provides the packaged flag.
pub fn updater_enabled_runtime() -> bool {
    unimplemented!("Electron app.isPackaged binding")
}
