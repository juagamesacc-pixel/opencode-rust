// source: src/config/paths.ts — exports: files, directories, fileInDirectory,
// ConfigPaths
// PROVISIONAL pending core flag/global/fs-util: target names
// (`{name}.jsonc`, `{name}.json`, ".opencode") + order + reversal verbatim.

/// source: projectFiles targets — verbatim.
pub fn project_file_targets(name: &str) -> [String; 2] {
    [format!("{}.jsonc", name), format!("{}.json", name)]
}

/// source: directories order — [Global.Path.config, ...project ups, ...home
/// ups, ...FLAG_DIR]; project block skipped when OPENCODE_DISABLE_PROJECT_CONFIG.
/// Verbatim composition rule.
pub fn directory_order(
    global_config: &str,
    project_ups: Option<Vec<String>>,
    home_ups: Vec<String>,
    flag_dir: Option<&str>,
) -> Vec<String> {
    let mut out = vec![global_config.to_string()];
    if let Some(ups) = project_ups {
        out.extend(ups);
    }
    out.extend(home_ups);
    if let Some(d) = flag_dir {
        out.push(d.to_string());
    }
    // source: unique() — verbatim dedupe, first wins.
    let mut seen = std::collections::HashSet::new();
    out.into_iter().filter(|x| seen.insert(x.clone())).collect()
}

/// source: OPENCODE_DISABLE_PROJECT_CONFIG flag — verbatim key.
pub const DISABLE_PROJECT_CONFIG_ENV: &str = "OPENCODE_DISABLE_PROJECT_CONFIG";
/// source: OPENCODE_CONFIG_DIR flag — verbatim key.
pub const CONFIG_DIR_ENV: &str = "OPENCODE_CONFIG_DIR";
/// source: ".opencode" target — verbatim.
pub const OPENCODE_DIR: &str = ".opencode";

/// source: fileInDirectory() — [join `.json`, join `.jsonc`], verbatim order.
pub fn file_in_directory(dir: &str, name: &str) -> [String; 2] {
    [
        format!("{}/{}.json", dir.trim_end_matches('/'), name),
        format!("{}/{}.jsonc", dir.trim_end_matches('/'), name),
    ]
}
