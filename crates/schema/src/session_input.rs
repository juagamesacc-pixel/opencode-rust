//! 1:1 port of packages/schema/src/session-input.ts
#![allow(non_snake_case)]
pub use crate::session_delivery::Delivery;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Admitted {
    pub admittedSeq: i64,
    pub id: crate::session_message::ID,
    pub sessionID: crate::session_id::SessionID,
    pub prompt: crate::prompt::Prompt,
    pub delivery: Delivery,
    pub timeCreated: crate::schema_primitives::DateTimeUtcFromMillis,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotedSeq: Option<i64>,
}
