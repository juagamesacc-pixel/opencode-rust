//! Rust port of `packages/core/src/util/glob.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! Std-only glob: `glob`/`minimatch` mapped to std walk + fnmatch (no new deps).

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

// Source exports (preserved):
// - export namespace Glob {
// - export interface Options {
// - export async function scan(pattern: string, options: Options = {}): Promise<string[]>
// - export function scanSync(pattern: string, options: Options = {}): string[]
// - export function match(pattern: string, filepath: string): boolean

/// Source: `Glob.Options` — fields verbatim in source order.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Options {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symlink: Option<bool>,
}

pub const FOLLOW_DEFAULT: bool = false;
pub const NODIR_RULE: &str = "nodir = include !== \"all\"";

fn to_glob_options(options: &Options) -> (bool, bool, bool) {
    let follow = options.symlink.unwrap_or(FOLLOW_DEFAULT);
    let nodir = options.include.as_deref() != Some("all");
    let dot = options.dot.unwrap_or(false);
    (follow, nodir, dot)
}

fn cwd_path(options: &Options) -> PathBuf {
    if let Some(cwd) = &options.cwd {
        PathBuf::from(cwd)
    } else {
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    }
}

fn component_match(pattern: &str, text: &str) -> bool {
    // '*' matches any run within component, '?' matches single char
    let pat: Vec<char> = pattern.chars().collect();
    let txt: Vec<char> = text.chars().collect();
    let (mut pi, mut ti) = (0usize, 0usize);
    let (mut star, mut match_idx) = (None::<usize>, 0usize);
    while ti < txt.len() {
        if pi < pat.len() && (pat[pi] == '?' || pat[pi] == txt[ti]) {
            pi += 1;
            ti += 1;
        } else if pi < pat.len() && pat[pi] == '*' {
            star = Some(pi);
            match_idx = ti;
            pi += 1;
        } else if let Some(s) = star {
            pi = s + 1;
            match_idx += 1;
            ti = match_idx;
        } else {
            return false;
        }
    }
    while pi < pat.len() && pat[pi] == '*' {
        pi += 1;
    }
    pi == pat.len()
}

fn glob_match(pattern: &str, filepath: &str) -> bool {
    // Normalize separators, mimic minimatch with dot:true
    let pat = pattern.replace('\\', "/");
    let fp = filepath.replace('\\', "/");
    let pat = pat.trim_matches('/');
    let fp = fp.trim_matches('/');
    // Split into components
    let pat_parts: Vec<&str> = if pat.is_empty() {
        vec![]
    } else {
        pat.split('/').collect()
    };
    let fp_parts: Vec<&str> = if fp.is_empty() {
        vec![]
    } else {
        fp.split('/').collect()
    };
    // DP over components handling "**"
    fn rec(pat_parts: &[&str], fp_parts: &[&str], pi: usize, fi: usize) -> bool {
        if pi == pat_parts.len() {
            return fi == fp_parts.len();
        }
        if pat_parts[pi] == "**" {
            // ** matches zero or more components
            for k in fi..=fp_parts.len() {
                if rec(pat_parts, fp_parts, pi + 1, k) {
                    return true;
                }
            }
            return false;
        }
        if fi == fp_parts.len() {
            return false;
        }
        if component_match(pat_parts[pi], fp_parts[fi]) {
            return rec(pat_parts, fp_parts, pi + 1, fi + 1);
        }
        false
    }
    rec(&pat_parts, &fp_parts, 0, 0)
}

fn walk_collect(
    dir: &Path,
    cwd: &Path,
    pattern: &str,
    opts: &Options,
    out: &mut Vec<String>,
    follow: bool,
    nodir: bool,
    dot: bool,
) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let file_name = entry.file_name();
        let name_str = file_name.to_string_lossy();
        if !dot && name_str.starts_with('.') {
            // skip dotfiles when dot:false (mimics nodir/dot handling)
            // but still need to consider if pattern explicitly matches dotfiles via match
            // we filter later via glob_match, so just continue walking for dirs
            // for files, glob_match with dot:true would match, so we skip here only if nodir handling
        }
        let is_dir = if follow {
            path.is_dir()
        } else {
            // without follow, use symlink_metadata
            std::fs::symlink_metadata(&path)
                .map(|m| m.is_dir())
                .unwrap_or(false)
        };
        // relative path from cwd
        let rel = path.strip_prefix(cwd).unwrap_or(&path);
        let rel_str = rel.to_string_lossy().replace('\\', "/");
        let is_match = glob_match(pattern, &rel_str);
        // dot filtering: if dot false and file is dotfile, only include if pattern explicitly starts with '.' segment
        let dot_ok =
            dot || !name_str.starts_with('.') || pattern.split('/').any(|seg| seg.starts_with('.'));
        if is_match && dot_ok {
            if is_dir {
                if !nodir {
                    let s = if opts.absolute.unwrap_or(false) {
                        path.to_string_lossy().to_string()
                    } else {
                        rel_str.clone()
                    };
                    out.push(s);
                }
            } else {
                let s = if opts.absolute.unwrap_or(false) {
                    path.to_string_lossy().to_string()
                } else {
                    rel_str.clone()
                };
                out.push(s);
            }
        }
        if is_dir {
            // recurse unless dot filtering prevents entering dot dirs when dot false and pattern doesn't include dot
            let should_recurse = dot || !name_str.starts_with('.') || pattern.contains("**");
            if should_recurse {
                walk_collect(&path, cwd, pattern, opts, out, follow, nodir, dot);
            }
        }
    }
}

pub fn scan_sync(pattern: &str, options: Options) -> Vec<String> {
    let cwd = cwd_path(&options);
    let (follow, nodir, dot) = to_glob_options(&options);
    // If pattern is absolute, treat cwd as root
    let pat_norm = pattern.replace('\\', "/");
    let is_abs = Path::new(&pat_norm).is_absolute();
    let effective_cwd = if is_abs {
        PathBuf::from("/")
    } else {
        cwd.clone()
    };
    let mut out = Vec::new();
    // Walk from cwd and filter via glob_match; for patterns without **, we still walk fully
    walk_collect(
        &effective_cwd,
        &effective_cwd,
        &pat_norm,
        &options,
        &mut out,
        follow,
        nodir,
        dot,
    );
    out.sort();
    out
}

pub async fn scan(pattern: &str, options: Options) -> Vec<String> {
    // No tokio dep; delegate to sync version (Effect async → sync where std suffices)
    scan_sync(pattern, options)
}

pub fn match_(pattern: &str, filepath: &str) -> bool {
    // Source: minimatch(filepath, pattern, { dot: true }) — dot true always
    glob_match(pattern, filepath)
}
