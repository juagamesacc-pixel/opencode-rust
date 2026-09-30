//! Rust port of `src/main/background-cli.ts` (opencode v1.18.30).
//!
//! Fully ported: `serviceUrl`, `endpoint`, `executableName`, the
//! `version.replace(/[^a-zA-Z0-9._-]/g, "-")` sanitization, the
//! `XDG_STATE_HOME` env override from `run()`, and the stdout redaction
//! decision. PROVISIONAL: `startBackgroundCli`/`installCli`/`run`
//! (child-process exec + fs + `app.getPath`/`process.resourcesPath`).
//!
//! Original file: `packages/desktop/src/main/background-cli.ts`

use std::collections::{HashMap, HashSet};

pub const DESKTOP_STATE_NAMES: [&str; 3] = [
    "ai.opencode.desktop.dev",
    "ai.opencode.desktop.beta",
    "ai.opencode.desktop",
];

pub const SIDECAR_USERNAME: &str = "opencode";

pub struct CliEndpoint {
    pub url: String,
    pub username: String,
    pub password: String,
}

pub struct EndpointParts {
    pub url: String,
    pub hostname: String,
    pub port: String,
}

/// Mirrors `serviceUrl(status)`: a bare URL passes through; otherwise a
/// `running <url>` status line is unwrapped. Uses the same absolute-URL
/// subset check as `URL.canParse` for these shapes.
pub fn service_url(status: &str) -> Option<String> {
    if can_parse_absolute_url(status) {
        return Some(status.to_string());
    }
    if !status.starts_with("running ") {
        return None;
    }
    let url = status["running ".len()..].trim();
    if can_parse_absolute_url(url) {
        return Some(url.to_string());
    }
    None
}

/// Mirrors `endpoint(url)`: `{ url, hostname, port }`, or `None` (the
/// source's `{}`) when the URL does not parse.
pub fn endpoint(url: Option<&str>) -> Option<EndpointParts> {
    let url = url?;
    if !can_parse_absolute_url(url) {
        return None;
    }
    let after_scheme = &url[url.find("://")? + 3..];
    let authority_end = after_scheme.find('/').unwrap_or(after_scheme.len());
    let authority = &after_scheme[..authority_end];
    let host = authority.rsplit('@').next().unwrap_or("");
    let (hostname, port) = match host.rfind(':') {
        Some(index) => (host[..index].to_string(), host[index + 1..].to_string()),
        None => (host.to_string(), String::new()),
    };
    Some(EndpointParts {
        url: url.to_string(),
        hostname,
        port,
    })
}

fn can_parse_absolute_url(value: &str) -> bool {
    let scheme_end = match value.find("://") {
        Some(index) if index > 0 => index,
        _ => return false,
    };
    let scheme = &value[..scheme_end];
    let mut chars = scheme.chars();
    match chars.next() {
        Some(first) if first.is_ascii_alphabetic() => (),
        _ => return false,
    }
    if !chars.all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '-' || c == '.') {
        return false;
    }
    !value[scheme_end + 3..].is_empty()
}

pub fn executable_name() -> &'static str {
    if std::env::consts::OS == "windows" {
        return "opencode-cli.exe";
    }
    "opencode-cli"
}

/// Mirrors `version.replace(/[^a-zA-Z0-9._-]/g, "-")` for the staged
/// CLI directory name.
pub fn sanitize_version_dir(version: &str) -> String {
    version
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-' {
                c
            } else {
                '-'
            }
        })
        .collect()
}

/// Mirrors the state-home candidate dedup in `startBackgroundCli`:
/// `[stateHome, shellStateHome, ...desktopStateNames.map(…)]` through a
/// `Set`, keeping `undefined` entries for the later `existsSync` filter.
pub fn state_home_candidates(
    state_home: Option<String>,
    shell_state_home: Option<String>,
    app_data: &str,
) -> Vec<Option<String>> {
    let mut seen = HashSet::new();
    let mut candidates = Vec::new();
    let mut push = |candidate: Option<String>| {
        if seen.insert(candidate.clone()) {
            candidates.push(candidate);
        }
    };
    push(state_home);
    push(shell_state_home);
    for name in DESKTOP_STATE_NAMES {
        push(Some(format!("{}/{}", app_data, name)));
    }
    candidates
}

