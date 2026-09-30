// source: src/cli/cmd/generate.ts — exports: [GenerateCommand]
// PROVISIONAL pending external `yargs` (host-provided; no new dep)
// PROVISIONAL pending `@opencode-ai/sdk` — createOpencodeClient pending crates/sdk
// PROVISIONAL pending `prettier` — format via prettier pending host

/// source: command "generate" — verbatim.
pub const COMMAND: &str = "generate";
/// source: import "../../server/server" — verbatim.
pub const SERVER_IMPORT: &str = "../../server/server";
/// source: x-codeSamples lang "js" — verbatim.
pub const X_CODE_SAMPLES: &str = "x-codeSamples";
/// source: prettier imports — verbatim.
pub const PRETTIER: &str = "prettier";
pub const PRETTIER_BABEL: &str = "prettier/plugins/babel";
pub const PRETTIER_ESTREE: &str = "prettier/plugins/estree";
/// source: join "\n" — verbatim.
pub const JOIN_SEP: &str = "\n";
/// source: code sample template — verbatim lines.
pub const CODE_SAMPLE_LINE0: &str = "import { createOpencodeClient } from \"@opencode-ai/sdk\"";
pub const CODE_SAMPLE_LINE2: &str = "const client = createOpencodeClient()";
/// source: `await client.${operation.operationId}({` — verbatim template.
pub const CODE_SAMPLE_TEMPLATE: &str = "await client.${operation.operationId}({";

/// source: `export const GenerateCommand` — shape as JSON value; CI verifies.
pub type GenerateCommand = serde_json::Value;
