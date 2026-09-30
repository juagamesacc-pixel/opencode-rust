//! Rust port of `packages/app/src/context/prompt-state.ts` (opencode v1.18.30).
//!
//! Source 274 lines. Exports: `TextPart`, `FileAttachmentPart`, `AgentPart`, `ImageAttachmentPart`, `ContentPart`, `Prompt`, `PromptModel`, `FileContextItem`, `ContextItem`, `PromptScope`, `DEFAULT_PROMPT`, `PromptStore`, `isPromptEqual`, `isCommentItem`, `createPromptSession`, `createDraftPromptSession`, `PromptSession`, `createPromptReady`, `createPromptState`.
//!
//! 1:1 notes:
//! - SolidJS reactivity/router → PROVISIONAL stubs flagged below.
//! - Persist keys, route keys, defaults, ordering preserved where applicable.
//! - Original file: `packages/app/src/context/prompt-state.ts`
// PROVISIONAL: pending solid-js/solid-router — mirrors packages/app/src/context/prompt-state.ts

#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

/// Mirrors `TextPart`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextPart {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `FileAttachmentPart`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileAttachmentPart {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `AgentPart`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentPart {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `ImageAttachmentPart`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageAttachmentPart {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `ContentPart`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentPart {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `Prompt`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Prompt {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `PromptModel`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PromptModel {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `FileContextItem`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileContextItem {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `ContextItem`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextItem {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `PromptScope`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PromptScope {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `DEFAULT_PROMPT`.
pub fn DEFAULT_PROMPT_value() -> String {
    String::new()
}

/// Mirrors `PromptStore`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PromptStore {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `isPromptEqual`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/prompt-state.ts
#[allow(non_snake_case)]
pub fn isPromptEqual(/* promptA: Prompt, promptB: Prompt */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `isCommentItem`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/prompt-state.ts
#[allow(non_snake_case)]
pub fn isCommentItem(/* item: ContextItem | (ContextItem & { key: string } */) -> serde_json::Value
{
    serde_json::json!({})
}

/// Mirrors `createPromptSession`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/prompt-state.ts
#[allow(non_snake_case)]
pub fn createPromptSession(/* 
  serverScope: ServerScope,
  scope: PromptScope,
  initial?: InitialPrompt,
  platform?: Platform,
 */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `createDraftPromptSession`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/prompt-state.ts
#[allow(non_snake_case)]
pub fn createDraftPromptSession(/* draftID: string, initial?: InitialPrompt */) -> serde_json::Value
{
    serde_json::json!({})
}

/// Mirrors `PromptSession`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PromptSession {
    // PROVISIONAL: fields pending full port
}

/// Mirrors `createPromptReady`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/prompt-state.ts
#[allow(non_snake_case)]
pub fn createPromptReady(/* session: Accessor<PromptSession> */) -> serde_json::Value {
    serde_json::json!({})
}

/// Mirrors `createPromptState`.
// PROVISIONAL: pending solid-js — mirrors packages/app/src/context/prompt-state.ts
#[allow(non_snake_case)]
pub fn createPromptState(/* initial?: InitialPrompt */) -> serde_json::Value {
    serde_json::json!({})
}
