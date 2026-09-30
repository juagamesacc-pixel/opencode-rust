// source: src/git/index.ts — exports: Kind, Base, Item, Stat, Patch,
// PatchOptions, Result, Options, Interface, Service, node, Git
// PROVISIONAL pending crates/core (layer-node, process) + intra-crate effect:
// process execution modelled as trait; arg vectors + parsing logic verbatim.

use serde::{Deserialize, Serialize};

/// source: cfg — verbatim git flags.
pub const CFG: &[&str] = &[
    "--no-optional-locks",
    "-c",
    "core.autocrlf=false",
    "-c",
    "core.fsmonitor=false",
    "-c",
    "core.longpaths=true",
    "-c",
    "core.symlinks=true",
    "-c",
    "core.quotepath=false",
];

/// source: Kind — verbatim.
pub type Kind = String;

/// source: kind() — verbatim mapping.
pub fn kind(code: &str) -> &'static str {
    if code == "??" {
        return "added";
    }
    if code.contains("U") {
        return "modified";
    }
    if code.contains("A") && !code.contains("D") {
        return "added";
    }
    if code.contains("D") && !code.contains("A") {
        return "deleted";
    }
    "modified"
}

/// source: Base — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Base {
    pub name: String,
    pub r#ref: String,
}

/// source: Item — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub file: String,
    pub code: String,
    pub status: String,
}

/// source: Stat — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stat {
    pub file: String,
    pub additions: i64,
    pub deletions: i64,
}

/// source: Patch — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patch {
    pub text: String,
    pub truncated: bool,
}

/// source: PatchOptions — verbatim (context default 3, applied at call sites).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_bytes: Option<u64>,
}

/// source: Result — verbatim shape (text() as method).
#[derive(Debug, Clone)]
pub struct GitResult {
    pub exit_code: i32,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub truncated: bool,
}

impl GitResult {
    /// source: text() — stdout as utf8, verbatim.
    pub fn text(&self) -> String {
        String::from_utf8_lossy(&self.stdout).to_string()
    }
}

/// source: fail() — verbatim fields.
pub fn fail(err: &str) -> GitResult {
    GitResult {
        exit_code: 1,
        stdout: Vec::new(),
        stderr: err.as_bytes().to_vec(),
        truncated: false,
    }
}

/// source: out() — text().trim(), verbatim.
pub fn out(result: &GitResult) -> String {
    result.text().trim().to_string()
}

/// source: nuls() — split NUL + filter Boolean, verbatim.
pub fn nuls(text: &str) -> Vec<String> {
    text.split('\0')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

/// source: Options — verbatim.
#[derive(Debug, Clone)]
pub struct Options {
    pub cwd: String,
    pub env: Option<std::collections::HashMap<String, String>>,
    pub max_output_bytes: Option<u64>,
}

/// source: Interface — full method table, verbatim names/order.
pub trait Interface {
    fn run(&self, args: &[String], opts: &Options) -> GitResult;
    fn branch(&self, cwd: &str) -> Option<String>;
    fn prefix(&self, cwd: &str) -> String;
    fn default_branch(&self, cwd: &str) -> Option<Base>;
    fn has_head(&self, cwd: &str) -> bool;
    fn merge_base(&self, cwd: &str, base: &str, head: Option<&str>) -> Option<String>;
    fn show(&self, cwd: &str, r#ref: &str, file: &str, prefix: Option<&str>) -> String;
    fn status(&self, cwd: &str) -> Vec<Item>;
    fn diff(&self, cwd: &str, r#ref: &str) -> Vec<Item>;
    fn stats(&self, cwd: &str, r#ref: &str) -> Vec<Stat>;
    fn patch(&self, cwd: &str, r#ref: &str, file: &str, options: Option<&PatchOptions>) -> Patch;
    fn patch_all(&self, cwd: &str, r#ref: &str, options: Option<&PatchOptions>) -> Patch;
    fn patch_untracked(&self, cwd: &str, file: &str, options: Option<&PatchOptions>) -> Patch;
    fn stat_untracked(&self, cwd: &str, file: &str) -> Option<Stat>;
    fn apply_patch(&self, cwd: &str, patch: &str) -> GitResult;
}

/// source: Service "@opencode/Git" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/Git";

/// source: node deps [AppProcess.node] — verbatim.
pub const NODE_SERVICE: &str = SERVICE_ID;
pub const NODE_DEPS: &[&str] = &["@opencode-ai/core/process.AppProcess"];
