//! Port of `packages/schema/src/v1/session.ts` (`SessionV1`).
//!
//! Source exports: `MessageID` (brand `MessageID`, loose `msg` prefix),
//! `PartID` (brand `PartID`, loose `prt` prefix), 7 `namedError` errors
//! (`OutputLengthError`, `AuthError`, `AbortedError`, `StructuredOutputError`,
//! `APIError`, `ContextOverflowError`, `ContentFilterError` with exact inner
//! `name` strings), `OutputFormatText`, `OutputFormatJsonSchema`, `Format`
//! (+ `OutputFormat` alias), all part variants + `Part` union, `Assistant`
//! error union, `TextPartInput`/`FilePartInput`/`AgentPartInput`/
//! `SubtaskPartInput`, `User`, `Assistant`, `Info`, `WithParts`, `SessionInfo`,
//! `PartDelta`, `Diff`, `Error`, `Event` (durable `sessionID` v1 for the first
//! 7 entries).
//!
//! Conventions: `{name, data}` error shape is one struct per error with a
//! verbatim `NAME` const (TS `namedError` returns `{Schema, EffectSchema}` of
//! the same shape — both map to the struct itself). Tagged unions are untagged
//! over member structs (each carries its own verbatim discriminator key) in
//! source order. `Any` maps to `serde_json::Value`, `Int` to `i64`, `Finite`
//! to `f64`. Source-private `SessionSummary`/`SessionTokens`/`SessionShare`/
//! `SessionRevert`/`SessionModel` stay module-private.
//!
//! Cross-lane: `SessionID` (`crate::session_id`), `WorkspaceID`
//! (`crate::workspace_id`), `Provider.ID` (`crate::provider`, Lane A),
//! `Model.ID` (`crate::model`, Lane A), `Project.ID` (`crate::project`,
//! Lane B), `FileDiff.Info` (`crate::file_diff`, this lane),
//! `PermissionV1.Ruleset` (`super::permission`, this lane),
//! `crate::identifier::ascending` (Lane A).

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Branded `MessageID` (loose `msg` prefix check, canonical `msg_…`).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct MessageID(pub String);

impl MessageID {
    /// Directional constructor preserved from source statics.
    pub fn ascending(id: Option<&str>) -> Self {
        match id {
            Some(value) => Self(value.to_string()),
            None => Self(format!("msg_{}", crate::identifier::ascending())),
        }
    }
}

impl<'de> Deserialize<'de> for MessageID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        if value.starts_with("msg") {
            Ok(Self(value))
        } else {
            Err(serde::de::Error::custom(format!(
                "MessageID must start with \"msg\": {value}"
            )))
        }
    }
}

impl std::fmt::Display for MessageID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for MessageID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Branded `PartID` (loose `prt` prefix check, canonical `prt_…`).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct PartID(pub String);

impl PartID {
    /// Directional constructor preserved from source statics.
    pub fn ascending(id: Option<&str>) -> Self {
        match id {
            Some(value) => Self(value.to_string()),
            None => Self(format!("prt_{}", crate::identifier::ascending())),
        }
    }
}

impl<'de> Deserialize<'de> for PartID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        if value.starts_with("prt") {
            Ok(Self(value))
        } else {
            Err(serde::de::Error::custom(format!(
                "PartID must start with \"prt\": {value}"
            )))
        }
    }
}

impl std::fmt::Display for PartID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for PartID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// `namedError("MessageOutputLengthError", {})`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputLengthError {
    pub name: String,
    pub data: OutputLengthErrorData,
}

impl OutputLengthError {
    /// Verbatim inner error name.
    pub const NAME: &'static str = "MessageOutputLengthError";
}

/// Data of `MessageOutputLengthError` (empty in source).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputLengthErrorData {}

/// `namedError("ProviderAuthError", { providerID, message })`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthError {
    pub name: String,
    pub data: AuthErrorData,
}

impl AuthError {
    /// Verbatim inner error name.
    pub const NAME: &'static str = "ProviderAuthError";
}

/// Data of `ProviderAuthError`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthErrorData {
    pub providerID: String,
    pub message: String,
}

/// `namedError("MessageAbortedError", { message })`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AbortedError {
    pub name: String,
    pub data: AbortedErrorData,
}

impl AbortedError {
    /// Verbatim inner error name.
    pub const NAME: &'static str = "MessageAbortedError";
}

