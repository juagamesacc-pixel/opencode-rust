//! Rust port of `packages/server/src/handlers/project-copy.ts` (opencode v1.18.30).
//!
//! Source 48 lines: `ProjectCopyHandler` with 3 ops, `badRequest` mapping ProjectCopy.Error -> ProjectCopyError,
//! `message` branches for each error variant.
//!
//! PROVISIONAL: `ProjectCopy.Service`, `Git.WorktreeError` pending `crates/core`.

pub const GROUP: &str = "server.projectCopy";
pub const OPERATIONS: &[&str] = &[
    "projectCopy.create",
    "projectCopy.remove",
    "projectCopy.refresh",
];

pub const ERROR_NAME: &str = "ProjectCopyError";

pub fn message_for_variant(variant: &str, directory: &str) -> String {
    match variant {
        "SourceDirectoryNotFoundError" => format!("Project copy source not found: {directory}"),
        "DestinationExistsError" => format!("Project copy destination already exists: {directory}"),
        "DirectoryUnavailableError" => format!("Project copy directory unavailable: {directory}"),
        "InvalidDirectoryError" => format!("Invalid project copy directory: {directory}"),
        "StrategyUnavailableError" => format!("Project copy strategy unavailable: {directory}"),
        _ => directory.to_string(),
    }
}
