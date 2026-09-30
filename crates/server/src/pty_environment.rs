//! Rust port of `packages/server/src/pty-environment.ts` (opencode v1.18.30).
//!
//! Source 19 lines. Exports: `Interface { get }`, `Service` (`"@opencode/ServerPtyEnvironment"`),
//! `layer` (succeed empty), `node` (`makeGlobalNode`).
//! Re-exports as `PtyEnvironment` via `export * as PtyEnvironment from "./pty-environment"`.
//!
//! 1:1 notes:
//! - Service ID `"@opencode/ServerPtyEnvironment"` verbatim.
//! - `get: ({ directory, cwd }) => Effect.succeed({})` — empty env map.
//! - `node` deps `[]` verbatim.

/// Service ID verbatim: `"@opencode/ServerPtyEnvironment"`.
pub const PTY_ENVIRONMENT_SERVICE_ID: &str = "@opencode/ServerPtyEnvironment";

/// Mirrors `Interface { get: (input: { directory: string; cwd: string }) => Effect<Record<string,string>> }`.
pub struct PtyEnvironmentInterface;

impl PtyEnvironmentInterface {
    /// Port of `get: () => Effect.succeed({})` — always returns empty map.
    pub fn get(_directory: &str, _cwd: &str) -> std::collections::HashMap<String, String> {
        std::collections::HashMap::new()
    }
}

/// Mirrors `Service` class (`Context.Service<..., Interface>("...")`).
pub struct Service;

impl Service {
    pub fn service_id() -> &'static str {
        PTY_ENVIRONMENT_SERVICE_ID
    }
}

/// Mirrors `layer = Layer.succeed(Service, of({ get: () => succeed({}) }))`.
pub struct Layer;

impl Layer {
    pub const SERVICE: &str = PTY_ENVIRONMENT_SERVICE_ID;
}

/// Mirrors `node = makeGlobalNode({ service: Service, layer, deps: [] })`.
///
/// `deps: []` verbatim.
pub struct Node;

impl Node {
    pub const SERVICE: &str = PTY_ENVIRONMENT_SERVICE_ID;
    pub const DEPS: &[&str] = &[];
}

/// Namespace re-export `export * as PtyEnvironment` — mirrors the self-export pattern.
pub mod PtyEnvironment {
    pub use super::{
        Layer, Node, PtyEnvironmentInterface as Interface, Service,
        PTY_ENVIRONMENT_SERVICE_ID as SERVICE_ID,
    };
    /// Returns empty env (mirrors the default `get`).
    pub fn get(directory: &str, cwd: &str) -> std::collections::HashMap<String, String> {
        super::PtyEnvironmentInterface::get(directory, cwd)
    }
}