/// Data of `MessageAbortedError`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AbortedErrorData {
    pub message: String,
}

/// `namedError("StructuredOutputError", { message, retries })`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructuredOutputError {
    pub name: String,
    pub data: StructuredOutputErrorData,
}

impl StructuredOutputError {
    /// Verbatim inner error name.
    pub const NAME: &'static str = "StructuredOutputError";
}

/// Data of `StructuredOutputError`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructuredOutputErrorData {
    pub message: String,
    pub retries: crate::schema_primitives::NonNegativeInt,
}

/// `namedError("APIError", { … })`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct APIError {
    pub name: String,
    pub data: APIErrorData,
}

impl APIError {
    /// Verbatim inner error name.
    pub const NAME: &'static str = "APIError";
}

/// Data of `APIError`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct APIErrorData {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statusCode: Option<crate::schema_primitives::NonNegativeInt>,
    pub isRetryable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responseHeaders: Option<BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responseBody: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, String>>,
}

/// `namedError("ContextOverflowError", { message, responseBody? })`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContextOverflowError {
    pub name: String,
    pub data: ContextOverflowErrorData,
}

impl ContextOverflowError {
    /// Verbatim inner error name.
    pub const NAME: &'static str = "ContextOverflowError";
}

/// Data of `ContextOverflowError`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContextOverflowErrorData {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responseBody: Option<String>,
}

/// `namedError("ContentFilterError", { message })`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentFilterError {
    pub name: String,
    pub data: ContentFilterErrorData,
}

impl ContentFilterError {
    /// Verbatim inner error name.
    pub const NAME: &'static str = "ContentFilterError";
}

/// Data of `ContentFilterError`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentFilterErrorData {
    pub message: String,
}

fn default_retry_count() -> crate::schema_primitives::NonNegativeInt {
    2
}

/// `OutputFormatText`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputFormatText {
    #[serde(rename = "type")]
    pub r#type: String,
}

impl OutputFormatText {
    /// Verbatim discriminator value.
    pub const TYPE: &'static str = "text";
}

/// `OutputFormatJsonSchema` (`retryCount` decodes to `2` when absent).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputFormatJsonSchema {
    #[serde(rename = "type")]
    pub r#type: String,
    pub schema: BTreeMap<String, serde_json::Value>,
    #[serde(default = "default_retry_count")]
    pub retryCount: crate::schema_primitives::NonNegativeInt,
}

impl OutputFormatJsonSchema {
    /// Verbatim discriminator value.
    pub const TYPE: &'static str = "json_schema";
}

/// `OutputFormat` union (`Format` in source, discriminator `type`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Format {
    Text(OutputFormatText),
    JsonSchema(OutputFormatJsonSchema),
}

/// `OutputFormat` alias from source.
pub type OutputFormat = Format;

/// `SnapshotPart`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotPart {
    pub id: PartID,
    pub sessionID: crate::session_id::SessionID,
    pub messageID: MessageID,
    #[serde(rename = "type")]
    pub r#type: String,
    pub snapshot: String,
}

/// `PatchPart`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchPart {
    pub id: PartID,
    pub sessionID: crate::session_id::SessionID,
    pub messageID: MessageID,
    #[serde(rename = "type")]
    pub r#type: String,
    pub hash: String,
    pub files: Vec<String>,
}

/// `TextPart` time block.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextPartTime {
    pub start: crate::schema_primitives::NonNegativeInt,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<crate::schema_primitives::NonNegativeInt>,
}

/// `TextPart`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextPart {
    pub id: PartID,
    pub sessionID: crate::session_id::SessionID,
    pub messageID: MessageID,
    #[serde(rename = "type")]
    pub r#type: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synthetic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignored: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<TextPartTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, serde_json::Value>>,
}

/// `ReasoningPart` time block.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReasoningPartTime {
    pub start: crate::schema_primitives::NonNegativeInt,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<crate::schema_primitives::NonNegativeInt>,
}

/// `ReasoningPart`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReasoningPart {
    pub id: PartID,
    pub sessionID: crate::session_id::SessionID,
    pub messageID: MessageID,
    #[serde(rename = "type")]
    pub r#type: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, serde_json::Value>>,
    pub time: ReasoningPartTime,
}

/// `FilePartSourceText` (shared `filePartSourceBase.text`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FilePartSourceText {
    pub value: String,
    pub start: f64,
    pub end: f64,
}

