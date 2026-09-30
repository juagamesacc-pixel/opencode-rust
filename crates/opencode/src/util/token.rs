// source: src/util/token.ts — `export { Token, estimate } from
// "@opencode-ai/core/util/token"` — wired to crates/core/src/util/token.rs (1:1).
// Verified API: crates/core/src/util/token.rs :: estimate(&str) -> usize via encode_utf16 count /4 rounded.
// No path dep yet (workspace Cargo.toml missing crates/core) — keep marker pending manifest, behavior is verbatim copy.
// PROVISIONAL pending workspace Cargo.toml path dep on crates/core (approval requested) — behavior already matches core.

/// source: Token — mirrors core re-export shape (core has only estimate; Token is opaque projection).
#[derive(Debug, Clone)]
pub struct Token {
    pub text: String,
}

/// source: estimate — `Math.max(0, Math.round(len / 4))` over UTF-16 units — verbatim from packages/core/src/util/token.ts via core port.
/// Core impl: `let units = input.encode_utf16().count(); (units as f64 / 4.0).round() as i64 .max(0) as usize`
pub fn estimate(text: &str) -> usize {
    let units = text.encode_utf16().count();
    let value = (units as f64 / 4.0).round() as i64;
    value.max(0) as usize
}
