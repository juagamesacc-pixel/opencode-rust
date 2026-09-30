// source: src/acp/profile.ts — exports: mark, duration, measure, ACPProfile
// (OPENCODE_ACP_PROFILE gate, `[acp-profile]` stderr line, finally-write verbatim).

/// source: OPENCODE_ACP_PROFILE === "1" gate — verbatim.
pub const PROFILE_ENV: &str = "OPENCODE_ACP_PROFILE";

/// source: enabled rule — verbatim.
pub fn profile_enabled(value: Option<&str>) -> bool {
    value == Some("1")
}

/// source: write() — `[acp-profile] ${name} ${round}ms${extra}` to stderr. Verbatim.
pub fn profile_line(name: &str, duration_ms: f64, fields: &[(&str, String)]) -> String {
    let extra = fields
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join(" ");
    if extra.is_empty() {
        format!("[acp-profile] {} {}ms", name, duration_ms.round() as i64)
    } else {
        format!(
            "[acp-profile] {} {}ms {}",
            name,
            duration_ms.round() as i64,
            extra
        )
    }
}

/// source: mark() suffix ".mark" — verbatim.
pub fn mark_name(name: &str) -> String {
    format!("{}.mark", name)
}
