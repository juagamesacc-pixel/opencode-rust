// source: src/util/module.ts — exports: Module.resolve
//
// PROVISIONAL pending node:module `createRequire(dir).resolve(id)` equivalent.
// Approximates Node's resolution: relative/absolute ids resolve within `dir`
// (trying the exact file, then appending `.js`/`.json`/`.node`, then
// `index.js`); bare package ids walk `node_modules` upward from `dir`. The TS
// swallows all errors and returns None (JS undefined) — mirrored.

use std::path::{Path, PathBuf};

const EXTENSIONS: &[&str] = &[".js", ".json", ".node"];

/// Mirrors require()'s exact-file-then-extension-then-index lookup.
fn resolve_file(base: &Path) -> Option<String> {
    if base.is_file() {
        return Some(base.to_string_lossy().into_owned());
    }
    for ext in EXTENSIONS {
        let mut name = base.as_os_str().to_owned();
        name.push(ext);
        let candidate = PathBuf::from(name);
        if candidate.is_file() {
            return Some(candidate.to_string_lossy().into_owned());
        }
    }
    let index = base.join("index.js");
    if index.is_file() {
        return Some(index.to_string_lossy().into_owned());
    }
    None
}

/// source: `Module.resolve(id, dir)` — returns resolved path or None.
pub fn resolve(id: &str, dir: &str) -> Option<String> {
    if id.starts_with('.') || id.starts_with('/') {
        let base = if id.starts_with('/') {
            PathBuf::from(id)
        } else {
            Path::new(dir).join(id)
        };
        return resolve_file(&base);
    }

    // Bare specifier: walk node_modules upward from dir.
    let mut cur = PathBuf::from(dir);
    loop {
        let candidate = cur.join("node_modules").join(id);
        if let Some(resolved) = resolve_file(&candidate) {
            return Some(resolved);
        }
        if !cur.pop() {
            break;
        }
    }
    None
}
