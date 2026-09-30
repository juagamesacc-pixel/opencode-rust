//! 1:1 port of packages/schema/src/session-delivery.ts
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Delivery {
    #[serde(rename = "steer")]
    Steer,
    #[serde(rename = "queue")]
    Queue,
}
