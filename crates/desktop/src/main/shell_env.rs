//! Rust port of `src/main/shell-env.ts` (opencode v1.18.30).
//!
//! `probe()` needs `node:child_process.spawnSync` and `getUserShell()`
//! needs `node:os.userInfo()`; neither has an in-workspace binding, so both
//! are PROVISIONAL. `load_shell_env_with` runs the full `loadShellEnv`
//! decision tree (including every `[server] …` log string) over an injected
//! probe so the logic stays testable without a subprocess runtime.
//!
//! Original file: `packages/desktop/src/main/shell-env.ts`

use std::collections::HashMap;

pub const TIMEOUT: u64 = 5_000;

pub enum Probe {
    Loaded(HashMap<String, String>),
    Timeout,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProbeMode {
    InteractiveLogin,
    Login,
}

impl ProbeMode {
    pub fn flag(&self) -> &'static str {
        match self {
            ProbeMode::InteractiveLogin => "-il",
            ProbeMode::Login => "-l",
        }
    }
}

pub fn resolve_user_shell(env_shell: Option<&str>, login_shell: Option<&str>) -> String {
    let env = env_shell.filter(|shell| !shell.is_empty());
    let login = login_shell.filter(|shell| !shell.is_empty() && *shell != "unknown");
    env.or(login).unwrap_or("/bin/sh").to_string()
}

// PROVISIONAL(packages/desktop/src/main/shell-env.ts): `userInfo().shell`
// (node:os) has no in-workspace binding. The `SHELL` env read is ported;
// the login-shell lookup is not.
pub fn get_user_shell() -> String {
    let env_shell = std::env::var("SHELL").ok();
    resolve_user_shell(env_shell.as_deref(), None)
}

pub fn parse_shell_env(out: &[u8]) -> HashMap<String, String> {
    let mut env = HashMap::new();
    // Mirrors `out.toString("utf8")`: invalid UTF-8 becomes U+FFFD, which
    // can never equal `=` or `\0`, so lossy conversion is equivalent here.
    for line in String::from_utf8_lossy(out).split('\0') {
        if line.is_empty() {
            continue;
        }
        let ix = line.find('=').map(|index| index as isize).unwrap_or(-1);
        if ix <= 0 {
            continue;
        }
        let ix = ix as usize;
        env.insert(line[..ix].to_string(), line[ix + 1..].to_string());
    }
    env
}

// PROVISIONAL(packages/desktop/src/main/shell-env.ts): needs
// `node:child_process.spawnSync` (`stdio: ignore/pipe/ignore`,
// `timeout: TIMEOUT`, `windowsHide: true`) plus the `[server] …` console
// logging on failure paths. Signature mirrors `probe(shell, mode)`.
pub fn probe_shell(_shell: &str, _mode: ProbeMode) -> Probe {
    unimplemented!("node:child_process.spawnSync binding")
}

pub fn is_nushell(shell: &str) -> bool {
    let name = shell
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or("")
        .to_ascii_lowercase();
    let raw = shell.to_ascii_lowercase();
    name == "nu" || name == "nu.exe" || raw.ends_with("\\nu.exe")
}

pub fn load_shell_env_with(
    probe: &dyn Fn(&str, ProbeMode) -> Probe,
    shell: &str,
    logger: &mut dyn FnMut(String),
) -> Option<HashMap<String, String>> {
    if is_nushell(shell) {
        logger(format!(
            "[server] Skipping shell env probe for nushell: {}",
            shell
        ));
        return None;
    }

    match probe(shell, ProbeMode::InteractiveLogin) {
        Probe::Loaded(value) => {
            logger(format!(
                "[server] Loaded shell environment with -il ({} vars)",
                value.len()
            ));
            return Some(value);
        }
        Probe::Timeout => {
            logger(format!("Interactive shell env probe timed out: {}", shell));
            return None;
        }
        Probe::Unavailable => {}
    }

    match probe(shell, ProbeMode::Login) {
        Probe::Loaded(value) => {
            logger(format!(
                "[server] Loaded shell environment with -l ({} vars)",
                value.len()
            ));
            return Some(value);
        }
        Probe::Unavailable | Probe::Timeout => {}
    }

    logger(format!(
        "[server] Falling back to app environment: {}",
        shell
    ));
    None
}

// PROVISIONAL(packages/desktop/src/main/shell-env.ts): delegates to the
// real subprocess `probe_shell` once the binding exists.
pub fn load_shell_env(
    _shell: &str,
    _logger: &mut dyn FnMut(String),
) -> Option<HashMap<String, String>> {
    unimplemented!("node:child_process.spawnSync binding")
}

pub fn merge_shell_env(
    shell: Option<HashMap<String, String>>,
    env: HashMap<String, String>,
) -> HashMap<String, String> {
    let mut merged = shell.unwrap_or_default();
    merged.extend(env);
    merged
}

#[cfg(test)]
mod tests {
    // Mirrors `src/main/shell-env.test.ts` (`describe("shell env")`).
    use super::*;

    #[test]
    fn parse_shell_env_supports_null_delimited_pairs() {
        let env = parse_shell_env(b"PATH=/usr/bin:/bin\0FOO=bar=baz\0\0");
        assert_eq!(env.get("PATH").map(String::as_str), Some("/usr/bin:/bin"));
        assert_eq!(env.get("FOO").map(String::as_str), Some("bar=baz"));
    }

    #[test]
    fn parse_shell_env_ignores_invalid_entries() {
        let env = parse_shell_env(b"INVALID\0=empty\0OK=1\0");
        assert_eq!(env.len(), 1);
        assert_eq!(env.get("OK").map(String::as_str), Some("1"));
    }

    #[test]
    fn merge_shell_env_keeps_explicit_overrides() {
        let env = merge_shell_env(
            HashMap::from([
                ("PATH".to_string(), "/shell/path".to_string()),
                ("HOME".to_string(), "/tmp/home".to_string()),
            ]),
            HashMap::from([
                ("PATH".to_string(), "/desktop/path".to_string()),
                ("OPENCODE_CLIENT".to_string(), "desktop".to_string()),
            ]),
        );
        assert_eq!(env.get("PATH").map(String::as_str), Some("/desktop/path"));
        assert_eq!(env.get("HOME").map(String::as_str), Some("/tmp/home"));
        assert_eq!(
            env.get("OPENCODE_CLIENT").map(String::as_str),
            Some("desktop")
        );
    }

    #[test]
    fn resolve_user_shell_falls_back_to_the_login_shell_before_bin_sh() {
        assert_eq!(
            resolve_user_shell(Some("/custom/env-shell"), Some("/bin/zsh")),
            "/custom/env-shell"
        );
        assert_eq!(resolve_user_shell(None, Some("/bin/zsh")), "/bin/zsh");
        assert_eq!(resolve_user_shell(None, Some("unknown")), "/bin/sh");
        assert_eq!(resolve_user_shell(None, None), "/bin/sh");
    }

    #[test]
    fn is_nushell_handles_path_and_binary_name() {
        assert!(is_nushell("nu"));
        assert!(is_nushell("/opt/homebrew/bin/nu"));
        assert!(is_nushell("C:\\Program Files\\nu.exe"));
        assert!(!is_nushell("/bin/zsh"));
    }
}
