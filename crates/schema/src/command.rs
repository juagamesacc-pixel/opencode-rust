//! Port of `packages/schema/src/command.ts`.
//!
//! Source exports: `Info` only. Cross-lane: `Model.Ref` is
//! `crate::model::Ref` (Lane A) per plan §4.

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

/// `CommandV2.Info`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Info {
    pub name: String,
    pub template: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<crate::model::Ref>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtask: Option<bool>,
}
