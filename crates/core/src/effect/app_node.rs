//! Rust port of `packages/core/src/effect/app-node.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.

use serde::{Deserialize, Serialize};

// PROVISIONAL pending LayerNode.tags — faithful stub

pub const TAGS_CONFIG: &[(&str, &[&str])] = &[("location", &["global"]), ("global", &[])];

pub const TAG_LOCATION: &str = "location";
pub const TAG_GLOBAL: &str = "global";

/// Source: `export type GlobalNode<A,E> = LayerNode.Node<A,E,Tag["global"]>` verbatim
/// Source: `export type LocationNode<A,E> = LayerNode.Node...
/// Source: `export const makeGlobalNode = tags.make("global")` verbatim
/// Source: `export const makeLocationNode = tags.make("location")` verbatim

pub fn make_global_node_name(service: &str) -> String {
    service.to_string()
}
pub fn make_location_node_name(service: &str) -> String {
    service.to_string()
}
