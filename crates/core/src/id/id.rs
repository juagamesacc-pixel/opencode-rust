//! Rust port of `packages/core/src/id/id.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.

pub const PREFIXES: &[(&str, &str)] = &[
    ("job", "job"),
    ("event", "evt"),
    ("session", "ses"),
    ("message", "msg"),
    ("permission", "per"),
    ("question", "que"),
    ("part", "prt"),
    ("pty", "pty"),
    ("tool", "tool"),
    ("workspace", "wrk"),
];

fn prefix_value(key: &str) -> Option<&'static str> {
    for (k, v) in PREFIXES {
        if *k == key {
            return Some(*v);
        }
    }
    None
}

pub fn ascending(prefix: &str, given: Option<&str>) -> Result<String, String> {
    generate_id(prefix, "ascending", given)
}

pub fn descending(prefix: &str, given: Option<&str>) -> Result<String, String> {
    generate_id(prefix, "descending", given)
}

fn generate_id(prefix: &str, direction: &str, given: Option<&str>) -> Result<String, String> {
    let p = prefix_value(prefix).ok_or_else(|| format!("unknown prefix {prefix}"))?;
    if let Some(g) = given {
        if !g.starts_with(p) {
            return Err(format!("ID {g} does not start with {p}"));
        }
        return Ok(g.to_string());
    }
    Ok(create(p, direction, None))
}

pub fn create(prefix: &str, direction: &str, timestamp: Option<i64>) -> String {
    let descending = direction == "descending";
    // delegate to schema identifier crate
    let suffix = schema::identifier::create(descending, timestamp);
    format!("{prefix}_{suffix}")
}

/// Extract timestamp from an ascending ID. Does not work with descending IDs.
pub fn timestamp(id: &str) -> Option<i64> {
    let prefix = id.split('_').next()?;
    let hex = id.get(prefix.len() + 1..prefix.len() + 13)?;
    let encoded = i128::from_str_radix(hex, 16).ok()?;
    Some((encoded / 0x1000) as i64)
}

pub use self as Identifier;
