//! Port of `packages/cli/src/framework/runtime.ts` (v1.18.30 @3104c14).
//!
//! Source exports: `Input<Value>` (type), `Handlers<Node>` (type),
//! `handler(_node, run)` (function, lines 35-40),
//! `handlers(root, handlers)` (function, lines 42-56),
//! `run(commands, handlers, options: { version })` (function, lines 58-60),
//! internal `provide(node, handlers)` (lines 62-77), plus
//! `export * as Runtime from "./runtime"` self-namespace.
//!
//! 1:1 notes:
//! - `handler()` returns `run` unchanged (identity, same as source).
//! - `handlers()` walks the `Spec` tree depth-first: a loader function value
//!   pushes one `LazyHandler`; an object value pushes `value.$` for the node
//!   itself then recurses `Object.entries(node.commands)` in insertion order.
//!   Same order, same entries.
//! - `run()` keeps the `{ version }` option and the provide-then-dispatch
//!   contract. Effect lazy `import()` becomes a boxed async handler fn.
//! - `provide()` keeps semantics: node with a registered handler gets the
//!   handler attached; nodes without handlers keep bare spec; subcommands
//!   recurse over `Object.values(node.commands)`.

use std::collections::BTreeMap;
use std::future::Future;
use std::pin::Pin;

use crate::daemon::DaemonService;
use crate::spec::Node;

/// Port of `Input<Value>` conditional type: the parsed input for a node.
/// Concrete shapes are defined per command in `commands.rs`; this is the
/// transport map passed to handlers.
pub type Input = BTreeMap<String, String>;

/// Boxed async handler: `(input) -> Effect<void, Error, Daemon.Service>`
/// becomes `async fn(&DaemonService, Input) -> Result<(), String>`.
pub type HandlerFn = Box<
    dyn Fn(&DaemonService, Input) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>>
        + Send
        + Sync,
>;

/// Port of internal `LazyHandler { spec, load }` (lines 23-26).
/// Source compares by `Command` object identity
/// (`handler.spec === node.spec`); for a 1:1 tree this is equivalent to the
/// node-path identity used here (e.g. `"service.start"`).
pub struct LazyHandler {
    pub spec_path: String,
    pub name: String,
    pub load: HandlerFn,
}

/// Port of `handler(_node, run)` (lines 35-40): identity, returns `run`.
pub fn handler<F>(_node_path: &str, run: F) -> F {
    run
}

/// A handler-tree value mirroring `RuntimeHandlers` (lines 28-33):
/// either a loader function or a map with optional `$` default plus children.
pub enum HandlerTree {
    Loader(HandlerFn),
    Branch {
        default: Option<HandlerFn>,
        children: BTreeMap<String, HandlerTree>,
    },
}

/// Port of `handlers(root, handlers)` (lines 42-56): depth-first walk.
/// Loader value -> push one entry. Object value -> push `value.$` for the
/// node itself, then recurse over `node.commands` in insertion order.
pub fn build_handlers(root: &Node, tree: HandlerTree) -> Vec<LazyHandler> {
    let mut result = Vec::new();
    walk_node(root, &root.name, tree, &mut result);
    result
}

fn walk_node(node: &Node, path: &str, tree: HandlerTree, result: &mut Vec<LazyHandler>) {
    match tree {
        HandlerTree::Loader(load) => result.push(LazyHandler {
            spec_path: path.to_string(),
            name: node.name.clone(),
            load,
        }),
        HandlerTree::Branch {
            default,
            mut children,
        } => {
            if let Some(load) = default {
                result.push(LazyHandler {
                    spec_path: path.to_string(),
                    name: node.name.clone(),
                    load,
                });
            }
            // Mirrors `for (const [name, child] of Object.entries(node.commands))`.
            for name in node.order.iter() {
                if let (Some(child), Some(sub)) = (node.commands.get(name), children.remove(name)) {
                    let child_path = format!("{path}.{name}");
                    walk_node(child, &child_path, sub, result);
                }
            }
        }
    }
}

/// Port of `run(commands, handlers, options: { version })` (lines 58-60).
/// Keeps the `version` option (source passes `{ version: "local" }` from
/// `src/index.ts:27`) and the provide-then-dispatch contract.
pub struct RunOptions {
    pub version: String,
}

pub fn run_options(version: &str) -> RunOptions {
    RunOptions {
        version: version.to_string(),
    }
}

/// Port of internal `provide(node, handlers)` (lines 62-77): attach the
/// registered handler where the handler spec matches the node, else keep the
/// bare spec; recurse into subcommands.
pub fn provide<'a>(node: &'a Node, handlers: &'a [LazyHandler], path: &str) -> Provided<'a> {
    let handler = handlers.iter().find(|h| h.spec_path == path);
    Provided {
        node,
        path: path.to_string(),
        handler,
    }
}

pub struct Provided<'a> {
    pub node: &'a Node,
    pub path: String,
    pub handler: Option<&'a LazyHandler>,
}
