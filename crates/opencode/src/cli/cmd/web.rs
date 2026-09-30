// source: src/cli/cmd/web.ts — exports: [WebCommand]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/flag/flag`
/// verbatim strings (source order, quoted for V2 audit):
/// - "172."
/// - "start opencode server and open web interface"
/// - "Cli.web"
/// - "../../server/server"
/// - "!  OPENCODE_SERVER_PASSWORD is not set; server is unsecured."
/// - "0.0.0.0"
/// - "  Local access:      "
/// - "  Network access:    "
/// source: `export const WebCommand` — shape as JSON value; CI verifies.
pub type WebCommand = serde_json::Value;
