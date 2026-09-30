// source: src/temporary.ts — yargs CLI entry (scriptName "opencode",
// wrap 100, help/version aliases, print-logs/log-level/pure options +
// OPENCODE_PRINT_LOGS/OPENCODE_LOG_LEVEL middleware, TuiThreadCommand).
// PROVISIONAL: CLI runtime wiring as data (no clap dep in this crate).

/// source: scriptName — verbatim.
pub const SCRIPT_NAME: &str = "opencode";
/// source: wrap 100 — verbatim.
pub const WRAP: usize = 100;
/// source: help/version text + aliases — verbatim.
pub const HELP_DESCRIBE: &str = "show help";
pub const HELP_ALIAS: &str = "h";
pub const VERSION_DESCRIBE: &str = "show version number";
pub const VERSION_ALIAS: &str = "v";
/// source: options — verbatim names/describes/choices.
pub const OPT_PRINT_LOGS: &str = "print-logs";
pub const OPT_PRINT_LOGS_DESCRIBE: &str = "print logs to stderr";
pub const OPT_LOG_LEVEL: &str = "log-level";
pub const OPT_LOG_LEVEL_DESCRIBE: &str = "log level";
pub const LOG_CHOICES: &[&str] = &["DEBUG", "INFO", "WARN", "ERROR"];
pub const OPT_PURE: &str = "pure";
pub const OPT_PURE_DESCRIBE: &str = "run without external plugins";
/// source: middleware env keys — verbatim.
pub const ENV_PRINT_LOGS: &str = "OPENCODE_PRINT_LOGS";
pub const ENV_LOG_LEVEL: &str = "OPENCODE_LOG_LEVEL";
/// source: parserConfiguration populate-- — verbatim.
pub const POPULATE_DASHDASH: &str = "populate--";
