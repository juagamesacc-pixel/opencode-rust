//! Rust port of `packages/core/src/shell.ts`.

pub const SIGKILL_TIMEOUT_MS: u64 = 200;

#[derive(Debug, Clone)]
pub struct Meta {
    pub deny: bool,
    pub login: bool,
    pub posix: bool,
    pub ps: bool,
}

pub fn meta_for(name: &str) -> Option<Meta> {
    match name {
        "bash" => Some(Meta {
            deny: false,
            login: true,
            posix: true,
            ps: false,
        }),
        "dash" => Some(Meta {
            deny: false,
            login: true,
            posix: true,
            ps: false,
        }),
        "fish" => Some(Meta {
            deny: true,
            login: true,
            posix: false,
            ps: false,
        }),
        "ksh" => Some(Meta {
            deny: false,
            login: true,
            posix: true,
            ps: false,
        }),
        "nu" => Some(Meta {
            deny: true,
            login: false,
            posix: false,
            ps: false,
        }),
        "powershell" => Some(Meta {
            deny: false,
            login: false,
            posix: false,
            ps: true,
        }),
        "pwsh" => Some(Meta {
            deny: false,
            login: false,
            posix: false,
            ps: true,
        }),
        "sh" => Some(Meta {
            deny: false,
            login: true,
            posix: true,
            ps: false,
        }),
        "zsh" => Some(Meta {
            deny: false,
            login: true,
            posix: true,
            ps: false,
        }),
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub struct Item {
    pub path: String,
    pub name: String,
    pub acceptable: bool,
}

pub fn name(file: &str) -> String {
    let base = std::path::Path::new(file)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(file);
    base.to_lowercase()
}

pub fn login(file: &str) -> bool {
    meta_for(&name(file)).map(|m| m.login).unwrap_or(false)
}
pub fn posix(file: &str) -> bool {
    meta_for(&name(file)).map(|m| m.posix).unwrap_or(false)
}
pub fn ps(file: &str) -> bool {
    meta_for(&name(file)).map(|m| m.ps).unwrap_or(false)
}

pub fn args(file: &str, command: &str, cwd: &str) -> Vec<String> {
    let n = name(file);
    match n.as_str() {
        "nu" | "fish" => vec!["-c".to_string(), command.to_string()],
        "zsh" => vec!["-l".to_string(), "-c".to_string(), format!("[[ -f ~/.zshenv ]] && source ~/.zshenv >/dev/null 2>&1 || true\n[[ -f \"${{ZDOTDIR:-$HOME}}/.zshrc\" ]] && source \"${{ZDOTDIR:-$HOME}}/.zshrc\" >/dev/null 2>&1 || true\ncd -- \"{cwd}\"\neval {}", serde_json::to_string(command).unwrap_or_default()), "opencode".to_string(), cwd.to_string()],
        "bash" => vec!["-l".to_string(), "-c".to_string(), format!("shopt -s expand_aliases\n[[ -f ~/.bashrc ]] && source ~/.bashrc >/dev/null 2>&1 || true\ncd -- \"{cwd}\"\neval {}", serde_json::to_string(command).unwrap_or_default()), "opencode".to_string(), cwd.to_string()],
        "cmd" => vec!["/c".to_string(), command.to_string()],
        _ if ps(file) => vec!["-NoProfile".to_string(), "-Command".to_string(), command.to_string()],
        _ => vec!["-c".to_string(), command.to_string()],
    }
}

pub fn preferred(config_shell: Option<&str>) -> Option<String> {
    if let Some(s) = config_shell {
        return Some(s.to_string());
    }
    std::env::var("SHELL").ok()
}

pub fn acceptable(config_shell: Option<&str>) -> Option<String> {
    if let Some(s) = config_shell {
        if meta_for(&name(s)).map(|m| !m.deny).unwrap_or(true) {
            return Some(s.to_string());
        }
        return None;
    }
    std::env::var("SHELL")
        .ok()
        .filter(|s| meta_for(&name(s)).map(|m| !m.deny).unwrap_or(true))
}

// PROVISIONAL pending process killTree + which + gitbash + list (requires fs + process natives).
