// source: src/cli/cmd/run/footer.question.tsx — exports: [RunQuestionBody]
// .tsx markup → data: component/element tree recorded as data below; no JSX runtime.
// PROVISIONAL pending crates/sdk: `@opencode-ai/sdk/v2`
/// verbatim strings (source order, quoted for V2 audit):
/// - "question"
/// - " || event.name === "
/// - ") ?? "
/// - "(not answered)"
/// - " (select all that apply)"
/// - "Type your own answer"
/// - "flex-start"
/// - "space-between"
/// source: `export function RunQuestionBody` — stub shell; CI verifies behavior.
pub fn RunQuestionBody(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
/// source: .tsx component tree — markup recorded as data (no JSX runtime).
pub const MARKUP_FOOTER_QUESTION: &str = "<tsx src=\"cli/cmd/run/footer.question.tsx\" />";
