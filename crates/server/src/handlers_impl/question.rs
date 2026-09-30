//! Rust port of `packages/server/src/handlers/question.ts` (opencode v1.18.30).
//!
//! Source 60 lines: `QuestionHandler` with helper `withOwnedQuestion` + 4 ops.
//! `missingRequest` => QuestionNotFoundError("Question request not found: {id}").
//!
//! PROVISIONAL: `QuestionV2.Service` pending `crates/core`.

pub const GROUP: &str = "server.question";
pub const OPERATIONS: &[&str] = &[
    "question.request.list",
    "session.question.list",
    "session.question.reply",
    "session.question.reject",
];

pub fn missing_message(id: &str) -> String {
    format!("Question request not found: {id}")
}

pub const NOT_FOUND_TAG: &str = "QuestionV2.NotFoundError";
pub const PROTOCOL_ERROR_TAG: &str = "QuestionNotFoundError";