/// Mirrors the `XDG_STATE_HOME` handling in `run()`: `undefined` deletes
/// the var, otherwise it is overridden.
pub fn apply_cli_state_home(env: &mut HashMap<String, String>, state_home: Option<&str>) {
    match state_home {
        Some(state_home) => {
            env.insert("XDG_STATE_HOME".to_string(), state_home.to_string());
        }
        None => {
            env.remove("XDG_STATE_HOME");
        }
    }
}

/// Mirrors the `options.redact ? "[redacted]" : stdout` decision in `run()`.
pub fn redact_stdout<'a>(stdout: &'a str, redact: bool) -> &'a str {
    if redact {
        return "[redacted]";
    }
    stdout
}

// PROVISIONAL(packages/desktop/src/main/background-cli.ts): needs
// child-process exec (`execFile`), fs (`existsSync`/`chmod`/`copyFile`/
// `mkdir`/`rename`/`rm`), and Electron (`app.isPackaged`,
// `app.getPath`, `process.resourcesPath`). Preserved source log text,
// byte-identical:
// - "v2 CLI executable resolved"
// - "v2 CLI background instance checked"
// - "v2 CLI background service ready"
// - "v2 CLI staged executable reused"
// - "v2 CLI executable staged"
// - "v2 CLI command started"
// - "v2 CLI command completed"
// - "v2 CLI command failed"
pub fn start_background_cli(_shell_state_home: Option<&str>) -> CliEndpoint {
    unimplemented!("child-process exec + fs + Electron app binding")
}

#[cfg(test)]
mod tests {
    // No `src/main/background-cli.test.ts` exists in the source; the cases
    // below pin the ported pure helpers to the source's inline behavior.
    use super::*;

    #[test]
    fn service_url_unwraps_running_status_lines() {
        assert_eq!(
            service_url("http://127.0.0.1:4096"),
            Some("http://127.0.0.1:4096".to_string())
        );
        assert_eq!(
            service_url("running http://127.0.0.1:4096"),
            Some("http://127.0.0.1:4096".to_string())
        );
        assert_eq!(service_url("stopped"), None);
        assert_eq!(service_url("running not-a-url"), None);
    }

    #[test]
    fn endpoint_splits_hostname_and_port() {
        let parts = endpoint(Some("http://127.0.0.1:4096")).expect("parses");
        assert_eq!(parts.hostname, "127.0.0.1");
        assert_eq!(parts.port, "4096");
        assert_eq!(parts.url, "http://127.0.0.1:4096");
        assert!(endpoint(None).is_none());
        assert!(endpoint(Some("not-a-url")).is_none());
    }

    #[test]
    fn sanitize_version_dir_replaces_unsafe_chars() {
        assert_eq!(sanitize_version_dir("1.16.2"), "1.16.2");
        assert_eq!(sanitize_version_dir("1.16.2+build/foo"), "1.16.2-build-foo");
    }

    #[test]
    fn state_home_candidates_dedupe_keeping_undefined() {
        let candidates = state_home_candidates(None, Some("/state".to_string()), "/appdata");
        assert_eq!(
            candidates,
            vec![
                None,
                Some("/state".to_string()),
                Some("/appdata/ai.opencode.desktop.dev".to_string()),
                Some("/appdata/ai.opencode.desktop.beta".to_string()),
                Some("/appdata/ai.opencode.desktop".to_string()),
            ]
        );
    }

    #[test]
    fn apply_cli_state_home_sets_or_deletes() {
        let mut env = HashMap::from([("XDG_STATE_HOME".to_string(), "/old".to_string())]);
        apply_cli_state_home(&mut env, Some("/new"));
        assert_eq!(env.get("XDG_STATE_HOME").map(String::as_str), Some("/new"));
        apply_cli_state_home(&mut env, None);
        assert!(!env.contains_key("XDG_STATE_HOME"));
    }

    #[test]
    fn redact_stdout_hides_secrets() {
        assert_eq!(redact_stdout("secret", true), "[redacted]");
        assert_eq!(redact_stdout("secret", false), "secret");
    }
}
