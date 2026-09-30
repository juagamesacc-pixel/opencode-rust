//! Rust port of `packages/schema/src/session.ts` (anomalyco/opencode v1.18.30).
//!
//! 1:1 exact translation — same names/signatures/behavior/edge cases/keys/defaults/ordering.
//! Source is spec. No improvements, renames, merges, splits, or reordering.
//!
//! `ID = SessionID` is a re-export (type alias); `Event = SessionEvent` is a
//! namespace re-export (`pub use`). `Info` and `ListAnchor` are structs.

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

/// `Session.ID` (= `SessionID`).
pub type ID = crate::session_id::SessionID;

/// `Session.Event` (= `SessionEvent` namespace).
pub use crate::session_event as Event;

/// Identifier `SessionV2.Info`. Field order is verbatim: id, parentID,
/// projectID, agent, model, cost, tokens, time, title, location, subpath,
/// revert. `tokens` / `time` are anonymous sub-structs ported here.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Info {
    pub id: ID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parentID: Option<ID>,
    pub projectID: crate::project::ID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<crate::agent::ID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<crate::model::Ref>,
    pub cost: f64,
    pub tokens: InfoTokens,
    pub time: InfoTime,
    pub title: String,
    pub location: crate::location::Ref,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subpath: Option<crate::schema_primitives::RelativePath>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revert: Option<crate::revert::State>,
}

/// Anonymous `tokens` sub-struct of `Info`
/// (`Schema.Struct({ input, output, reasoning, cache: { read, write } })`, all `Schema.Finite`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoTokens {
    pub input: f64,
    pub output: f64,
    pub reasoning: f64,
    pub cache: InfoTokensCache,
}

/// Anonymous `cache` sub-struct of `Info.tokens`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoTokensCache {
    pub read: f64,
    pub write: f64,
}

/// Anonymous `time` sub-struct of `Info`
/// (`Schema.Struct({ created: DateTimeUtcFromMillis, updated: DateTimeUtcFromMillis, archived: DateTimeUtcFromMillis.pipe(optional) })`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoTime {
    pub created: crate::schema_primitives::DateTimeUtcFromMillis,
    pub updated: crate::schema_primitives::DateTimeUtcFromMillis,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<crate::schema_primitives::DateTimeUtcFromMillis>,
}

/// Identifier `Session.ListAnchor`. Field order is verbatim: id, time, direction.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListAnchor {
    pub id: ID,
    pub time: f64,
    pub direction: ListAnchorDirection,
}

/// Closed literal set `["previous", "next"]` of `ListAnchor.direction`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ListAnchorDirection {
    #[serde(rename = "previous")]
    Previous,
    #[serde(rename = "next")]
    Next,
}
