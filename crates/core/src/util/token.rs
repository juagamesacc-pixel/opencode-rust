// source: src/util/token.ts — exports: Token (estimate), estimate
//
// TS: `export * as Token from "./token"` (self-reexport) + `estimate`.
// `Math.round(input.length / 4)` over UTF-16 units; `Math.max(0, …)`.

pub const CHARS_PER_TOKEN: usize = 4;

/// source: `estimate(input)` — `Math.max(0, Math.round(len / 4))`.
pub fn estimate(input: &str) -> usize {
    let units = input.encode_utf16().count();
    let value = (units as f64 / CHARS_PER_TOKEN as f64).round() as i64;
    value.max(0) as usize
}
