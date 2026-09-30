// source: src/project/instance-context.ts — exports: InstanceContext,
// context, containsPath (worktree "/" guard verbatim).

use serde::{Deserialize, Serialize};

/// source: InstanceContext { directory, worktree, project } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceContext {
    pub directory: String,
    pub worktree: String,
    pub project: super::project::Info,
}

/// source: context name "instance" — verbatim.
pub const CONTEXT_NAME: &str = "instance";

/// source: containsPath() — directory OR (worktree unless worktree === "/").
/// Verbatim boundary rule.
pub fn contains_path(filepath: &str, ctx: &InstanceContext) -> bool {
    if crate::util::filesystem::contains(&ctx.directory, filepath) {
        return true;
    }
    if ctx.worktree == "/" {
        return false;
    }
    crate::util::filesystem::contains(&ctx.worktree, filepath)
}
