// source: src/cli/cmd/upgrade.ts — exports: [UpgradeCommand]
// PROVISIONAL pending external `yargs` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/installation/version`
/// verbatim strings (source order, quoted for V2 audit):
/// - "upgrade [target]"
/// - "upgrade opencode to the latest or a specific version"
/// - "version to upgrade to, for ex '0.1.48' or 'v0.1.48'"
/// - "installation method to use"
/// - "Install anyways?"
/// - "Using method: "
/// - "Upgrading..."
/// - "Upgrade failed"
/// source: `export const UpgradeCommand` — shape as JSON value; CI verifies.
pub type UpgradeCommand = serde_json::Value;
