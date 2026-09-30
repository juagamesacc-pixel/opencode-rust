//! Port of `packages/schema/src/pty-ticket.ts`.
//!
//! Source exports: `ConnectToken` only. Cross-lane: `PositiveInt` is
//! `crate::schema_primitives::PositiveInt` (Lane A, `i64`).

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

/// `PtyTicket.ConnectToken`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectToken {
    pub ticket: String,
    pub expires_in: crate::schema_primitives::PositiveInt,
}
