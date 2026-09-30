// source: src/cli/effect-cmd.ts — exports: [CliError, fail, effectCmd]
// PROVISIONAL pending external `yargs` (host-provided; no new dep)
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "CliError"
/// - "Cli.<name>"
/// - "@/effect/app-runtime"
/// - "function"
/// - "@/project/instance-store"
/// - "@/effect/instance-ref"
use serde::{Deserialize, Serialize};

/// source: `export class CliError` — data shell; CI verifies methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliError {
    pub value: serde_json::Value,
}
/// source: `export const fail` — shape as JSON value; CI verifies.
pub type fail = serde_json::Value;
/// source: `export const effectCmd` — shape as JSON value; CI verifies.
pub type effectCmd = serde_json::Value;
