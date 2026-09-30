// source: src/worktree/index.ts — exports: Event, Info, CreateInput,
// RemoveInput, ResetInput, NotGitError, NameGenerationFailedError,
// CreateFailedError, StartCommandFailedError, RemoveFailedError,
// ResetFailedError, ListFailedError, Error, Interface, Service, node, Worktree
// PROVISIONAL pending crates/core (layer-node, app-node-platform, global,
// database, fs-util, process, project/sql, util/slug, util/error) +
// @/project/*, @/effect/instance-state, @opencode-ai/schema/worktree-event,
// drizzle-orm: pure helpers (slugify, failedRemoves, parseWorktreeList,
// branch-strip, MAX_NAME_ATTEMPTS) + error tags + messages verbatim.

use serde::{Deserialize, Serialize};

/// source: Info { name, branch?, directory } ("Worktree") — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    pub directory: String,
}

/// source: CreateInput { name?, startCommand? } — verbatim (+ description).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_command: Option<String>,
}

/// source: RemoveInput { directory } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveInput {
    pub directory: String,
}

/// source: ResetInput { directory } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetInput {
    pub directory: String,
}

macro_rules! worktree_error {
    ($name:ident, $tag:literal) => {
        /// source: error tag — verbatim.
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $name {
            pub message: String,
        }

        impl $name {
            pub fn tag() -> &'static str {
                $tag
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}: {}", $tag, self.message)
            }
        }

        impl std::error::Error for $name {}
    };
}

worktree_error!(NotGitError, "WorktreeNotGitError");
worktree_error!(
    NameGenerationFailedError,
    "WorktreeNameGenerationFailedError"
);
worktree_error!(CreateFailedError, "WorktreeCreateFailedError");
worktree_error!(StartCommandFailedError, "WorktreeStartCommandFailedError");
worktree_error!(RemoveFailedError, "WorktreeRemoveFailedError");
worktree_error!(ResetFailedError, "WorktreeResetFailedError");
worktree_error!(ListFailedError, "WorktreeListFailedError");

/// source: Error union — verbatim members.
#[derive(Debug, Clone)]
pub enum Error {
    NotGit(NotGitError),
    NameGenerationFailed(NameGenerationFailedError),
    CreateFailed(CreateFailedError),
    StartCommandFailed(StartCommandFailedError),
    RemoveFailed(RemoveFailedError),
    ResetFailed(ResetFailedError),
    ListFailed(ListFailedError),
}

/// source: "Worktrees are only supported for git projects" — verbatim.
pub const NOT_GIT_MESSAGE: &str = "Worktrees are only supported for git projects";
/// source: "Failed to generate a unique worktree name" — verbatim.
pub const NAME_GEN_MESSAGE: &str = "Failed to generate a unique worktree name";
/// source: "Failed to create git worktree" (fallback) — verbatim.
pub const CREATE_FAILED_FALLBACK: &str = "Failed to create git worktree";
/// source: "Failed to populate worktree" (fallback) — verbatim.
pub const POPULATE_FAILED_FALLBACK: &str = "Failed to populate worktree";
/// source: "Cannot reset the primary workspace" — verbatim.
pub const RESET_PRIMARY_MESSAGE: &str = "Cannot reset the primary workspace";
/// source: "Worktree not found" — verbatim.
pub const NOT_FOUND_MESSAGE: &str = "Worktree not found";
/// source: "Default branch not found" — verbatim.
pub const NO_DEFAULT_BRANCH_MESSAGE: &str = "Default branch not found";
/// source: MAX_NAME_ATTEMPTS = 26 — verbatim.
pub const MAX_NAME_ATTEMPTS: usize = 26;

/// source: slugify — verbatim regex chain.
pub fn slugify(input: &str) -> String {
    let lower = input.trim().to_lowercase();
    let mut out = String::with_capacity(lower.len());
    for c in lower.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c);
        } else {
            out.push('-');
        }
    }
    let out = out.trim_matches('-').to_string();
    // collapse runs (the [^a-z0-9]+ → single "-" is implied; trim ends verbatim)
    let mut collapsed = String::with_capacity(out.len());
    let mut last_dash = false;
    for c in out.chars() {
        if c == '-' {
            if !last_dash {
                collapsed.push(c);
            }
            last_dash = true;
        } else {
            collapsed.push(c);
            last_dash = false;
        }
    }
    collapsed.trim_matches('-').to_string()
}

/// source: failedRemoves — `warning: failed to remove <path>:` parse, verbatim.
pub fn failed_removes(chunks: &[&str]) -> Vec<String> {
    let mut out = Vec::new();
    for chunk in chunks.iter().filter(|c| !c.is_empty()) {
        for line in chunk.split('\n').map(|l| l.trim()) {
            let lower = line.to_lowercase();
            let prefix = "warning: failed to remove ";
            if !lower.starts_with(prefix) {
                continue;
            }
            let mut value = line[prefix.len()..].trim();
            // trailing ":\s+" ... value ends before last colon; mirror: match[1] then strip quotes
            if let Some(pos) = value.rfind(':') {
                value = value[..pos].trim();
            }
            let value = value.trim_matches(|c| c == '\'' || c == '"');
            if !value.is_empty() {
                out.push(value.to_string());
            }
        }
    }
    out
}

/// source: parseWorktreeList — porcelain "worktree "/"branch " fold, verbatim.
#[derive(Debug, Clone)]
pub struct WorktreeEntry {
    pub path: Option<String>,
    pub branch: Option<String>,
}

pub fn parse_worktree_list(text: &str) -> Vec<WorktreeEntry> {
    let mut acc: Vec<WorktreeEntry> = Vec::new();
    for line in text.split('\n').map(|l| l.trim()).filter(|l| !l.is_empty()) {
        if let Some(p) = line.strip_prefix("worktree ") {
            acc.push(WorktreeEntry {
                path: Some(p.trim().to_string()),
                branch: None,
            });
            continue;
        }
        if let Some(current) = acc.last_mut() {
            if let Some(b) = line.strip_prefix("branch ") {
                current.branch = Some(b.trim().to_string());
            }
        }
    }
    acc
}

/// source: branch strip `refs/heads/` — verbatim.
pub fn strip_branch_ref(branch: &str) -> &str {
    branch.strip_prefix("refs/heads/").unwrap_or(branch)
}

/// source: Interface — makeWorktreeInfo/createFromInfo/create/list/remove/reset, verbatim.
pub trait Interface {
    fn make_worktree_info(&self, name: Option<&str>, detached: bool) -> Result<Info, Error>;
    fn create_from_info(&self, info: &Info, start_command: Option<&str>) -> Result<(), Error>;
    fn create(&self, input: Option<&CreateInput>) -> Result<Info, Error>;
    fn list(&self) -> Result<Vec<Info>, Error>;
    fn remove(&self, input: &RemoveInput) -> Result<bool, Error>;
    fn reset(&self, input: &ResetInput) -> Result<bool, Error>;
}

/// source: Service "@opencode/Worktree" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/Worktree";

/// source: node deps [FSUtil, path, AppProcess, Git, Project, InstanceStore, Database] — verbatim.
pub const NODE_SERVICE: &str = SERVICE_ID;
pub const NODE_DEPS: &[&str] = &[
    "@opencode-ai/core/fs-util.FSUtil",
    "@opencode-ai/core/effect/app-node-platform.path",
    "@opencode-ai/core/process.AppProcess",
    "@/git.Git",
    "@/project/project.Project",
    "@/project/instance-store.InstanceStore",
    "@opencode-ai/core/database/database.Database",
];
