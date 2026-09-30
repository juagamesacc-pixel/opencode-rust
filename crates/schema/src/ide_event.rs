//! Port of `packages/schema/src/ide-event.ts`.
//!
//! Source exports: `Installed`, `Definitions` (`ide.installed` `{ide}`).
//! Uses the `Event.define` namespace form in source.

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

/// Payload of `ide.installed`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstalledPayload {
    pub ide: String,
}

/// `ide.installed` definition marker.
pub struct Installed;
impl Installed {
    pub const TYPE: &'static str = "ide.installed";
}

/// Verbatim declaration order: `Installed`.
pub const Definitions: &[&'static str] = &[Installed::TYPE];
