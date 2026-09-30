//! Rust port of `src/main/apps.ts` (opencode v1.18.30).
//!
//! Ported exactly: the platform gates in `checkAppExists` / `resolveAppPath`,
//! the macOS search-path construction (including the `HOME` third location),
//! the `where` output normalization, the `.cmd`/`.bat` `%~dp0` resolution
//! (slash → backslash, `.`/`..` path collapsing), the extensionless
//! `.cmd`/`.bat` probe, the alphanumeric-key derivation, and the
//! substring-matching scan over three parent directories.
//!
//! PROVISIONAL: `access`/`readFile`/`readdir` (Node `fs`) and `execFile`
//! (Node `child_process`) have no in-workspace Rust binding, so the two
//! entry points and the existence probes are PROVISIONAL while the pure
//! string/path logic is ported and tested.
//!
//! Original file: `packages/desktop/src/main/apps.ts`

use crate::main::migrate::Platform;

/// Mirrors the `exists(path)` helper's decision input.
pub trait PathProbe {
    fn exists(&self, path: &str) -> bool;
}

/// Mirrors `checkAppExists(appName)`'s platform gate: Windows and Linux
/// always answer `true`; only macOS probes.
pub fn check_app_exists_platform(
    platform: Platform,
    app_name: &str,
    probe: &dyn PathProbe,
) -> bool {
    match platform {
        Platform::Win32 | Platform::Other => true,
        Platform::Darwin => check_macos_app(app_name, probe),
    }
}

/// Mirrors `resolveAppPath(appName)`'s platform gate: a non-Windows platform
/// returns the name unchanged; Windows falls through to
/// [`resolve_windows_app_path`]. The `where` output is supplied by the
/// caller because `execFile` is PROVISIONAL.
pub fn resolve_app_path_platform(
    platform: Platform,
    app_name: &str,
    where_output: Option<&str>,
    probe: &dyn PathProbe,
    read_file: &dyn Fn(&str) -> String,
    read_dir: &dyn Fn(&str) -> Vec<String>,
) -> Option<String> {
    if platform != Platform::Win32 {
        return Some(app_name.to_string());
    }
    let paths = split_where_output(where_output?);
    resolve_windows_app_path(app_name, &paths, probe, read_file, read_dir)
}

/// Mirrors the `locations` array of `checkMacosApp`, in order. `home` is the
/// `process.env.HOME` read; a `None` omits the third entry.
pub fn macos_app_locations(app_name: &str, home: Option<&str>) -> Vec<String> {
    let mut locations = vec![
        format!("/Applications/{}.app", app_name),
        format!("/System/Applications/{}.app", app_name),
    ];
    if let Some(home) = home.filter(|home| !home.is_empty()) {
        locations.push(format!("{}/Applications/{}.app", home, app_name));
    }
    locations
}

/// Mirrors `checkMacosApp(appName)`.
fn check_macos_app(app_name: &str, probe: &dyn PathProbe) -> bool {
    for location in macos_app_locations(app_name, std::env::var("HOME").ok().as_deref()) {
        if probe.exists(&location) {
            return true;
        }
    }
    // The source's final fallback is `execFilePromise("which", [appName])`.
    false
}

/// Mirrors the `hasExt(path, ext)` helper: `extname(path).toLowerCase()`.
pub fn has_ext(path: &str, ext: &str) -> bool {
    extname(path).to_lowercase() == format!(".{}", ext)
}

/// Mirrors `node:path.extname(path)` for the ASCII paths `where` returns.
pub fn extname(path: &str) -> String {
    let base = path.rsplit(['/', '\\']).next().unwrap_or(path);
    match base.rfind('.') {
        // A leading dot is not an extension separator in `path.extname`.
        Some(0) | None => String::new(),
        Some(index) => base[index..].to_string(),
    }
}

/// Mirrors `node:path.dirname(path)`.
pub fn dirname(path: &str) -> String {
    let trimmed = path.trim_end_matches(['/', '\\']);
    match trimmed.rfind(['/', '\\']) {
        Some(0) => trimmed[..1].to_string(),
        Some(index) => trimmed[..index].to_string(),
        None => ".".to_string(),
    }
}

