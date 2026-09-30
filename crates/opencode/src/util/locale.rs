// source: src/util/locale.ts — `export * from "@opencode-ai/tui/util/locale"` +
// `export { Locale }`.
// PROVISIONAL pending @opencode-ai/tui (crates/tui): locale surface mirrored.

/// source: Locale — PROVISIONAL pending tui/util/locale.
#[derive(Debug, Clone)]
pub struct Locale {
    pub tag: String,
}

impl Locale {
    pub fn new(tag: impl Into<String>) -> Self {
        Self { tag: tag.into() }
    }
}