/// `Range` endpoint.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RangeEndpoint {
    pub line: crate::schema_primitives::NonNegativeInt,
    pub character: crate::schema_primitives::NonNegativeInt,
}

/// `Range`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Range {
    pub start: RangeEndpoint,
    pub end: RangeEndpoint,
}

/// `FileSource`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileSource {
    pub text: FilePartSourceText,
    #[serde(rename = "type")]
    pub r#type: String,
    pub path: String,
}

/// `SymbolSource`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SymbolSource {
    pub text: FilePartSourceText,
    #[serde(rename = "type")]
    pub r#type: String,
    pub path: String,
    pub range: Range,
    pub name: String,
    pub kind: crate::schema_primitives::NonNegativeInt,
}

/// `ResourceSource`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSource {
    pub text: FilePartSourceText,
    #[serde(rename = "type")]
    pub r#type: String,
    pub clientName: String,
    pub uri: String,
}

/// `FilePartSource` union (discriminator `type`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FilePartSource {
    File(FileSource),
    Symbol(SymbolSource),
    Resource(ResourceSource),
}

/// `FilePart`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FilePart {
    pub id: PartID,
    pub sessionID: crate::session_id::SessionID,
    pub messageID: MessageID,
    #[serde(rename = "type")]
    pub r#type: String,
    pub mime: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<FilePartSource>,
}

/// `AgentPart` source block.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentPartSource {
    pub value: String,
    pub start: crate::schema_primitives::NonNegativeInt,
    pub end: crate::schema_primitives::NonNegativeInt,
}

/// `AgentPart`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentPart {
    pub id: PartID,
    pub sessionID: crate::session_id::SessionID,
    pub messageID: MessageID,
    #[serde(rename = "type")]
    pub r#type: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<AgentPartSource>,
}

/// `CompactionPart` (`tail_start_id` keeps its verbatim snake_case key).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CompactionPart {
    pub id: PartID,
    pub sessionID: crate::session_id::SessionID,
    pub messageID: MessageID,
    #[serde(rename = "type")]
    pub r#type: String,
    pub auto: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overflow: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tail_start_id: Option<MessageID>,
}

/// `SubtaskPart` model block.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubtaskPartModel {
    pub providerID: crate::provider::ID,
    pub modelID: crate::model::ID,
}

/// `SubtaskPart`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubtaskPart {
    pub id: PartID,
    pub sessionID: crate::session_id::SessionID,
    pub messageID: MessageID,
    #[serde(rename = "type")]
    pub r#type: String,
    pub prompt: String,
    pub description: String,
    pub agent: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<SubtaskPartModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
}

/// `RetryPart` time block.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RetryPartTime {
    pub created: crate::schema_primitives::NonNegativeInt,
}

/// `RetryPart`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RetryPart {
    pub id: PartID,
    pub sessionID: crate::session_id::SessionID,
    pub messageID: MessageID,
    #[serde(rename = "type")]
    pub r#type: String,
    pub attempt: crate::schema_primitives::NonNegativeInt,
    pub error: APIError,
    pub time: RetryPartTime,
}

/// `StepStartPart`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StepStartPart {
    pub id: PartID,
    pub sessionID: crate::session_id::SessionID,
    pub messageID: MessageID,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<String>,
}

/// `StepFinishPart` tokens block.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StepFinishPartTokens {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    pub input: f64,
    pub output: f64,
    pub reasoning: f64,
    pub cache: StepFinishPartTokensCache,
}

/// `StepFinishPart` tokens cache block.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StepFinishPartTokensCache {
    pub read: f64,
    pub write: f64,
}

/// `StepFinishPart`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StepFinishPart {
    pub id: PartID,
    pub sessionID: crate::session_id::SessionID,
    pub messageID: MessageID,
    #[serde(rename = "type")]
    pub r#type: String,
    pub reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<String>,
    pub cost: f64,
    pub tokens: StepFinishPartTokens,
}

/// `ToolStatePending`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToolStatePending {
    pub status: String,
    pub input: BTreeMap<String, serde_json::Value>,
    pub raw: String,
}

/// `ToolStateRunning` time block.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToolStateRunningTime {
    pub start: crate::schema_primitives::NonNegativeInt,
}

/// `ToolStateRunning`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToolStateRunning {
    pub status: String,
    pub input: BTreeMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, serde_json::Value>>,
    pub time: ToolStateRunningTime,
}

