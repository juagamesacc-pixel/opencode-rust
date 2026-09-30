// source: src/config/tui-migrate.ts — exports: migrateTuiConfig
// PROVISIONAL pending jsonc-parser + tui config + core flag/global:
// TUI_SCHEMA_URL, backup suffix, strip keys, tabSize 2, file-order rules verbatim.

/// source: TUI_SCHEMA_URL — verbatim.
pub const TUI_SCHEMA_URL: &str = "https://opencode.ai/tui.json";

/// source: backup suffix ".tui-migration.bak" — verbatim.
pub const BACKUP_SUFFIX: &str = ".tui-migration.bak";

/// source: stripped legacy keys — verbatim order.
pub const LEGACY_KEYS: &[&str] = &["theme", "keybinds", "tui"];

/// source: modify formatting — insertSpaces true, tabSize 2. Verbatim.
pub const TAB_SIZE: usize = 2;

/// source: opencodeFiles() order — global dir files, findUp rootFirst,
/// per-directory files, FLAG file; unique + exists filter. Verbatim.
pub fn opencode_file_order(
    global_files: Vec<String>,
    find_up: Vec<String>,
    dir_files: Vec<String>,
    flag_file: Option<String>,
) -> Vec<String> {
    let mut files = Vec::new();
    files.extend(global_files);
    files.extend(find_up);
    files.extend(dir_files);
    if let Some(f) = flag_file {
        files.push(f);
    }
    let mut seen = std::collections::HashSet::new();
    files
        .into_iter()
        .filter(|f| seen.insert(f.clone()))
        .collect()
}

/// source: payload $schema first, then theme/keybinds/tui-spread. Verbatim order.
pub const PAYLOAD_SCHEMA_KEY: &str = "$schema";
