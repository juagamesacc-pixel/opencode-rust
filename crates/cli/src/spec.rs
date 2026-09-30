//! Port of `packages/cli/src/framework/spec.ts` (v1.18.30 @3104c14).
//!
//! Source exports: `Node` (interface), `Any` (type), `Children` (type),
//! `make(name, options)` (function), plus `export * as Spec from "./spec"`
//! self-namespace re-export.
//!
//! 1:1 notes:
//! - `Spec.make(name, { description?, params?, commands? })` keeps the same
//!   name, same options shape, same behavior: builds a command node whose
//!   `commands` map is keyed by child `name` (via `Object.fromEntries`
//!   over `options.commands ?? []`).
//! - Generic `ChildrenOf<Commands>` type-level keying is expressed as a
//!   runtime `BTreeMap<String, Node>` keyed by child name; ordering of keys
//!   is not observable in source (object map), iteration order follows
//!   insertion via the `order` vec.
//! - No renames, no new behavior.

use std::collections::BTreeMap;

/// Port of `Node<Name, Spec, Commands>` interface in spec.ts:
/// `{ readonly name, readonly spec, readonly commands }`.
/// `spec` here carries the CLI-facing metadata (description + param schema
/// name); full Effect `Command` wiring lives in `runtime.rs` / `commands.rs`.
#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub description: Option<String>,
    pub commands: BTreeMap<String, Node>,
    /// Child insertion order (mirrors `options.commands` array order).
    pub order: Vec<String>,
}

/// Port of `Children = Readonly<Record<string, Any>>`.
pub type Children = BTreeMap<String, Node>;

/// Port of `Any = Node<string, Command.Command<...>, Children>`.
pub type Any = Node;

/// Port of `Options<Config, Commands>` in spec.ts:
/// `{ readonly description?, readonly params?, readonly commands? }`.
/// `params` is opaque here; concrete clap wiring lives in `commands.rs`.
#[derive(Debug, Clone, Default)]
pub struct Options {
    pub description: Option<String>,
    pub commands: Vec<Node>,
}

/// Port of `make<const Name, const Config, const Commands>(name, options = {})`
/// in spec.ts (lines 22-36).
pub fn make(name: &str, options: Options) -> Node {
    let mut commands = BTreeMap::new();
    let mut order = Vec::new();
    for child in options.commands {
        order.push(child.name.clone());
        commands.insert(child.name.clone(), child);
    }
    Node {
        name: name.to_string(),
        description: options.description,
        commands,
        order,
    }
}
