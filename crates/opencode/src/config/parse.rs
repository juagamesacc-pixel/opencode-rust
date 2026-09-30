// source: src/config/parse.ts — exports: jsonc, schema, ConfigParse
// PROVISIONAL pending jsonc-parser + effect Schema + core v1/config/error:
// jsonc error-report layout (`printParseErrorCode at line L, column C` +
// `   Line L: ...` + caret at column+9, wrapped in --- JSONC Input/Errors/End
// markers) verbatim; schema() decode options verbatim.

/// source: jsonc error block markers — verbatim.
pub const JSONC_HEAD: &str = "\n--- JSONC Input ---\n";
pub const JSONC_MID: &str = "\n--- Errors ---\n";
pub const JSONC_TAIL: &str = "\n--- End ---";

/// source: single jsonc issue line — `{code} at line {line}, column {column}`
/// + optional `   Line {line}: {text}` + caret padded to column+9. Verbatim.
pub fn jsonc_issue(code: &str, line: usize, column: usize, problem_line: Option<&str>) -> String {
    let error = format!("{} at line {}, column {}", code, line, column);
    match problem_line {
        None => error,
        Some(text) => {
            let caret = format!("{}^", " ".repeat(column + 9));
            format!("{}\n   Line {}: {}\n{}", error, line, text, caret)
        }
    }
}

/// source: jsonc() wrapper — verbatim block assembly.
pub fn jsonc_error_block(text: &str, issues: &str) -> String {
    format!(
        "{}{}{}{}{}{}",
        JSONC_HEAD, text, JSONC_MID, issues, JSONC_TAIL, ""
    )
}

/// source: schema() decode options — verbatim.
pub const SCHEMA_ERRORS: &str = "all";
pub const SCHEMA_EXCESS: &str = "ignore";
pub const SCHEMA_ORDER: &str = "original";
