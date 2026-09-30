//! Rust port of `src/main/logging.ts` (opencode v1.18.30).
//!
//! Fully ported: the size/retention constants, `safeLogName`, the log-file
//! resolution (`resolvePathFn`), the tail slicing, the ISO stamp
//! formatting, `isBrokenPipe`, `serverLogRoots`, and the collect-entry
//! naming. PROVISIONAL: the `electron-log` transports, `crashReporter`,
//! `netLog`, the zip export (`@zip.js/zip.js`), and every fs call.
//!
//! Original file: `packages/desktop/src/main/logging.ts`

pub const MAX_LOG_AGE_DAYS: u64 = 7;
pub const TAIL_LINES: usize = 1000;
pub const EXPORT_WINDOW_MS: u64 = 24 * 60 * 60 * 1000;
pub const MAX_EXPORT_FILE_SIZE: u64 = 50 * 1024 * 1024;
pub const NET_LOG_SIZE: u64 = 20 * 1024 * 1024;
pub const LOG_FILE_MAX_SIZE: u64 = 5 * 1024 * 1024;

pub const SCOPE_MAIN: &str = "main";
pub const SCOPE_RENDERER: &str = "renderer";
pub const NETWORK_NETLOG_NAME: &str = "network.netlog";
pub const MANIFEST_NAME: &str = "manifest.json";
pub const DEBUG_ZIP_PREFIX: &str = "opencode-debug-";

/// Mirrors `safeLogName`: anything outside `[a-z0-9_.-]` becomes `_`;
/// empty input falls back to `"main"`.
pub fn safe_log_name(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect();
    if cleaned.is_empty() {
        return SCOPE_MAIN.to_string();
    }
    cleaned
}

/// Mirrors the `resolvePathFn`: `{run}/{safeLogName(scope ?? (processType
/// === "renderer" ? "renderer" : "main"))}.log`.
pub fn resolve_log_file(run_dir: &str, scope: Option<&str>, process_type: Option<&str>) -> String {
    let name = scope.unwrap_or(if process_type == Some("renderer") {
        SCOPE_RENDERER
    } else {
        SCOPE_MAIN
    });
    format!("{}/{}.log", run_dir, safe_log_name(name))
}

/// Mirrors the `tail()` slicing over already-read file contents.
pub fn tail_lines(contents: &str) -> String {
    let lines: Vec<&str> = contents.split('\n').collect();
    let start = lines.len().saturating_sub(TAIL_LINES);
    lines[start..].join("\n")
}

/// Mirrors `stamp()`: `toISOString()` with `[-:]` stripped and the
/// fractional-seconds `Z` suffix removed.
pub fn format_stamp(iso: &str) -> String {
    let squashed: String = iso.chars().filter(|c| *c != '-' && *c != ':').collect();
    match squashed.find('.') {
        Some(index) if squashed[index..].ends_with('Z') => squashed[..index].to_string(),
        _ => squashed,
    }
}

/// Mirrors `isBrokenPipe`: an object with `code === "EPIPE"`.
pub fn is_broken_pipe(code: Option<&str>) -> bool {
    code == Some("EPIPE")
}

/// Mirrors `serverLogRoots()`: `{xdgData}/opencode/log` plus
/// `{userData}/opencode/log`, deduplicated.
pub fn server_log_roots(xdg_data_home: Option<&str>, home: &str, user_data: &str) -> Vec<String> {
    let xdg = xdg_data_home
        .map(str::to_string)
        .unwrap_or_else(|| format!("{}/.local/share", home));
    let mut roots = vec![
        format!("{}/opencode/log", xdg),
        format!("{}/opencode/log", user_data),
    ];
    roots.dedup();
    roots
}

/// Mirrors the collect-entry naming: `{prefix}/{relative}` with `\` → `/`.
pub fn collect_entry_name(prefix: &str, dir: &str, file: &str) -> String {
    let relative = file.strip_prefix(&format!("{}/", dir)).unwrap_or(file);
    format!("{}/{}", prefix, relative.replace('\\', "/"))
}

/// Mirrors the collect-entry freshness/size/type guards.
pub fn collect_entry_kept(mtime_ms: f64, cutoff_ms: f64, size: u64, file: &str) -> bool {
    if mtime_ms < cutoff_ms {
        return false;
    }
    if size > MAX_EXPORT_FILE_SIZE {
        return false;
    }
    if file.ends_with(".heapsnapshot") {
        return false;
    }
    true
}

