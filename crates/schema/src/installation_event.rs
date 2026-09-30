//! Port of `packages/schema/src/installation-event.ts`.
//!
//! Source exports: `Updated`, `UpdateAvailable`, `Definitions`
//! (`installation.updated`, `installation.update-available`, each `{version}`).
//! Uses the `Event.define`/`Event.inventory` namespace form in source.

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

/// Payload of `installation.updated`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdatedPayload {
    pub version: String,
}

/// `installation.updated` definition marker.
pub struct Updated;
impl Updated {
    pub const TYPE: &'static str = "installation.updated";
}

/// Payload of `installation.update-available`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateAvailablePayload {
    pub version: String,
}

/// `installation.update-available` definition marker.
pub struct UpdateAvailable;
impl UpdateAvailable {
    pub const TYPE: &'static str = "installation.update-available";
}

/// Verbatim declaration order: `Updated`, `UpdateAvailable`.
pub const Definitions: &[&'static str] = &[Updated::TYPE, UpdateAvailable::TYPE];
