// source: packages/tui/src/util/system.ts (20 lines, v1.18.30)
// 1:1 port — platform/arch mapping and terminal description verbatim;
// kernel release reads the same `os.release()` value via procfs.

#![allow(dead_code)]

/// Mirrors `describeOS` (`Name release (arch)`).
pub fn describe_os() -> String {
    let name = match std::env::consts::OS {
        "macos" => "macOS",
        "windows" => "Windows",
        "linux" => "Linux",
        other => other,
    };
    format!("{} {} ({})", name, os_release(), std::env::consts::ARCH)
}

fn os_release() -> String {
    std::fs::read_to_string("/proc/sys/kernel/osrelease")
        .map(|s| s.trim().to_string())
        .unwrap_or_default()
}

/// Mirrors `describeTerminal` (env vars verbatim).
pub fn describe_terminal() -> String {
    let program = std::env::var("TERM_PROGRAM")
        .or_else(|_| std::env::var("TERM"))
        .unwrap_or_else(|_| "unknown".to_string());
    let version = std::env::var("TERM_PROGRAM_VERSION")
        .map(|v| format!(" {v}"))
        .unwrap_or_default();
    let multiplexer = if std::env::var("TMUX").is_ok() {
        " in tmux"
    } else if std::env::var("STY").is_ok() {
        " in screen"
    } else {
        ""
    };
    format!("{program}{version}{multiplexer}")
}
