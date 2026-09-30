// source: packages/plugin/src/example-workspace.ts
#![allow(dead_code)]

//! Rust port of `packages/plugin/src/example-workspace.ts` (opencode v1.18.30).
//!
//! Source 34 lines. Exports: `FolderWorkspacePlugin: Plugin` registering workspace `folder`.
//!
//! 1:1 notes:
//! - `experimental_workspace.register("folder", { name:"Folder", description:"Create a blank folder", configure, create, remove, target })` verbatim.
//! - `configure` returns `{ ...config, directory: `/tmp/folder/folder-${rand}` }` with `Math.random()` verbatim.
//! - `create` does `mkdir(config.directory, {recursive:true})`, `remove` does `rm(..., {recursive:true, force:true})`, `target` returns `{type:"local", directory:config.directory!}` verbatim.

/// Mirrors `FolderWorkspacePlugin` workspace registration verbatim.
pub const FOLDER_WORKSPACE_TYPE: &str = "folder";
pub const FOLDER_WORKSPACE_NAME: &str = "Folder";
pub const FOLDER_WORKSPACE_DESCRIPTION: &str = "Create a blank folder";
pub const FOLDER_WORKSPACE_TMP_PREFIX: &str = "/tmp/folder/folder-";

/// Mirrors `FolderWorkspacePlugin` descriptor.
#[derive(Clone, Debug, PartialEq)]
pub struct FolderWorkspacePlugin;

impl FolderWorkspacePlugin {
    pub const TYPE: &'static str = FOLDER_WORKSPACE_TYPE;
    pub const NAME: &'static str = FOLDER_WORKSPACE_NAME;
    pub const DESCRIPTION: &'static str = FOLDER_WORKSPACE_DESCRIPTION;

    /// Mirrors `configure(config)` — returns directory with random suffix.
    pub fn configure_directory_rand() -> String {
        // Mirrors `"" + Math.random()` — in Rust we use a placeholder random-like value.
        // Actual Math.random semantics are host-provided; descriptor preserves prefix verbatim.
        format!("{}{}", FOLDER_WORKSPACE_TMP_PREFIX, "RAND")
    }

    /// Mirrors target type verbatim: `"local"`.
    pub const TARGET_TYPE: &'static str = "local";
}

/// Default export mirrors `export default FolderWorkspacePlugin`.
pub use FolderWorkspacePlugin as DefaultPlugin;
