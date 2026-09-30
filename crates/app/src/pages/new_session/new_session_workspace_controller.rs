//! Rust port of `packages/app/src/pages/new-session/new-session-workspace-controller.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — same names/behavior/edge-cases/error-strings.
//! Rename: `new-session/new-session-workspace-controller.ts` -> `new_session/new_session_workspace_controller.rs` (kebab -> snake_case).

pub fn resolve_new_session_worktree(input: ResolveWorktreeInput) -> String {
    if !input.enabled {
        return "main".to_string();
    }
    if let Some(selected) = input.selected {
        return selected;
    }
    if let Some(pw) = input.project_worktree {
        if input.directory != pw {
            return input.directory;
        }
    }
    "main".to_string()
}

pub struct ResolveWorktreeInput {
    pub enabled: bool,
    pub selected: Option<String>,
    pub directory: String,
    pub project_worktree: Option<String>,
}

pub fn normalize_new_session_worktree(
    value: &str,
    directory: &str,
    project_worktree: Option<&str>,
) -> String {
    if value == "main" {
        if let Some(pw) = project_worktree {
            if pw != directory {
                return pw.to_string();
            }
        }
    }
    value.to_string()
}

pub struct ResolveBranchInput {
    pub worktree: String,
    pub local: Option<String>,
}

pub fn resolve_new_session_branch<F>(
    input: ResolveBranchInput,
    worktree_branch: F,
) -> Option<String>
where
    F: Fn(&str) -> Option<String>,
{
    if input.worktree == "main" || input.worktree == "create" {
        return input.local;
    }
    worktree_branch(&input.worktree).or(input.local)
}

// PROVISIONAL: solid-js signals for controller — mirrors createNewSessionWorkspaceController
// Pending solid-js — descriptor struct
#[derive(Clone, Debug)]
pub struct NewSessionWorkspaceControllerDescriptor {
    pub visible: bool,
    pub selected_worktree: String,
    pub project_root: String,
    pub branch: Option<String>,
}
