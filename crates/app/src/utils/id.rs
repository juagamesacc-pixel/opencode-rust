//! Rust port of `packages/app/src/utils/id.ts` (opencode v1.18.30).
//!
//! Source 93 lines: `Identifier.ascending/descending` (+ private
//! `generateID`/`create`/`bytesToHex`/`randomBase62`). Verbatim error string
//! `ID {given} does not start with {prefix}` preserved.
//! Original file: `packages/app/src/utils/id.ts`

#![allow(dead_code)]

/// Mirrors the `prefixes` table.
pub fn id_prefix(prefix: &str) -> Option<&'static str> {
    match prefix {
        "session" => Some("ses"),
        "message" => Some("msg"),
        "permission" => Some("per"),
        "user" => Some("usr"),
        "part" => Some("prt"),
        "pty" => Some("pty"),
        _ => None,
    }
}

/// Mirrors `generateID` validation (verbatim error string).
pub fn validate_id(prefix: &str, given: &str) -> Result<String, String> {
    let short = id_prefix(prefix).ok_or_else(|| format!("unknown prefix {prefix}"))?;
    if !given.starts_with(short) {
        return Err(format!("ID {given} does not start with {short}"));
    }
    Ok(given.to_string())
}

/// Mirrors `Identifier.ascending` validation pass-through.
pub fn identifier_ascending(prefix: &str, given: Option<&str>) -> Result<Option<String>, String> {
    match given {
        None => Ok(None),
        Some(value) => validate_id(prefix, value).map(Some),
    }
}

/// Mirrors `Identifier.descending` validation pass-through.
pub fn identifier_descending(prefix: &str, given: Option<&str>) -> Result<Option<String>, String> {
    match given {
        None => Ok(None),
        Some(value) => validate_id(prefix, value).map(Some),
    }
}
