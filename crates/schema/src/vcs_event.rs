//! Port of `packages/schema/src/vcs-event.ts`.
//!
//! Source exports: `BranchUpdated`, `Definitions` (`vcs.branch.updated`
//! `{branch?}`). Uses the `Event.define` namespace form in source.

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

/// Payload of `vcs.branch.updated`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BranchUpdatedPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
}

/// `vcs.branch.updated` definition marker.
pub struct BranchUpdated;
impl BranchUpdated {
    pub const TYPE: &'static str = "vcs.branch.updated";
}

/// Verbatim declaration order: `BranchUpdated`.
pub const Definitions: &[&'static str] = &[BranchUpdated::TYPE];
