// source: src/cli/cmd/account.ts — exports: [defaultConsoleUrl, formatAccountLabel, formatOrgLine, LoginCommand, LogoutCommand, SwitchCommand, OrgsCommand, OpenCommand, ConsoleCommand]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "./cmd"
/// - " (active)"
/// - "https://opencode.ai/console"
/// - " + UI.Style.TEXT_NORMAL : "
/// - "Log in"
/// - "Go to: "
/// - "Enter code: "
/// - "Waiting for authorization..."
/// source: `defaultConsoleUrl = "https://opencode.ai/console"` — verbatim.
pub const defaultConsoleUrl: &str = "https://opencode.ai/console";
/// source: `export const formatAccountLabel` — shape as JSON value; CI verifies.
pub type formatAccountLabel = serde_json::Value;
/// source: `export const formatOrgLine` — shape as JSON value; CI verifies.
pub type formatOrgLine = serde_json::Value;
/// source: `export const LoginCommand` — shape as JSON value; CI verifies.
pub type LoginCommand = serde_json::Value;
/// source: `export const LogoutCommand` — shape as JSON value; CI verifies.
pub type LogoutCommand = serde_json::Value;
/// source: `export const SwitchCommand` — shape as JSON value; CI verifies.
pub type SwitchCommand = serde_json::Value;
/// source: `export const OrgsCommand` — shape as JSON value; CI verifies.
pub type OrgsCommand = serde_json::Value;
/// source: `export const OpenCommand` — shape as JSON value; CI verifies.
pub type OpenCommand = serde_json::Value;
/// source: `export const ConsoleCommand` — shape as JSON value; CI verifies.
pub type ConsoleCommand = serde_json::Value;
