//! 1:1 port of packages/schema/src/session-compaction-event.ts
#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Compacted {
    pub sessionID: crate::session_id::SessionID,
}
impl Compacted {
    pub const TYPE: &'static str = "session.compacted";
}
pub mod Event {
    pub use super::Compacted;
    pub const Definitions: &[&'static str] = &[Compacted::TYPE];
}
