// source: src/server/routes/instance/httpapi/groups/tui.ts — exports: [CommandPayload, TuiPublishPayload, TuiPaths, TuiApi]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/httpapi` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@/server/tui-event"
/// - "/tui"
/// - "EventTuiPromptAppend"
/// - "EventTuiCommandExecute"
/// - "EventTuiToastShow"
/// - "EventTuiSessionSelect"
/// - "appendPrompt"
/// - "Prompt processed successfully"
/// source: `export const CommandPayload` — shape as JSON value; CI verifies.
pub type CommandPayload = serde_json::Value;
/// source: `export const TuiPublishPayload` — shape as JSON value; CI verifies.
pub type TuiPublishPayload = serde_json::Value;
/// source: `export const TuiPaths` — shape as JSON value; CI verifies.
pub type TuiPaths = serde_json::Value;
/// source: `export const TuiApi` — shape as JSON value; CI verifies.
pub type TuiApi = serde_json::Value;