/// Mirrors the 14-field debug `manifest()` shape.
#[derive(Debug, Clone)]
pub struct LogManifest {
    pub generated: String,
    pub version: String,
    pub name: String,
    pub packaged: bool,
    pub platform: String,
    pub arch: String,
    pub user_data: String,
    pub logs: String,
    pub current_run: String,
    pub crash_dumps: String,
    pub server_logs: Vec<String>,
    pub net_log: Option<String>,
}

// PROVISIONAL(packages/desktop/src/main/logging.ts): `initLogging`
// (`electron-log` file/console transports, `resolvePathFn`, run-directory
// setup, 7-day `cleanup`), `initCrashReporter` (`crashReporter.start({
// uploadToServer: false, compress: true })`), `startNetLog`/`exportDebugLogs`
// (`netLog`, `@zip.js/zip.js` `ZipWriter`, `shell.showItemInFolder`), the
// module-level `write(name, message, extra?, level?)` (needs the `run`
// directory + scoped logger), `tail()` (needs the active log file), and
// `getLogger`. Preserved source log text, byte-identical:
// - "crash reporter started", "net log started", "failed to stop net log",
//   "exporting debug logs", "failed to restart net log",
//   "failed to collect unresponsive sample" (see `unresponsive`),
//   "child process gone", "app render process gone".

pub fn init_logging() {
    unimplemented!("electron-log transports + fs binding")
}

pub fn init_crash_reporter() {
    unimplemented!("Electron crashReporter + fs binding")
}

pub fn start_net_log() {
    unimplemented!("Electron netLog binding")
}

pub fn export_debug_logs() -> String {
    unimplemented!("Electron netLog + zip.js + fs binding")
}

/// Mirrors the `electron-log` module-level `write(name, message, extra?, level?)`.
pub fn write_log(
    _name: &str,
    _message: &str,
    _extra: Option<serde_json::Value>,
    _level: Option<&str>,
) {
    // PROVISIONAL(packages/desktop/src/main/logging.ts): the real transport
    // is `electron-log` (file + console transports). It sits on the hot path
    // of the fully ported onboarding flow, so it is a no-op rather than a
    // panic; the source log text is preserved at every call site.
}

/// Mirrors the `electron-log` logger instance `getLogger()` returns.
#[derive(Debug, Default, Clone, Copy)]
pub struct Logger;

impl Logger {
    pub fn log(&self, _message: &str, _data: Option<serde_json::Value>) {}
    pub fn warn(&self, _message: &str, _data: Option<serde_json::Value>) {}
    pub fn error(&self, _message: &str, _data: Option<serde_json::Value>) {}
}

/// PROVISIONAL(packages/desktop/src/main/logging.ts): `getLogger()` returns a
/// scoped `electron-log` logger; the in-workspace stub keeps the pure-call
/// contract (`logger.log/warn/error(message, data?)`) available to callers.
pub fn get_logger() -> Logger {
    Logger
}

#[cfg(test)]
mod tests {
    // No `src/main/logging.test.ts` exists in the source; the cases below
    // pin the ported pure helpers to the source's inline behavior.
    use super::*;

    #[test]
    fn safe_log_name_sanitizes_scopes() {
        assert_eq!(safe_log_name("window"), "window");
        assert_eq!(safe_log_name("a/b:c"), "a_b_c");
        assert_eq!(safe_log_name(""), "main");
    }

    #[test]
    fn resolve_log_file_prefers_scope_then_process_type() {
        assert_eq!(resolve_log_file("/run", Some("pty"), None), "/run/pty.log");
        assert_eq!(
            resolve_log_file("/run", None, Some("renderer")),
            "/run/renderer.log"
        );
        assert_eq!(resolve_log_file("/run", None, None), "/run/main.log");
    }

    #[test]
    fn tail_lines_keeps_the_last_thousand() {
        let contents = (0..1005)
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        let tail = tail_lines(&contents);
        let lines: Vec<&str> = tail.split('\n').collect();
        assert_eq!(lines.len(), 1000);
        assert_eq!(lines[0], "5");
    }

    #[test]
    fn format_stamp_squashes_iso() {
        assert_eq!(format_stamp("2026-09-27T03:07:33.123Z"), "20260927T030733");
    }

    #[test]
    fn server_log_roots_dedupes_identical_dirs() {
        assert_eq!(
            server_log_roots(None, "/home/luke", "/data"),
            vec!["/home/luke/.local/share/opencode/log", "/data/opencode/log"]
        );
        assert_eq!(
            server_log_roots(Some("/data"), "/home/luke", "/data"),
            vec!["/data/opencode/log"]
        );
    }

    #[test]
    fn collect_entry_naming_normalizes_separators() {
        assert_eq!(
            collect_entry_name("desktop", "/logs/run", "/logs/run/main.log"),
            "desktop/main.log"
        );
    }
}