/// Mirrors the `where` output normalization.
pub fn split_where_output(output: &str) -> Vec<String> {
    output
        .split('\n')
        .map(|line| line.strip_suffix('\r').unwrap_or(line).trim().to_string())
        .filter(|line| !line.is_empty())
        .collect()
}

/// Mirrors the `key` derivation: keep `[a-z0-9]`, lowercase, join.
pub fn app_key(app_name: &str) -> String {
    app_name
        .chars()
        .filter(|value| value.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

/// Mirrors the `suffix` path resolution of `resolveCmd`: the token after
/// `%~dp0`, `/` → `\`, drop empty and `.` parts, then collapse `..` against
/// the current path.
pub fn resolve_dp0_suffix(base: &str, suffix: &str) -> String {
    let normalized = suffix.replace('/', "\\");
    let mut current = base.to_string();
    for part in normalized.split('\\') {
        if part.is_empty() || part == "." {
            continue;
        }
        if part == ".." {
            current = dirname(&current);
            continue;
        }
        current = if current.ends_with('/') || current.ends_with('\\') {
            format!("{}{}", current, part)
        } else {
            format!("{}/{}", current, part)
        };
    }
    current
}

/// Mirrors the token scan inside `resolveCmd`: split on `"`, trim, and for
/// every token containing `.exe` try the `%~dp0` expansion first, then the
/// token itself. `probe` supplies the two `exists` calls.
pub fn resolve_cmd(path: &str, content: &str, probe: &dyn PathProbe) -> Option<String> {
    let base = dirname(path);
    for token in content.split('"').map(|value| value.trim()) {
        let lower = token.to_lowercase();
        if !lower.contains(".exe") {
            continue;
        }
        if let Some(index) = lower.find("%~dp0") {
            let resolved = resolve_dp0_suffix(&base, &token[index + 5..]);
            if probe.exists(&resolved) {
                return Some(resolved);
            }
        }
        if probe.exists(token) {
            return Some(token.to_string());
        }
    }
    None
}

/// Mirrors the parent-directory scan in `resolveWindowsAppPath`.
pub fn candidate_dirs(path: &str) -> Vec<String> {
    let one = dirname(path);
    let two = dirname(&one);
    let three = dirname(&two);
    vec![one, two, three]
}

/// Mirrors the `stem` → `name` derivation of the directory scan.
pub fn entry_name(entry: &str) -> String {
    let stem = if entry.len() >= 4 && entry[entry.len() - 4..].eq_ignore_ascii_case(".exe") {
        &entry[..entry.len() - 4]
    } else {
        entry
    };
    app_key(stem)
}

/// Mirrors the `name.includes(key) || key.includes(name)` match.
pub fn name_matches(name: &str, key: &str) -> bool {
    name.contains(key) || key.contains(name)
}

/// Mirrors `function resolveWindowsAppPath(appName)` given the `where`
/// output, the two filesystem probes, and the file readers.
pub fn resolve_windows_app_path(
    app_name: &str,
    paths: &[String],
    probe: &dyn PathProbe,
    read_file: &dyn Fn(&str) -> String,
    read_dir: &dyn Fn(&str) -> Vec<String>,
) -> Option<String> {
    if let Some(exe) = paths.iter().find(|path| has_ext(path, "exe")) {
        return Some(exe.clone());
    }
    for path in paths {
        if has_ext(path, "cmd") || has_ext(path, "bat") {
            if let Some(resolved) = resolve_cmd(path, &read_file(path), probe) {
                return Some(resolved);
            }
        }
        if extname(path).is_empty() {
            let cmd = format!("{}.cmd", path);
            if probe.exists(&cmd) {
                if let Some(resolved) = resolve_cmd(&cmd, &read_file(&cmd), probe) {
                    return Some(resolved);
                }
            }
            let bat = format!("{}.bat", path);
            if probe.exists(&bat) {
                if let Some(resolved) = resolve_cmd(&bat, &read_file(&bat), probe) {
                    return Some(resolved);
                }
            }
        }
    }

    let key = app_key(app_name);
    if !key.is_empty() {
        for path in paths {
            for dir in candidate_dirs(path) {
                for entry in read_dir(&dir) {
                    let candidate = join_path(&dir, &entry);
                    if !has_ext(&candidate, "exe") {
                        continue;
                    }
                    let name = entry_name(&entry);
                    if name_matches(&name, &key) {
                        return Some(candidate);
                    }
                }
            }
        }
    }

    paths.first().cloned()
}

/// Mirrors `node:path.join(dir, entry)` for the mixed separators the Windows
/// scan produces.
fn join_path(dir: &str, entry: &str) -> String {
    if dir.ends_with('/') || dir.ends_with('\\') {
        format!("{}{}", dir, entry)
    } else {
        format!("{}/{}", dir, entry)
    }
}

/// Mirrors `export function checkAppExists(appName)`.
pub fn check_app_exists(app_name: &str) -> bool {
    // PROVISIONAL(packages/desktop/src/main/apps.ts): the macOS branch needs
    // `access` and `execFile("which", …)`.
    let _ = app_name;
    unimplemented!("checkAppExists needs the Node fs/child_process runtimes")
}

/// Mirrors `export function resolveAppPath(appName)`.
pub fn resolve_app_path(app_name: &str) -> Option<String> {
    // PROVISIONAL(packages/desktop/src/main/apps.ts): the Windows branch
    // needs `execFile("where", …)`, `readFile` and `readdir`.
    let _ = app_name;
    unimplemented!("resolveAppPath needs the Node fs/child_process runtimes")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct Existing<'a>(&'a [&'a str]);

    impl PathProbe for Existing<'_> {
        fn exists(&self, path: &str) -> bool {
            self.0.contains(&path)
        }
    }

    #[test]
    fn windows_and_linux_always_report_the_app_as_present() {
        let probe = Existing(&[]);
        assert!(check_app_exists_platform(Platform::Win32, "Cursor", &probe));
        assert!(check_app_exists_platform(Platform::Other, "Cursor", &probe));
    }

    #[test]
    fn macos_probes_the_three_search_locations_in_order() {
        assert_eq!(
            macos_app_locations("Cursor", Some("/Users/luke")),
            vec![
                "/Applications/Cursor.app",
                "/System/Applications/Cursor.app",
                "/Users/luke/Applications/Cursor.app"
            ]
        );
        assert_eq!(macos_app_locations("Cursor", None).len(), 2);
        assert_eq!(
            macos_app_locations("Cursor", Some("")).len(),
            2,
            "an empty HOME is skipped"
        );

        let probe = Existing(&["/System/Applications/Cursor.app"]);
        assert!(check_app_exists_platform(
            Platform::Darwin,
            "Cursor",
            &probe
        ));
        assert!(!check_app_exists_platform(
            Platform::Darwin,
            "Cursor",
            &Existing(&[])
        ));
    }

    #[test]
    fn non_windows_resolution_is_the_name_itself() {
        let no_files = |_: &str| String::new();
        let no_dirs = |_: &str| Vec::new();
        let probe = Existing(&[]);
        assert_eq!(
            resolve_app_path_platform(
                Platform::Darwin,
                "Cursor",
                None,
                &probe,
                &no_files,
                &no_dirs
            ),
            Some("Cursor".to_string())
        );
        assert_eq!(
            resolve_app_path_platform(Platform::Other, "Cursor", None, &probe, &no_files, &no_dirs),
            Some("Cursor".to_string())
        );
        // Windows needs the `where` output; without it the source's `try`
        // returns null.
        assert_eq!(
            resolve_app_path_platform(Platform::Win32, "Cursor", None, &probe, &no_files, &no_dirs),
            None
        );
    }

    #[test]
    fn windows_resolution_prefers_an_exe_then_the_batch_target() {
        let no_files = |_: &str| String::new();
        let no_dirs = |_: &str| Vec::new();
        let probe = Existing(&["C:/tools/lib/x.exe"]);

        // An `.exe` in the `where` output wins outright.
        assert_eq!(
            resolve_windows_app_path(
                "Cursor",
                &["C:\\a\\b.exe".to_string(), "C:\\a\\c.cmd".to_string()],
                &probe,
                &no_files,
                &no_dirs
            ),
            Some("C:\\a\\b.exe".to_string())
        );

        // Otherwise the `.cmd` is expanded via `%~dp0`.
        assert_eq!(
            resolve_windows_app_path(
                "Cursor",
                &["C:\\tools\\run.cmd".to_string()],
                &probe,
                &|path: &str| "\"x.exe\" %~dp0\\lib\\x.exe\"".to_string(),
                &no_dirs
            ),
            Some("C:/tools/lib/x.exe".to_string())
        );

        // Otherwise the first `where` entry is the answer.
        assert_eq!(
            resolve_windows_app_path(
                "Cursor",
                &["C:\\a\\thing".to_string()],
                &Existing(&[]),
                &no_files,
                &no_dirs
            ),
            Some("C:\\a\\thing".to_string())
        );
    }

    #[test]
    fn windows_resolution_falls_back_to_the_directory_scan() {
        let probe = Existing(&[]);
        let no_files = |_: &str| String::new();
        let read_dirs = |dir: &str| {
            if dir == "C:\\Program Files" {
                vec!["Cursor.exe".to_string(), "notes.txt".to_string()]
            } else {
                Vec::new()
            }
        };
        assert_eq!(
            resolve_windows_app_path(
                "cursor",
                &["C:\\Program Files\\cursor\\bin".to_string()],
                &probe,
                &no_files,
                &read_dirs
            ),
            Some("C:/Program Files/Cursor.exe".to_string())
        );
    }

    #[test]
    fn splits_and_trims_the_where_output() {
        assert_eq!(
            split_where_output("  C:\\a\\b.exe \r\n\r\n C:\\a\\c.cmd \n"),
            vec!["C:\\a\\b.exe", "C:\\a\\c.cmd"]
        );
    }

    #[test]
    fn matches_extensions_case_insensitively() {
        assert!(has_ext("C:\\a\\b.EXE", "exe"));
        assert!(has_ext("C:\\a\\b.cmd", "cmd"));
        assert!(has_ext("C:\\a\\b.bat", "bat"));
        assert!(!has_ext("C:\\a\\b", "exe"));
        assert!(
            !has_ext(".gitignore", "exe"),
            "a leading dot is not an extension"
        );
    }

    #[test]
    fn derives_the_alphanumeric_key() {
        assert_eq!(app_key("Git Bash"), "gitbash");
        assert_eq!(app_key("VS-Code_1.2"), "vscode12");
        assert_eq!(app_key("!!!"), "");
    }

    #[test]
    fn expands_dp0_suffixes() {
        assert_eq!(resolve_dp0_suffix("C:\\a", "\\b.exe"), "C:/a/b.exe");
        assert_eq!(resolve_dp0_suffix("C:\\a", "\\.\\b.exe"), "C:/a/b.exe");
        assert_eq!(resolve_dp0_suffix("C:\\a", "\\..\\b.exe"), "C:/b.exe");
        assert_eq!(
            resolve_dp0_suffix("C:\\a\\b", "\\..\\..\\c.exe"),
            "C:/c.exe"
        );
    }

    #[test]
    fn resolves_a_batch_file_target() {
        let content = "\"C:\\tools\\x.exe\"";
        let probe = Existing(&["C:/tools/lib/x.exe"]);
        assert_eq!(
            resolve_cmd("C:\\tools\\run.cmd", content, &probe),
            Some("C:/tools/lib/x.exe".to_string())
        );
        // No `%~dp0`: the token itself is probed directly.
        let direct = Existing(&["D:/other/x.exe"]);
        assert_eq!(
            resolve_cmd("C:\\tools\\run.cmd", content, &direct),
            Some("D:/other/x.exe".to_string())
        );
        assert_eq!(
            resolve_cmd("C:\\tools\\run.cmd", content, &Existing(&[])),
            None
        );
    }

    #[test]
    fn scans_three_parent_directories() {
        assert_eq!(
            candidate_dirs("C:\\a\\b\\c.exe"),
            vec!["C:\\a\\b", "C:\\a", "C:"]
        );
    }

    #[test]
    fn matches_directory_entries_by_key() {
        assert_eq!(entry_name("Code.exe"), "code");
        assert_eq!(entry_name("code.EXE"), "code");
        assert!(name_matches("code", "code"));
        assert!(name_matches("vscode", "code"));
        assert!(!name_matches("editor", "code"));
    }
}
