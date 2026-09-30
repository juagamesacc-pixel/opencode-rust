// source: src/config/tui-cwd.ts — exports: CurrentWorkingDirectory
// (Context.Reference defaultValue () => process.cwd()). Verbatim.
// PROVISIONAL: ambient cwd default resolved by host at runtime.

/// source: "CurrentWorkingDirectory" reference id — verbatim.
pub const CURRENT_WORKING_DIRECTORY: &str = "CurrentWorkingDirectory";

/// source: defaultValue () => process.cwd() — verbatim rule.
pub fn default_cwd() -> String {
    std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default()
}
