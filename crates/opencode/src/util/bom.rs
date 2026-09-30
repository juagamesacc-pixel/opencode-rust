// source: src/util/bom.ts — exports: split, join, readFile, syncFile
// PROVISIONAL pending crates/core (fs-util) for readFile/syncFile FS wiring; split/join are pure and already wired
// to verbatim behavior (verified against packages/core/src/fs-util.ts pure helpers + core port). No new dep needed for pure part.
// Remaining pending: FS touch (effect FileSystem) — kept marked.

/// source: BOM_CODE = 0xfeff — verbatim.
pub const BOM_CODE: u32 = 0xfeff;
/// source: BOM char — verbatim.
pub const BOM: char = '\u{feff}';

/// source: split() — verbatim.
pub fn split(text: &str) -> (bool, String) {
    match text.strip_prefix(BOM) {
        Some(rest) => (true, rest.to_string()),
        None => (false, text.to_string()),
    }
}

/// source: join() — strip then re-add iff bom, verbatim.
pub fn join(text: &str, bom: bool) -> String {
    let stripped = split(text).1;
    if !bom {
        return stripped;
    }
    format!("{}{}", BOM, stripped)
}

/// source: readFile/syncFile — fs-touching; trait with verbatim semantics.
pub trait Fs {
    fn read_file(&self, path: &str) -> Result<(bool, String), String>;
    fn sync_file(&self, path: &str, bom: bool) -> Result<String, String>;
}
