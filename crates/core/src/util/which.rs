//! Rust port of `packages/core/src/util/which.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! Std-only PATH search; Global.Path.bin approximated via env/known bin dir.

use std::path::{Path, PathBuf};

pub const NOTHROW: bool = true;

fn global_bin() -> String {
    // Source Global.Path.bin is platform-managed; std approximation: cargo bin or /usr/local/bin
    std::env::var("OPENCODE_BIN_PATH").unwrap_or_else(|_| "/usr/local/bin".to_string())
}

fn path_delimiter() -> char {
    if cfg!(windows) {
        ';'
    } else {
        ':'
    }
}

fn is_executable(p: &Path) -> bool {
    if !p.is_file() {
        return false;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(meta) = std::fs::metadata(p) {
            return meta.permissions().mode() & 0o111 != 0;
        }
        false
    }
    #[cfg(not(unix))]
    {
        true
    }
}

fn pathexts(env_patsext: Option<&str>) -> Vec<String> {
    if cfg!(windows) {
        let raw = env_patsext
            .map(|s| s.to_string())
            .or_else(|| std::env::var("PATHEXT").ok())
            .or_else(|| std::env::var("PathExt").ok())
            .unwrap_or_else(|| ".EXE;.CMD;.BAT;.COM".to_string());
        raw.split(';')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    } else {
        vec![]
    }
}

/// Source: `which(cmd, env?)` — PATH/Path precedence, PATHEXT/PathExt, nothrow → null.
/// `env_path`/`env_path_ext` mirror the optional `env` param; when None, falls back
/// to `process.env` (std::env here).
pub fn which(cmd: &str, env_path: Option<&str>, env_path_ext: Option<&str>) -> Option<String> {
    if cmd.is_empty() {
        return None;
    }
    // Direct path case: if cmd contains slash, check directly
    if cmd.contains('/') || (cfg!(windows) && cmd.contains('\\')) {
        let p = Path::new(cmd);
        if is_executable(p) {
            return Some(p.to_string_lossy().to_string());
        }
        // Try PATHEXT variants on windows
        for ext in pathexts(env_path_ext) {
            let with_ext = format!("{}{}", cmd, ext);
            let p2 = Path::new(&with_ext);
            if is_executable(p2) {
                return Some(p2.to_string_lossy().to_string());
            }
        }
        return None;
    }
    let base = env_path
        .map(|s| s.to_string())
        .or_else(|| std::env::var("PATH").ok())
        .or_else(|| std::env::var("Path").ok())
        .unwrap_or_default();
    let bin = global_bin();
    let full = if base.is_empty() {
        bin
    } else {
        format!("{}{}{}", base, path_delimiter(), bin)
    };
    let pathext = pathexts(env_path_ext);
    for dir in full.split(path_delimiter()) {
        if dir.is_empty() {
            continue;
        }
        let candidate = PathBuf::from(dir).join(cmd);
        if is_executable(&candidate) {
            return Some(candidate.to_string_lossy().to_string());
        }
        if cfg!(windows) {
            for ext in &pathext {
                let with_ext = candidate.to_string_lossy().to_string() + ext;
                let p2 = Path::new(&with_ext);
                if is_executable(p2) {
                    return Some(p2.to_string_lossy().to_string());
                }
            }
        }
    }
    None
}
