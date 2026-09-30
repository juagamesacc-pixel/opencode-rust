//! 1:1 port of packages/schema/src/session-message.ts
#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// NOTE(target uncertain, plan §4): crate paths below follow existing session.rs/event.rs patterns; verify crate::session_id::SessionID, crate::model::Ref, crate::prompt::{Prompt, FileAttachment}, crate::llm::{ProviderMetadata, ToolContent}, crate::schema_primitives::{DateTimeUtcFromMillis, RelativePath}.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ID(pub String);
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnknownError {
    #[serde(rename = "type")]
    pub r#type: String,
    pub message: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseTime {
    pub created: crate::schema_primitives::DateTimeUtcFromMillis,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentSwitched {
    pub id: ID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    pub time: BaseTime,
    pub agent: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelSwitched {
    pub id: ID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    pub time: BaseTime,
    pub model: crate::model::Ref,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: ID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    pub time: BaseTime,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<crate::prompt::FileAttachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agents: Option<Vec<String>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Synthetic {
    pub id: ID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    pub time: BaseTime,
    pub sessionID: crate::session_id::SessionID,
    pub text: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct System {
    pub id: ID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    pub time: BaseTime,
    pub text: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShellTime {
    pub created: crate::schema_primitives::DateTimeUtcFromMillis,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<crate::schema_primitives::DateTimeUtcFromMillis>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Shell {
    pub id: ID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    pub time: ShellTime,
    pub callID: String,
    pub command: String,
    pub output: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolStatePending {
    pub input: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolStateRunning {
    pub input: HashMap<String, serde_json::Value>,
    pub structured: HashMap<String, serde_json::Value>,
    pub content: Vec<crate::llm::ToolContent>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolStateCompleted {
    pub input: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<crate::prompt::FileAttachment>>,
    pub content: Vec<crate::llm::ToolContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputPaths: Option<Vec<String>>,
    pub structured: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolStateError {
    pub input: HashMap<String, serde_json::Value>,
    pub content: Vec<crate::llm::ToolContent>,
    pub structured: HashMap<String, serde_json::Value>,
    pub error: UnknownError,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum ToolState {
    #[serde(rename = "pending")]
    Pending(ToolStatePending),
    #[serde(rename = "running")]
    Running(ToolStateRunning),
    #[serde(rename = "completed")]
    Completed(ToolStateCompleted),
    #[serde(rename = "error")]
    Error(ToolStateError),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProviderDetail {
    pub executed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::llm::ProviderMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resultMetadata: Option<crate::llm::ProviderMetadata>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolTime {
    pub created: crate::schema_primitives::DateTimeUtcFromMillis,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ran: Option<crate::schema_primitives::DateTimeUtcFromMillis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<crate::schema_primitives::DateTimeUtcFromMillis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pruned: Option<crate::schema_primitives::DateTimeUtcFromMillis>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssistantTool {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub state: ToolState,
    pub time: ToolTime,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssistantText {
    pub id: String,
    pub text: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssistantReasoning {
    pub id: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub providerMetadata: Option<crate::llm::ProviderMetadata>,
    pub time: ShellTime,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AssistantContent {
    #[serde(rename = "text")]
    Text(AssistantText),
    #[serde(rename = "reasoning")]
    Reasoning(AssistantReasoning),
    #[serde(rename = "tool")]
    Tool(AssistantTool),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssistantSnapshot {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<crate::schema_primitives::RelativePath>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssistantTokensCache {
    pub read: f64,
    pub write: f64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssistantTokens {
    pub input: f64,
    pub output: f64,
    pub reasoning: f64,
    pub cache: AssistantTokensCache,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssistantTime {
    pub created: crate::schema_primitives::DateTimeUtcFromMillis,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<crate::schema_primitives::DateTimeUtcFromMillis>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Assistant {
    pub id: ID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    pub time: AssistantTime,
    pub agent: String,
    pub model: crate::model::Ref,
    pub content: Vec<AssistantContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<AssistantSnapshot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<f64>,
    pub tokens: AssistantTokens,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<UnknownError>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Compaction {
    pub reason: CompactionReason,
    pub summary: String,
    pub recent: String,
    pub id: ID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    pub time: BaseTime,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CompactionReason {
    Auto,
    Manual,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Message {
    #[serde(rename = "agent-switched")]
    AgentSwitched(AgentSwitched),
    #[serde(rename = "model-switched")]
    ModelSwitched(ModelSwitched),
    #[serde(rename = "user")]
    User(User),
    #[serde(rename = "synthetic")]
    Synthetic(Synthetic),
    #[serde(rename = "system")]
    System(System),
    #[serde(rename = "shell")]
    Shell(Shell),
    #[serde(rename = "assistant")]
    Assistant(Assistant),
    #[serde(rename = "compaction")]
    Compaction(Compaction),
}
pub type Type = String;