/// `ToolStateCompleted` time block.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToolStateCompletedTime {
    pub start: crate::schema_primitives::NonNegativeInt,
    pub end: crate::schema_primitives::NonNegativeInt,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compacted: Option<crate::schema_primitives::NonNegativeInt>,
}

/// `ToolStateCompleted`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToolStateCompleted {
    pub status: String,
    pub input: BTreeMap<String, serde_json::Value>,
    pub output: String,
    pub title: String,
    pub metadata: BTreeMap<String, serde_json::Value>,
    pub time: ToolStateCompletedTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<FilePart>>,
}

/// `ToolStateError` time block.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToolStateErrorTime {
    pub start: crate::schema_primitives::NonNegativeInt,
    pub end: crate::schema_primitives::NonNegativeInt,
}

/// `ToolStateError`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToolStateError {
    pub status: String,
    pub input: BTreeMap<String, serde_json::Value>,
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, serde_json::Value>>,
    pub time: ToolStateErrorTime,
}

/// `ToolState` union (discriminator `status`, source order).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolState {
    Pending(ToolStatePending),
    Running(ToolStateRunning),
    Completed(ToolStateCompleted),
    Error(ToolStateError),
}

/// `ToolPart`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToolPart {
    pub id: PartID,
    pub sessionID: crate::session_id::SessionID,
    pub messageID: MessageID,
    #[serde(rename = "type")]
    pub r#type: String,
    pub callID: String,
    pub tool: String,
    pub state: ToolState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, serde_json::Value>>,
}

/// `User` message summary block.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    pub diffs: Vec<crate::file_diff::Info>,
}

/// `User` message model block.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserModel {
    pub providerID: crate::provider::ID,
    pub modelID: crate::model::ID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}

/// `User` message time block.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserTime {
    pub created: f64,
}

/// `User` message.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: MessageID,
    pub sessionID: crate::session_id::SessionID,
    pub role: String,
    pub time: UserTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<Format>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<UserSummary>,
    pub agent: String,
    pub model: UserModel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<BTreeMap<String, bool>>,
}

/// `Part` union (discriminator `type`, source order).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Part {
    Text(TextPart),
    Subtask(SubtaskPart),
    Reasoning(ReasoningPart),
    File(FilePart),
    Tool(ToolPart),
    StepStart(StepStartPart),
    StepFinish(StepFinishPart),
    Snapshot(SnapshotPart),
    Patch(PatchPart),
    Agent(AgentPart),
    Retry(RetryPart),
    Compaction(CompactionPart),
}

/// Inline `namedError("UnknownError", { message, ref? })` from the assistant
/// error union (source position: inside `AssistantErrorSchema`, after
/// `AuthError`; kept here in verbatim union order).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnknownError {
    pub name: String,
    pub data: UnknownErrorData,
}

impl UnknownError {
    /// Verbatim inner error name.
    pub const NAME: &'static str = "UnknownError";
}

/// Data of `UnknownError`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnknownErrorData {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
}

/// Assistant error union (discriminator `name`, source order).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssistantError {
    Auth(AuthError),
    Unknown(UnknownError),
    OutputLength(OutputLengthError),
    Aborted(AbortedError),
    StructuredOutput(StructuredOutputError),
    ContextOverflow(ContextOverflowError),
    ContentFilter(ContentFilterError),
    Api(APIError),
}

/// `TextPartInput`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextPartInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<PartID>,
    #[serde(rename = "type")]
    pub r#type: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synthetic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignored: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<TextPartTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, serde_json::Value>>,
}

/// `FilePartInput`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FilePartInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<PartID>,
    #[serde(rename = "type")]
    pub r#type: String,
    pub mime: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<FilePartSource>,
}

/// `AgentPartInput`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentPartInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<PartID>,
    #[serde(rename = "type")]
    pub r#type: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<AgentPartSource>,
}

/// `SubtaskPartInput`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubtaskPartInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<PartID>,
    #[serde(rename = "type")]
    pub r#type: String,
    pub prompt: String,
    pub description: String,
    pub agent: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<SubtaskPartModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
}

/// `Assistant` message time block.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssistantTime {
    pub created: crate::schema_primitives::NonNegativeInt,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<crate::schema_primitives::NonNegativeInt>,
}

/// `Assistant` message path block.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssistantPath {
    pub cwd: String,
    pub root: String,
}

