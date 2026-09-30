// source: packages/tui/src/runtime.tsx (9 lines, v1.18.30)
// 1:1 port — `path.relative` is implemented component-wise for absolute
// paths (same empty/parent/absolute guards, verbatim).

#![allow(dead_code)]

/// Minimal `path.relative` for absolute paths (mirrors node's result for
/// the inputs used here: `""`, `..`-prefixed, or a forward relative path).
pub fn relative_path(from: &str, to: &str) -> String {
    let from = from.replace('\\', "/");
    let to = to.replace('\\', "/");
    let mut from_parts: Vec<&str> = from.split('/').filter(|p| !p.is_empty()).collect();
    let mut to_parts: Vec<&str> = to.split('/').filter(|p| !p.is_empty()).collect();
    // Strip Windows drive letters for comparison parity (node keeps them).
    let from_drive = from_parts
        .first()
        .map(|p| p.ends_with(':'))
        .unwrap_or(false);
    let to_drive = to_parts.first().map(|p| p.ends_with(':')).unwrap_or(false);
    if from_drive || to_drive {
        if from_parts.first() != to_parts.first() {
            return to;
        }
        from_parts.remove(0);
        to_parts.remove(0);
    }
    let mut common = 0;
    while common < from_parts.len()
        && common < to_parts.len()
        && from_parts[common] == to_parts[common]
    {
        common += 1;
    }
    let mut out = vec![".."; from_parts.len() - common];
    out.extend_from_slice(&to_parts[common..]);
    out.join("/")
}

/// Mirrors `abbreviateHome`.
pub fn abbreviate_home(input: &str, home: &str) -> String {
    if home.is_empty() {
        return input.to_string();
    }
    let relative = relative_path(home, input);
    if relative.is_empty() {
        return "~".to_string();
    }
    if relative == ".." || relative.starts_with("../") || relative.starts_with('/') {
        return input.to_string();
    }
    format!("~/{}", relative)
}
