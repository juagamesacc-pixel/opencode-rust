// source: src/cli/cmd/run/demo.ts — exports: [demo, createRunDemo]
// PROVISIONAL pending crates/sdk: `@opencode-ai/sdk/v2`
/// verbatim strings (source order, quoted for V2 audit):
/// - "markdown"
/// - "reasoning"
/// - "question"
/// - "external"
/// - "checklist"
/// - "# Direct Mode Demo"
/// - "This is a realistic assistant response for direct-mode formatting checks."
/// - "It mixes **bold**, _italic_, `inline code`, links, code fences, and tables in one streamed reply."
/// source: `demo = 42` — verbatim.
pub const demo: i64 = 42;
/// source: `export function createRunDemo` — stub shell; CI verifies behavior.
pub fn createRunDemo(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
