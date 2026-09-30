// source: src/lsp/diagnostic.ts — exports: pretty, report, Diagnostic
// (severity map, 1-based line/col, MAX_PER_FILE=20, errors-only filter,
// `... and N more` suffix, <diagnostics> envelope verbatim).

/// source: MAX_PER_FILE = 20 — verbatim.
pub const MAX_PER_FILE: usize = 20;

/// source: severity map — verbatim (default 1 → ERROR).
pub fn severity_name(severity: Option<u32>) -> &'static str {
    match severity.unwrap_or(1) {
        1 => "ERROR",
        2 => "WARN",
        3 => "INFO",
        4 => "HINT",
        _ => "ERROR",
    }
}

/// source: pretty() — `${SEV} [${line}:${col}] ${msg}` 1-based. Verbatim.
pub fn pretty(severity: Option<u32>, line0: u32, col0: u32, message: &str) -> String {
    format!(
        "{} [{}:{}] {}",
        severity_name(severity),
        line0 + 1,
        col0 + 1,
        message
    )
}

/// source: report() — errors (severity 1) only, empty → "", envelope. Verbatim.
pub fn report(file: &str, errors: &[String]) -> String {
    if errors.is_empty() {
        return String::new();
    }
    let limited: Vec<&String> = errors.iter().take(MAX_PER_FILE).collect();
    let more = errors.len().saturating_sub(MAX_PER_FILE);
    let suffix = if more > 0 {
        format!("\n... and {} more", more)
    } else {
        String::new()
    };
    format!(
        "<diagnostics file=\"{}\">\n{}{}\n</diagnostics>",
        file,
        limited
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join("\n"),
        suffix
    )
}
