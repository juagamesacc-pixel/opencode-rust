//! Rust port of `src/main/wsl/runtime.ts` (opencode v1.18.30).
//!
//! Fully ported: `wslArgs`, `summarize`, `firstLine`, `shellEscape`,
//! `detectOutputEncoding` (byte-level heuristic), the
//! `parseInstalledDistros`/`parseOnlineDistros` line parsers (hand-rolled
//! ASCII-whitespace matchers — no regex crate is available — reproducing the
//! source patterns `/^\s*(\*)?\s*(.*?)\s{2,}\S+\s+(\d+)\s*$/` and
//! `/^([A-Za-z0-9._-]+)\s{2,}(.+)$/` with lazy-split semantics),
//! `withTimeout`, the `resolveSystem32Command` path math, every literal
//! script/argument vector (`runPowerShell` flags, the elevated-install and
//! distro/opencode install commands, the `$HOME/.opencode` probe, the
//! `--version` probe, the bash/curl capability probes), and the
//! runtime/distro probe decision maps. PROVISIONAL: `runCommand`
//! (`node:child_process.spawn` + timeout guard), `runInteractiveCommand`
//! (native `node-pty`), `openWslTerminal` (detached `cmd.exe` spawn), and
//! every async probe/install entry point built on them.
//!
//! Original file: `packages/desktop/src/main/wsl/runtime.ts`

use crate::main::native_translations::native_t;
use crate::main::wsl::policy::wsl_terminal_args;
use crate::preload::types::{WslDistroProbe, WslInstalledDistro, WslOnlineDistro, WslRuntimeCheck};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandStream {
    Stdout,
    Stderr,
}

impl CommandStream {
    pub fn as_str(&self) -> &'static str {
        match self {
            CommandStream::Stdout => "stdout",
            CommandStream::Stderr => "stderr",
        }
    }
}

