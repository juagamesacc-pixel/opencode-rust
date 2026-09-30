// source: src/cli/cmd/models.ts — exports: [ModelsCommand]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/models-dev`
// PROVISIONAL pending crates/core: `@opencode-ai/core/provider`
/// verbatim strings (source order, quoted for V2 audit):
/// - "models [provider]"
/// - "list all available models"
/// - "provider"
/// - "provider ID to filter models by"
/// - "use more verbose model output (includes metadata like costs)"
/// - "refresh the models cache from models.dev"
/// - "Cli.models"
/// - "@/provider/provider"
/// source: `export const ModelsCommand` — shape as JSON value; CI verifies.
pub type ModelsCommand = serde_json::Value;
