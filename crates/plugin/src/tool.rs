// source: packages/plugin/src/tool.ts
#![allow(dead_code)]

//! Rust port of `packages/plugin/src/tool.ts` (opencode v1.18.30).
//!
//! Source 54 lines. Exports: `ToolContext`, `ToolAttachment`, `ToolResult`, `tool`, `ToolDefinition`.
//!
//! 1:1 notes:
//! - `ToolContext` fields verbatim order: sessionID, messageID, agent, directory, worktree, abort, metadata, ask.
//! - `ToolAttachment.type` literal `"file"` verbatim.
//! - `ToolResult` string|object union verbatim.
//! - `tool` fn returns input verbatim; `tool.schema = z` verbatim.
//!
//! PROVISIONAL: `zod` (`z.ZodRawShape`, `z.infer`, `z.ZodObject`) pending `zod` crate — stub via `serde_json::Value`.
//! `AbortSignal` pending DOM runtime — stub via `serde_json::Value`.

use serde::{Deserialize, Serialize};

/// Mirrors `AskInput` from `tool.ts`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AskInput {
    pub permission: String,
    pub patterns: Vec<String>,
    pub always: Vec<String>,
    pub metadata: serde_json::Value,
}

/// Mirrors `ToolContext`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToolContext {
    #[serde(rename = "sessionID")]
    pub session_id: String,
    #[serde(rename = "messageID")]
    pub message_id: String,
    pub agent: String,
    pub directory: String,
    pub worktree: String,
    pub abort: serde_json::Value,
    pub metadata: serde_json::Value,
    pub ask: serde_json::Value,
}

/// Mirrors `ToolAttachment` with `type: "file"` verbatim.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToolAttachment {
    #[serde(rename = "type")]
    pub r#type: String,
    pub mime: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

impl ToolAttachment {
    pub const TYPE_FILE: &'static str = "file";
}

/// Mirrors `ToolResult` union.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolResult {
    Text(String),
    Object {
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        output: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        metadata: Option<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        attachments: Option<Vec<ToolAttachment>>,
    },
}

/// Mirrors `tool` input descriptor.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToolInput {
    pub description: String,
    pub args: serde_json::Value,
    pub execute: serde_json::Value,
}

/// Mirrors `tool` function — returns input verbatim.
///
/// In TS: `export function tool<Args extends z.ZodRawShape>(input) { return input }`
/// `tool.schema = z` — zod re-export.
pub fn tool(input: ToolInput) -> ToolInput {
    input
}

/// Mirrors `tool.schema = z` — zod namespace stub verbatim.
pub const TOOL_SCHEMA_ZOD: &str = "zod";

/// Mirrors `ToolDefinition = ReturnType<typeof tool>`.
pub type ToolDefinition = ToolInput;

/// PROVISIONAL: `zod` pending zod crate.
pub mod zod_provisional {
    pub const PACKAGE: &str = "zod";
    pub const PENDING_CRATE: &str = "zod";
}