pub struct WslCommandLine {
    pub stream: CommandStream,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct WslCommandResult {
    pub code: Option<i32>,
    pub signal: Option<String>,
    pub stdout: String,
    pub stderr: String,
}

// PROVISIONAL(packages/desktop/src/main/wsl/runtime.ts): `AbortSignal`
// has no in-workspace binding; kept as an opaque placeholder so
// `RunWslOptions` keeps its `signal` field.
pub struct WslAbortSignal {
    _private: (),
}

#[derive(Debug, Clone, Default)]
pub struct RunWslOptions {
    pub timeout_ms: Option<u64>,
    pub signal: Option<WslAbortSignal>,
}

pub const DEFAULT_WSL_TIMEOUT_MS: u64 = 20_000;
pub const DEFAULT_WSL_INSTALL_TIMEOUT_MS: u64 = 15 * 60_000;

pub fn wsl_args(args: Vec<String>, distro: Option<&str>, user: Option<&str>) -> Vec<String> {
    let mut out = Vec::new();
    // Mirrors `...(distro ? ["-d", distro] : [])`: empty strings are falsy.
    if let Some(distro) = distro.filter(|distro| !distro.is_empty()) {
        out.push("-d".to_string());
        out.push(distro.to_string());
    }
    if let Some(user) = user.filter(|user| !user.is_empty()) {
        out.push("--user".to_string());
        out.push(user.to_string());
    }
    out.push("--".to_string());
    out.extend(args);
    out
}

pub fn with_timeout(opts: Option<RunWslOptions>, timeout_ms: u64) -> RunWslOptions {
    let mut next = opts.unwrap_or_default();
    if next.timeout_ms.is_none() {
        next.timeout_ms = Some(timeout_ms);
    }
    next
}

pub fn summarize(value: &str) -> String {
    value
        .split('\n')
        .map(|line| line.trim_end_matches('\r').trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn first_line(value: &str) -> Option<String> {
    value
        .split('\n')
        .map(|line| line.trim_end_matches('\r').trim())
        .find(|line| !line.is_empty())
        .map(str::to_string)
}

pub fn shell_escape(value: &str) -> String {
    format!("'{}'", value.replace('\'', r#"'"'"'"#))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputEncoding {
    Utf8,
    Utf16Le,
}

pub fn detect_output_encoding(chunk: &[u8]) -> OutputEncoding {
    if chunk.len() >= 2 && chunk[0] == 0xff && chunk[1] == 0xfe {
        return OutputEncoding::Utf16Le;
    }
    let pairs = chunk.len() / 2;
    if pairs < 2 {
        return OutputEncoding::Utf8;
    }
    let odd_zeroes = (0..pairs).filter(|index| chunk[index * 2 + 1] == 0).count();
    let even_zeroes = (0..pairs).filter(|index| chunk[index * 2] == 0).count();
    if odd_zeroes >= pairs.div_ceil(3) && even_zeroes * 2 <= odd_zeroes {
        OutputEncoding::Utf16Le
    } else {
        OutputEncoding::Utf8
    }
}

/// Mirrors `parseInstalledDistros`: `/^\s*(\*)?\s*(.*?)\s{2,}\S+\s+(\d+)\s*$/`
/// with lazy-name semantics (earliest split wins), `name` header skipped,
/// `version` via base-10 parse (`NaN` → `None`).
pub fn parse_installed_distros(output: &str) -> Vec<WslInstalledDistro> {
    let mut distros = Vec::new();
    for raw_line in output.split('\n') {
        let line = raw_line.strip_suffix('\r').unwrap_or(raw_line);
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let (is_default, rest) = match trimmed.strip_prefix('*') {
            Some(rest) => (true, rest.trim_start()),
            None => (false, trimmed),
        };
        let Some((name, version)) = split_installed_line(rest) else {
            continue;
        };
        let name = name.trim();
        if name.is_empty() || name.eq_ignore_ascii_case("name") {
            continue;
        }
        distros.push(WslInstalledDistro {
            name: name.to_string(),
            version: version.parse::<i64>().ok(),
            is_default,
        });
    }
    distros
}

fn is_space(byte: u8) -> bool {
    byte == b' ' || byte == b'\t'
}

fn split_installed_line(rest: &str) -> Option<(&str, &str)> {
    let bytes = rest.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if is_space(bytes[index]) {
            let mut run = index;
            while run < bytes.len() && is_space(bytes[run]) {
                run += 1;
            }
            if run - index >= 2 {
                let mut token = run;
                while token < bytes.len() && !is_space(bytes[token]) {
                    token += 1;
                }
                if token > run {
                    let mut gap = token;
                    while gap < bytes.len() && is_space(bytes[gap]) {
                        gap += 1;
                    }
                    if gap > token {
                        let digits = &rest[gap..];
                        if !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit()) {
                            return Some((&rest[..index], digits));
                        }
                    }
                }
            }
        }
        index += 1;
    }
    None
}

/// Mirrors `parseOnlineDistros`: `/^([A-Za-z0-9._-]+)\s{2,}(.+)$/` on the
/// trimmed line, `NAME` header skipped.
pub fn parse_online_distros(output: &str) -> Vec<WslOnlineDistro> {
    let mut distros = Vec::new();
    for raw_line in output.split('\n') {
        let line = raw_line.strip_suffix('\r').unwrap_or(raw_line);
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let name_len = trimmed
            .bytes()
            .take_while(|byte| {
                byte.is_ascii_alphanumeric() || *byte == b'.' || *byte == b'_' || *byte == b'-'
            })
            .count();
        if name_len == 0 {
            continue;
        }
        let after_name = &trimmed[name_len..];
        let spaces = after_name
            .bytes()
            .take_while(|byte| is_space(*byte))
            .count();
        if spaces < 2 {
            continue;
        }
        let name = &trimmed[..name_len];
        if name.eq_ignore_ascii_case("name") {
            continue;
        }
        let label = after_name[spaces..].trim();
        distros.push(WslOnlineDistro {
            name: name.to_string(),
            label: label.to_string(),
        });
    }
    distros
}

/// Mirrors the `version.code !== 0` decision map in `probeWslRuntime`.
pub fn interpret_runtime_probe(result: &WslCommandResult) -> WslRuntimeCheck {
    if result.code != Some(0) {
        let output = if !result.stderr.is_empty() {
            &result.stderr
        } else {
            &result.stdout
        };
        let summary = summarize(output);
        let error = if summary.is_empty() {
            native_t("desktop.wsl.error.unavailable", &[])
        } else {
            summary
        };
        return WslRuntimeCheck {
            available: false,
            version: None,
            error: Some(error),
        };
    }
    WslRuntimeCheck {
        available: true,
        version: first_line(&result.stdout),
        error: None,
    }
}

/// Mirrors the non-executable branch of `probeWslDistro`.
pub fn interpret_distro_probe_failure(name: &str, result: &WslCommandResult) -> WslDistroProbe {
    let output = if !result.stderr.is_empty() {
        &result.stderr
    } else {
        &result.stdout
    };
    let summary = summarize(output);
    let error = if summary.is_empty() {
        native_t("desktop.wsl.error.executeDistro", &[])
    } else {
        summary
    };
    WslDistroProbe {
        name: name.to_string(),
        can_execute: false,
        has_bash: false,
        has_curl: false,
        error: Some(error),
    }
}

/// Mirrors the success assembly at the end of `probeWslDistro`.
pub fn assemble_distro_probe(
    name: &str,
    bash_code: Option<i32>,
    bash_stdout: &str,
    curl_code: Option<i32>,
    curl_stdout: &str,
) -> WslDistroProbe {
    WslDistroProbe {
        name: name.to_string(),
        can_execute: true,
        has_bash: bash_code == Some(0) && summarize(bash_stdout) == "yes",
        has_curl: curl_code == Some(0) && summarize(curl_stdout) == "yes",
        error: None,
    }
}

/// Mirrors `summarize(result.stderr || result.stdout) || nativeT(key)` used
/// by the install/list entry points.
pub fn command_error(
    stderr: &str,
    stdout: &str,
    fallback_key: &str,
    params: &[(&str, String)],
) -> String {
    let summary = summarize(if !stderr.is_empty() { stderr } else { stdout });
    if summary.is_empty() {
        return native_t(fallback_key, params);
    }
    summary
}

/// Mirrors the `powershell.exe` argument vector in `runPowerShell`.
pub fn powershell_args(command: &str) -> Vec<String> {
    vec![
        "-NoProfile".to_string(),
        "-NonInteractive".to_string(),
        "-ExecutionPolicy".to_string(),
        "Bypass".to_string(),
        "-Command".to_string(),
        command.to_string(),
    ]
}

/// Mirrors the elevated `wsl --install --no-distribution` script in
/// `installWslRuntimeElevated` (joined with `"; "`).
pub fn elevated_install_script() -> String {
    [
        "$ErrorActionPreference = 'Stop'",
        "$process = Start-Process -FilePath 'wsl.exe' -Verb RunAs -ArgumentList @('--install','--no-distribution') -Wait -PassThru",
        "if ($null -ne $process.ExitCode) { exit $process.ExitCode }",
    ]
    .join("; ")
}

/// Mirrors the `wsl.exe --install` argument vector in `installWslDistro`.
pub fn distro_install_args(name: &str) -> Vec<String> {
    vec![
        "--install".to_string(),
        "-d".to_string(),
        name.to_string(),
        "--web-download".to_string(),
        "--no-launch".to_string(),
    ]
}

/// Mirrors the `opencode.ai/install` script in `installWslOpencode`.
pub fn opencode_install_script(version: &str) -> String {
    format!(
        "curl -fsSL https://opencode.ai/install | bash -s -- --version {}",
        shell_escape(version)
    )
}

pub const RESOLVE_OPENCODE_SCRIPT: &str = r#"if [ -x "$HOME/.opencode/bin/opencode" ]; then printf "%s\n" "$HOME/.opencode/bin/opencode"; fi"#;

/// Mirrors the version probe in `readWslCommandVersion`.
pub fn command_version_script(command: &str) -> String {
    format!("{} --version 2>/dev/null || true", shell_escape(command))
}

pub const BASH_CAPABILITY_SCRIPT: &str = "command -v bash >/dev/null && printf yes || printf no";
pub const CURL_CAPABILITY_SCRIPT: &str = "command -v curl >/dev/null && printf yes || printf no";
pub const DISTRO_EXEC_PROBE_ARGS: [&str; 1] = ["/bin/true"];

/// Mirrors `resolveSystem32Command` over injected env/existence checks
/// (`process.env.SystemRoot ?? process.env.windir` + `existsSync`).
pub fn resolve_system32_command(
    command: &str,
    system_root: Option<&str>,
    exists: &dyn Fn(&str) -> bool,
) -> String {
    let Some(root) = system_root else {
        return command.to_string();
    };
    let resolved = format!("{}\\System32\\{}", root, command);
    if exists(&resolved) {
        return resolved;
    }
    command.to_string()
}

// PROVISIONAL(packages/desktop/src/main/wsl/runtime.ts): needs
// `node:child_process.spawn` (`stdio: ignore/pipe/pipe`,
// `windowsHide: true`, `AbortSignal`), the `DEFAULT_WSL_TIMEOUT_MS`
// kill-guard, and the UTF-16LE/UTF-8 streaming decoders. Preserved source
// error text: `nativeT("desktop.wsl.error.commandTimeout",
// { command, args, timeout })`.
pub fn run_wsl(_args: Vec<String>, _opts: Option<RunWslOptions>) -> WslCommandResult {
    unimplemented!("node:child_process.spawn binding")
}

// PROVISIONAL(packages/desktop/src/main/wsl/runtime.ts): `wsl.exe`
// flavor of `runCommand` (`runWslInDistro`/`runWslSh` arg building is
// `wsl_args`, ported above).
pub fn run_wsl_in_distro(
    _args: Vec<String>,
    _distro: Option<&str>,
    _opts: Option<RunWslOptions>,
) -> WslCommandResult {
    unimplemented!("node:child_process.spawn binding")
}

// PROVISIONAL(packages/desktop/src/main/wsl/runtime.ts): needs the native
// `node-pty` addon (`pty.spawn` with `xterm-color`, 80×24, conpty) plus
// timeout/abort handling. Preserved source error text:
// `nativeT("desktop.wsl.error.commandTimeout", …)` and the
// `DOMException("Aborted", "AbortError")` abort path.
pub fn run_interactive_command(
    _command: &str,
    _args: Vec<String>,
    _opts: Option<RunWslOptions>,
    _default_timeout_ms: u64,
) -> WslCommandResult {
    unimplemented!("native node-pty binding")
}

// PROVISIONAL(packages/desktop/src/main/wsl/runtime.ts): needs
// `node:child_process.spawn` (`cmd.exe` detached, args from
// `wsl_terminal_args`, `windowsHide: true`).
pub fn open_wsl_terminal(_distro: Option<&str>) {
    unimplemented!("node:child_process.spawn binding")
}

// PROVISIONAL(packages/desktop/src/main/wsl/runtime.ts): async entry
// points over the stubs above — `probeWslRuntime`,
// `listInstalledWslDistros`, `listOnlineWslDistros`,
// `installWslRuntimeElevated`, `installWslDistro`, `installWslOpencode`,
// `probeWslDistro`, `resolveWslOpencode`, `readWslCommandVersion`.
// Their decision maps are `interpret_runtime_probe`,
// `interpret_distro_probe_failure`, `assemble_distro_probe`, and
// `command_error` above. Preserved `nativeT` keys:
// `desktop.wsl.error.unavailable`, `.listInstalled`, `.listOnline`,
// `.executeDistro`, `.installWsl`, `.installDistro`, `.installOpencode`.
pub fn probe_wsl_runtime(_opts: Option<RunWslOptions>) -> WslRuntimeCheck {
    unimplemented!("node:child_process.spawn binding")
}

#[cfg(test)]
mod tests {
    // No `src/main/wsl/runtime.test.ts` exists in the source; the cases
    // below pin the ported pure helpers to the source's inline behavior.
    use super::*;

    #[test]
    fn wsl_args_prefixes_distro_and_user() {
        assert_eq!(
            wsl_args(vec!["bash".to_string()], Some("Debian"), None),
            vec!["-d", "Debian", "--", "bash"]
                .into_iter()
                .map(str::to_string)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            wsl_args(vec!["bash".to_string()], None, None),
            vec!["--", "bash"]
                .into_iter()
                .map(str::to_string)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            wsl_args(vec![], Some("Debian"), Some("luke"))[..3],
            ["-d".to_string(), "Debian".to_string(), "--user".to_string()]
        );
    }

    #[test]
    fn summarize_joins_trimmed_non_empty_lines() {
        assert_eq!(summarize("  a  \r\n\r\nb\n"), "a\nb");
        assert_eq!(summarize("   "), "");
    }

    #[test]
    fn first_line_returns_the_first_non_empty_line() {
        assert_eq!(
            first_line("\n  WSL version: 2.6.1 \nmore"),
            Some("WSL version: 2.6.1".to_string())
        );
        assert_eq!(first_line("  \n "), None);
    }

    #[test]
    fn shell_escape_wraps_single_quotes() {
        assert_eq!(shell_escape("1.16.2"), "'1.16.2'");
        assert_eq!(shell_escape("a'b"), "'a'\"'\"'b'");
    }

    #[test]
    fn detect_output_encoding_spots_utf16le() {
        assert_eq!(
            detect_output_encoding(&[0xff, 0xfe, 0x57, 0x00]),
            OutputEncoding::Utf16Le
        );
        // "W S L" with every second byte zero: 3 of 4 odd bytes zero.
        assert_eq!(
            detect_output_encoding(&[0x57, 0x00, 0x53, 0x00, 0x4c, 0x00, 0x20, 0x00]),
            OutputEncoding::Utf16Le
        );
        assert_eq!(detect_output_encoding(b"WSL version"), OutputEncoding::Utf8);
        assert_eq!(detect_output_encoding(&[]), OutputEncoding::Utf8);
    }

    #[test]
    fn parse_installed_distros_reads_verbose_table() {
        let output = "  NAME                   STATE           VERSION\r\n* Ubuntu-24.04         Running         2\r\n  Debian               Stopped         2\r\n";
        let distros = parse_installed_distros(output);
        assert_eq!(distros.len(), 2);
        assert_eq!(distros[0].name, "Ubuntu-24.04");
        assert_eq!(distros[0].version, Some(2));
        assert!(distros[0].is_default);
        assert_eq!(distros[1].name, "Debian");
        assert!(!distros[1].is_default);
    }

    #[test]
    fn parse_online_distros_reads_name_label_table() {
        let output =
            "NAME      FRIENDLY NAME\r\nDebian    Debian GNU/Linux\r\nUbuntu    Ubuntu\r\n";
        let distros = parse_online_distros(output);
        assert_eq!(distros.len(), 2);
        assert_eq!(distros[0].name, "Debian");
        assert_eq!(distros[0].label, "Debian GNU/Linux");
    }
}
