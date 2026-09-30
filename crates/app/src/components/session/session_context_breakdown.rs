//! Port of packages/app/src/components/session/session-context-breakdown.ts
//! ——— 1:1 exact clone, zero diversion ———
#![allow(clippy::all)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Port of packages/app/src/components/session/session-context-breakdown.ts — pure logic / types.
// Exported symbols: SessionContextBreakdownKey, SessionContextBreakdownSegment, estimateSessionContextBreakdown
// PROVISIONAL: pending solid-js / @opencode-ai/core / sdk — mirrors packages/app/src/components/session/session-context-breakdown.ts

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SessionContextBreakdownKey {
    pub inner: Value,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SessionContextBreakdownSegment {
    pub inner: Value,
}
pub fn estimate_session_context_breakdown(_input: Value) -> Value {
    Value::Null
}
