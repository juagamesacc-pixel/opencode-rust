//! Rust port of `src/main/store.ts` (opencode v1.18.30).
//!
//! PROVISIONAL (`electron-store` + `app.getPath("userData")` + `rmSync`).
//! The module-load comment about `app.setPath("userData", …)` ordering is
//! preserved verbatim below, and the store construction options are kept as
//! constants.
//!
//! Original file: `packages/desktop/src/main/store.ts`

use crate::main::store_keys::SETTINGS_STORE;

pub const STORE_FILE_EXTENSION: &str = "";
pub const STORE_DOT_NOTATION: bool = false;

// We cannot instantiate the electron-store at module load time because
// module import hoisting causes this to run before app.setPath("userData", ...)
// in index.ts has executed, which would result in files being written to the default directory
// (e.g. bad: %APPDATA%\@opencode-ai\desktop\opencode.settings vs good: %APPDATA%\ai.opencode.desktop.dev\opencode.settings).

// PROVISIONAL(packages/desktop/src/main/store.ts): `getStore(name =
// SETTINGS_STORE)` needs the cached `electron-store` instances rooted at
// `app.getPath("userData")` (`fileExtension: ""`,
// `accessPropertiesByDotNotation: false`).
pub fn default_store_name() -> &'static str {
    SETTINGS_STORE
}

pub fn get_store(_name: Option<&str>) {
    unimplemented!("electron-store binding")
}

// PROVISIONAL(packages/desktop/src/main/store.ts):
// `removeStoreFileIfEmpty(name)` needs `deleteStoreFileIfEmpty` over
// `app.getPath("userData")` plus cache eviction.
pub fn remove_store_file_if_empty(_name: &str) {
    unimplemented!("electron-store + fs binding")
}

// PROVISIONAL(packages/desktop/src/main/store.ts): `removeStoreFile(name)`
// needs `rmSync(join(userData, name), { force: true })` plus eviction.
pub fn remove_store_file(_name: &str) {
    unimplemented!("fs binding")
}
