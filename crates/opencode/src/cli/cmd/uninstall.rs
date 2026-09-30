// source: src/cli/cmd/uninstall.ts — exports: [UninstallCommand]
// PROVISIONAL pending external `yargs` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/global`
/// verbatim strings (source order, quoted for V2 audit):
/// - "uninstall"
/// - "uninstall opencode and remove all related files"
/// - "keep-config"
/// - "keep configuration files"
/// - "keep-data"
/// - "keep session data and snapshots"
/// - "dry-run"
/// - "show what would be removed without removing"
/// source: `export const UninstallCommand` — shape as JSON value; CI verifies.
pub type UninstallCommand = serde_json::Value;