/// `Assistant` message tokens block.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssistantTokens {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    pub input: f64,
    pub output: f64,
    pub reasoning: f64,
    pub cache: AssistantTokensCache,
}

/// `Assistant` message tokens cache block.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssistantTokensCache {
    pub read: f64,
    pub write: f64,
}

/// `Assistant` message.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Assistant {
    pub id: MessageID,
    pub sessionID: crate::session_id::SessionID,
    pub role: String,
    pub time: AssistantTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<AssistantError>,
    pub parentID: MessageID,
    pub modelID: crate::model::ID,
    pub providerID: crate::provider::ID,
    pub mode: String,
    pub agent: String,
    pub path: AssistantPath,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<bool>,
    pub cost: f64,
    pub tokens: AssistantTokens,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish: Option<String>,
}

/// `Message` union (`Info` in source, discriminator `role`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Info {
    User(User),
    Assistant(Assistant),
}

/// `WithParts`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WithParts {
    pub info: Info,
    pub parts: Vec<Part>,
}

/// Source-private `SessionSummary` (field order verbatim; public visibility is
/// required for the `SessionInfo` public interface).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionSummary {
    pub additions: f64,
    pub deletions: f64,
    pub files: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diffs: Option<Vec<crate::file_diff::Info>>,
}

/// Source-private `SessionTokens` (field order verbatim; public visibility is
/// required for the `SessionInfo` public interface).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionTokens {
    pub input: f64,
    pub output: f64,
    pub reasoning: f64,
    pub cache: SessionTokensCache,
}

/// Source-private session tokens cache block.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionTokensCache {
    pub read: f64,
    pub write: f64,
}

/// Source-private `SessionShare`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionShare {
    pub url: String,
}

/// Source-private `SessionRevert` (field order verbatim; public visibility is
/// required for the `SessionInfo` public interface).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionRevert {
    pub messageID: MessageID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partID: Option<PartID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff: Option<String>,
}

/// Source-private `SessionModel` (field order verbatim; public visibility is
/// required for the `SessionInfo` public interface).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionModel {
    pub id: crate::model::ID,
    pub providerID: crate::provider::ID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}

/// Source-private session time block.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionTime {
    pub created: crate::schema_primitives::NonNegativeInt,
    pub updated: crate::schema_primitives::NonNegativeInt,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compacting: Option<crate::schema_primitives::NonNegativeInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<f64>,
}

/// `Session` (`SessionInfo` in source).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionInfo {
    pub id: crate::session_id::SessionID,
    pub slug: String,
    pub projectID: crate::project::ID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaceID: Option<crate::workspace_id::WorkspaceID>,
    pub directory: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parentID: Option<crate::session_id::SessionID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<SessionSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens: Option<SessionTokens>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share: Option<SessionShare>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<SessionModel>,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, serde_json::Value>>,
    pub time: SessionTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<super::permission::Ruleset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revert: Option<SessionRevert>,
}

/// Payload of `session.created` / `session.updated` / `session.deleted`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionEventInfoPayload {
    pub sessionID: crate::session_id::SessionID,
    pub info: SessionInfo,
}

/// Payload of `message.updated` (`{sessionID, info}`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageUpdatedPayload {
    pub sessionID: crate::session_id::SessionID,
    pub info: Info,
}

/// Payload of `message.removed` (`{sessionID, messageID}`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageRemovedPayload {
    pub sessionID: crate::session_id::SessionID,
    pub messageID: MessageID,
}

/// Payload of `message.part.updated` (`{sessionID, part, time}`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartUpdatedPayload {
    pub sessionID: crate::session_id::SessionID,
    pub part: Part,
    pub time: f64,
}

/// Payload of `message.part.removed` (`{sessionID, messageID, partID}`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartRemovedPayload {
    pub sessionID: crate::session_id::SessionID,
    pub messageID: MessageID,
    pub partID: PartID,
}

/// Payload of `message.part.delta`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartDeltaPayload {
    pub sessionID: crate::session_id::SessionID,
    pub messageID: MessageID,
    pub partID: PartID,
    pub field: String,
    pub delta: String,
}

/// Payload of `session.diff`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiffPayload {
    pub sessionID: crate::session_id::SessionID,
    pub diff: Vec<crate::file_diff::Info>,
}

/// Payload of `session.error` (`sessionID` optional, `error` optional).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sessionID: Option<crate::session_id::SessionID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<AssistantError>,
}

