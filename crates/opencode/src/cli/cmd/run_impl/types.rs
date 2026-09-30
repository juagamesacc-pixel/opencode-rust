// source: src/cli/cmd/run/types.ts — exports: [RunFilePart, RunPromptPart, RunCommand, RunProvider, RunPrompt, FooterQueuedPrompt, RunAgent, RunResource, RunInput, EntryKind, FooterPhase, FooterState, FooterPatch, RunDiffStyle, TurnSummary, ScrollbackOptions, ToolCodeSnapshot, ToolDiffSnapshot, ToolTaskSnapshot, ToolTodoSnapshot, ToolQuestionSnapshot, ToolSnapshot, EntryLayout, RunEntryBody (+18 more)]
// PROVISIONAL pending crates/sdk: `@opencode-ai/sdk/v2`
// PROVISIONAL pending crates/tui: `@opencode-ai/tui/config`
/// verbatim strings (source order, quoted for V2 audit):
/// - "provider"
/// - "experimental"
/// - "resource"
/// - "assistant"
/// - "reasoning"
/// - "question"
/// - "markdown"
/// - "structured"
/// source: `export type RunFilePart` — shape as JSON value; CI verifies.
pub type RunFilePart = serde_json::Value;
/// source: `export type RunPromptPart` — shape as JSON value; CI verifies.
pub type RunPromptPart = serde_json::Value;
/// source: `export type RunCommand` — shape as JSON value; CI verifies.
pub type RunCommand = serde_json::Value;
/// source: `export type RunProvider` — shape as JSON value; CI verifies.
pub type RunProvider = serde_json::Value;
/// source: `export type RunPrompt` — shape as JSON value; CI verifies.
pub type RunPrompt = serde_json::Value;
/// source: `export type FooterQueuedPrompt` — shape as JSON value; CI verifies.
pub type FooterQueuedPrompt = serde_json::Value;
/// source: `export type RunAgent` — shape as JSON value; CI verifies.
pub type RunAgent = serde_json::Value;
/// source: `export type RunResource` — shape as JSON value; CI verifies.
pub type RunResource = serde_json::Value;
/// source: `export type RunInput` — shape as JSON value; CI verifies.
pub type RunInput = serde_json::Value;
/// source: `export type EntryKind` — shape as JSON value; CI verifies.
pub type EntryKind = serde_json::Value;
/// source: `export type FooterPhase` — shape as JSON value; CI verifies.
pub type FooterPhase = serde_json::Value;
/// source: `export type FooterState` — shape as JSON value; CI verifies.
pub type FooterState = serde_json::Value;
/// source: `export type FooterPatch` — shape as JSON value; CI verifies.
pub type FooterPatch = serde_json::Value;
/// source: `export type RunDiffStyle` — shape as JSON value; CI verifies.
pub type RunDiffStyle = serde_json::Value;
/// source: `export type TurnSummary` — shape as JSON value; CI verifies.
pub type TurnSummary = serde_json::Value;
/// source: `export type ScrollbackOptions` — shape as JSON value; CI verifies.
pub type ScrollbackOptions = serde_json::Value;
/// source: `export type ToolCodeSnapshot` — shape as JSON value; CI verifies.
pub type ToolCodeSnapshot = serde_json::Value;
/// source: `export type ToolDiffSnapshot` — shape as JSON value; CI verifies.
pub type ToolDiffSnapshot = serde_json::Value;
/// source: `export type ToolTaskSnapshot` — shape as JSON value; CI verifies.
pub type ToolTaskSnapshot = serde_json::Value;
/// source: `export type ToolTodoSnapshot` — shape as JSON value; CI verifies.
pub type ToolTodoSnapshot = serde_json::Value;
/// source: `export type ToolQuestionSnapshot` — shape as JSON value; CI verifies.
pub type ToolQuestionSnapshot = serde_json::Value;
/// source: `export type ToolSnapshot` — shape as JSON value; CI verifies.
pub type ToolSnapshot = serde_json::Value;
/// source: `export type EntryLayout` — shape as JSON value; CI verifies.
pub type EntryLayout = serde_json::Value;
/// source: `export type RunEntryBody` — shape as JSON value; CI verifies.
pub type RunEntryBody = serde_json::Value;
/// source: `export type FooterView` — shape as JSON value; CI verifies.
pub type FooterView = serde_json::Value;
/// source: `export type FooterPromptRoute` — shape as JSON value; CI verifies.
pub type FooterPromptRoute = serde_json::Value;
/// source: `export type FooterSubagentTab` — shape as JSON value; CI verifies.
pub type FooterSubagentTab = serde_json::Value;
/// source: `export type FooterSubagentDetail` — shape as JSON value; CI verifies.
pub type FooterSubagentDetail = serde_json::Value;
/// source: `export type FooterSubagentState` — shape as JSON value; CI verifies.
pub type FooterSubagentState = serde_json::Value;
/// source: `export type FooterOutput` — shape as JSON value; CI verifies.
pub type FooterOutput = serde_json::Value;
/// source: `export type FooterEvent` — shape as JSON value; CI verifies.
pub type FooterEvent = serde_json::Value;
/// source: `export type PermissionReply` — shape as JSON value; CI verifies.
pub type PermissionReply = serde_json::Value;
/// source: `export type QuestionReply` — shape as JSON value; CI verifies.
pub type QuestionReply = serde_json::Value;
/// source: `export type QuestionReject` — shape as JSON value; CI verifies.
pub type QuestionReject = serde_json::Value;
/// source: `export type RunTuiConfig` — shape as JSON value; CI verifies.
pub type RunTuiConfig = serde_json::Value;
/// source: `export type StreamPhase` — shape as JSON value; CI verifies.
pub type StreamPhase = serde_json::Value;
/// source: `export type StreamSource` — shape as JSON value; CI verifies.
pub type StreamSource = serde_json::Value;
/// source: `export type StreamToolState` — shape as JSON value; CI verifies.
pub type StreamToolState = serde_json::Value;
/// source: `export type StreamCommit` — shape as JSON value; CI verifies.
pub type StreamCommit = serde_json::Value;
/// source: `export type LocalReplayAnchor` — shape as JSON value; CI verifies.
pub type LocalReplayAnchor = serde_json::Value;
/// source: `export type LocalReplayRow` — shape as JSON value; CI verifies.
pub type LocalReplayRow = serde_json::Value;
/// source: `export type FooterApi` — shape as JSON value; CI verifies.
pub type FooterApi = serde_json::Value;
