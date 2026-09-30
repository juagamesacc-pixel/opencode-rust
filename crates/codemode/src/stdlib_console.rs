//! Port of `src/stdlib/console.ts`.

/// Console methods, verbatim. Mirrors `consoleMethods`.
pub const CONSOLE_METHODS: &[&str] = &["log", "info", "debug", "warn", "error", "dir", "table"];

/// Mirrors `consoleMethods.has(name)`.
pub fn is_console_method(name: &str) -> bool {
    CONSOLE_METHODS.contains(&name)
}

/// Console formatting recursion ceiling; deeper values render as `"..."`.
/// Mirrors `MAX_CONSOLE_DEPTH = 32`.
pub const MAX_CONSOLE_DEPTH: usize = 32;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn console_depth_verbatim() {
        assert_eq!(MAX_CONSOLE_DEPTH, 32);
        assert!(is_console_method("table"));
    }
}
