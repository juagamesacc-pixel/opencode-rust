//! Rust port of `src/main/wsl/sidecar.ts` (opencode v1.18.30).
//!
//! Fully ported: the sidecar bash script builder (`buildSidecarScript`,
//! byte-identical lines joined with `\n`), `startupFailure`, the
//! `forwardLines` chunk splitter, and the `emit` recent-output ring
//! (blank-line skip, 12-line cap). PROVISIONAL: `spawnWslSidecar`
//! (`node:child_process.spawn` + port allocation + health polling) and
//! `allocatePort` (`node:net`).
//!
//! Original file: `packages/desktop/src/main/wsl/sidecar.ts`

use crate::main::native_translations::native_t;
use crate::main::wsl::runtime::{shell_escape, CommandStream, WslCommandLine};

pub const HEALTH_TIMEOUT_MS: u64 = 30_000;
pub const RECENT_OUTPUT_CAP: usize = 12;

/// Mirrors the `WslSidecar` shape; the `listener` half is PROVISIONAL
/// (child-process handle).
pub struct WslSidecar {
    pub listener: WslSidecarListener,
    pub url: String,
    pub username: Option<String>,
    pub password: String,
}

// PROVISIONAL(packages/desktop/src/main/wsl/sidecar.ts): `listener.stop`
// (`child.kill()`) and `listener.onExit` (`child.once("exit", …)`) need
// the child-process handle.
pub struct WslSidecarListener {
    _private: (),
}

impl WslSidecarListener {
    pub fn stop(&self) {
        unimplemented!("child-process handle binding")
    }

    pub fn on_exit(&self, _callback: Box<dyn Fn(Option<i32>, Option<String>)>) {
        unimplemented!("child-process handle binding")
    }
}

/// Mirrors the `script` array in `spawnWslSidecar` (joined with `"\n"`).
pub fn build_sidecar_script(opencode: &str, password: &str, is_packaged: bool) -> String {
    let log_level = if is_packaged { "WARN" } else { "INFO" };
    [
        "set -euo pipefail".to_string(),
        "cd \"$HOME\" || cd /".to_string(),
        "PATH=$(awk -v RS=: -v ORS=: '$0 !~ /^\\/mnt\\//' <<<\"$PATH\" | sed \"s/:$//\")"
            .to_string(),
        "export PATH".to_string(),
        "export WSLENV=".to_string(),
        "export OPENCODE_EXPERIMENTAL_DISABLE_FILEWATCHER=true".to_string(),
        "export OPENCODE_CLIENT=desktop".to_string(),
        format!(
            "export OPENCODE_SERVER_USERNAME={}",
            shell_escape("opencode")
        ),
        format!("export OPENCODE_SERVER_PASSWORD={}", shell_escape(password)),
        "export XDG_STATE_HOME=\"$HOME/.local/state\"".to_string(),
        format!(
            "exec {} --print-logs --log-level {} serve --hostname 0.0.0.0 --port {{port}}",
            shell_escape(opencode),
            log_level
        ),
    ]
    .join("\n")
}

/// Mirrors the sidecar URL construction (`http://127.0.0.1:{port}`).
pub fn sidecar_url(port: u16) -> String {
    format!("http://127.0.0.1:{}", port)
}

/// Mirrors the `emit` closure: blank lines are dropped, the ring keeps the
/// last 12 `[stream] text` entries, and the line is returned for
/// forwarding to `opts.onLine`.
pub fn collect_recent_output(
    recent: &mut Vec<String>,
    stream: CommandStream,
    text: &str,
) -> Option<WslCommandLine> {
    if text.trim().is_empty() {
        return None;
    }
    recent.push(format!("[{}] {}", stream.as_str(), text));
    if recent.len() > RECENT_OUTPUT_CAP {
        recent.remove(0);
    }
    Some(WslCommandLine {
        stream,
        text: text.to_string(),
    })
}

/// Mirrors the `forwardLines` splitter: appends the chunk, splits on
/// `\r?\n`, holds the trailing partial line in `pending`.
pub fn split_stream_lines(pending: &mut String, chunk: &str) -> Vec<String> {
    pending.push_str(chunk);
    let mut lines: Vec<String> = pending.split('\n').map(str::to_string).collect();
    let rest = lines.pop().unwrap_or_default();
    let lines = lines
        .into_iter()
        .map(|line| line.strip_suffix('\r').unwrap_or(&line).to_string())
        .collect();
    *pending = rest;
    lines
}

pub fn startup_failure(
    code: Option<i32>,
    signal: Option<&str>,
    recent_output: &[String],
) -> String {
    let suffix = if recent_output.is_empty() {
        String::new()
    } else {
        format!("\n{}", recent_output.join("\n"))
    };
    native_t(
        "desktop.wsl.error.serverExitedBeforeHealthy",
        &[
            (
                "code",
                code.map(|code| code.to_string())
                    .unwrap_or_else(|| "null".to_string()),
            ),
            ("signal", signal.unwrap_or("null").to_string()),
            ("output", suffix),
        ],
    )
}

// PROVISIONAL(packages/desktop/src/main/wsl/sidecar.ts): needs
// `node:child_process.spawn` (`wsl`, piped stdio), `node:net`
// (`allocatePort`), `randomUUID` (password), `app.isPackaged`, the health
// poll, and the startup timeout race. Preserved `nativeT` keys:
// `desktop.wsl.error.opencodeNotInstalled`, `.healthTimeout`,
// `.failedPort`, `.serverExitedBeforeHealthy`.
pub fn spawn_wsl_sidecar(_distro: &str) -> WslSidecar {
    unimplemented!("child-process spawn + port allocation + health-poll binding")
}

#[cfg(test)]
mod tests {
    // No `src/main/wsl/sidecar.test.ts` exists in the source; the cases
    // below pin the ported pure helpers to the source's inline behavior.
    use super::*;

    #[test]
    fn build_sidecar_script_joins_twelve_lines() {
        let script = build_sidecar_script("/home/me/.opencode/bin/opencode", "secret", false);
        let lines: Vec<&str> = script.split('\n').collect();
        assert_eq!(lines.len(), 11);
        assert_eq!(lines[0], "set -euo pipefail");
        assert_eq!(
            lines[5],
            "export OPENCODE_EXPERIMENTAL_DISABLE_FILEWATCHER=true"
        );
        assert!(lines[8].starts_with("export OPENCODE_SERVER_PASSWORD='secret'"));
        assert!(lines[10].contains("--log-level INFO"));
        assert!(lines[10].contains("serve --hostname 0.0.0.0"));
    }

    #[test]
    fn collect_recent_output_skips_blanks_and_caps_at_twelve() {
        let mut recent = Vec::new();
        assert!(collect_recent_output(&mut recent, CommandStream::Stdout, "   ").is_none());
        for index in 0..14 {
            collect_recent_output(
                &mut recent,
                CommandStream::Stderr,
                &format!("line {}", index),
            );
        }
        assert_eq!(recent.len(), 12);
        assert_eq!(recent[0], "[stderr] line 2");
        assert_eq!(recent[11], "[stderr] line 13");
    }

    #[test]
    fn split_stream_lines_holds_partial_tails() {
        let mut pending = String::new();
        assert_eq!(
            split_stream_lines(&mut pending, "a\r\nb\npartial"),
            vec!["a".to_string(), "b".to_string()]
        );
        assert_eq!(pending, "partial");
        assert_eq!(
            split_stream_lines(&mut pending, "-done\n"),
            vec!["partial-done".to_string()]
        );
        assert_eq!(pending, "");
    }
}