/// SessionV1 event definitions.
///
/// The first seven entries carry durable `{aggregate: "sessionID", version: 1}`
/// coordinates (source `options`); `PartDelta`, `Diff`, `Error` are transient.
/// `Definitions` order is verbatim from source.
pub mod Event {
    /// `session.created` (durable).
    pub struct Created;
    impl Created {
        pub const TYPE: &'static str = "session.created";
        pub const DURABLE_AGGREGATE: Option<&'static str> = Some("sessionID");
        pub const DURABLE_VERSION: Option<i64> = Some(1);
    }

    /// `session.updated` (durable).
    pub struct Updated;
    impl Updated {
        pub const TYPE: &'static str = "session.updated";
        pub const DURABLE_AGGREGATE: Option<&'static str> = Some("sessionID");
        pub const DURABLE_VERSION: Option<i64> = Some(1);
    }

    /// `session.deleted` (durable).
    pub struct Deleted;
    impl Deleted {
        pub const TYPE: &'static str = "session.deleted";
        pub const DURABLE_AGGREGATE: Option<&'static str> = Some("sessionID");
        pub const DURABLE_VERSION: Option<i64> = Some(1);
    }

    /// `message.updated` (durable).
    pub struct MessageUpdated;
    impl MessageUpdated {
        pub const TYPE: &'static str = "message.updated";
        pub const DURABLE_AGGREGATE: Option<&'static str> = Some("sessionID");
        pub const DURABLE_VERSION: Option<i64> = Some(1);
    }

    /// `message.removed` (durable).
    pub struct MessageRemoved;
    impl MessageRemoved {
        pub const TYPE: &'static str = "message.removed";
        pub const DURABLE_AGGREGATE: Option<&'static str> = Some("sessionID");
        pub const DURABLE_VERSION: Option<i64> = Some(1);
    }

    /// `message.part.updated` (durable).
    pub struct PartUpdated;
    impl PartUpdated {
        pub const TYPE: &'static str = "message.part.updated";
        pub const DURABLE_AGGREGATE: Option<&'static str> = Some("sessionID");
        pub const DURABLE_VERSION: Option<i64> = Some(1);
    }

    /// `message.part.removed` (durable).
    pub struct PartRemoved;
    impl PartRemoved {
        pub const TYPE: &'static str = "message.part.removed";
        pub const DURABLE_AGGREGATE: Option<&'static str> = Some("sessionID");
        pub const DURABLE_VERSION: Option<i64> = Some(1);
    }

    /// `message.part.delta` (transient) — payload [`super::PartDeltaPayload`].
    pub struct PartDelta;
    impl PartDelta {
        pub const TYPE: &'static str = "message.part.delta";
        pub const DURABLE_AGGREGATE: Option<&'static str> = None;
        pub const DURABLE_VERSION: Option<i64> = None;
    }

    /// `session.diff` (transient) — payload [`super::DiffPayload`].
    pub struct Diff;
    impl Diff {
        pub const TYPE: &'static str = "session.diff";
        pub const DURABLE_AGGREGATE: Option<&'static str> = None;
        pub const DURABLE_VERSION: Option<i64> = None;
    }

    /// `session.error` (transient) — payload [`super::ErrorPayload`].
    pub struct Error;
    impl Error {
        pub const TYPE: &'static str = "session.error";
        pub const DURABLE_AGGREGATE: Option<&'static str> = None;
        pub const DURABLE_VERSION: Option<i64> = None;
    }

    /// Verbatim declaration order: `Created`, `Updated`, `Deleted`,
    /// `MessageUpdated`, `MessageRemoved`, `PartUpdated`, `PartRemoved`,
    /// `PartDelta`, `Diff`, `Error`.
    pub const Definitions: &[&'static str] = &[
        Created::TYPE,
        Updated::TYPE,
        Deleted::TYPE,
        MessageUpdated::TYPE,
        MessageRemoved::TYPE,
        PartUpdated::TYPE,
        PartRemoved::TYPE,
        PartDelta::TYPE,
        Diff::TYPE,
        Error::TYPE,
    ];
}

/// `session.diff` definition (also reachable as `Event::Diff`).
pub use Event::Diff;
/// `session.error` definition (also reachable as `Event::Error`).
pub use Event::Error;
/// `message.part.delta` definition (also reachable as `Event::PartDelta`).
pub use Event::PartDelta;
