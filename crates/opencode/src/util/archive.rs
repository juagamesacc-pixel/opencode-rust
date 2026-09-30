// source: src/util/archive.ts — exports: extractZip, Archive
// PROVISIONAL: process execution via @/util/process trait; platform rule verbatim.

/// source: win32 powershell command — verbatim template.
pub fn powershell_cmd(zip_path: &str, dest_dir: &str) -> String {
    format!("$global:ProgressPreference = 'SilentlyContinue'; Expand-Archive -Path '{}' -DestinationPath '{}' -Force", zip_path, dest_dir)
}

/// source: powershell argv — verbatim.
pub const POWERSHELL_ARGV: &[&str] = &["powershell", "-NoProfile", "-NonInteractive", "-Command"];

/// source: unzip argv — verbatim (["unzip", "-o", "-q", zip, "-d", dest]).
pub fn unzip_argv(zip_path: &str, dest_dir: &str) -> Vec<String> {
    vec![
        "unzip".into(),
        "-o".into(),
        "-q".into(),
        zip_path.into(),
        "-d".into(),
        dest_dir.into(),
    ]
}
