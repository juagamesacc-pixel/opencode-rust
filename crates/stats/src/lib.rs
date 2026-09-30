// Rust port of @opencode-ai/stats @ v1.18.30 (commit 3104c14).
// Source: packages/stats
//
// The stats package aggregates usage telemetry (model/provider/geo/retention
// statistics) for the public stats site. This port preserves the pure domain
// logic 1:1 (aggregation math, model normalization, SQL-query construction,
// response shaping) plus the MySQL schema descriptors. The Effect/Planetscale/
// SST runtimes do not exist in the Rust port; the database access services are
// represented as descriptors with their query shapes, marked PROVISIONAL where
// the runtime behavior cannot be replicated.
#![allow(clippy::all)]

pub mod database;
pub mod domain;
pub mod schema;
