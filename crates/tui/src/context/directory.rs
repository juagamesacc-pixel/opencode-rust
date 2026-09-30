// source: packages/tui/src/context/directory.ts (17 lines, v1.18.30)
// 1:1 port — the memo becomes a pure function of its three inputs.

#![allow(dead_code)]

use crate::runtime::abbreviate_home;

/// Mirrors `useDirectory` — abbreviated instance directory plus `:branch`.
pub fn directory_label(
    instance_directory: &str,
    cwd: &str,
    home: &str,
    branch: Option<&str>,
) -> String {
    let base = if instance_directory.is_empty() {
        cwd
    } else {
        instance_directory
    };
    let result = abbreviate_home(base, home);
    match branch {
        Some(branch) => format!("{result}:{branch}"),
        None => result,
    }
}
