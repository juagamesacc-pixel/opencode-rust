//! Rust port of `packages/core/src/database/schema.sql.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.

/// Source: `export const Timestamps = { time_created, time_updated }` verbatim
pub const TIMESTAMPS_CREATED: &str = "time_created";
pub const TIMESTAMPS_UPDATED: &str = "time_updated";

pub const TIME_CREATED_DEFAULT: &str = "Date.now()";
pub const TIME_UPDATED_ON_UPDATE: &str = "Date.now()";
