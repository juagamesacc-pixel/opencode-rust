//! Rust port of `src/renderer/cli.ts` (opencode v1.18.30).
//!
//! `installCli` needs `initI18n()`, `window.api.installCli()`, and
//! `window.alert` — all PROVISIONAL here. Preserved exactly: the success
//! path alerts `t("desktop.cli.installed.message", { path })`, the failure
//! path alerts `t("desktop.cli.failed.message", { error: String(e) })`.
//!
//! Original file: `packages/desktop/src/renderer/cli.ts`

pub const CLI_INSTALLED_MESSAGE_KEY: &str = "desktop.cli.installed.message";
pub const CLI_FAILED_MESSAGE_KEY: &str = "desktop.cli.failed.message";

// PROVISIONAL(packages/desktop/src/renderer/cli.ts): `await initI18n()`
// then `window.api.installCli()` with `window.alert` on both paths.
pub fn install_cli() {
    unimplemented!("window.api.installCli + window.alert binding")
}
