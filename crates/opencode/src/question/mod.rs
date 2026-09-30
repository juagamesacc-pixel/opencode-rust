// source: src/question/index.ts — exports: Option, Info, Prompt, Tool,
// Request, Answer, Reply, Replied, Rejected, Event, RejectedError,
// NotFoundError, Interface, Service, node, Question
// PROVISIONAL pending @opencode-ai/schema/question-v1, @/session/schema,
// @/event-v2-bridge, crates/core layer-node/instance-state: schema shapes
// mirrored with verbatim field names; ask/reply/reject/list state machine
// (pending map keyed by request id, finalizer rejects all) preserved.

pub mod schema;

use serde::{Deserialize, Serialize};

use crate::question::schema::QuestionID;

/// source: Option (QuestionV1.Option) — PROVISIONAL pending schema crate.
// keep source name Option for custom struct; use ::std::option::Option for generic wire types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Option {
    pub label: String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<String>,
}

/// source: Info (QuestionV1.Info) — PROVISIONAL pending schema crate.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub question: String,
    pub header: String,
    pub options: Vec<Option>,
}

/// source: Prompt (QuestionV1.Prompt) — PROVISIONAL pending schema crate.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Prompt {
    pub info: Info,
}

/// source: Tool (QuestionV1.Tool) — PROVISIONAL pending schema crate.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tool {
    pub name: String,
}

/// source: Request (QuestionV1.Request) — verbatim fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub id: QuestionID,
    pub session_id: String,
    pub questions: Vec<Info>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tool: ::std::option::Option<Tool>,
}

/// source: Answer = ReadonlyArray — verbatim (array of option labels).
pub type Answer = Vec<String>;

/// source: Reply (QuestionV1.Reply) — verbatim fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reply {
    pub session_id: String,
    pub request_id: QuestionID,
    pub answers: Vec<Answer>,
}

/// source: Replied / Rejected event tags — verbatim names.
pub const REPLIED: &str = "replied";
pub const REJECTED: &str = "rejected";
pub const ASKED: &str = "asked";

/// source: RejectedError ("QuestionRejectedError") — verbatim message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RejectedError {}

impl std::fmt::Display for RejectedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The user dismissed this question")
    }
}

impl std::error::Error for RejectedError {}

/// source: NotFoundError ("Question.NotFoundError" { requestID }) — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotFoundError {
    pub request_id: QuestionID,
}

impl std::fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Question request not found: {}", self.request_id.0)
    }
}

impl std::error::Error for NotFoundError {}

/// source: "Question request not found: {id}" — verbatim server-side message.
/// (mirrors crates/server handlers_impl/question.rs shared string.)
pub fn not_found_message(request_id: &QuestionID) -> String {
    format!("Question request not found: {}", request_id.0)
}

/// source: Interface — ask/reply/reject/list, verbatim signatures.
pub trait Interface {
    fn ask(
        &mut self,
        session_id: &str,
        questions: Vec<Info>,
        tool: ::std::option::Option<Tool>,
    ) -> Result<QuestionID, RejectedError>;
    fn reply(&mut self, request_id: &QuestionID, answers: Vec<Answer>)
        -> Result<(), NotFoundError>;
    fn reject(&mut self, request_id: &QuestionID) -> Result<(), NotFoundError>;
    fn list(&self) -> Vec<Request>;
}

/// source: reply/reject unknown-request branch — logWarning + NotFoundError.
/// Verbatim log strings extracted as consts.
pub const LOG_REPLY_UNKNOWN: &str = "reply for unknown request";
pub const LOG_REJECT_UNKNOWN: &str = "reject for unknown request";

/// source: Service "@opencode/Question" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/Question";

/// source: node deps [EventV2Bridge.node] — verbatim.
pub const NODE_SERVICE: &str = SERVICE_ID;
pub const NODE_DEPS: &[&str] = &["@/event-v2-bridge.EventV2Bridge"];
