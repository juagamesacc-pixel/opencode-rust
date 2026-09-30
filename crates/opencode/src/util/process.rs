// source: src/util/process.ts — exports: Stdio, Shell, Options, RunOptions,
// Result, TextResult, RunFailedError, Child, spawn, run, stop, text, lines, Process
// PROVISIONAL: cross-spawn/AbortSignal/stream plumbing modelled as data +
// verbatim messages, argv tables, and pure rules; execution runs on CI-wired host.

/// source: Options — verbatim fields.
#[derive(Debug, Clone, Default)]
pub struct Options {
    pub cwd: Option<String>,
    pub env: Option<std::collections::HashMap<String, String>>,
    pub stdin: Option<String>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub shell: Option<String>,
    pub kill: Option<String>,
    pub timeout: Option<u64>,
}

/// source: RunOptions — + nothrow, verbatim.
#[derive(Debug, Clone, Default)]
pub struct RunOptions {
    pub base: Options,
    pub nothrow: bool,
}

/// source: Result { code, stdout, stderr } — verbatim.
#[derive(Debug, Clone)]
pub struct ProcResult {
    pub code: i32,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

/// source: RunFailedError — name "ProcessRunFailedError", message template verbatim.
#[derive(Debug, Clone)]
pub struct RunFailedError {
    pub cmd: Vec<String>,
    pub code: i32,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub message: String,
}

impl RunFailedError {
    /// source: constructor message — stderr-trim branch, verbatim.
    pub fn new(cmd: &[String], code: i32, stdout: Vec<u8>, stderr: Vec<u8>) -> Self {
        let text = String::from_utf8_lossy(&stderr).trim().to_string();
        let message = if text.is_empty() {
            format!("Command failed with code {}: {}", code, cmd.join(" "))
        } else {
            format!(
                "Command failed with code {}: {}\n{}",
                code,
                cmd.join(" "),
                text
            )
        };
        Self {
            cmd: cmd.to_vec(),
            code,
            stdout,
            stderr,
            message,
        }
    }
}

impl std::fmt::Display for RunFailedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for RunFailedError {}

/// source: "Command is required" — verbatim.
pub const CMD_REQUIRED: &str = "Command is required";
/// source: "Process output not available" — verbatim.
pub const NO_OUTPUT: &str = "Process output not available";
/// source: abort kill default SIGTERM + SIGKILL grace 5000 (ms<=0 skip) — verbatim.
pub const ABORT_KILL: &str = "SIGTERM";
pub const ABORT_FORCE_KILL: &str = "SIGKILL";
pub const ABORT_GRACE_MS: u64 = 5_000;
/// source: stop() win32 taskkill argv — verbatim.
pub fn taskkill_argv(pid: u32) -> Vec<String> {
    vec![
        "taskkill".into(),
        "/pid".into(),
        pid.to_string(),
        "/T".into(),
        "/F".into(),
    ]
}

/// source: env merge — null → {}, provided → {...process.env, ...env}, else inherit.
/// Verbatim rule as pure fn over explicit maps.
pub fn merge_env(
    process_env: &std::collections::HashMap<String, String>,
    env: Option<&std::collections::HashMap<String, String>>,
) -> Option<std::collections::HashMap<String, String>> {
    match env {
        None => None,
        Some(e) => {
            let mut out = process_env.clone();
            out.extend(e.clone());
            Some(out)
        }
    }
}

/// source: lines() split /\r?\n/ + filter Boolean — verbatim.
pub fn lines_of(text: &str) -> Vec<String> {
    text.split('\n')
        .map(|l| l.strip_suffix('\r').unwrap_or(l).to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

/// source: text() — stdout.toString(), verbatim.
pub fn text_of(stdout: &[u8]) -> String {
    String::from_utf8_lossy(stdout).to_string()
}

/// source: windowsHide = platform === "win32" — verbatim.
pub fn windows_hide(win32: bool) -> bool {
    win32
}
