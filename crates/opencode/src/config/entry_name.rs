// source: src/config/entry-name.ts — exports: configEntryNameFromPath
// (stripPrefix anchored; see #25713 comment preserved in source).

/// source: configEntryNameFromPath — verbatim (backslash normalize, anchored
/// prefix strip else basename, strip extension).
pub fn config_entry_name_from_path(relative_path: &str, prefixes: &[&str]) -> String {
    let normalized = relative_path.replace('\\', "/");
    let mut candidate: Option<String> = None;
    for prefix in prefixes {
        if let Some(stripped) = normalized.strip_prefix(prefix) {
            candidate = Some(stripped.to_string());
            break;
        }
    }
    let candidate = candidate.unwrap_or_else(|| {
        normalized
            .rsplit('/')
            .next()
            .unwrap_or(&normalized)
            .to_string()
    });
    match candidate.rfind('.') {
        Some(i) if candidate[i..].len() > 1 && !candidate[i + 1..].contains('/') => {
            candidate[..i].to_string()
        }
        _ => candidate,
    }
}
